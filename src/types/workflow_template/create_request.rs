use serde::{Deserialize, Serialize};

use crate::types::CreateOptions;

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateRequest {
    #[serde(rename = "createOptions", skip_serializing_if = "Option::is_none")]
    pub create_options: Option<Box<CreateOptions>>,

    #[serde(rename = "namespace", skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,

    #[serde(rename = "template", skip_serializing_if = "Option::is_none")]
    pub template: Option<Box<super::WorkflowTemplate>>,
}
