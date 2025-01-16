use std::collections::HashMap;

use serde::{Deserialize, Serialize};

/// `ArtifactGCSpec` specifies the Artifacts that need to be deleted.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct ArtifactGCSpec {
    /// `ArtifactsByNode` maps Node name to information pertaining
    /// to Artifacts on that Node.
    #[serde(rename = "artifactsByNode", skip_serializing_if = "Option::is_none")]
    pub artifacts_by_node: Option<HashMap<String, super::ArtifactNodeSpec>>,
}

impl ArtifactGCSpec {
    pub fn new() -> Self {
        ArtifactGCSpec {
            ..Default::default()
        }
    }
}
