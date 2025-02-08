use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct SubmitRequest {
    #[serde(rename = "namespace", skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,

    #[serde(rename = "resourceKind", skip_serializing_if = "Option::is_none")]
    pub resource_kind: Option<String>,

    #[serde(rename = "resourceName", skip_serializing_if = "Option::is_none")]
    pub resource_name: Option<String>,

    #[serde(rename = "submitOptions", skip_serializing_if = "Option::is_none")]
    pub submit_options: Option<Box<super::SubmitOptions>>,
}
