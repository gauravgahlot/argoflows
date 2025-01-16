use serde::{Deserialize, Serialize};

/// `Sequence` expands a workflow step into numeric range.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Sequence {
    #[serde(rename = "count", skip_serializing_if = "Option::is_none")]
    pub count: Option<String>,

    #[serde(rename = "end", skip_serializing_if = "Option::is_none")]
    pub end: Option<String>,

    /// `Format` is a printf format string to format the value in the sequence.
    #[serde(rename = "format", skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,

    #[serde(rename = "start", skip_serializing_if = "Option::is_none")]
    pub start: Option<String>,
}

impl Sequence {
    pub fn new() -> Self {
        Sequence {
            ..Default::default()
        }
    }
}
