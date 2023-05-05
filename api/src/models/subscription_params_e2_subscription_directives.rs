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

/// SubscriptionParamsE2SubscriptionDirectives : Optional. If not set Submgr uses its default values

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct SubscriptionParamsE2SubscriptionDirectives {
    /// How long time response is waited from E2 node
    #[serde(
        rename = "E2TimeoutTimerValue",
        skip_serializing_if = "Option::is_none"
    )]
    pub e2_timeout_timer_value: Option<u32>,
    /// How many times E2 subscription request is retried
    #[serde(rename = "E2RetryCount", skip_serializing_if = "Option::is_none")]
    pub e2_retry_count: Option<u32>,
    /// Subscription needs RMR route from E2Term to xApp
    #[serde(rename = "RMRRoutingNeeded", skip_serializing_if = "Option::is_none")]
    pub rmr_routing_needed: Option<bool>,
}

impl SubscriptionParamsE2SubscriptionDirectives {
    /// Optional. If not set Submgr uses its default values
    pub fn new() -> SubscriptionParamsE2SubscriptionDirectives {
        SubscriptionParamsE2SubscriptionDirectives {
            e2_timeout_timer_value: None,
            e2_retry_count: None,
            rmr_routing_needed: None,
        }
    }
}
