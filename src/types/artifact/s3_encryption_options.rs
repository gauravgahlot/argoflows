use k8s_openapi::api::core;
use serde::{Deserialize, Serialize};

/// `S3EncryptionOptions` used to determine encryption options during s3 operations.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct S3EncryptionOptions {
    /// `EnableEncryption` tells the driver to encrypt objects if set to true.
    /// If `KMSKeyId` and `ServerSideCustomerKeySecret` are not set, SSE-S3 will be used.
    #[serde(rename = "enableEncryption", skip_serializing_if = "Option::is_none")]
    pub enable_encryption: Option<bool>,

    /// `KMSEncryptionContext` is a json blob that contains an encryption context.
    /// See https://docs.aws.amazon.com/kms/latest/developerguide/concepts.html#encrypt_context
    /// for more information.
    #[serde(
        rename = "kmsEncryptionContext",
        skip_serializing_if = "Option::is_none"
    )]
    pub kms_encryption_context: Option<String>,

    /// `KMSKeyId` tells the driver to encrypt the object using the specified KMS Key.
    #[serde(rename = "kmsKeyId", skip_serializing_if = "Option::is_none")]
    pub kms_key_id: Option<String>,

    #[serde(
        rename = "serverSideCustomerKeySecret",
        skip_serializing_if = "Option::is_none"
    )]
    pub server_side_customer_key_secret: Option<Box<core::v1::SecretKeySelector>>,
}

impl S3EncryptionOptions {
    pub fn new() -> Self {
        S3EncryptionOptions {
            ..Default::default()
        }
    }
}
