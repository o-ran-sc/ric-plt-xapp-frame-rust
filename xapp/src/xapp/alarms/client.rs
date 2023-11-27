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

use std::time::{SystemTime, UNIX_EPOCH};

use reqwest::blocking::Client as ReqwestClient;

use crate::XAppError;

use super::types::{Alarm, AlarmAction, AlarmMessage};

pub(crate) struct AlarmClient {
    pub(crate) http_client: ReqwestClient,
}

impl AlarmClient {
    pub(crate) fn new() -> Self {
        Self {
            http_client: ReqwestClient::new(),
        }
    }

    pub(crate) fn raise(&self, alarm: Alarm) -> Result<(), XAppError> {
        self.send_alarm_message(alarm, AlarmAction::Raise)
    }

    pub(crate) fn clear(&self, alarm: Alarm) -> Result<(), XAppError> {
        self.send_alarm_message(alarm, AlarmAction::Clear)
    }

    pub(crate) fn clear_all(&self) -> Result<(), XAppError> {
        Ok(())
    }

    fn send_alarm_message(&self, alarm: Alarm, action: AlarmAction) -> Result<(), XAppError> {
        let alarm_time = SystemTime::now();
        let alarm_time = alarm_time
            .duration_since(UNIX_EPOCH)
            .expect("time reversal")
            .as_secs();

        let alarm_message = AlarmMessage {
            alarm,
            action,
            alarm_time,
        };

        let json = serde_json::to_string(&alarm_message)
            .map_err(|e| XAppError(format!("serde_json: {}", e)))?;

        self.send_alarm(json)
    }

    fn send_alarm(&self, json: String) -> Result<(), XAppError> {
        let plt_ns = std::env::var("PLT_NAMESPACE").unwrap_or("ricplt".to_string());

        let alarm_mgr_url = format!(
            "http://service-{}-alarmmanager-http.{}:8080",
            plt_ns, plt_ns
        );
        let path = format!("{}/{}", alarm_mgr_url, "ric/v1/alarms");

        log::debug!("Sending Alarm Json: {}, URL: {}", json, path);
        let response = self
            .http_client
            .post(path)
            .header(reqwest::header::CONTENT_TYPE, "application/json")
            .body(json)
            .send()
            .map_err(|e| XAppError(format!("Error sending request: {}", e)))?;

        if response.status().is_success() {
            log::debug!("Alarm sent Succesfully!");
        } else {
            // We are not erroring out on Server returning an Error.
            log::warn!(" Server returned! {}", response.status());
        }

        Ok(())
    }
}
