/*
 * RIC appmgr
 *
 * This is a draft API for RIC appmgr
 *
 * The version of the OpenAPI document: 0.3.3
 * 
 * Generated by: https://openapi-generator.tech
 */

/// EventType : Event which is subscribed

/// Event which is subscribed
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum EventType {
    #[serde(rename = "deployed")]
    Deployed,
    #[serde(rename = "undeployed")]
    Undeployed,
    #[serde(rename = "created")]
    Created,
    #[serde(rename = "modified")]
    Modified,
    #[serde(rename = "deleted")]
    Deleted,
    #[serde(rename = "restarted")]
    Restarted,
    #[serde(rename = "all")]
    All,

}

impl ToString for EventType {
    fn to_string(&self) -> String {
        match self {
            Self::Deployed => String::from("deployed"),
            Self::Undeployed => String::from("undeployed"),
            Self::Created => String::from("created"),
            Self::Modified => String::from("modified"),
            Self::Deleted => String::from("deleted"),
            Self::Restarted => String::from("restarted"),
            Self::All => String::from("all"),
        }
    }
}

impl Default for EventType {
    fn default() -> EventType {
        Self::Deployed
    }
}




