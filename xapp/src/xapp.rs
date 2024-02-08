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

use std::convert::TryInto;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::{channel as std_sync_channel, Sender as StdSender};
use std::sync::{Arc, Mutex};
use std::thread::JoinHandle;

use tokio::sync::mpsc::{channel as sync_channel, Sender};

use rmr::{RMRClient, RMRError, RMRProcessor, RMRProcessorFn, RMRReceiver};

pub use registration_api::models::{ConfigMetadata, XAppConfig};
use rnib::{entities::NbIdentity, RnibApi};
use sdl::RedisStorage;

use crate::XAppError;

use self::alarms::client::AlarmClient;
use self::metrics::MetricsRegistry;

// XApp modules
pub(crate) mod alarms;
pub(crate) mod metrics;

pub(crate) mod registration;
pub(crate) mod subscription;

pub(crate) mod webserver;

pub(crate) const DEFAULT_XAPP_NS: &str = "ricxapp";

/// The main XApp structure
///
/// An application using this structure, should create an instance of this structure and use this
/// instance during the application. This is a wrapper structure over underlying RMR, SDL and RNIB
/// APIs of the RIC platform.
pub struct XApp<T> {
    // App Configuration
    config: XAppConfig,

    // Thread for receiving RMR Messages
    receiver: Arc<Mutex<RMRReceiver>>,
    receiver_thread: Option<JoinHandle<Result<(), RMRError>>>,

    // A Thread for processing Received RMR Messages
    processor: Arc<Mutex<RMRProcessor<T>>>,
    processor_thread: Option<JoinHandle<()>>,

    // Client communicating with SDL
    sdl_client: Arc<Mutex<RedisStorage>>,

    // App housekeeping data
    app_is_running: Arc<AtomicBool>,
    app_is_registered: Arc<AtomicBool>,
    app_name: Option<String>,
    app_instance_name: Option<String>,

    // Client for communicating with Alarm Manager
    alarm_client: Mutex<AlarmClient>,

    // Metrics support for the XApp
    metrics: Option<Arc<Mutex<MetricsRegistry>>>,
    metrics_thread: Option<JoinHandle<Result<(), XAppError>>>,

    // Web Server for serving health, metrics etc.
    webserver_thread: Option<JoinHandle<Result<(), XAppError>>>,
    _ws_data_tx: Option<Sender<String>>,
}

