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

/// SubscriptionParamsClientEndpoint : xApp service address and port
#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct SubscriptionParamsClientEndpoint {
    /// xApp service address name like 'service-ricxapp-xappname-http.ricxapp'
    #[serde(rename = "Host", skip_serializing_if = "Option::is_none")]
    pub host: Option<String>,
    /// xApp HTTP service address port
    #[serde(rename = "HTTPPort", skip_serializing_if = "Option::is_none")]
    pub http_port: Option<u32>,
    /// xApp RMR service address port
    #[serde(rename = "RMRPort", skip_serializing_if = "Option::is_none")]
    pub rmr_port: Option<u32>,
}

impl SubscriptionParamsClientEndpoint {
    /// xApp service address and port
    pub fn new() -> SubscriptionParamsClientEndpoint {
        SubscriptionParamsClientEndpoint {
            host: None,
            http_port: None,
            rmr_port: None,
        }
    }
}
