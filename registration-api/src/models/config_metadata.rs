/*
 * RIC appmgr
 *
 * This is a draft API for RIC appmgr
 *
 * The version of the OpenAPI document: 0.3.3
 *
 * Generated by: https://openapi-generator.tech
 */

// Following structure is edited because currently used structure uses configType

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ConfigMetadata {
    /// Name of the xApp
    #[serde(rename = "xappName")]
    pub xapp_name: String,
    /// Name of the namespace
    #[serde(rename = "configType")]
    pub config_type: String,
}

impl ConfigMetadata {
    pub fn new(xapp_name: String, config_type: String) -> ConfigMetadata {
        ConfigMetadata {
            xapp_name,
            config_type,
        }
    }
}