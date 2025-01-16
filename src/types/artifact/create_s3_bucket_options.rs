use serde::{Deserialize, Serialize};

/// `CreateS3BucketOptions` options used to determine
/// automatic automatic bucket-creation process.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct CreateS3BucketOptions {
    /// `ObjectLocking` Enable object locking.
    #[serde(rename = "objectLocking", skip_serializing_if = "Option::is_none")]
    pub object_locking: Option<bool>,
}

impl CreateS3BucketOptions {
    pub fn new() -> Self {
        CreateS3BucketOptions {
            ..Default::default()
        }
    }
}
