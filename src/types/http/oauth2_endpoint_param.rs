use serde::{Deserialize, Serialize};

/// `EndpointParam` is for requesting optional fields that should
/// be sent in the oauth request.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OAuth2EndpointParam {
    /// `Name` is the header name
    #[serde(rename = "key")]
    pub key: String,

    /// `Value` is the literal value to use for the header
    #[serde(rename = "value", skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

impl OAuth2EndpointParam {
    pub fn new(key: &str, value: Option<String>) -> Self {
        OAuth2EndpointParam { key, value }
    }
}
