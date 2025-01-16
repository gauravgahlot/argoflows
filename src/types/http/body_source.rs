use serde::{Deserialize, Serialize};

/// `HTTPBodySource` contains the source of the HTTP body.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct HTTPBodySource {
    #[serde(rename = "bytes", skip_serializing_if = "Option::is_none")]
    pub bytes: Option<String>,
}

impl HTTPBodySource {
    pub fn new() -> Self {
        HTTPBodySource {
            ..Default::default()
        }
    }
}
