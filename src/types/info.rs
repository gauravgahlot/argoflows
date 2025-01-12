use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct InfoResponse {
    #[serde(rename = "columns", skip_serializing_if = "Option::is_none")]
    pub columns: Option<Vec<Column>>,
    #[serde(rename = "links", skip_serializing_if = "Option::is_none")]
    pub links: Option<Vec<Link>>,
    #[serde(rename = "managedNamespace", skip_serializing_if = "Option::is_none")]
    pub managed_namespace: Option<String>,
    #[serde(rename = "modals", skip_serializing_if = "Option::is_none")]
    pub modals: Option<std::collections::HashMap<String, bool>>,
    #[serde(rename = "navColor", skip_serializing_if = "Option::is_none")]
    pub nav_color: Option<String>,
}

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct UserInfoResponse {
    #[serde(rename = "email", skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(rename = "emailVerified", skip_serializing_if = "Option::is_none")]
    pub email_verified: Option<bool>,
    #[serde(rename = "groups", skip_serializing_if = "Option::is_none")]
    pub groups: Option<Vec<String>>,
    #[serde(rename = "issuer", skip_serializing_if = "Option::is_none")]
    pub issuer: Option<String>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "serviceAccountName", skip_serializing_if = "Option::is_none")]
    pub service_account_name: Option<String>,
    #[serde(
        rename = "serviceAccountNamespace",
        skip_serializing_if = "Option::is_none"
    )]
    pub service_account_namespace: Option<String>,
    #[serde(rename = "subject", skip_serializing_if = "Option::is_none")]
    pub subject: Option<String>,
}

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Version {
    #[serde(rename = "buildDate")]
    pub build_date: String,
    #[serde(rename = "compiler")]
    pub compiler: String,
    #[serde(rename = "gitCommit")]
    pub git_commit: String,
    #[serde(rename = "gitTag")]
    pub git_tag: String,
    #[serde(rename = "gitTreeState")]
    pub git_tree_state: String,
    #[serde(rename = "goVersion")]
    pub go_version: String,
    #[serde(rename = "platform")]
    pub platform: String,
    #[serde(rename = "version")]
    pub version: String,
}

/// A custom `Column` that will be exposed in the Workflow List View.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Column {
    /// The key of the label or annotation, e.g., `workflows.argoproj.io/completed`.
    #[serde(rename = "key")]
    pub key: String,
    /// The name of this column, e.g., `Workflow Completed`.
    #[serde(rename = "name")]
    pub name: String,
    /// The type of this column, `label` or `annotation`.
    #[serde(rename = "type")]
    pub r#type: String,
}

/// A `Link` to another app.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Link {
    /// The name of the link, E.g. `Workflow Logs` or `Pod Logs`
    #[serde(rename = "name")]
    pub name: String,
    /// `workflow`, `pod`, `pod-logs`, `event-source-logs`, `sensor-logs`, `workflow-list` or `chat`
    #[serde(rename = "scope")]
    pub scope: String,
    /// The `URL` can contain `${metadata.namespace}`, `${metadata.name}`,
    /// `${status.startedAt}`, `${status.finishedAt}` or any other element
    /// in workflow yaml, e.g.
    /// `${io.argoproj.workflow.v1alpha1.metadata.annotations.userDefinedKey}`
    #[serde(rename = "url")]
    pub url: String,
}
