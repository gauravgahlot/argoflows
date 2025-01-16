use k8s_openapi::api::core;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct ContainerSetTemplate {
    #[serde(rename = "containers")]
    pub containers: Vec<super::ContainerNode>,

    #[serde(rename = "retryStrategy", skip_serializing_if = "Option::is_none")]
    pub retry_strategy: Option<Box<super::ContainerSetRetryStrategy>>,

    #[serde(rename = "volumeMounts", skip_serializing_if = "Option::is_none")]
    pub volume_mounts: Option<Vec<core::v1::VolumeMount>>,
}

impl ContainerSetTemplate {
    pub fn new(containers: Vec<super::ContainerNode>) -> Self {
        ContainerSetTemplate {
            containers,
            ..Default::default()
        }
    }
}
