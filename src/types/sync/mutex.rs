use serde::{Deserialize, Serialize};

/// `Mutex` holds mutex configuration.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Mutex {
    /// Name of the mutex
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// `Namespace` is the namespace of the mutex,
    /// default: [namespace of workflow].
    #[serde(rename = "namespace", skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
}

impl Mutex {
    pub fn new() -> Self {
        Mutex {
            ..Default::default()
        }
    }
}
