use serde::{Deserialize, Serialize};

/// `Gauge` is a `Gauge` prometheus metric.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Gauge {
    /// `Operation` defines the operation to apply with value and the metrics'
    /// current value.
    #[serde(rename = "operation", skip_serializing_if = "Option::is_none")]
    pub operation: Option<String>,

    /// `Realtime` emits this metric in real time if applicable.
    #[serde(rename = "realtime")]
    pub realtime: bool,

    /// `Value` is the value to be used in the operation with the metric's
    /// current value.
    /// If no operation is set, value is the value of the metric.
    #[serde(rename = "value")]
    pub value: String,
}

impl Gauge {
    pub fn new(realtime: bool, value: &str) -> Gauge {
        Gauge {
            realtime,
            value: value.to_string(),
            ..Default::default()
        }
    }
}
