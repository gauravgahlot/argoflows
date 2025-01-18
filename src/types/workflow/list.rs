use k8s_openapi::apimachinery::pkg::apis::meta::v1 as metav1;
use serde::{Deserialize, Serialize};

/// `WorkflowList` is list of `Workflow` resources.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct WorkflowList {
    /// `APIVersion` defines the versioned schema of this representation of an
    /// object. Servers should convert recognized schemas to the latest
    /// internal value, and may reject unrecognized values. More info:
    /// https://git.io.k8s.community/contributors/devel/sig-architecture/api-conventions.md#resources
    #[serde(rename = "apiVersion", skip_serializing_if = "Option::is_none")]
    pub api_version: Option<String>,

    #[serde(rename = "items")]
    pub items: Vec<super::Workflow>,

    /// `Kind` is a string value representing the REST resource this object
    /// represents. Servers may infer this from the endpoint the client submits
    /// requests to. Cannot be updated. In CamelCase. More info:
    /// https://git.io.k8s.community/contributors/devel/sig-architecture/api-conventions.md#types-kinds
    #[serde(rename = "kind", skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,

    #[serde(rename = "metadata")]
    pub metadata: Box<metav1::ListMeta>,
}

impl WorkflowList {
    pub fn new(items: Vec<super::Workflow>, metadata: metav1::ListMeta) -> Self {
        WorkflowList {
            items,
            metadata: Box::new(metadata),
            ..Default::default()
        }
    }
}
