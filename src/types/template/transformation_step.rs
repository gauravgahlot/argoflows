use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct TransformationStep {
    /// `Expression` defines an expr expression to apply.
    #[serde(rename = "expression")]
    pub expression: String,
}

impl TransformationStep {
    pub fn new(expression: String) -> Self {
        TransformationStep { expression }
    }
}
