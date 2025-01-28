use serde::{Deserialize, Serialize};

/// `ArtifactPaths` expands a step from a collection of artifacts.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct ArtifactPaths {
    #[serde(rename = "archive", skip_serializing_if = "Option::is_none")]
    pub archive: Option<Box<super::ArchiveStrategy>>,

    /// `ArchiveLogs` indicates if the container logs should be archived
    #[serde(rename = "archiveLogs", skip_serializing_if = "Option::is_none")]
    pub archive_logs: Option<bool>,

    #[serde(rename = "artifactGC", skip_serializing_if = "Option::is_none")]
    pub artifact_gc: Option<Box<super::ArtifactGC>>,

    #[serde(rename = "artifactory", skip_serializing_if = "Option::is_none")]
    pub artifactory: Option<Box<super::ArtifactoryArtifact>>,

    #[serde(rename = "azure", skip_serializing_if = "Option::is_none")]
    pub azure: Option<Box<super::AzureArtifact>>,

    /// Has this been deleted?
    #[serde(rename = "deleted", skip_serializing_if = "Option::is_none")]
    pub deleted: Option<bool>,

    /// From allows an artifact to reference an artifact from a previous step
    #[serde(rename = "from", skip_serializing_if = "Option::is_none")]
    pub from: Option<String>,

    /// `FromExpression`, if defined, is evaluated to specify the value for the artifact
    #[serde(rename = "fromExpression", skip_serializing_if = "Option::is_none")]
    pub from_expression: Option<String>,

    #[serde(rename = "gcs", skip_serializing_if = "Option::is_none")]
    pub gcs: Option<Box<super::GCSArtifact>>,

    #[serde(rename = "git", skip_serializing_if = "Option::is_none")]
    pub git: Option<Box<super::GitArtifact>>,

    /// `GlobalName` exports an output artifact to the global scope, making it
    /// available as '{{io.argoproj.workflow.v1alpha1.outputs.artifacts.XXXX}}
    /// and in workflow.status.outputs.artifacts.
    #[serde(rename = "globalName", skip_serializing_if = "Option::is_none")]
    pub global_name: Option<String>,

    #[serde(rename = "hdfs", skip_serializing_if = "Option::is_none")]
    pub hdfs: Option<Box<super::HDFSArtifact>>,

    #[serde(rename = "http", skip_serializing_if = "Option::is_none")]
    pub http: Option<Box<super::HTTPArtifact>>,

    /// mode bits to use on this file, must be a value between 0 and 0777
    /// set when loading input artifacts.
    #[serde(rename = "mode", skip_serializing_if = "Option::is_none")]
    pub mode: Option<i32>,

    /// name of the artifact. must be unique within a template's inputs/outputs.
    #[serde(rename = "name")]
    pub name: String,

    /// Make Artifacts optional, if Artifacts doesn't generate or exist
    #[serde(rename = "optional", skip_serializing_if = "Option::is_none")]
    pub optional: Option<bool>,

    #[serde(rename = "oss", skip_serializing_if = "Option::is_none")]
    pub oss: Option<Box<super::OSSArtifact>>,

    /// Path is the container path to the artifact
    #[serde(rename = "path", skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,

    #[serde(rename = "raw", skip_serializing_if = "Option::is_none")]
    pub raw: Option<Box<super::RawArtifact>>,

    /// If mode is set, apply the permission recursively into the artifact if it is a folder
    #[serde(rename = "recurseMode", skip_serializing_if = "Option::is_none")]
    pub recurse_mode: Option<bool>,

    #[serde(rename = "s3", skip_serializing_if = "Option::is_none")]
    pub s3: Option<Box<super::S3Artifact>>,

    /// `SubPath` allows an artifact to be sourced from a subpath within the specified source
    #[serde(rename = "subPath", skip_serializing_if = "Option::is_none")]
    pub sub_path: Option<String>,
}

impl ArtifactPaths {
    pub fn new(name: &str) -> ArtifactPaths {
        ArtifactPaths {
            name: name.to_string(),
            ..Default::default()
        }
    }
}
