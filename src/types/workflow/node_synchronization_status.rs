use serde::{Deserialize, Serialize};

/// `NodeSynchronizationStatus` stores the status of a node.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct NodeSynchronizationStatus {
    /// `Waiting` is the name of the lock that this node is waiting for.
    #[serde(rename = "waiting", skip_serializing_if = "Option::is_none")]
    pub waiting: Option<String>,
}

impl NodeSynchronizationStatus {
    pub fn new(waiting: &str) -> Self {
        NodeSynchronizationStatus {
            waiting: Some(waiting.to_string()),
        }
    }
}
