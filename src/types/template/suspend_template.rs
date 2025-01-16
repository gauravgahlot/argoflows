use serde::{Deserialize, Serialize};

/// `SuspendTemplate` is a template subtype to suspend a workflow
/// at a predetermined point in time.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct SuspendTemplate {
    /// Duration is the seconds to wait before automatically resuming a template.
    /// Must be a string. Default unit is seconds. Could also be a Duration,
    /// e.g.: \"2m\", \"6h\".
    #[serde(rename = "duration", skip_serializing_if = "Option::is_none")]
    pub duration: Option<String>,
}

impl SuspendTemplate {
    pub fn new() -> Self {
        SuspendTemplate { duration: None }
    }
}
