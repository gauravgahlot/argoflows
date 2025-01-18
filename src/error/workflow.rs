use serde::{Deserialize, Serialize};
use serde_json;

use super::GatewayRuntimeError;

/// Typed error of method [`api::workflow::create_workflow`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateWorkflowError {
    DefaultResponse(GatewayRuntimeError),
    UnknownValue(serde_json::Value),
}

/// Typed error of method [`api::workflow::delete_workflow`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteWorkflowError {
    DefaultResponse(GatewayRuntimeError),
    UnknownValue(serde_json::Value),
}

/// Typed error of method [`api::workflow::get_workflow`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetWorkflowError {
    DefaultResponse(GatewayRuntimeError),
    UnknownValue(serde_json::Value),
}

/// Typed error of method [`api::workflow::lint_workflow`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum LintWorkflowError {
    DefaultResponse(GatewayRuntimeError),
    UnknownValue(serde_json::Value),
}

/// Typed error of method [`api::workflow::list_workflows`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListWorkflowsError {
    DefaultResponse(GatewayRuntimeError),
    UnknownValue(serde_json::Value),
}

/// Typed error of method [`api::workflow::pod_logs`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PodLogsError {
    DefaultResponse(GatewayRuntimeError),
    UnknownValue(serde_json::Value),
}

/// Typed error of method [`api::workflow::resubmit_workflow`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ResubmitWorkflowError {
    DefaultResponse(GatewayRuntimeError),
    UnknownValue(serde_json::Value),
}

/// Typed error of method [`api::workflow::resume_workflow`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ResumeWorkflowError {
    DefaultResponse(GatewayRuntimeError),
    UnknownValue(serde_json::Value),
}

/// Typed error of method [`api::workflow::retry_workflow`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RetryWorkflowError {
    DefaultResponse(GatewayRuntimeError),
    UnknownValue(serde_json::Value),
}

/// Typed error of method [`api::workflow::set_workflow`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SetWorkflowError {
    DefaultResponse(GatewayRuntimeError),
    UnknownValue(serde_json::Value),
}

/// Typed error of method [`api::workflow::stop_workflow`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum StopWorkflowError {
    DefaultResponse(GatewayRuntimeError),
    UnknownValue(serde_json::Value),
}

/// Typed error of method [`api::workflow::submit_workflow`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SubmitWorkflowError {
    DefaultResponse(GatewayRuntimeError),
    UnknownValue(serde_json::Value),
}

/// Typed error of method [`api::workflow::suspend_workflow`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SuspendWorkflowError {
    DefaultResponse(GatewayRuntimeError),
    UnknownValue(serde_json::Value),
}

/// Typed error of method [`api::workflow::terminate_workflow`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum TerminateWorkflowError {
    DefaultResponse(GatewayRuntimeError),
    UnknownValue(serde_json::Value),
}

/// Typed error of method [`api::workflow::watch_events`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum WatchEventsError {
    DefaultResponse(GatewayRuntimeError),
    UnknownValue(serde_json::Value),
}

/// Typed error of method [`api::workflow::watch_workflows`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum WatchWorkflowsError {
    DefaultResponse(GatewayRuntimeError),
    UnknownValue(serde_json::Value),
}

/// Typed error of method [`api::workflow::workflow_logs`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum WorkflowLogsError {
    DefaultResponse(GatewayRuntimeError),
    UnknownValue(serde_json::Value),
}
