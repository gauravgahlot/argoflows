use k8s_openapi::api::core;

use serde::{Deserialize, Serialize};

/// `AzureArtifact` is the location of a an Azure Storage artifact.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct AzureArtifact {
    #[serde(rename = "accountKeySecret", skip_serializing_if = "Option::is_none")]
    pub account_key_secret: Option<Box<core::v1::SecretKeySelector>>,

    /// `Blob` is the blob name (i.e., path) in the container where the
    ///  artifact resides.
    #[serde(rename = "blob")]
    pub blob: String,

    /// `Container` is the container where resources will be stored.
    #[serde(rename = "container")]
    pub container: String,

    /// `Endpoint` is the service url associated with an account. It is most
    /// likely \"https://<ACCOUNT_NAME>.blob.core.windows.net\".
    #[serde(rename = "endpoint")]
    pub endpoint: String,

    /// `UseSDKCreds` tells the driver to figure out credentials based on SDK
    /// defaults.
    #[serde(rename = "useSDKCreds", skip_serializing_if = "Option::is_none")]
    pub use_sdk_creds: Option<bool>,
}

impl AzureArtifact {
    pub fn new(blob: &str, container: &str, endpoint: &str) -> Self {
        AzureArtifact {
            blob,
            container,
            endpoint,
            ..Default::default()
        }
    }
}
