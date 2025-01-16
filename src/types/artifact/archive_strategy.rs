use serde::{Deserialize, Serialize};

/// `ArchiveStrategy` describes how to archive files/directory when saving artifacts.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct ArchiveStrategy {
    /// `NoneStrategy` indicates to skip tar process and upload the files
    /// or directory tree as independent files. Note that if the artifact
    /// is a directory, the artifact driver must support the ability
    /// to save/load the directory appropriately.
    #[serde(rename = "none", skip_serializing_if = "Option::is_none")]
    pub none: Option<serde_json::Value>,

    #[serde(rename = "tar", skip_serializing_if = "Option::is_none")]
    pub tar: Option<Box<super::TarStrategy>>,

    /// `ZipStrategy` will unzip zipped input artifacts
    #[serde(rename = "zip", skip_serializing_if = "Option::is_none")]
    pub zip: Option<serde_json::Value>,
}

impl ArchiveStrategy {
    pub fn new() -> Self {
        ArchiveStrategy {
            ..Default::default()
        }
    }
}
