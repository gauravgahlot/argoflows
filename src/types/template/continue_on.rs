use serde::{Deserialize, Serialize};

/// `ContinueOn` defines if a workflow should continue even if a task or step
/// fails/errors. It can be specified if the workflow should continue when
/// the pod errors, fails or both.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct ContinueOn {
    #[serde(rename = "error", skip_serializing_if = "Option::is_none")]
    pub error: Option<bool>,

    #[serde(rename = "failed", skip_serializing_if = "Option::is_none")]
    pub failed: Option<bool>,
}

impl ContinueOn {
    pub fn new() -> Self {
        ContinueOn {
            ..Default::default()
        }
    }
}
