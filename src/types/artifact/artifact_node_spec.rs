use serde::{Deserialize, Serialize};

/// `ArtifactNodeSpec` specifies the Artifacts that need to be deleted for a given Node.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct ArtifactNodeSpec {
    #[serde(rename = "archiveLocation", skip_serializing_if = "Option::is_none")]
    pub archive_location: Option<Box<super::ArtifactLocation>>,

    /// `Artifacts` maps artifact name to Artifact description.
    #[serde(rename = "artifacts", skip_serializing_if = "Option::is_none")]
    pub artifacts: Option<::std::collections::HashMap<String, super::Artifact>>,
}

impl ArtifactNodeSpec {
    pub fn new() -> Self {
        ArtifactNodeSpec {
            ..Default::default()
        }
    }
}
