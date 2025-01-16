use serde::{Deserialize, Serialize};

/// `RetryAffinity` prevents running steps on the same host.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct RetryAffinity {
    /// `RetryNodeAntiAffinity` is a placeholder for future expansion,
    /// only empty nodeAntiAffinity is allowed. In order to prevent
    /// running steps on the same host, it uses \"kubernetes.io/hostname\".
    #[serde(rename = "nodeAntiAffinity", skip_serializing_if = "Option::is_none")]
    pub node_anti_affinity: Option<serde_json::Value>,
}

impl RetryAffinity {
    pub fn new() -> Self {
        RetryAffinity {
            ..Default::default()
        }
    }
}


/// `RetryStrategy` provides controls on how to retry a workflow step.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct RetryStrategy {
    #[serde(rename = "affinity", skip_serializing_if = "Option::is_none")]
    pub affinity: Option<Box<super::RetryAffinity>>,

    #[serde(rename = "backoff", skip_serializing_if = "Option::is_none")]
    pub backoff: Option<Box<super::Backoff>>,

    /// `Expression` is a condition expression for when a node will be retried.
    /// If it evaluates to false, the node will not be retried and
    /// the retry strategy will be ignored.
    #[serde(rename = "expression", skip_serializing_if = "Option::is_none")]
    pub expression: Option<String>,

    #[serde(rename = "limit", skip_serializing_if = "Option::is_none")]
    pub limit: Option<String>,

    /// `RetryPolicy` is a policy of `NodePhase` statuses that will be retried.
    #[serde(rename = "retryPolicy", skip_serializing_if = "Option::is_none")]
    pub retry_policy: Option<String>,
}

impl RetryStrategy {
    pub fn new() -> Self {
        RetryStrategy {
            ..Default::default()
        }
    }
}
