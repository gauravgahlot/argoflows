use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct RetryRequest {
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(rename = "namespace", skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,

    #[serde(rename = "nodeFieldSelector", skip_serializing_if = "Option::is_none")]
    pub node_field_selector: Option<String>,

    #[serde(rename = "parameters", skip_serializing_if = "Option::is_none")]
    pub parameters: Option<Vec<String>>,

    #[serde(rename = "restartSuccessful", skip_serializing_if = "Option::is_none")]
    pub restart_successful: Option<bool>,
}
