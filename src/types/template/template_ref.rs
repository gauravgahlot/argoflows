use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct TemplateRef {
    /// `ClusterScope` indicates the referred template is cluster
    /// scoped (i.e. a ClusterWorkflowTemplate).
    #[serde(rename = "clusterScope", skip_serializing_if = "Option::is_none")]
    pub cluster_scope: Option<bool>,

    /// `Name` is the resource name of the template.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// `Template` is the name of referred template in the resource.
    #[serde(rename = "template", skip_serializing_if = "Option::is_none")]
    pub template: Option<String>,
}

impl TemplateRef {
    pub fn new() -> Self {
        TemplateRef {
            ..Default::default()
        }
    }
}
