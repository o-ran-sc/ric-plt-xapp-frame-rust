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

//! Alarms management for XApp
//!
//! This module defines client side API for interacting with Alarm Manager of the NearRT RIC
//! Platform.

pub mod types;

pub(crate) mod client;

mod alarm_xapp {
    use crate::{XApp, XAppError};

    use super::types;

    impl<T> XApp<T> {
        /// Raise the Alarm with given Alarm ID, Cause, Severity and Additional Info
        pub fn raise_alarm(
            &self,
            id: i32,
            severity: crate::AlarmSeverity,
            identifying_info: String,
            additional_info: String,
        ) -> Result<(), crate::XAppError> {
            self.alarm_raise_or_clear(id, severity, identifying_info, additional_info, true)
        }

        /// Clear the Alarm with given Alarm ID, Cause, Severity and Additional Info
        pub fn clear_alarm(
            &self,
            id: i32,
            severity: crate::AlarmSeverity,
            identifying_info: String,
            additional_info: String,
        ) -> Result<(), crate::XAppError> {
            self.alarm_raise_or_clear(id, severity, identifying_info, additional_info, false)
        }

        /// Clear All Alarms for the App
        pub fn clear_all_alarms(&self) -> Result<(), crate::XAppError> {
            let client = self
                .alarm_client
                .lock()
                .expect("Corrupted AlarmClient Mutex");

            (*client).clear_all()
        }

        fn alarm_raise_or_clear(
            &self,
            id: i32,
            severity: crate::AlarmSeverity,
            identifying_info: String,
            additional_info: String,
            raise: bool,
        ) -> Result<(), XAppError> {
            let alarm = types::Alarm {
                managed_object_id: "RIC".to_string(),
                application_id: self
                    .app_name
                    .clone()
                    .unwrap_or("xapp-frame-rust".to_string()),
                specific_problem: id,
                perceived_severity: severity,
                identifying_info,
                additional_info,
            };

            let client = self
                .alarm_client
                .lock()
                .expect("Corrupted AlarmClient Mutex");

            if raise {
                (*client).raise(alarm)
            } else {
                (*client).clear(alarm)
            }
        }
    }
}
