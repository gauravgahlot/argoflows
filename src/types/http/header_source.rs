use k8s_openapi::api::core;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct HTTPHeaderSource {
    #[serde(rename = "secretKeyRef", skip_serializing_if = "Option::is_none")]
    pub secret_key_ref: Option<Box<core::v1::SecretKeySelector>>,
}

impl HTTPHeaderSource {
    pub fn new() -> Self {
        HTTPHeaderSource {
            ..Default::default()
        }
    }
}
