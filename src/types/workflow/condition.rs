use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Condition {
    /// `Message` is the condition message.
    #[serde(rename = "message", skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,

    /// `Status` is the status of the condition.
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,

    /// `Type` is the type of condition.
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

impl Condition {
    pub fn new() -> Self {
        Condition {
            ..Default::default()
        }
    }
}
