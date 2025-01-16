use serde::{Deserialize, Serialize};

/// `WorkflowTemplateRef` is a reference to a WorkflowTemplate resource.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct WorkflowTemplateRef {
    /// `ClusterScope` indicates the referred template is cluster
    /// scoped (i.e. a `ClusterWorkflowTemplate`).
    #[serde(rename = "clusterScope", skip_serializing_if = "Option::is_none")]
    pub cluster_scope: Option<bool>,

    /// `Name` is the resource name of the workflow template.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl WorkflowTemplateRef {
    pub fn new() -> Self {
        WorkflowTemplateRef {
            ..Default::default()
        }
    }
}
