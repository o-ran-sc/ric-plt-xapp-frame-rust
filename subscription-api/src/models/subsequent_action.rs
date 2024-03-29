/*
 * RIC subscription
 *
 * This is the initial REST API for RIC subscription
 *
 * The version of the OpenAPI document: 0.0.4
 *
 * Generated by: https://openapi-generator.tech
 */

use serde_derive::{Deserialize, Serialize};

/// SubsequentAction : SubsequentAction is an OPTIONAL IE
#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct SubsequentAction {
    #[serde(rename = "SubsequentActionType")]
    pub subsequent_action_type: SubsequentActionType,
    #[serde(rename = "TimeToWait")]
    pub time_to_wait: TimeToWait,
}

impl SubsequentAction {
    /// SubsequentAction is an OPTIONAL IE
    pub fn new(
        subsequent_action_type: SubsequentActionType,
        time_to_wait: TimeToWait,
    ) -> SubsequentAction {
        SubsequentAction {
            subsequent_action_type,
            time_to_wait,
        }
    }
}

///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum SubsequentActionType {
    #[serde(rename = "continue")]
    Continue,
    #[serde(rename = "wait")]
    Wait,
}

impl Default for SubsequentActionType {
    fn default() -> SubsequentActionType {
        Self::Continue
    }
}
///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum TimeToWait {
    #[serde(rename = "zero")]
    Zero,
    #[serde(rename = "w1ms")]
    W1ms,
    #[serde(rename = "w2ms")]
    W2ms,
    #[serde(rename = "w5ms")]
    W5ms,
    #[serde(rename = "w10ms")]
    W10ms,
    #[serde(rename = "w20ms")]
    W20ms,
    #[serde(rename = "w30ms")]
    W30ms,
    #[serde(rename = "w40ms")]
    W40ms,
    #[serde(rename = "w50ms")]
    W50ms,
    #[serde(rename = "w100ms")]
    W100ms,
    #[serde(rename = "w200ms")]
    W200ms,
    #[serde(rename = "w500ms")]
    W500ms,
    #[serde(rename = "w1s")]
    W1s,
    #[serde(rename = "w2s")]
    W2s,
    #[serde(rename = "w5s")]
    W5s,
    #[serde(rename = "w10s")]
    W10s,
    #[serde(rename = "w20s")]
    W20s,
    #[serde(rename = "w60s")]
    W60s,
}

impl Default for TimeToWait {
    fn default() -> TimeToWait {
        Self::Zero
    }
}
