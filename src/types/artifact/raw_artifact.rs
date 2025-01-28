use serde::{Deserialize, Serialize};

/// `RawArtifact` allows raw string content to be placed
/// as an artifact in a container.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RawArtifact {
    /// `Data` is the string contents of the artifact.
    #[serde(rename = "data")]
    pub data: String,
}

impl RawArtifact {
    pub fn new(data: &str) -> Self {
        RawArtifact { data: data.to_string(), }
    }
}
