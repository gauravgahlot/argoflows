use serde::{Deserialize, Serialize};

use crate::types::{artifact, workflow};

/// `Outputs` hold parameters, artifacts, and results from a step.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Outputs {
    /// `Artifacts` holds the list of output artifacts produced by a step
    #[serde(rename = "artifacts", skip_serializing_if = "Option::is_none")]
    pub artifacts: Option<Vec<artifact::Artifact>>,

    /// `ExitCode` holds the exit code of a script template
    #[serde(rename = "exitCode", skip_serializing_if = "Option::is_none")]
    pub exit_code: Option<String>,

    /// `Parameters` holds the list of output parameters produced by a step
    #[serde(rename = "parameters", skip_serializing_if = "Option::is_none")]
    pub parameters: Option<Vec<workflow::Parameter>>,

    /// `Result` holds the result (stdout) of a script template
    #[serde(rename = "result", skip_serializing_if = "Option::is_none")]
    pub result: Option<String>,
}

impl Outputs {
    pub fn new() -> Self {
        Outputs {
            ..Default::default()
        }
    }
}
