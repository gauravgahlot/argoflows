use k8s_openapi::api::core;
use serde::{Deserialize, Serialize};

/// `S3Artifact` is the location of an S3 artifact.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct S3Artifact {
    #[serde(rename = "accessKeySecret", skip_serializing_if = "Option::is_none")]
    pub access_key_secret: Option<Box<core::v1::SecretKeySelector>>,

    /// `Bucket` is the name of the bucket.
    #[serde(rename = "bucket", skip_serializing_if = "Option::is_none")]
    pub bucket: Option<String>,

    #[serde(rename = "caSecret", skip_serializing_if = "Option::is_none")]
    pub ca_secret: Option<Box<core::v1::SecretKeySelector>>,

    #[serde(
        rename = "createBucketIfNotPresent",
        skip_serializing_if = "Option::is_none"
    )]
    pub create_bucket_if_not_present: Option<Box<super::CreateS3BucketOptions>>,

    #[serde(rename = "encryptionOptions", skip_serializing_if = "Option::is_none")]
    pub encryption_options: Option<Box<super::S3EncryptionOptions>>,

    /// `Endpoint` is the hostname of the bucket endpoint.
    #[serde(rename = "endpoint", skip_serializing_if = "Option::is_none")]
    pub endpoint: Option<String>,

    /// `Insecure` will connect to the service with TLS.
    #[serde(rename = "insecure", skip_serializing_if = "Option::is_none")]
    pub insecure: Option<bool>,

    /// `Key` is the key in the bucket where the artifact resides.
    #[serde(rename = "key", skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,

    /// `Region` contains the optional bucket region.
    #[serde(rename = "region", skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,

    /// `RoleARN` is the Amazon Resource Name (ARN) of the role to assume.
    #[serde(rename = "roleARN", skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,

    #[serde(rename = "secretKeySecret", skip_serializing_if = "Option::is_none")]
    pub secret_key_secret: Option<Box<core::v1::SecretKeySelector>>,

    /// `UseSDKCreds` tells the driver to figure out credentials based on sdk defaults.
    #[serde(rename = "useSDKCreds", skip_serializing_if = "Option::is_none")]
    pub use_sdk_creds: Option<bool>,
}

impl S3Artifact {
    pub fn new() -> Self {
        S3Artifact {
            ..Default::default()
        }
    }
}
