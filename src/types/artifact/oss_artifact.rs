use k8s_openapi::api::core;
use serde::{Deserialize, Serialize};

/// `OSSArtifact` is the location of an Alibaba Cloud OSS artifact.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct OSSArtifact {
    #[serde(rename = "accessKeySecret", skip_serializing_if = "Option::is_none")]
    pub access_key_secret: Option<Box<core::v1::SecretKeySelector>>,

    /// `Bucket` is the name of the bucket
    #[serde(rename = "bucket", skip_serializing_if = "Option::is_none")]
    pub bucket: Option<String>,

    /// `CreateBucketIfNotPresent` tells the driver to attempt to create the
    /// OSS bucket for output artifacts, if it doesn't exist.
    #[serde(
        rename = "createBucketIfNotPresent",
        skip_serializing_if = "Option::is_none"
    )]
    pub create_bucket_if_not_present: Option<bool>,

    /// `Endpoint` is the hostname of the bucket endpoint
    #[serde(rename = "endpoint", skip_serializing_if = "Option::is_none")]
    pub endpoint: Option<String>,

    /// `Key` is the path in the bucket where the artifact resides
    #[serde(rename = "key")]
    pub key: String,

    #[serde(rename = "lifecycleRule", skip_serializing_if = "Option::is_none")]
    pub lifecycle_rule: Option<Box<super::OSSLifecycleRule>>,

    #[serde(rename = "secretKeySecret", skip_serializing_if = "Option::is_none")]
    pub secret_key_secret: Option<Box<core::v1::SecretKeySelector>>,

    /// `SecurityToken` is the user's temporary security token. For more details,
    /// check out: https://www.alibabacloud.com/help/doc-detail/100624.html.
    #[serde(rename = "securityToken", skip_serializing_if = "Option::is_none")]
    pub security_token: Option<String>,

    /// `UseSDKCreds` tells the driver to figure out credentials based on sdk defaults.
    #[serde(rename = "useSDKCreds", skip_serializing_if = "Option::is_none")]
    pub use_sdk_creds: Option<bool>,
}

impl OSSArtifact {
    pub fn new(key: String) -> Self {
        OSSArtifact {
            key,
            ..Default::default()
        }
    }
}
