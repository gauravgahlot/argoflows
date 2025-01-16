use serde::{Deserialize, Serialize};

/// `Counter` is the `Counter` prometheus metric.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Counter {
    /// `Value` is the value of the metric.
    #[serde(rename = "value")]
    pub value: String,
}

impl Counter {
    pub fn new(value: String) -> Counter {
        Counter { value }
    }
}
