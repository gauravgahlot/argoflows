use serde::{Deserialize, Serialize};

/// `Metrics` are a list of metrics emitted from a workflow or template.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Metrics {
    /// `Prometheus` is a list of prometheus metrics to be emitted.
    #[serde(rename = "prometheus")]
    pub prometheus: Vec<prometheus::Prometheus>,
}

impl Metrics {
    pub fn new(prometheus: Vec<prometheus::Prometheus>) -> Metrics {
        Metrics { prometheus }
    }
}

mod counter;
pub use self::counter::Counter;

mod gauge;
pub use self::gauge::Gauge;

mod histogram;
pub use self::histogram::Histogram;

mod prometheus;
pub use self::prometheus::Prometheus;
