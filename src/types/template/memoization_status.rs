use serde::{Deserialize, Serialize};

/// `MemoizationStatus` is the status of this memoized node.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct MemoizationStatus {
    /// `Cache` is the name of the cache that was used.
    #[serde(rename = "cacheName")]
    pub cache_name: String,

    /// `Hit` indicates whether this node was created from a cache entry.
    #[serde(rename = "hit")]
    pub hit: bool,

    /// `Key` is the name of the key used for this node's cache.
    #[serde(rename = "key")]
    pub key: String,
}

impl MemoizationStatus {
    pub fn new(cache_name: &str, hit: bool, key: &str) -> Self {
        MemoizationStatus {
            cache_name,
            hit,
            key,
        }
    }
}
