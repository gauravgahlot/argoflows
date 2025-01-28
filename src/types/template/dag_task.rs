use crate::types::workflow;
use serde::{Deserialize, Serialize};

/// `DAGTask` represents a node in the graph during DAG execution.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct DAGTask {
    #[serde(rename = "arguments", skip_serializing_if = "Option::is_none")]
    pub arguments: Option<Box<super::Arguments>>,

    #[serde(rename = "continueOn", skip_serializing_if = "Option::is_none")]
    pub continue_on: Option<Box<super::ContinueOn>>,

    /// `Dependencies` are name of other targets which this depends on.
    #[serde(rename = "dependencies", skip_serializing_if = "Option::is_none")]
    pub dependencies: Option<Vec<String>>,

    /// `Depends` are name of other targets which this depends on
    #[serde(rename = "depends", skip_serializing_if = "Option::is_none")]
    pub depends: Option<String>,

    /// `Hooks` hold the lifecycle hook which is invoked at lifecycle of task,
    /// irrespective of the success, failure, or error status of the primary task.
    #[serde(rename = "hooks", skip_serializing_if = "Option::is_none")]
    pub hooks: Option<::std::collections::HashMap<String, workflow::LifecycleHook>>,

    #[serde(rename = "inline", skip_serializing_if = "Option::is_none")]
    pub inline: Option<Box<super::Template>>,

    /// `Name` is the name of the target.
    #[serde(rename = "name")]
    pub name: String,

    /// `OnExit` is a template reference which is invoked at the end of the
    /// template, irrespective of the success, failure, or error of the primary
    /// template. DEPRECATED: Use Hooks[exit].Template instead.
    #[serde(rename = "onExit", skip_serializing_if = "Option::is_none")]
    pub on_exit: Option<String>,

    /// `Name` of template to execute.
    #[serde(rename = "template", skip_serializing_if = "Option::is_none")]
    pub template: Option<String>,

    #[serde(rename = "templateRef", skip_serializing_if = "Option::is_none")]
    pub template_ref: Option<Box<super::TemplateRef>>,

    /// `When` is an expression in which the task should conditionally execute.
    #[serde(rename = "when", skip_serializing_if = "Option::is_none")]
    pub when: Option<String>,

    /// `WithItems` expands a task into multiple parallel tasks from
    /// the items in the list.
    #[serde(rename = "withItems", skip_serializing_if = "Option::is_none")]
    pub with_items: Option<Vec<serde_json::Value>>,

    /// `WithParam` expands a task into multiple parallel tasks from the value
    /// in the parameter, which is expected to be a JSON list.
    #[serde(rename = "withParam", skip_serializing_if = "Option::is_none")]
    pub with_param: Option<String>,

    #[serde(rename = "withSequence", skip_serializing_if = "Option::is_none")]
    pub with_sequence: Option<Box<super::Sequence>>,
}

impl DAGTask {
    pub fn new(name: &str) -> Self {
        DAGTask {
            name: name.to_string(),
            ..Default::default()
        }
    }
}
