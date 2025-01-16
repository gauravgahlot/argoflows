use k8s_openapi::api::core;
use serde::{Deserialize, Serialize};

/// `BasicAuth` describes the secret selectors required for basic authentication.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct BasicAuth {
    #[serde(rename = "passwordSecret", skip_serializing_if = "Option::is_none")]
    pub password_secret: Option<Box<core::v1::SecretKeySelector>>,

    #[serde(rename = "usernameSecret", skip_serializing_if = "Option::is_none")]
    pub username_secret: Option<Box<core::v1::SecretKeySelector>>,
}

impl BasicAuth {
    pub fn new() -> Self {
        BasicAuth {
            ..Default::default()
        }
    }
}
