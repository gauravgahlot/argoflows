use serde::{Deserialize, Serialize};

/// `Header` indicate a key-value request header to be used
/// when fetching artifacts over HTTP.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Header {
    /// `Name` is the header name.
    #[serde(rename = "name")]
    pub name: String,

    /// `Value` is the literal value to use for the header.
    #[serde(rename = "value")]
    pub value: String,
}

impl Header {
    pub fn new(name: &str, value: &str) -> Self {
        Header { name, value }
    }
}
