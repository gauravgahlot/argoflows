use std::collections::HashMap;

use serde::{Deserialize, Serialize};

/// `Metadata` defines the pod metadata.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Metadata {
    #[serde(rename = "annotations", skip_serializing_if = "Option::is_none")]
    pub annotations: Option<HashMap<String, String>>,

    #[serde(rename = "labels", skip_serializing_if = "Option::is_none")]
    pub labels: Option<HashMap<String, String>>,
}

impl Metadata {
    pub fn new(
        annotations: Option<HashMap<String, String>>,
        labels: Option<HashMap<String, String>>,
    ) -> Self {
        Metadata {
            annotations,
            labels,
        }
    }
}
