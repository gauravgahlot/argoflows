use serde::{Deserialize, Serialize};

/// `ArtGCStatus` maintains state related to `ArtifactGC`.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct ArtGCStatus {
    /// If this is true, we already checked to see if we need to do it and we don't.
    #[serde(rename = "notSpecified", skip_serializing_if = "Option::is_none")]
    pub not_specified: Option<bool>,

    /// Have completed Pods been processed? (mapped by Pod name) used to
    /// prevent re-processing the Status of a Pod more than once.
    #[serde(rename = "podsRecouped", skip_serializing_if = "Option::is_none")]
    pub pods_recouped: Option<::std::collections::HashMap<String, bool>>,

    /// Have Pods been started to perform this strategy?
    /// (enables us not to re-process what we've already done).
    #[serde(
        rename = "strategiesProcessed",
        skip_serializing_if = "Option::is_none"
    )]
    pub strategies_processed: Option<::std::collections::HashMap<String, bool>>,
}

impl ArtGCStatus {
    pub fn new() -> Self {
        ArtGCStatus {
            ..Default::default()
        }
    }
}
