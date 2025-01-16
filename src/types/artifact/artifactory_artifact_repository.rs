use k8s_openapi::api::core;

use serde::{Deserialize, Serialize};

/// `ArtifactoryArtifactRepository` defines the controller configuration for
/// an artifactory artifact repository.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct ArtifactoryArtifactRepository {
    /// `KeyFormat` defines the format of how to store keys and can reference
    /// workflow variables.
    #[serde(rename = "keyFormat", skip_serializing_if = "Option::is_none")]
    pub key_format: Option<String>,

    #[serde(rename = "passwordSecret", skip_serializing_if = "Option::is_none")]
    pub password_secret: Option<Box<core::v1::SecretKeySelector>>,

    /// `RepoURL` is the url for artifactory repo.
    #[serde(rename = "repoURL", skip_serializing_if = "Option::is_none")]
    pub repo_url: Option<String>,

    #[serde(rename = "usernameSecret", skip_serializing_if = "Option::is_none")]
    pub username_secret: Option<Box<core::v1::SecretKeySelector>>,
}

impl ArtifactoryArtifactRepository {
    pub fn new() -> Self {
        ArtifactoryArtifactRepository {
            ..Default::default()
        }
    }
}
