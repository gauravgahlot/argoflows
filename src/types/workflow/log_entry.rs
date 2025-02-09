use serde::{Deserialize, Serialize};

use crate::error::GatewayRuntimeStreamError;

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct LogEntry {
    #[serde(rename = "content", skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,

    #[serde(rename = "podName", skip_serializing_if = "Option::is_none")]
    pub pod_name: Option<String>,
}

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct StreamLogEntry {
    #[serde(rename = "error", skip_serializing_if = "Option::is_none")]
    pub error: Option<Box<GatewayRuntimeStreamError>>,

    #[serde(rename = "result", skip_serializing_if = "Option::is_none")]
    pub result: Option<Box<LogEntry>>,
}
