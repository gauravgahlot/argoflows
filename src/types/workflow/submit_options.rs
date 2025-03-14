use k8s_openapi::apimachinery::pkg::apis::meta::v1 as metav1;
use serde::{Deserialize, Serialize};

/// `SubmitOpts` are workflow submission options.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct SubmitOptions {
    /// `Annotations` adds to `metadata.labels`
    #[serde(rename = "annotations", skip_serializing_if = "Option::is_none")]
    pub annotations: Option<String>,

    /// `DryRun` validates the workflow on the client-side without creating it.
    /// This option is not supported in API
    #[serde(rename = "dryRun", skip_serializing_if = "Option::is_none")]
    pub dry_run: Option<bool>,

    /// `Entrypoint` overrides `spec.entrypoint`
    #[serde(rename = "entryPoint", skip_serializing_if = "Option::is_none")]
    pub entry_point: Option<String>,

    /// `GenerateName` overrides `metadata.generateName`
    #[serde(rename = "generateName", skip_serializing_if = "Option::is_none")]
    pub generate_name: Option<String>,

    /// `Labels` adds to `metadata.labels`
    #[serde(rename = "labels", skip_serializing_if = "Option::is_none")]
    pub labels: Option<String>,

    /// `Name` overrides `metadata.name`
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(rename = "ownerReference", skip_serializing_if = "Option::is_none")]
    pub owner_reference: Option<Box<metav1::OwnerReference>>,

    /// `Parameters` passes input parameters to workflow
    #[serde(rename = "parameters", skip_serializing_if = "Option::is_none")]
    pub parameters: Option<Vec<String>>,

    /// Set the `podPriorityClassName` of the workflow
    #[serde(
        rename = "podPriorityClassName",
        skip_serializing_if = "Option::is_none"
    )]
    pub pod_priority_class_name: Option<String>,

    /// `Priority` is used if controller is configured to process limited
    /// number of workflows in parallel, higher priority workflows are
    /// processed first.
    #[serde(rename = "priority", skip_serializing_if = "Option::is_none")]
    pub priority: Option<i32>,

    /// `ServerDryRun` validates the workflow on the server-side without
    /// creating it
    #[serde(rename = "serverDryRun", skip_serializing_if = "Option::is_none")]
    pub server_dry_run: Option<bool>,

    /// `ServiceAccount` runs all pods in the workflow using specified
    /// `ServiceAccount`.
    #[serde(rename = "serviceAccount", skip_serializing_if = "Option::is_none")]
    pub service_account: Option<String>,
}
