use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
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
    pub fn new(config_map: Option<&str>, key: Option<&str>) -> Self {
        let config_map = config_map.unwrap_or_default();
        let key = key.unwrap_or_default();

        ArtifactRepositoryRef {
            config_map: Some(config_map.to_string()),
            key: Some(key.to_string()),
        }
    }
}

impl Default for ArtifactRepositoryRef {
    fn default() -> Self {
        ArtifactRepositoryRef {
            config_map: Some("artifact-repositories".to_string()),
            key: None,
        }
    }
}
