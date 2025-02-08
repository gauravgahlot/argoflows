use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct LintRequest {
    #[serde(rename = "namespace", skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,

    #[serde(rename = "workflow", skip_serializing_if = "Option::is_none")]
    pub workflow: Option<Box<super::Workflow>>,
}
