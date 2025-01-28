use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct ContainerSetRetryStrategy {
    /// `Duration` is the time between each retry, examples values are
    /// \"300ms\", \"1s\" or \"5m\". Valid time units are
    /// \"ns\", \"us\" (or \"µs\"), \"ms\", \"s\", \"m\", \"h\".
    #[serde(rename = "duration", skip_serializing_if = "Option::is_none")]
    pub duration: Option<String>,

    #[serde(rename = "retries")]
    pub retries: String,
}

impl ContainerSetRetryStrategy {
    pub fn new(retries: &str) -> Self {
        ContainerSetRetryStrategy {
            retries: retries.to_string(),
            ..Default::default()
        }
    }
}
