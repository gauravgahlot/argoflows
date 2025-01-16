use k8s_openapi::apimachinery::pkg::apis::meta::v1 as metav1;
use serde::{Deserialize, Serialize};

use crate::types::workflow_template::WorkflowTemplate;

/// `WorkflowTemplateList` is list of `WorkflowTemplate` resources.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct WorkflowTemplateList {
    /// `APIVersion` defines the versioned schema of this representation of an
    /// object. Servers should convert recognized schemas to the latest internal
    /// value, and may reject unrecognized values.
    ///
    /// More info:
    /// https://git.io.k8s.community/contributors/devel/sig-architecture/api-conventions.md#resources
    #[serde(rename = "apiVersion", skip_serializing_if = "Option::is_none")]
    pub api_version: Option<String>,

    #[serde(rename = "items")]
    pub items: Vec<WorkflowTemplate>,

    /// `Kind` is a string value representing the REST resource this object
    /// represents. Servers may infer this from the endpoint the client submits
    /// requests to. Cannot be updated. In CamelCase.
    ///
    /// More info:
    /// https://git.io.k8s.community/contributors/devel/sig-architecture/api-conventions.md#types-kinds
    #[serde(rename = "kind", skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,

    #[serde(rename = "metadata")]
    pub metadata: Box<metav1::ListMeta>,
}

impl WorkflowTemplateList {
    pub fn new(items: Vec<WorkflowTemplate>, metadata: metav1::ListMeta) -> Self {
        WorkflowTemplateList {
            items,
            metadata: Box::new(metadata),
            ..Default::default()
        }
    }
}
