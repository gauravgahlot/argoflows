use serde::{Deserialize, Serialize};

use crate::error::GatewayRuntimeStreamError;

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct WatchEvent {
    #[serde(rename = "object", skip_serializing_if = "Option::is_none")]
    pub object: Option<Box<super::Workflow>>,

    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct StreamWatchEvent {
    #[serde(rename = "error", skip_serializing_if = "Option::is_none")]
    pub error: Option<Box<GatewayRuntimeStreamError>>,

    #[serde(rename = "result", skip_serializing_if = "Option::is_none")]
    pub result: Option<Box<WatchEvent>>,
}
