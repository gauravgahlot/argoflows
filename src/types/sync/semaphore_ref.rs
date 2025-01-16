use k8s_openapi::api::core;
use serde::{Deserialize, Serialize};

/// `SemaphoreRef` is a reference of Semaphore.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct SemaphoreRef {
    #[serde(rename = "configMapKeyRef", skip_serializing_if = "Option::is_none")]
    pub config_map_key_ref: Option<Box<core::v1::ConfigMapKeySelector>>,

    /// Namespace is the namespace of the configmap,
    /// default: [namespace of workflow].
    #[serde(rename = "namespace", skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
}

impl SemaphoreRef {
    pub fn new() -> Self {
        SemaphoreRef {
            ..Default::default()
        }
    }
}
