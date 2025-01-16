use std::collections::HashMap;

use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct WorkflowMetadata {
    #[serde(rename = "annotations", skip_serializing_if = "Option::is_none")]
    pub annotations: Option<HashMap<String, String>>,

    #[serde(rename = "labels", skip_serializing_if = "Option::is_none")]
    pub labels: Option<HashMap<String, String>>,

    #[serde(rename = "labelsFrom", skip_serializing_if = "Option::is_none")]
    pub labels_from: Option<HashMap<String, super::LabelValueFrom>>,
}

impl WorkflowMetadata {
    pub fn new() -> Self {
        WorkflowMetadata {
            ..Default::default()
        }
    }
}
