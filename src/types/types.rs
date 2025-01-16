use reqwest;
use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub struct ResponseContent<T> {
    pub status: reqwest::StatusCode,
    pub content: String,
    pub entity: Option<T>,
}

/// `CreateOptions` may be provided when creating an API object.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateOptions {
    #[serde(rename = "dryRun", skip_serializing_if = "Option::is_none")]
    pub dry_run: Option<Vec<String>>,

    #[serde(rename = "fieldManager", skip_serializing_if = "Option::is_none")]
    pub field_manager: Option<String>,

    #[serde(rename = "fieldValidation", skip_serializing_if = "Option::is_none")]
    pub field_validation: Option<String>,
}

impl CreateOptions {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }
}
