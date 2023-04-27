// ==================================================================================
//   Copyright (c) 2022 Caurus
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

//! Functionality related to handling incoming RMR Messages.
//!
//! We maintain a `HashMap` of Message Type -> Handler functions. A public API is provided to
//! register the handler functions for different message types. Receives data on an internal
//! 'channel' and processes the data (ie. calls either the handler if found or a default handler.

use std::collections::HashMap;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::Receiver;
use std::sync::{Arc, Mutex};
use std::thread::{self, JoinHandle};
use std::time::Duration;

use crate::{RMRClient, RMRError, RMRMessageBuffer};

pub type RMRProcessorFn =
    fn(msg: &mut RMRMessageBuffer, client: &RMRClient) -> Result<(), RMRError>;

fn default_processor_fn(msg: &mut RMRMessageBuffer, _client: &RMRClient) -> Result<(), RMRError> {
    log::debug!(
        "Default processor function called for MessageType: {}",
        msg.msgtype
    );
    Ok(())
}

/// RMRProcessor: Processing the received RMR Messages
///
/// `RMRProcessor` is responsible for processing the received RMR messages, that are sent on a
/// channel.
pub struct RMRProcessor {
    data_rx: Receiver<RMRMessageBuffer>,
    client: Arc<Mutex<RMRClient>>,
    is_running: Arc<AtomicBool>,
    handlers: HashMap<i32, RMRProcessorFn>,
    default: RMRProcessorFn,
}

impl RMRProcessor {
    pub fn new(
        data_rx: Receiver<RMRMessageBuffer>,
        client: Arc<Mutex<RMRClient>>,
        is_running: Arc<AtomicBool>,
    ) -> Self {
        Self {
            data_rx,
            client,
            is_running,
            handlers: HashMap::new(),
            default: default_processor_fn,
        }
    }

    pub fn register_processor(&mut self, msgtype: i32, func: RMRProcessorFn) {
        let _existing = self.handlers.insert(msgtype, func);
    }

    /// Start the `RMRProcessor` thread.
    ///
    /// Upon an error on the `data_rx` channel, returns from the thread.
    pub fn start(this: Arc<Mutex<Self>>) -> JoinHandle<()> {
        thread::spawn(move || loop {
            let processor = this.lock().expect("RMRProcessor Mutex Corrupted.");
            match processor.data_rx.recv_timeout(Duration::from_millis(1000)) {
                Ok(m) => processor.process_msg(m),
                Err(timeout) => {
                    log::trace!("timeoout in processor thread: {:?}", timeout);
                }
            }
            if !processor.is_running.load(Ordering::Relaxed) {
                break;
            }
            log::info!("Processor thread stopped!");
        })
    }

    // Right now it simply responds to the sender.
    // TODO: Implement it as a HashMap of MessageType and processor function and implement
    // processor functions.
    fn process_msg(&self, mut msg: RMRMessageBuffer) {
        let handler = self.handlers.get(&msg.msgtype).unwrap_or(&self.default);
        let client = self
            .client
            .lock()
            .expect("RMR Client Mutex Corrupted in RMRProcessor");
        let _ = handler(&mut msg, &client);
        msg.free();
    }
}
