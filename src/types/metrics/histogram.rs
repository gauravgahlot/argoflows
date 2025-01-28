use serde::{Deserialize, Serialize};

/// `Histogram` is a `Histogram` prometheus metric.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Histogram {
    /// `Buckets` is a list of bucket divisors for the histogram.
    #[serde(rename = "buckets")]
    pub buckets: Vec<f32>,

    /// `Value` is the value of the metric.
    #[serde(rename = "value")]
    pub value: String,
}

impl Histogram {
    pub fn new(buckets: Vec<f32>, value: &str) -> Histogram {
        Histogram { buckets, value: value.to_string() }
    }
}
