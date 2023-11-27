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

//! Management of Registration and Deregistration of XApps
//!
//! This module implements APIs for interacting with App Manager of the NearRT RIC Platform.

use std::sync::atomic::Ordering;

use registration_api::models::{DeregisterRequest, RegisterRequest};

use super::{XApp, XAppError};

const APP_MGR_HOST: &str = "http://service-ricplt-appmgr-http.ricplt:8080";
const REGISTRATION_URL: &str = "ric/v1/register";
const CONFIG_PATH: &str = "/ric/v1/config";
const DEFAULT_XAPP_NS: &str = "ricxapp";

impl XApp {
    /// Register the XApp with the App Manager
    pub fn register_xapp(
        &mut self,
        xapp_name: &str,
        xapp_instance_name: &str,
        config: &str,
        xapp_ns: Option<&str>,
    ) -> Result<(), XAppError> {
        let ns = xapp_ns.unwrap_or(DEFAULT_XAPP_NS);
        let http_host = Self::get_from_env(ns, xapp_name, "http", "host");
        let http_port = Self::get_from_env(ns, xapp_name, "http", "port");
        let http_endpoint = format!("{}:{}", http_host, http_port);

        let rmr_host = Self::get_from_env(ns, xapp_name, "rmr", "host");
        let rmr_port = Self::get_from_env(ns, xapp_name, "rmr", "port");
        let rmr_endpoint = format!("{}:{}", rmr_host, rmr_port);

        log::info!(
            "HTTP Endpoint: {}, RMR Endpoint: {}",
            http_endpoint,
            rmr_endpoint
        );

        let reg_request = RegisterRequest {
            app_name: xapp_name.to_string(),
            app_instance_name: xapp_instance_name.to_string(),
            app_version: None,
            config_path: Some(CONFIG_PATH.to_string()),
            config: Some(config.to_string()),
            http_endpoint,
            rmr_endpoint,
        };

        let json = serde_json::to_string(&reg_request)
            .map_err(|e| XAppError(format!("serde_json: {}", e)))?;

        let req_client = reqwest::blocking::Client::new();
        let path = format!("{}/{}", APP_MGR_HOST, REGISTRATION_URL);

        log::debug!("Sending Registration Request: '{}' to '{}'", json, path);
        let response = req_client
            .post(path)
            .header(reqwest::header::CONTENT_TYPE, "application/json")
            .body(json)
            .send()
            .map_err(|e| XAppError(format!("Error sending request: {}", e)))?;

        if response.status().is_success() {
            log::info!(
                "Registration Response Code: {}, Body: {}",
                response.status(),
                response.text().unwrap()
            );

            self.app_is_registered.store(true, Ordering::SeqCst);
            let _ = self.app_name.replace(xapp_name.to_string());
            let _ = self
                .app_instance_name
                .replace(xapp_instance_name.to_string());

            Ok(())
        } else {
            Err(XAppError(format!("Error : {}", response.status())))
        }
    }

    /// Deregister XApp
    pub fn deregister_xapp(&self) -> Result<(), XAppError> {
        let deregister_request = DeregisterRequest {
            app_name: self.app_name.clone().unwrap().clone(),
            app_instance_name: self.app_instance_name.clone().unwrap().clone(),
        };

        let json = serde_json::to_string(&deregister_request)
            .map_err(|e| XAppError(format!("serde_json: {}", e)))?;

        let req_client = reqwest::blocking::Client::new();
        let path = format!("{}/{}", APP_MGR_HOST, REGISTRATION_URL);

        log::debug!("Sending Deregistration Request: '{}' to '{}'", json, path);
        let response = req_client
            .post(path)
            .header(reqwest::header::CONTENT_TYPE, "application/json")
            .body(json)
            .send()
            .map_err(|e| XAppError(format!("Error sending request: {}", e)));

        match response {
            Ok(response) => {
                if response.status().is_success() {
                    log::info!(
                        "Deregistration for XApp {} Successful",
                        self.app_name.clone().unwrap()
                    );
                } else {
                    log::error!(
                        "DeRegistration Response Code: {} for XApp {}",
                        response.status(),
                        self.app_name.clone().unwrap()
                    );
                }
            }
            Err(e) => {
                log::error!(
                    "Error: '{}' during Dergeistering XApp: {}",
                    e,
                    self.app_name.clone().unwrap()
                );
            }
        }

        self.app_is_registered.store(false, Ordering::SeqCst);
        Ok(())
    }

    #[inline(always)]
    fn get_from_env(ns: &str, xapp: &str, service: &str, typ: &str) -> String {
        let env_name = format!("SERVICE_{}_{}_{}_SERVICE_{}", ns, xapp, service, typ,);
        let env_name = env_name.replace(['-'], "_").to_uppercase();
        std::env::var(env_name).expect("Env Not Set!")
    }
}
