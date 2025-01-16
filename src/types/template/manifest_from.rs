use crate::types::artifact;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ManifestFrom {
    #[serde(rename = "artifact")]
    pub artifact: Box<artifact::Artifact>,
}

impl ManifestFrom {
    pub fn new(artifact: artifact::Artifact) -> Self {
        ManifestFrom {
            artifact: Box::new(artifact),
        }
    }
}
