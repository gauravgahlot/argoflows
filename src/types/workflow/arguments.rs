use crate::types::artifact::Artifact;
use serde::{Deserialize, Serialize};

/// `Arguments` to a template.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Arguments {
    /// `Artifacts` is the list of artifacts to pass to the template or workflow.
    #[serde(rename = "artifacts", skip_serializing_if = "Option::is_none")]
    pub artifacts: Option<Vec<Artifact>>,

    /// `Parameters` is the list of parameters to pass to the template or workflow
    #[serde(rename = "parameters", skip_serializing_if = "Option::is_none")]
    pub parameters: Option<Vec<super::Parameter>>,
}

impl Arguments {
    pub fn new() -> Self {
        Arguments {
            ..Default::default()
        }
    }
}
