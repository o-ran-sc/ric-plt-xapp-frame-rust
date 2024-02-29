// ==================================================================================
//   Copyright (c) 2024 Abhijit Gadgil
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

use std::sync::OnceLock;

use tokio::runtime::Handle;
use tokio::sync::mpsc::Receiver as TokioSyncReceiver;
use tokio::sync::RwLock;

use axum::{routing::get, Json, Router};

static METRICS_RECEIVER: OnceLock<RwLock<String>> = OnceLock::new();

async fn metrics_receiver() -> String {
    let value = METRICS_RECEIVER.get_or_init(|| RwLock::new(String::new()));
    let value = value.read().await;
    (*value).clone()
}

#[tokio::main]
pub(crate) async fn run_ready_live_server(
    config: crate::XAppConfig,
    mut data_rx: TokioSyncReceiver<String>,
) -> Result<(), crate::XAppError> {
    log::info!("Starting Ready and Alive handlers!");

    let port_num = crate::XApp::port_from_config(&config, "http")?;

    let webapp = Router::new()
        .route("/ric/v1/health/ready", get(|| async { Json("OK") }))
        .route("/ric/v1/health/alive", get(|| async { Json("OK") }))
        .route("/ric/v1/config", get(|| async { Json(vec![config]) }))
        .route("/ric/v1/metrics", get(metrics_receiver));

    let bind_address = format!("0.0.0.0:{port_num}");
    let server =
        axum::Server::bind(&bind_address.parse().unwrap()).serve(webapp.into_make_service());

    tokio::pin!(server);

    let handle = Handle::current();
    loop {
        tokio::select! {
            Some(v) = handle.block_on(async { data_rx.recv()}) => {
                let mut parts = v.split(':');
                if let Some(v) = parts.next() {
                    if v == "metrics" {
                        if let Some(metrics) = parts.next() {
                            let value = METRICS_RECEIVER.get_or_init(|| RwLock::new(String::new()));
                            let mut value = value.write().await;
                            value.clear();
                            value.push_str(metrics);
                        }
                    }
                } else {
                    break;
                }
            }
            _ = &mut server => break,

        }
    }

    Ok(())
}
