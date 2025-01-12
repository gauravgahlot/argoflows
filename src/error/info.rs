use serde::{Deserialize, Serialize};

/// struct for typed errors of method [`api::info::collect_event`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CollectEventError {
    DefaultResponse(super::GatewayRuntimeError),
    UnknownValue(serde_json::Value),
}

/// Struct for typed errors of method [`api::info::get_info`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetInfoError {
    DefaultResponse(super::GatewayRuntimeError),
    UnknownValue(serde_json::Value),
}

/// Struct for typed errors of method [`api::info::get_user_info`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UserInfoError {
    DefaultResponse(super::GatewayRuntimeError),
    UnknownValue(serde_json::Value),
}

/// Struct for typed errors of method [`api::info::get_version`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetVersionError {
    DefaultResponse(super::GatewayRuntimeError),
    UnknownValue(serde_json::Value),
}
