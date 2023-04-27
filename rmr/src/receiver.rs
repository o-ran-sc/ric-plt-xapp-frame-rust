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

//! Functionality related to receiving RMR messages

use std::convert::TryInto;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::Sender;
use std::sync::{Arc, Mutex};
use std::thread::{self, JoinHandle};
use std::time::Duration;

use crate::{RMRClient, RMRError, RMRMessageBuffer};

/// RMRReceiver: Receives incoming RMR Messages and sends them on a channel to consume.
///
/// For a given `RMRClient` (Unique per port), `RMRReceiver` receives the channel. The main API of
/// `RMRReceiver` is `start`, which runs it's own thread. The 'running' of the thread is controlled
/// by a variable `is_running`, which can be shared with the calling 'controller' thread.
pub struct RMRReceiver {
    client: Arc<Mutex<RMRClient>>, // Mainly for using `RMRContext` right now.
    data_tx: Sender<RMRMessageBuffer>, // Received RMR messages will be sent to the channel.
    is_running: Arc<AtomicBool>,   // Required to 'signal' receiver thread to stop.
}

impl RMRReceiver {
    /// Create `RMRReceiver`.
    pub fn new(
        client: Arc<Mutex<RMRClient>>,
        data_tx: Sender<RMRMessageBuffer>,
        is_running: Arc<AtomicBool>,
    ) -> RMRReceiver {
        RMRReceiver {
            client,
            data_tx,
            is_running,
        }
    }

    /// Start the Receiver thread
    ///
    /// First waits till the unerlying RMR context is ready. After that registers our own receive
    /// 'fd' with an `epoll` and waits for incoming data through events signalled by the `epoll`.
    /// Once a valid `payload` is received, a new `RMRMessageBuffer` is created which is sent on a
    /// channel. The receiver of the channel will proces the message.
    pub fn start(this: Arc<Mutex<Self>>) -> JoinHandle<Result<(), RMRError>> {
        let mut _counter = 0;
        thread::spawn(move || {
            log::info!("Starting receiver thread!");
            let receiver = this.lock().expect("RMRReceiver Lock Corrupted.");
            //Wait for RMR to be Ready first
            loop {
                let client = receiver.client.lock().expect("RMR ContextMutex Corrupted");
                if client.is_ready() {
                    break;
                } else {
                    //TODO: Log
                    log::warn!("Waiting for RMR Client to be ready!");
                    _counter += 1;
                    thread::sleep(Duration::from_secs(1));
                }

                if !receiver.is_running.load(Ordering::Relaxed) {
                    log::error!("RMR Not Yet Ready, Receiverd stopped!");
                    return Err(RMRError);
                }
            }
            log::info!("RMR Client Ready!");
            // Setup the  Epoll poller for the recvfd.
            let epoll_fd = epoll::create(false).expect("Epoll Create Failed!");

            let client = receiver
                .client
                .lock()
                .expect("RMR Context Mutex Corrupted.");
            let rmr_fd = client
                .get_recv_fd()
                .expect("RMR Context Get Receive FD failed.");
            drop(client);

            let event = epoll::Event::new(epoll::Events::EPOLLIN, rmr_fd.try_into().unwrap());
            epoll::ctl(
                epoll_fd,
                epoll::ControlOptions::EPOLL_CTL_ADD,
                rmr_fd,
                event,
            )
            .expect("Epoll ctl failed");

            loop {
                if !receiver.is_running.load(Ordering::Relaxed) {
                    break;
                }
                let mut events = [epoll::Event::new(epoll::Events::empty(), 0); 1];
                let result = epoll::wait(epoll_fd, 1000, &mut events).expect("Epoll Wait Failed");
                if result == 0 {
                    continue;
                }

                let client = receiver
                    .client
                    .lock()
                    .expect("RMR Context Mutex corrupted.");
                let recv_mbuf = client.alloc_msg().expect("RMR Alloc Message Failed.");
                let recv_mbuf = client.rcv_msg(recv_mbuf).expect("RMR Recv Message failed.");
                // We don't need the client anymore - let someone else get it if they want it.
                drop(client);

                let msg_buffer = RMRMessageBuffer::new(recv_mbuf);
                log::debug!(
                    "state: {}, length: {}, payload_size: {}",
                    msg_buffer.get_state(),
                    msg_buffer.get_length(),
                    msg_buffer.get_payload_size()
                );
                let _ = receiver.data_tx.send(RMRMessageBuffer::new(recv_mbuf));
            }
            log::info!("Receiver thread stopped!");
            Ok(())
        })
    }
}
