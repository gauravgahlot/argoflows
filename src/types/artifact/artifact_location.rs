use serde::{Deserialize, Serialize};

/// `ArtifactLocation` describes a location for a single or multiple artifacts.
/// It is used as single artifact in the context of inputs/outputs
/// (e.g. outputs.artifacts.artname). It is also used to describe the location
/// of multiple artifacts such as the archive location of a single workflow step,
/// which the executor will use as a default location to store its files.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct ArtifactLocation {
    /// ArchiveLogs indicates if the container logs should be archived
    #[serde(rename = "archiveLogs", skip_serializing_if = "Option::is_none")]
    pub archive_logs: Option<bool>,

    #[serde(rename = "artifactory", skip_serializing_if = "Option::is_none")]
    pub artifactory: Option<Box<super::ArtifactoryArtifact>>,

    #[serde(rename = "azure", skip_serializing_if = "Option::is_none")]
    pub azure: Option<Box<super::AzureArtifact>>,

    #[serde(rename = "gcs", skip_serializing_if = "Option::is_none")]
    pub gcs: Option<Box<super::GCSArtifact>>,

    #[serde(rename = "git", skip_serializing_if = "Option::is_none")]
    pub git: Option<Box<super::GitArtifact>>,

    #[serde(rename = "hdfs", skip_serializing_if = "Option::is_none")]
    pub hdfs: Option<Box<super::HDFSArtifact>>,

    #[serde(rename = "http", skip_serializing_if = "Option::is_none")]
    pub http: Option<Box<super::HTTPArtifact>>,

    #[serde(rename = "oss", skip_serializing_if = "Option::is_none")]
    pub oss: Option<Box<super::OSSArtifact>>,

    #[serde(rename = "raw", skip_serializing_if = "Option::is_none")]
    pub raw: Option<Box<super::RawArtifact>>,

    #[serde(rename = "s3", skip_serializing_if = "Option::is_none")]
    pub s3: Option<Box<super::S3Artifact>>,
}

impl ArtifactLocation {
    pub fn new() -> Self {
        ArtifactLocation {
            ..Default::default()
        }
    }
}
