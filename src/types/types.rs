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

#[derive(Default)]
pub struct ListOptions {
    pub label_selector: Option<String>,
    pub field_selector: Option<String>,
    pub watch: Option<bool>,
    pub allow_watch_bookmarks: Option<bool>,
    pub resource_version: Option<String>,
    pub resource_version_match: Option<String>,
    pub timeout_seconds: Option<String>,
    pub limit: Option<String>,
    pub r#continue: Option<String>,
    pub send_initial_events: Option<bool>,
}

#[derive(Default)]
pub struct LogOptions {
    pub container: Option<String>,
    pub follow: Option<bool>,
    pub previous: Option<bool>,
    pub since_seconds: Option<String>,
    pub since_time_period_seconds: Option<String>,
    pub since_time_period_nanos: Option<i32>,
    pub timestamps: Option<bool>,
    pub tail_lines: Option<String>,
    pub limit_bytes: Option<String>,
    pub insecure_skip_tls_verify_backend: Option<bool>,
}

#[derive(Default)]
pub struct Preconditions {
    pub uid: Option<String>,
    pub resource_version: Option<String>,
}
