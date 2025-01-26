use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LabelValueFrom {
    #[serde(rename = "expression")]
    pub expression: String,
}

impl LabelValueFrom {
    pub fn new(expression: &str) -> LabelValueFrom {
        LabelValueFrom {
            expression,
        }
    }
}
