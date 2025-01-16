use k8s_openapi::api::core;
use serde::{Deserialize, Serialize};

/// `ValueFrom` describes a location in which to obtain the value to a parameter.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct ValueFrom {
    #[serde(rename = "configMapKeyRef", skip_serializing_if = "Option::is_none")]
    pub config_map_key_ref: Option<Box<core::v1::ConfigMapKeySelector>>,

    /// `Default` specifies a value to be used if retrieving the value from the
    /// specified source fails.
    #[serde(rename = "default", skip_serializing_if = "Option::is_none")]
    pub default: Option<String>,

    /// Selector (https://github.com/antonmedv/expr) that is evaluated against
    /// the event to get the value of the parameter. E.g. `payload.message`.
    #[serde(rename = "event", skip_serializing_if = "Option::is_none")]
    pub event: Option<String>,

    /// `Expression`, if defined, is evaluated to specify the value for
    /// the parameter.
    #[serde(rename = "expression", skip_serializing_if = "Option::is_none")]
    pub expression: Option<String>,

    /// `JQFilter` expression against the resource object in resource templates.
    #[serde(rename = "jqFilter", skip_serializing_if = "Option::is_none")]
    pub jq_filter: Option<String>,

    /// `JSONPath` of a resource to retrieve an output parameter value from in
    /// resource templates.
    #[serde(rename = "jsonPath", skip_serializing_if = "Option::is_none")]
    pub json_path: Option<String>,

    /// `Parameter` reference to a step or dag task in which to retrieve an
    /// output parameter value from (e.g. '{{steps.mystep.outputs.myparam}}').
    #[serde(rename = "parameter", skip_serializing_if = "Option::is_none")]
    pub parameter: Option<String>,

    /// `Path` in the container to retrieve an output parameter value from
    /// in container templates.
    #[serde(rename = "path", skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,

    /// SuppliedValueFrom is a placeholder for a value to be filled in directly,
    /// either through the CLI, API, etc.
    #[serde(rename = "supplied", skip_serializing_if = "Option::is_none")]
    pub supplied: Option<serde_json::Value>,
}

impl ValueFrom {
    pub fn new() -> Self {
        ValueFrom {
            ..Default::default()
        }
    }
}
