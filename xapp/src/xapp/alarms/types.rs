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

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Alarm {
    #[serde(rename = "managedObjectId")]
    pub(crate) managed_object_id: String,

    #[serde(rename = "applicationId")]
    pub(crate) application_id: String,

    #[serde(rename = "specificProblem")]
    pub(crate) specific_problem: i32,

    #[serde(rename = "perceivedSeverity")]
    pub(crate) perceived_severity: AlarmSeverity,

    #[serde(rename = "identifyingInfo")]
    pub(crate) identifying_info: String,

    #[serde(rename = "additionalInfo")]
    pub(crate) additional_info: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum AlarmSeverity {
    #[serde(rename = "UNSPECIFIED")]
    Unspecified,

    #[serde(rename = "MAJOR")]
    Major,

    #[serde(rename = "MINOR")]
    Minor,

    #[serde(rename = "WARNING")]
    Warning,

    #[serde(rename = "CLEARED")]
    Cleared,

    #[serde(rename = "DEFAULT")]
    Def,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum AlarmAction {
    #[serde(rename = "RAISE")]
    Raise,

    #[serde(rename = "CLEAR")]
    Clear,

    #[serde(rename = "CLEARALL")]
    ClearAll,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AlarmMessage {
    #[serde(rename = "Alarm")]
    pub(crate) alarm: Alarm,

    #[serde(rename = "AlarmAction")]
    pub(crate) action: AlarmAction,

    #[serde(rename = "AlarmTime")]
    pub(crate) alarm_time: u64,
}
