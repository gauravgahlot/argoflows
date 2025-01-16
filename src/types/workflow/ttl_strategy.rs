use serde::{Deserialize, Serialize};

/// `TTLStrategy` is the strategy for the time to live depending on
/// if the workflow succeeded or failed.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct TTLStrategy {
    /// `SecondsAfterCompletion` is the number of seconds to live after completion
    #[serde(
        rename = "secondsAfterCompletion",
        skip_serializing_if = "Option::is_none"
    )]
    pub seconds_after_completion: Option<i32>,

    /// `SecondsAfterFailure` is the number of seconds to live after failure
    #[serde(
        rename = "secondsAfterFailure",
        skip_serializing_if = "Option::is_none"
    )]
    pub seconds_after_failure: Option<i32>,

    /// `SecondsAfterSuccess` is the number of seconds to live after success
    #[serde(
        rename = "secondsAfterSuccess",
        skip_serializing_if = "Option::is_none"
    )]
    pub seconds_after_success: Option<i32>,
}

impl TTLStrategy {
    pub fn new() -> Self {
        TTLStrategy {
            ..Default::default()
        }
    }
}
