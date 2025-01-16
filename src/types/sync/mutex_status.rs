use serde::{Deserialize, Serialize};

/// `MutexStatus` contains which objects hold  mutex locks, and which objects
/// this workflow is waiting on to release locks.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct MutexStatus {
    /// `Holding` is a list of mutexes and their respective objects that are
    /// held by mutex lock for this io.argoproj.workflow.v1alpha1.
    #[serde(rename = "holding", skip_serializing_if = "Option::is_none")]
    pub holding: Option<Vec<super::MutexHolding>>,

    /// `Waiting` is a list of mutexes and their respective objects this
    /// workflow is waiting for.
    #[serde(rename = "waiting", skip_serializing_if = "Option::is_none")]
    pub waiting: Option<Vec<super::MutexHolding>>,
}

impl MutexStatus {
    pub fn new() -> Self {
        MutexStatus {
            ..Default::default()
        }
    }
}
