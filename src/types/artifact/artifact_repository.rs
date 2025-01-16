use serde::{Deserialize, Serialize};

/// `ArtifactRepository` represents an artifact repository in which a controller
/// will store its artifacts.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct ArtifactRepository {
    /// `ArchiveLogs` enables log archiving.
    #[serde(rename = "archiveLogs", skip_serializing_if = "Option::is_none")]
    pub archive_logs: Option<bool>,

    #[serde(rename = "artifactory", skip_serializing_if = "Option::is_none")]
    pub artifactory: Option<Box<super::ArtifactoryArtifactRepository>>,

    #[serde(rename = "azure", skip_serializing_if = "Option::is_none")]
    pub azure: Option<Box<super::AzureArtifactRepository>>,

    #[serde(rename = "gcs", skip_serializing_if = "Option::is_none")]
    pub gcs: Option<Box<super::GCSArtifactRepository>>,

    #[serde(rename = "hdfs", skip_serializing_if = "Option::is_none")]
    pub hdfs: Option<Box<super::HDFSArtifactRepository>>,

    #[serde(rename = "oss", skip_serializing_if = "Option::is_none")]
    pub oss: Option<Box<super::OSSArtifactRepository>>,

    #[serde(rename = "s3", skip_serializing_if = "Option::is_none")]
    pub s3: Option<Box<super::S3ArtifactRepository>>,
}

impl ArtifactRepository {
    pub fn new() -> Self {
        ArtifactRepository {
            ..Default::default()
        }
    }
}
