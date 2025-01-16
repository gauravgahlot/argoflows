use serde::{Deserialize, Serialize};

/// `SynchronizationStatus` stores the status of semaphore and mutex.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct SynchronizationStatus {
    #[serde(rename = "mutex", skip_serializing_if = "Option::is_none")]
    pub mutex: Option<Box<super::MutexStatus>>,

    #[serde(rename = "semaphore", skip_serializing_if = "Option::is_none")]
    pub semaphore: Option<Box<super::SemaphoreStatus>>,
}

impl SynchronizationStatus {
    pub fn new() -> Self {
        SynchronizationStatus {
            ..Default::default()
        }
    }
}
