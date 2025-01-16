use k8s_openapi::api::core;
use serde::{Deserialize, Serialize};

/// `ClientCertAuth` holds necessary information for client
/// authentication via certificates.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct ClientCertAuth {
    #[serde(rename = "clientCertSecret", skip_serializing_if = "Option::is_none")]
    pub client_cert_secret: Option<Box<core::v1::SecretKeySelector>>,

    #[serde(rename = "clientKeySecret", skip_serializing_if = "Option::is_none")]
    pub client_key_secret: Option<Box<core::v1::SecretKeySelector>>,
}

impl ClientCertAuth {
    pub fn new() -> Self {
        ClientCertAuth {
            ..Default::default()
        }
    }
}
