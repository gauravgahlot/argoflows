use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Version {
    #[serde(rename = "buildDate")]
    pub build_date: String,
    #[serde(rename = "compiler")]
    pub compiler: String,
    #[serde(rename = "gitCommit")]
    pub git_commit: String,
    #[serde(rename = "gitTag")]
    pub git_tag: String,
    #[serde(rename = "gitTreeState")]
    pub git_tree_state: String,
    #[serde(rename = "goVersion")]
    pub go_version: String,
    #[serde(rename = "platform")]
    pub platform: String,
    #[serde(rename = "version")]
    pub version: String,
}
