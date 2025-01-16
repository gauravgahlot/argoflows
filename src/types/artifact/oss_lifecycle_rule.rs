use serde::{Deserialize, Serialize};

/// `OSSLifecycleRule` specifies how to manage bucket's lifecycle
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct OSSLifecycleRule {
    /// `MarkDeletionAfterDays` is the number of days before
    /// we delete objects in the bucket.
    #[serde(
        rename = "markDeletionAfterDays",
        skip_serializing_if = "Option::is_none"
    )]
    pub mark_deletion_after_days: Option<i32>,

    /// `MarkInfrequentAccessAfterDays` is the number of days before we convert
    /// the objects in the bucket to Infrequent Access (IA) storage type.
    #[serde(
        rename = "markInfrequentAccessAfterDays",
        skip_serializing_if = "Option::is_none"
    )]
    pub mark_infrequent_access_after_days: Option<i32>,
}

impl OSSLifecycleRule {
    pub fn new() -> Self {
        OSSLifecycleRule {
            ..Default::default()
        }
    }
}
