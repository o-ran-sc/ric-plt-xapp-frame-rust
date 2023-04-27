// ==================================================================================
//   Copyright (c) 2022 Caurus
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

//! ORAN-SC xApp crate for Rust framework

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{mpsc, Arc, Mutex};
use std::thread::JoinHandle;

use rmr::{RMRClient, RMRError, RMRProcessor, RMRProcessorFn, RMRReceiver};

use sdl::RedisStorage;

#[derive(Debug)]
pub struct XAppError(String);

impl std::fmt::Display for XAppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

pub struct XApp {
    receiver: Arc<Mutex<RMRReceiver>>,
    receiver_thread: Option<JoinHandle<Result<(), RMRError>>>,

    processor: Arc<Mutex<RMRProcessor>>,
    processor_thread: Option<JoinHandle<()>>,

    _sdl_client: Arc<Mutex<RedisStorage>>,

    app_is_running: Arc<AtomicBool>,
}

impl XApp {
    pub fn new(rmr_port: &str, rmr_flags: u32) -> Result<Self, XAppError> {
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
        Ok(Self {
            receiver: Arc::new(Mutex::new(receiver)),
            processor: Arc::new(Mutex::new(processor)),
            _sdl_client: Arc::new(Mutex::new(sdl_client)),
            receiver_thread: None,
            processor_thread: None,
            app_is_running,
        })
    }

    pub fn register_handler(&self, msgtype: i32, handler: RMRProcessorFn) {
        let mut processor = self
            .processor
            .lock()
            .expect("RMRProcessor Mutex in XApp corrupted");
        (*processor).register_processor(msgtype, handler);
        log::debug!("Handler registered for message type: {}", msgtype);
    }

    pub fn start(&mut self) {
        // Mark: App is running to be true.
        self.app_is_running.store(true, Ordering::Relaxed);

        let receiver_thread = RMRReceiver::start(Arc::clone(&self.receiver));
        self.receiver_thread = Some(receiver_thread);

        let processor_thread = RMRProcessor::start(Arc::clone(&self.processor));
        self.processor_thread = Some(processor_thread);
        log::info!("xapp started!");
    }

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
            log::debug!("Waiting for Receiver thread to join!");
            let _ = processor_thread.unwrap().join();
            log::debug!("Receiver thread joined!");
        }
    }

    pub fn stop(&self) {
        log::info!("Stopping XApp!");

        self.app_is_running.store(false, Ordering::Relaxed);
    }
}

impl From<XAppError> for std::io::Error {
    fn from(x: XAppError) -> Self {
        std::io::Error::new(std::io::ErrorKind::Other, format!("{}", x))
    }
}

impl From<RMRError> for XAppError {
    fn from(_r: RMRError) -> Self {
        XAppError("RMRError".to_string())
    }
}
