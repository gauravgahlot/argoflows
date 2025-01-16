use k8s_openapi::api::core;
use serde::{Deserialize, Serialize};

/// `OAuth2Auth` holds all information for client authentication via OAuth2 tokens.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct OAuth2Auth {
    #[serde(rename = "clientIDSecret", skip_serializing_if = "Option::is_none")]
    pub client_id_secret: Option<Box<core::v1::SecretKeySelector>>,

    #[serde(rename = "clientSecretSecret", skip_serializing_if = "Option::is_none")]
    pub client_secret_secret: Option<Box<core::v1::SecretKeySelector>>,

    #[serde(rename = "endpointParams", skip_serializing_if = "Option::is_none")]
    pub endpoint_params: Option<Vec<super::OAuth2EndpointParam>>,

    #[serde(rename = "scopes", skip_serializing_if = "Option::is_none")]
    pub scopes: Option<Vec<String>>,

    #[serde(rename = "tokenURLSecret", skip_serializing_if = "Option::is_none")]
    pub token_url_secret: Option<Box<core::v1::SecretKeySelector>>,
}

impl OAuth2Auth {
    pub fn new() -> Self {
        OAuth2Auth {
            ..Default::default()
        }
    }
}
