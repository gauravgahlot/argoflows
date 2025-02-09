use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct SetRequest {
    #[serde(rename = "message", skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,

    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(rename = "namespace", skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,

    #[serde(rename = "nodeFieldSelector", skip_serializing_if = "Option::is_none")]
    pub node_field_selector: Option<String>,

    #[serde(rename = "outputParameters", skip_serializing_if = "Option::is_none")]
    pub output_parameters: Option<String>,

    #[serde(rename = "phase", skip_serializing_if = "Option::is_none")]
    pub phase: Option<String>,
}
