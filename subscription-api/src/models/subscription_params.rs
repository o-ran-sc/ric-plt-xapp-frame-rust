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

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct SubscriptionParams {
    /// Optional subscription ID (Submgr allocates if not given)
    #[serde(rename = "SubscriptionId", skip_serializing_if = "Option::is_none")]
    pub subscription_id: Option<String>,
    #[serde(rename = "ClientEndpoint")]
    pub client_endpoint: Box<crate::models::SubscriptionParamsClientEndpoint>,
    #[serde(rename = "Meid")]
    pub meid: String,
    #[serde(rename = "RANFunctionID")]
    pub ran_function_id: u32,
    #[serde(
        rename = "E2SubscriptionDirectives",
        skip_serializing_if = "Option::is_none"
    )]
    pub e2_subscription_directives:
        Option<Box<crate::models::SubscriptionParamsE2SubscriptionDirectives>>,
    #[serde(rename = "SubscriptionDetails")]
    pub subscription_details: Vec<crate::models::SubscriptionDetail>,
}

impl SubscriptionParams {
    pub fn new(
        client_endpoint: crate::models::SubscriptionParamsClientEndpoint,
        meid: String,
        ran_function_id: u32,
        subscription_details: Vec<crate::models::SubscriptionDetail>,
    ) -> SubscriptionParams {
        SubscriptionParams {
            subscription_id: None,
            client_endpoint: Box::new(client_endpoint),
            meid,
            ran_function_id,
            e2_subscription_directives: None,
            subscription_details,
        }
    }
}
