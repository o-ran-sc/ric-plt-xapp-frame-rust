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

use std::time::Duration;

use rmr::{RMRClient, RMRError, RMRMessageBuffer};
use xapp::XApp;

fn get_config_data() -> xapp::XAppConfig {
    let config_json = r#"{
        "messaging": {
            "ports" : [
                {
                    "name": "rmrdata",
                    "port": 4562
                },
                {
                    "name": "http",
                    "port": 8080
                }
            ]
        }
    }"#;

    xapp::XAppConfig {
        metadata: Box::new(xapp::ConfigMetadata {
            xapp_name: "int-tests".to_string(),
            config_type: "json".to_string(),
        }),
        config: serde_json::from_str(config_json).unwrap(),
    }
}

fn handle_pong_msg(msg: &mut RMRMessageBuffer, client: &RMRClient) -> Result<(), RMRError> {
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
                Err(RMRError)
            }
        }
        Err(_) => Err(RMRError),
    }
}

fn main() {
    println!("A very basic XApp that responds to Ping Messages");

    let mut xapp = XApp::from_config(get_config_data()).unwrap();

    xapp.register_handler(60000, handle_pong_msg);

    xapp.start();

    std::thread::sleep(Duration::from_secs(10000));

    xapp.stop();

    xapp.join();
}
