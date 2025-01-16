use serde::{Deserialize, Serialize};

/// `Data` is a data template.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Data {
    #[serde(rename = "source")]
    pub source: Box<super::DataSource>,

    /// `Transformation` applies a set of transformations.
    #[serde(rename = "transformation")]
    pub transformation: Vec<super::TransformationStep>,
}

impl Data {
    pub fn new(source: super::DataSource, transformation: Vec<super::TransformationStep>) -> Self {
        Data {
            source: Box::new(source),
            transformation,
        }
    }
}
