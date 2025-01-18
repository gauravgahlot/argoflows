use serde::{Deserialize, Serialize};

use crate::types::CreateOptions;

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateRequest {
    #[serde(rename = "createOptions", skip_serializing_if = "Option::is_none")]
    pub create_options: Option<Box<CreateOptions>>,

    /// This field is no longer used.
    #[serde(rename = "instanceID", skip_serializing_if = "Option::is_none")]
    pub instance_id: Option<String>,

    #[serde(rename = "namespace", skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,

    #[serde(rename = "serverDryRun", skip_serializing_if = "Option::is_none")]
    pub server_dry_run: Option<bool>,

    #[serde(rename = "workflow", skip_serializing_if = "Option::is_none")]
    pub workflow: Option<Box<super::Workflow>>,
}
