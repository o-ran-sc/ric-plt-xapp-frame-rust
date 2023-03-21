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

use std::env;
use std::process::Child;
use std::sync::{
    atomic::{AtomicU16, AtomicU8, Ordering},
    Mutex,
};
use std::thread;

use sdl::SdlStorageApi;

static TEST_REDIS_SERVER_STARTED: AtomicU8 = AtomicU8::new(0_u8);
const TEST_REDIS_SERVER_PORT: AtomicU16 = AtomicU16::new(63790_u16);
static CHILD_PID: Mutex<Option<Child>> = Mutex::new(None);

fn start_test_redis_server() {
    if TEST_REDIS_SERVER_STARTED.fetch_add(1_u8, Ordering::Relaxed) == 0 {
        let port = TEST_REDIS_SERVER_PORT.fetch_add(1, Ordering::Relaxed);

        let mut global_child = CHILD_PID.lock().unwrap();
        let child = std::process::Command::new("/usr/bin/redis-server")
            .arg("--port")
            .arg(format!("{}", port))
            .arg("--save")
            .arg("\"\"")
            .arg("--appendonly")
            .arg("no")
            .spawn()
            .expect("Failed to spawn process");

        global_child.replace(child);

        // Let the server start. Kinda: ugly but fine
        thread::sleep(std::time::Duration::from_secs(2));

        env::set_var("DBAAS_SERVICE_HOST", "localhost");
        env::set_var("DBAAS_SERVICE_PORT", format!("{}", port));
    } else {
        let global_child = CHILD_PID.lock().unwrap();
        if global_child.is_none() {
            thread::sleep(std::time::Duration::from_secs(2));
        }
        let port = TEST_REDIS_SERVER_PORT.load(Ordering::Relaxed);
        env::set_var("DBAAS_SERVICE_HOST", "localhost");
        env::set_var("DBAAS_SERVICE_PORT", format!("{}", port));
    }
}

fn stop_test_redis_server() {
    if TEST_REDIS_SERVER_STARTED.fetch_sub(1_u8, Ordering::Relaxed) == 1 {
        let mut global_child = CHILD_PID.lock().unwrap();
        let mut child = global_child.take().unwrap();
        child.kill().expect("command wasn't running.");
    }
}

#[test]
fn sdl_client_is_ready() {
    start_test_redis_server();

    let sdl_client = sdl::RedisStorage::new_from_env();
    assert!(sdl_client.is_ok(), "{:#?}", sdl_client.err().unwrap());
    let mut sdl_client = sdl_client.unwrap();

    assert!(sdl_client.is_ready("ready"));

    stop_test_redis_server();
}

#[test]
fn sdl_client_can_set_get() {
    start_test_redis_server();

    let sdl_client = sdl::RedisStorage::new_from_env();
    assert!(sdl_client.is_ok(), "{:#?}", sdl_client.err());

    let mut sdl_client = sdl_client.unwrap();
    let data_map = sdl::DataMap::from([("hello".to_string(), "world".as_bytes().to_vec()); 1]);
    let result = sdl_client.set("set-tests", &data_map);
    assert!(result.is_ok(), "{:#?}", result.err().unwrap());

    let keys = sdl_client.list_keys("set-tests", "*");
    assert!(keys.is_ok(), "{:#?}", keys.err().unwrap());
    let keys = keys.unwrap();
    assert!(keys.len() == 1);

    let value = sdl_client.get("set-tests", &sdl::KeySet::from(["hello".to_string()]));
    assert!(value.is_ok(), "{:#?}", value.err().unwrap());
    let value = value.unwrap();
    assert_eq!(
        value,
        sdl::DataMap::from([("hello".to_string(), b"world".to_vec())])
    );

    stop_test_redis_server();
}
