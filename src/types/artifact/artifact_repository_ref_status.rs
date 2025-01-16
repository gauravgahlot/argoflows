use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct ArtifactRepositoryRefStatus {
    #[serde(rename = "artifactRepository", skip_serializing_if = "Option::is_none")]
    pub artifact_repository: Option<Box<super::ArtifactRepository>>,

    /// The name of the config map. Defaults to \"artifact-repositories\".
    #[serde(rename = "configMap", skip_serializing_if = "Option::is_none")]
    pub config_map: Option<String>,

    /// If this ref represents the default artifact repository,
    /// rather than a config map.
    #[serde(rename = "default", skip_serializing_if = "Option::is_none")]
    pub default: Option<bool>,

    /// The config map key. Defaults to the value of the
    /// \"workflows.argoproj.io/default-artifact-repository\" annotation.
    #[serde(rename = "key", skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,

    /// The namespace of the config map. Defaults to the workflow's namespace,
    /// or the controller's namespace (if found).
    #[serde(rename = "namespace", skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
}

impl ArtifactRepositoryRefStatus {
    pub fn new() -> Self {
        ArtifactRepositoryRefStatus {
            ..Default::default()
        }
    }
}
