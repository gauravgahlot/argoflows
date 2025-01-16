use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct LifecycleHook {
    #[serde(rename = "arguments", skip_serializing_if = "Option::is_none")]
    pub arguments: Option<Box<super::Arguments>>,

    /// Expression is a condition expression for when a node will be retried. If it
    /// evaluates to false, the node will not be retried and the retry strategy will be ignored
    #[serde(rename = "expression", skip_serializing_if = "Option::is_none")]
    pub expression: Option<String>,

    /// Template is the name of the template to execute by the hook
    #[serde(rename = "template", skip_serializing_if = "Option::is_none")]
    pub template: Option<String>,

    #[serde(rename = "templateRef", skip_serializing_if = "Option::is_none")]
    pub template_ref: Option<Box<super::TemplateRef>>,
}

impl LifecycleHook {
    pub fn new() -> Self {
        LifecycleHook {
            ..Default::default()
        }
    }
}
