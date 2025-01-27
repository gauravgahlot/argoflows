use k8s_openapi::api::core;

use serde::{Deserialize, Serialize};

/// `AzureArtifactRepository` defines the controller configuration for an Azure
/// Blob Storage artifact repository.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct AzureArtifactRepository {
    #[serde(rename = "accountKeySecret", skip_serializing_if = "Option::is_none")]
    pub account_key_secret: Option<Box<core::v1::SecretKeySelector>>,

    /// `BlobNameFormat` is defines the format of how to store blob names.
    /// Can reference workflow variables.
    #[serde(rename = "blobNameFormat", skip_serializing_if = "Option::is_none")]
    pub blob_name_format: Option<String>,

    /// `Container` is the container where resources will be stored
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

impl AzureArtifactRepository {
    pub fn new(container: &str, endpoint: &str) -> Self {
        AzureArtifactRepository {
            container: container.to_string(),
            endpoint: endpoint.to_string(),
            ..Default::default()
        }
    }
}
