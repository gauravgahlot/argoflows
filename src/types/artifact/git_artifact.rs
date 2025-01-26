use k8s_openapi::api::core;
use serde::{Deserialize, Serialize};

/// `GitArtifact` is the location of an git artifact.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct GitArtifact {
    /// `Branch` is the branch to fetch when `SingleBranch` is enabled
    #[serde(rename = "branch", skip_serializing_if = "Option::is_none")]
    pub branch: Option<String>,

    /// `Depth` specifies clones/fetches should be shallow and include
    /// the given number of commits from the branch tip.
    #[serde(rename = "depth", skip_serializing_if = "Option::is_none")]
    pub depth: Option<i32>,

    /// `DisableSubmodules` disables submodules during git clone
    #[serde(rename = "disableSubmodules", skip_serializing_if = "Option::is_none")]
    pub disable_submodules: Option<bool>,

    /// `Fetch` specifies a number of refs that should be fetched before checkout
    #[serde(rename = "fetch", skip_serializing_if = "Option::is_none")]
    pub fetch: Option<Vec<String>>,

    /// `InsecureIgnoreHostKey` disables SSH strict host key checking during git clone
    #[serde(
        rename = "insecureIgnoreHostKey",
        skip_serializing_if = "Option::is_none"
    )]
    pub insecure_ignore_host_key: Option<bool>,

    #[serde(rename = "passwordSecret", skip_serializing_if = "Option::is_none")]
    pub password_secret: Option<Box<core::v1::SecretKeySelector>>,

    /// `Repo` is the git repository
    #[serde(rename = "repo")]
    pub repo: String,

    /// `Revision` is the git commit, tag, branch to checkout
    #[serde(rename = "revision", skip_serializing_if = "Option::is_none")]
    pub revision: Option<String>,

    /// `SingleBranch` enables single branch clone, using the `branch` parameter
    #[serde(rename = "singleBranch", skip_serializing_if = "Option::is_none")]
    pub single_branch: Option<bool>,

    #[serde(
        rename = "sshPrivateKeySecret",
        skip_serializing_if = "Option::is_none"
    )]
    pub ssh_private_key_secret: Option<Box<core::v1::SecretKeySelector>>,

    #[serde(rename = "usernameSecret", skip_serializing_if = "Option::is_none")]
    pub username_secret: Option<Box<core::v1::SecretKeySelector>>,
}

impl GitArtifact {
    pub fn new(repo: &str) -> GitArtifact {
        GitArtifact {
            repo,
            ..Default::default()
        }
    }
}
