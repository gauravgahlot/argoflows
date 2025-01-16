use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct HTTPHeader {
    #[serde(rename = "name")]
    pub name: String,

    #[serde(rename = "value", skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,

    #[serde(rename = "valueFrom", skip_serializing_if = "Option::is_none")]
    pub value_from: Option<Box<super::HTTPHeaderSource>>,
}

impl HTTPHeader {
    pub fn new(name: String) -> Self {
        HTTPHeader {
            name,
            ..Default::default()
        }
    }
}
