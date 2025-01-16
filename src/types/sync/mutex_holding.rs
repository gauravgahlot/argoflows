use serde::{Deserialize, Serialize};

/// `MutexHolding` describes the mutex and the object which is holding it.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct MutexHolding {
    /// Holder is a reference to the object which holds the Mutex.
    /// Holding Scenario:
    ///    1. Current workflow's NodeID which is holding the lock.
    ///       e.g: ${NodeID}
    /// Waiting Scenario:
    ///     1. Current workflow or other workflow NodeID which is holding the lock.
    ///       e.g: ${WorkflowName}/${NodeID}
    #[serde(rename = "holder", skip_serializing_if = "Option::is_none")]
    pub holder: Option<String>,

    /// `Reference` for the mutex e.g: ${namespace}/mutex/${mutexName}.
    #[serde(rename = "mutex", skip_serializing_if = "Option::is_none")]
    pub mutex: Option<String>,
}

impl MutexHolding {
    pub fn new() -> Self {
        MutexHolding {
            ..Default::default()
        }
    }
}
