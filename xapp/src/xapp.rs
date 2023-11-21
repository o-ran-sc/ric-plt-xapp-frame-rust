// ==================================================================================
//   Copyright (c) 2023 Abhijit Gadgil
//
//   Licensed under the Apache License, Version 2.0 (the "License");
//   you may not use this file except in compliance with the License.
//   You may obtain a copy of the License at
//
//       http://www.apache.org/licenses/LICENSE-2.0
//
//   Unless required by applicable law or agreed to in writing, software
//   distributed under the License is distributed on an "AS IS" BASIS,
//   WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
//   See the License for the specific language governing permissions and
//   limitations under the License.
// ==================================================================================

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{mpsc, Arc, Mutex};
use std::thread::JoinHandle;

use rmr::{RMRClient, RMRError, RMRProcessor, RMRProcessorFn, RMRReceiver};

use rnib::{entities::NbIdentity, RnibApi};
use sdl::RedisStorage;

use crate::XAppError;

static XAPP_FRAME_INITIALIZED: AtomicBool = AtomicBool::new(false);

/// The main XApp structure
///
/// An application using this structure, should create an instance of this structure and use this
/// instance during the application. This is a wrapper structure over underlying RMR, SDL and RNIB
/// APIs of the RIC platform.
pub struct XApp {
    receiver: Arc<Mutex<RMRReceiver>>,
    receiver_thread: Option<JoinHandle<Result<(), RMRError>>>,

    processor: Arc<Mutex<RMRProcessor>>,
    processor_thread: Option<JoinHandle<()>>,

    sdl_client: Arc<Mutex<RedisStorage>>,

    app_is_running: Arc<AtomicBool>,

    app_is_registered: Arc<AtomicBool>,
    app_name: Option<String>,
    app_instance_name: Option<String>,
}

impl XApp {
    /// Create a new XApp struct.
    ///
    /// This is the main structure for the SDK. All Xapp actions will typically be performed with a
    /// handle to this structure.
    pub fn new(rmr_port: &str, rmr_flags: u32) -> Result<Self, XAppError> {
        let initialized = XAPP_FRAME_INITIALIZED.load(Ordering::SeqCst);

        if initialized {
            Err(XAppError("XApp already initialized".to_string()))
        } else {
            let client = RMRClient::new(rmr_port, RMRClient::RMR_MAX_RCV_BYTES, rmr_flags)?;
            let receiver_client = Arc::new(Mutex::new(client));
            let processor_client = Arc::clone(&receiver_client);

            let app_is_running = Arc::new(AtomicBool::new(false));
            let receiver_running = Arc::clone(&app_is_running);
            let processor_running = Arc::clone(&app_is_running);

            let (data_tx, data_rx) = mpsc::channel();
            let receiver = RMRReceiver::new(receiver_client, data_tx, receiver_running);
            let processor = RMRProcessor::new(data_rx, processor_client, processor_running);

            // Uses `DBAAS_SERVICE_HOST` and `DBAAS_SERVICE_PORT` env variables setup.
            let sdl_client = RedisStorage::new_from_env().map_err(|e| XAppError(e.to_string()))?;

            let app_is_registered = Arc::new(AtomicBool::new(false));
            XAPP_FRAME_INITIALIZED.store(true, Ordering::SeqCst);

            Ok(Self {
                receiver: Arc::new(Mutex::new(receiver)),
                processor: Arc::new(Mutex::new(processor)),
                sdl_client: Arc::new(Mutex::new(sdl_client)),
                receiver_thread: None,
                processor_thread: None,
                app_is_running,
                app_is_registered,
                app_name: None,
                app_instance_name: None,
            })
        }
    }

    /// Register an RMR Message handler function.
    ///
    /// The registered function will perform all RMR processing.
    pub fn register_handler(&self, msgtype: i32, handler: RMRProcessorFn) {
        let mut processor = self
            .processor
            .lock()
            .expect("RMRProcessor Mutex in XApp corrupted");
        (*processor).register_processor(msgtype, handler);
        log::debug!("Handler registered for message type: {}", msgtype);
    }

    /// Start the application
    ///
    /// Starts the RMR receiver and processor threads for the application. An xApp should call this
    /// function to start running the application, after registering any RMR message handlers.
    ///
    /// ```ignore
    /// fn rmr_message_logger_handler(...) {
    /// ...
    /// }
    ///
    /// ...
    /// let mut xapp = Xapp::new(...);
    ///
    /// xapp.register_hanlder(10000, rmr_message_logger_handler);
    ///
    /// xapp.start();
    /// ...
    /// ```
    pub fn start(&mut self) {
        // Mark: App is running to be true.
        self.app_is_running.store(true, Ordering::Relaxed);

        let receiver_thread = RMRReceiver::start(Arc::clone(&self.receiver));
        self.receiver_thread = Some(receiver_thread);

        let processor_thread = RMRProcessor::start(Arc::clone(&self.processor));
        self.processor_thread = Some(processor_thread);
        log::info!("xapp started!");
    }

    /// Join the application threads.
    pub fn join(&mut self) {
        // Make sure that both the threads are stopped.
        //
        if self.receiver_thread.is_some() {
            let receiver_thread = self.receiver_thread.take();
            log::debug!("Waiting for Receiver thread to join!");
            let _ = receiver_thread.unwrap().join();
            log::debug!("Receiver thread joined!");
        }

        if self.processor_thread.is_some() {
            let processor_thread = self.processor_thread.take();
            log::debug!("Waiting for Processor thread to join!");
            let _ = processor_thread.unwrap().join();
            log::debug!("Processor thread joined!");
        }
    }

    /// Check if RMR is ready!
    ///
    /// An application should use this function to wait for the RMR to be ready before going ahead
    /// with other application
    ///
    /// ```ignore
    /// ...
    /// let xapp = Xapp::new(...);
    ///
    /// loop {
    ///     if xapp.is_rmr_ready() {
    ///         break;
    ///     }
    /// }
    ///
    /// // Do Ready processing
    ///```
    pub fn is_rmr_ready(&self) -> bool {
        let receiver = self.receiver.clone();
        RMRReceiver::is_ready(receiver)
    }

    /// Stop the XApp
    pub fn stop(&mut self) {
        log::info!("Stopping XApp!");

        let registered = self.app_is_registered.load(Ordering::SeqCst);
        if registered {
            let _ = self.deregister_xapp();
        }
        self.app_is_running.store(false, Ordering::Relaxed);
    }

    /// Get Nodeb IDs using RNIB API
    pub fn rnib_get_nodeb_ids(&self) -> Result<Vec<NbIdentity>, XAppError> {
        let mut client = self.sdl_client.lock().expect(" SDL Client Lock currupted!");
        client.get_nodeb_ids().map_err(|e| e.into())
    }
}

mod registration;
mod subscription;

#[cfg(test)]
mod tests {

    #[test]
    fn test_no_two_xapp_instances() {
        let xapp_1 = crate::XApp::new("1234", 0);
        assert!(xapp_1.is_ok());

        let xapp_2 = crate::XApp::new("2345", 0);
        assert!(xapp_2.is_err());
    }
}
