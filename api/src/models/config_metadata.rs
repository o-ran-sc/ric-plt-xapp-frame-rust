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
pub struct ConfigMetadata {
    /// Name of the xApp
    #[serde(rename = "xappName")]
    pub xapp_name: String,
    /// The type of the content
    #[serde(rename = "configType")]
    pub config_type: ConfigType,
}

impl ConfigMetadata {
    pub fn new(xapp_name: String, config_type: ConfigType) -> ConfigMetadata {
        ConfigMetadata {
            xapp_name,
            config_type,
        }
    }
}

/// The type of the content
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ConfigType {
    #[serde(rename = "json")]
    Json,
    #[serde(rename = "xml")]
    Xml,
    #[serde(rename = "other")]
    Other,
}

impl Default for ConfigType {
    fn default() -> ConfigType {
        Self::Json
    }
}
