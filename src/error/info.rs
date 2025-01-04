use serde::{Deserialize, Serialize};

/// Struct for typed errors of method [`api::info::get_version`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetVersionError {
    DefaultResponse(super::GatewayRuntimeError),
    UnknownValue(serde_json::Value),
}
