use crate::types::artifact;
use serde::{Deserialize, Serialize};

/// `DataSource` sources external data into a data template.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct DataSource {
    #[serde(rename = "artifactPaths", skip_serializing_if = "Option::is_none")]
    pub artifact_paths: Option<Box<artifact::ArtifactPaths>>,
}

impl DataSource {
    pub fn new() -> Self {
        DataSource {
            ..Default::default()
        }
    }
}
