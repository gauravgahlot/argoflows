use std::time;

use k8s_openapi::apimachinery::pkg::apis::meta;
use serde::{Deserialize, Serialize};

/// `PodGC` describes how to delete completed pods as they complete.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct PodGC {
    #[serde(
        rename = "deleteDelayDuration",
        skip_serializing_if = "Option::is_none"
    )]
    pub delete_delay_duration: Option<Box<time::Duration>>,

    #[serde(rename = "labelSelector", skip_serializing_if = "Option::is_none")]
    pub label_selector: Option<Box<meta::v1::LabelSelector>>,

    /// `Strategy` is the strategy to use. One of `OnPodCompletion`,
    /// `OnPodSuccess`, `OnWorkflowCompletion`, `OnWorkflowSuccess`.
    /// If unset, does not delete Pods.
    #[serde(rename = "strategy", skip_serializing_if = "Option::is_none")]
    pub strategy: Option<String>,
}

impl PodGC {
    pub fn new() -> Self {
        PodGC {
            ..Default::default()
        }
    }
}