impl<T: Send + 'static> XApp<T> {
    /// Create a new XApp struct.
    ///
    /// Deprecated! Use `from_config` API instead.
    ///
    /// This is the main structure for the SDK. All Xapp actions will typically be performed with a
    /// handle to this structure.
    #[deprecated(since = "0.3.0-dev", note = "please use `from_config` instead.")]
    pub fn new(
        rmr_port: &str,
        rmr_flags: u32,
        config: XAppConfig,
        sender: StdSender<T>,
    ) -> Result<Self, XAppError> {
        let client = RMRClient::new(rmr_port, RMRClient::RMR_MAX_RCV_BYTES, rmr_flags)?;
        let receiver_client = Arc::new(Mutex::new(client));
        let processor_client = Arc::clone(&receiver_client);

        let app_is_running = Arc::new(AtomicBool::new(false));
        let receiver_running = Arc::clone(&app_is_running);
        let processor_running = Arc::clone(&app_is_running);

        let (data_tx, data_rx) = std_sync_channel();
        let receiver = RMRReceiver::new(receiver_client, data_tx, receiver_running);
        let processor = RMRProcessor::new(data_rx, processor_client, processor_running, sender);

        // Uses `DBAAS_SERVICE_HOST` and `DBAAS_SERVICE_PORT` env variables setup.
        let sdl_client = RedisStorage::new_from_env().map_err(|e| XAppError(e.to_string()))?;

        let app_is_registered = Arc::new(AtomicBool::new(false));

        Ok(Self {
            config,

            receiver: Arc::new(Mutex::new(receiver)),
            receiver_thread: None,

            processor: Arc::new(Mutex::new(processor)),
            processor_thread: None,

            sdl_client: Arc::new(Mutex::new(sdl_client)),

            app_is_running,
            app_is_registered,
            app_name: None,
            app_instance_name: None,

            alarm_client: Mutex::new(AlarmClient::new()),

            metrics: None,
            metrics_thread: None,

            webserver_thread: None,
            _ws_data_tx: None,
        })
    }

    /// Create a new XApp struct using the given `XappConfig`
    ///
    pub fn from_config(config: XAppConfig, app_tx: StdSender<T>) -> Result<Self, XAppError> {
        // We Validate HTTP port number right here, so that we don't have to rely on timing of
        // webserver starting to eror then.
        // TODO: Proper `config` validation.
        let _http_port_num = Self::port_from_config(&config, "http")?;
        let rmr_port_num = Self::port_from_config(&config, "rmrdata")?;
        let port_num_str = format!("{}", rmr_port_num);

        let metrics = metrics::registry_for_ns_app(DEFAULT_XAPP_NS, &config.metadata.xapp_name)?;

        #[allow(deprecated)]
        let mut xapp = Self::new(&port_num_str, RMRClient::RMRFL_NONE, config, app_tx)?;

        let metrics = Arc::new(Mutex::new(metrics));

        let _ = xapp.metrics.replace(metrics);

        Ok(xapp)
    }

    /// Register an RMR Message handler function.
    ///
    /// The registered function will perform all RMR processing.
    pub fn register_handler(&self, msgtype: i32, handler: RMRProcessorFn<T>) {
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

        let (ws_data_tx, ws_data_rx) = sync_channel::<String>(2);

        if let Some(ref metrics) = self.metrics {
            let app_is_running = Arc::clone(&self.app_is_running);
            let metrics = Arc::clone(metrics);
            let metrics_thread = std::thread::spawn(move || {
                metrics::run_metrics_server(metrics, ws_data_tx.clone(), app_is_running)
            });
            // TODO: Make sure it is None.
            self.metrics_thread = Some(metrics_thread);
        }

        let config = self.config.clone();
        let webserver_thread =
            std::thread::spawn(move || webserver::run_ready_live_server(config, ws_data_rx));
        self.webserver_thread = Some(webserver_thread);

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

        // TODO: How to stop webserver thread?
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

    pub(crate) fn port_from_config(config: &XAppConfig, service: &str) -> Result<u16, XAppError> {
        let mut port_num = -1;
        let ports = &config.config["messaging"]["ports"];
        if !ports.is_null() {
            if let Some(ports) = ports.as_array() {
                for port in ports {
                    let rmr_port = &port["name"];
                    if rmr_port.as_str() == Some(service) {
                        port_num = port["port"].as_i64().unwrap().try_into().unwrap();
                    }
                }
            }
        }
        port_num
            .try_into()
            .map_err(|_| XAppError("Invalid Port number in the config.".to_string()))
    }
}

#[cfg(test)]
mod tests {

    pub(crate) fn get_config_data(rmr_port_num: u16) -> crate::XAppConfig {
        let config_json = format!(
            r#"{{
        "messaging": {{
            "ports" : [
                {{
                    "name": "rmrdata",
                    "port": {rmr_port_num}
                }},
                {{
                    "name": "http",
                    "port": 8080
                }}
            ]
        }}
        }}"#
        );

        crate::XAppConfig {
            metadata: Box::new(crate::ConfigMetadata {
                xapp_name: "tests".to_string(),
                config_type: "json".to_string(),
            }),
            config: serde_json::from_str(&config_json).unwrap(),
        }
    }

    #[test]
    fn test_no_two_xapp_instances() {
        let (app_tx, _) = std::sync::mpsc::channel::<()>();
        let xapp_1 = crate::XApp::from_config(get_config_data(4560_u16), app_tx.clone());
        assert!(xapp_1.is_ok());
        let xapp_1 = xapp_1.unwrap();
        assert!(xapp_1.metrics.is_some());

        #[allow(deprecated)]
        let xapp_2 = crate::XApp::new("2345", 0, get_config_data(4560), app_tx);
        assert!(xapp_2.is_err());
    }

    #[test]
    fn test_config_validate_http_rmrdata() {
        let config_jsons = [
            r#"{
        "messaging": {
            "ports" : [
                {
                    "name": "rmrdata",
                    "port": 4444
                }
            ]
        }
        }"#,
            r#"{
        "messaging": {
            "ports" : [
                {
                    "name": "http",
                    "port": 4444
                }
            ]
        }
        }"#,
        ];

        let (app_tx, _) = std::sync::mpsc::channel::<()>();

        for config_json in config_jsons {
            let config = crate::XAppConfig {
                metadata: Box::new(crate::ConfigMetadata {
                    xapp_name: "tests".to_string(),
                    config_type: "json".to_string(),
                }),
                config: serde_json::from_str(config_json).unwrap(),
            };

            let xapp_1 = crate::XApp::from_config(config, app_tx.clone());
            assert!(xapp_1.is_err());
        }
    }
}
