use k8s_openapi::api::core;
use serde::{Deserialize, Serialize};

use crate::types::{artifact, sync, template};

/// `WorkflowStatus` contains overall status information about a workflow.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct WorkflowStatus {
    #[serde(rename = "artifactGCStatus", skip_serializing_if = "Option::is_none")]
    pub artifact_gc_status: Option<Box<artifact::ArtGCStatus>>,

    #[serde(
        rename = "artifactRepositoryRef",
        skip_serializing_if = "Option::is_none"
    )]
    pub artifact_repository_ref: Option<Box<artifact::ArtifactRepositoryRefStatus>>,

    /// `Compressed` and base64 decoded Nodes map.
    #[serde(rename = "compressedNodes", skip_serializing_if = "Option::is_none")]
    pub compressed_nodes: Option<String>,

    /// `Conditions` is a list of conditions the Workflow may have.
    #[serde(rename = "conditions", skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<super::Condition>>,

    /// `EstimatedDuration` in seconds.
    #[serde(rename = "estimatedDuration", skip_serializing_if = "Option::is_none")]
    pub estimated_duration: Option<i32>,

    /// Time is a wrapper around time.Time which supports correct marshaling to
    /// YAML and JSON.  Wrappers are provided for many of the factory methods
    /// that the time package offers.
    #[serde(rename = "finishedAt", skip_serializing_if = "Option::is_none")]
    pub finished_at: Option<String>,

    /// A human readable message indicating details about why the workflow is
    /// in this condition.
    #[serde(rename = "message", skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,

    /// `Nodes` is a mapping between a node ID and the node's status.
    #[serde(rename = "nodes", skip_serializing_if = "Option::is_none")]
    pub nodes: Option<::std::collections::HashMap<String, super::NodeStatus>>,

    /// Whether on not node status has been offloaded to a database. If exists,
    /// then Nodes and CompressedNodes will be empty. This will actually be
    /// populated with a hash of the offloaded data.
    #[serde(
        rename = "offloadNodeStatusVersion",
        skip_serializing_if = "Option::is_none"
    )]
    pub offload_node_status_version: Option<String>,

    #[serde(rename = "outputs", skip_serializing_if = "Option::is_none")]
    pub outputs: Option<Box<template::Outputs>>,

    /// `PersistentVolumeClaims` tracks all PVCs that were created as part of the
    /// io.argoproj.workflow.v1alpha1. The contents of this list are drained
    /// at the end of the workflow.
    #[serde(
        rename = "persistentVolumeClaims",
        skip_serializing_if = "Option::is_none"
    )]
    pub persistent_volume_claims: Option<Vec<core::v1::Volume>>,

    /// `Phase` a simple, high-level summary of where the workflow is in its
    /// lifecycle. Will be \"\" (Unknown), \"Pending\", or \"Running\" before
    /// the workflow is completed, and \"Succeeded\", \"Failed\" or \"Error\"
    /// once the workflow has completed.
    #[serde(rename = "phase", skip_serializing_if = "Option::is_none")]
    pub phase: Option<String>,

    /// `Progress` to completion.
    #[serde(rename = "progress", skip_serializing_if = "Option::is_none")]
    pub progress: Option<String>,

    /// `ResourcesDuration` is the total for the workflow.
    #[serde(rename = "resourcesDuration", skip_serializing_if = "Option::is_none")]
    pub resources_duration: Option<::std::collections::HashMap<String, i64>>,

    /// Time is a wrapper around time.Time which supports correct marshaling to
    /// YAML and JSON.  Wrappers are provided for many of the factory methods
    /// that the time package offers.
    #[serde(rename = "startedAt", skip_serializing_if = "Option::is_none")]
    pub started_at: Option<String>,

    /// `StoredTemplates` is a mapping between a template ref and the node's status.
    #[serde(rename = "storedTemplates", skip_serializing_if = "Option::is_none")]
    pub stored_templates: Option<::std::collections::HashMap<String, template::Template>>,

    #[serde(
        rename = "storedWorkflowTemplateSpec",
        skip_serializing_if = "Option::is_none"
    )]
    pub stored_workflow_template_spec: Option<Box<super::WorkflowSpec>>,

    #[serde(rename = "synchronization", skip_serializing_if = "Option::is_none")]
    pub synchronization: Option<Box<sync::SynchronizationStatus>>,

    /// Have task results been completed? (mapped by Pod name) used to prevent premature garbage collection of artifacts.
    #[serde(
        rename = "taskResultsCompleted",
        skip_serializing_if = "Option::is_none"
    )]
    pub task_results_completed: Option<::std::collections::HashMap<String, bool>>,
}

impl WorkflowStatus {
    pub fn new() -> Self {
        WorkflowStatus {
            ..Default::default()
        }
    }
}
