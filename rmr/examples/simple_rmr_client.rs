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

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

use rmr;

fn handle_pong_msg(
    msg: &mut rmr::RMRMessageBuffer,
    client: &rmr::RMRClient,
    _sender: mpsc::Sender<()>,
) -> Result<(), rmr::RMRError> {
    match serde_json::from_slice::<serde_json::map::Map<_, _>>(msg.get_payload()) {
        Ok(mut m) => {
            if m.contains_key("test_send") {
                let v = m.remove("test_send").unwrap();
                m.insert("ACK".to_string(), v);
                let m = serde_json::to_string(&m).unwrap(); // OK to unwrap directly
                eprintln!("{}", m);
                let _ = msg.set_payload(m.as_bytes());
                let _ = msg.set_mtype(60001);

                client.rts_msg(msg).expect("Send to Sender Failed.");
                Ok(())
            } else {
                Err(rmr::RMRError)
            }
        }
        Err(_) => Err(rmr::RMRError),
    }
}

fn main() -> Result<(), std::io::Error> {
    println!("simple client!");

    let (data_tx, data_rx) = mpsc::channel();

    let (app_tx, _app_rx) = mpsc::channel();

    let receiver_client = Arc::new(Mutex::new(rmr::RMRClient::new("4562", 0, 0)?));
    let processor_client = Arc::clone(&receiver_client);

    let is_running = Arc::new(AtomicBool::new(true));

    let receiver = rmr::RMRReceiver::new(receiver_client, data_tx, Arc::clone(&is_running));
    let receiver = Arc::new(Mutex::new(receiver));

    let receiver_thread = rmr::RMRReceiver::start(receiver);

    let mut processor =
        rmr::RMRProcessor::new(data_rx, processor_client, Arc::clone(&is_running), app_tx);

    processor.register_processor(60000, handle_pong_msg);
    let processor_thread = rmr::RMRProcessor::start(Arc::new(Mutex::new(processor)));

    // Just wait for a few seconds in main thread..
    thread::sleep(Duration::from_secs(25));

    // This should stop the `receiver_thread` and should also stop the `processor_thread` after an
    // error on the `recv`.
    is_running.store(false, Ordering::Relaxed);

    // Reuse the client - to receive data on the data_rx and do rmr_rts
    let receiver_result = receiver_thread.join();
    // TODO: Make sure how to handle processor result as well
    let _ = processor_thread.join();

    receiver_result.unwrap()?;

    Ok(())
}
