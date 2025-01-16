use serde::{Deserialize, Serialize};

/// `VolumeClaimGC` describes how to delete volumes from completed Workflows.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct VolumeClaimGC {
    /// `Strategy` is the strategy to use. One of `OnWorkflowCompletion`,
    /// `OnWorkflowSuccess`. Defaults to `OnWorkflowSuccess`.
    #[serde(rename = "strategy", skip_serializing_if = "Option::is_none")]
    pub strategy: Option<String>,
}

impl VolumeClaimGC {
    pub fn new(strategy: &str) -> Self {
        VolumeClaimGC {
            strategy: Some(strategy.to_string()),
        }
    }
}
