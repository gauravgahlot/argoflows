use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct ArtifactRepositoryRef {
    /// The name of the config map. Defaults to \"artifact-repositories\".
    #[serde(rename = "configMap", skip_serializing_if = "Option::is_none")]
    pub config_map: Option<String>,

    /// The config map key. Defaults to the value of the
    /// \"workflows.argoproj.io/default-artifact-repository\" annotation.
    #[serde(rename = "key", skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
}

impl ArtifactRepositoryRef {
    pub fn new() -> Self {
        ArtifactRepositoryRef {
            ..Default::default()
        }
    }
}
