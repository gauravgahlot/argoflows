use serde::{Deserialize, Serialize};

/// `Backoff` is a backoff strategy to use within `RetryStrategy`.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Backoff {
    /// `Duration` is the amount to back off. Default unit is seconds,
    /// but could also be a duration (e.g. \"2m\", \"1h\").
    #[serde(rename = "duration", skip_serializing_if = "Option::is_none")]
    pub duration: Option<String>,

    #[serde(rename = "factor", skip_serializing_if = "Option::is_none")]
    pub factor: Option<String>,

    /// `MaxDuration` is the maximum amount of time allowed for a
    /// workflow in the backoff strategy.
    #[serde(rename = "maxDuration", skip_serializing_if = "Option::is_none")]
    pub max_duration: Option<String>,
}

impl Backoff {
    pub fn new() -> Self {
        Backoff {
            ..Default::default()
        }
    }
}
