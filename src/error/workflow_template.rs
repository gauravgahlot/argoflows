use serde::{Deserialize, Serialize};
use serde_json;

use super::GatewayRuntimeError;

/// Struct for typed errors of method [`api::workflow_template::create_workflow_template`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateWorkflowTemplateError {
    DefaultResponse(GatewayRuntimeError),
    UnknownValue(serde_json::Value),
}

/// Struct for typed errors of method [`api::workflow_template::delete_workflow_template`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteWorkflowTemplateError {
    DefaultResponse(GatewayRuntimeError),
    UnknownValue(serde_json::Value),
}

/// Struct for typed errors of method [`api::workflow_template::get_workflow_template`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetWorkflowTemplateError {
    DefaultResponse(GatewayRuntimeError),
    UnknownValue(serde_json::Value),
}

/// Struct for typed errors of method [`api::workflow_template::lint_workflow_template`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum LintWorkflowTemplateError {
    DefaultResponse(GatewayRuntimeError),
    UnknownValue(serde_json::Value),
}

/// Struct for typed errors of method [`api::workflow_template::list_workflow_templates`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListWorkflowTemplatesError {
    DefaultResponse(GatewayRuntimeError),
    UnknownValue(serde_json::Value),
}

/// Struct for typed errors of method [`api::workflow_template::update_workflow_template`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateWorkflowTemplateError {
    DefaultResponse(GatewayRuntimeError),
    UnknownValue(serde_json::Value),
}
