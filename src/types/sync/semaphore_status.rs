use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct SemaphoreStatus {
    /// `Holding` stores the list of resource acquired synchronization
    /// lock for workflows.
    #[serde(rename = "holding", skip_serializing_if = "Option::is_none")]
    pub holding: Option<Vec<super::SemaphoreHolding>>,

    /// `Waiting` indicates the list of current synchronization lock holders.
    #[serde(rename = "waiting", skip_serializing_if = "Option::is_none")]
    pub waiting: Option<Vec<super::SemaphoreHolding>>,
}

impl SemaphoreStatus {
    pub fn new() -> Self {
        SemaphoreStatus {
            ..Default::default()
        }
    }
}
