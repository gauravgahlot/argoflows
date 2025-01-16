use serde::{Deserialize, Serialize};

use crate::types::Metadata;

/// `ArtifactGC` describes how to delete artifacts from completed Workflows -
/// this is embedded into the `WorkflowLevelArtifactGC`, and also used for
/// individual Artifacts to override that as needed.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct ArtifactGC {
    #[serde(rename = "podMetadata", skip_serializing_if = "Option::is_none")]
    pub pod_metadata: Option<Box<Metadata>>,

    /// `ServiceAccountName` is an optional field for specifying the Service
    /// Account that should be assigned to the Pod doing the deletion.
    #[serde(rename = "serviceAccountName", skip_serializing_if = "Option::is_none")]
    pub service_account_name: Option<String>,

    /// `Strategy` is the strategy to use.
    #[serde(rename = "strategy", skip_serializing_if = "Option::is_none")]
    pub strategy: Option<String>,
}

impl ArtifactGC {
    pub fn new() -> Self {
        ArtifactGC {
            ..Default::default()
        }
    }
}
