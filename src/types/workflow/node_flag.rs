use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct NodeFlag {
    /// `Hooked` tracks whether or not this node was triggered by hook or onExit.
    #[serde(rename = "hooked", skip_serializing_if = "Option::is_none")]
    pub hooked: Option<bool>,

    /// `Retried` tracks whether or not this node was retried by retryStrategy.
    #[serde(rename = "retried", skip_serializing_if = "Option::is_none")]
    pub retried: Option<bool>,
}

impl NodeFlag {
    pub fn new() -> Self {
        NodeFlag {
            ..Default::default()
        }
    }
}
