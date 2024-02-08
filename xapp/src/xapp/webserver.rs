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

use axum::{routing::get, Json, Router};

#[tokio::main]
pub(crate) async fn run_ready_live_server(
    config: crate::XAppConfig,
) -> Result<(), crate::XAppError> {
    log::info!("Starting Ready and Alive handlers!");

    let port_num = crate::XApp::port_from_config(&config, "http")?;

    let webapp = Router::new()
        .route("/ric/v1/health/ready", get(|| async { Json("OK") }))
        .route("/ric/v1/health/alive", get(|| async { Json("OK") }))
        .route("/ric/v1/config", get(|| async { Json(vec![config]) }));

    let bind_address = format!("0.0.0.0:{port_num}");
    axum::Server::bind(&bind_address.parse().unwrap())
        .serve(webapp.into_make_service())
        .await
        .unwrap();

    Ok(())
}
