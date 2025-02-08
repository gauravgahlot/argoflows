use k8s_openapi::apimachinery::pkg::apis::meta::v1 as metav1;

use crate::config::Config;
use crate::error::{
    workflow::{
        CreateWorkflowError, DeleteWorkflowError, GetWorkflowError, LintWorkflowError,
        ListWorkflowsError, SubmitWorkflowError,
    },
    Error,
};
use crate::types::{
    workflow::{CreateRequest, LintRequest, SubmitRequest, Workflow, WorkflowList},
    ListOptions, ResponseContent,
};

pub fn create_workflow(
    config: &Config,
    namespace: &str,
    body: CreateRequest,
) -> Result<Workflow, Error<CreateWorkflowError>> {
    let uri = format!(
        "{}/api/v1/workflows/{namespace}",
        config.host,
        namespace = super::urlencode(namespace)
    );

    let mut req_builder = config
        .client
        .request(reqwest::Method::POST, uri.as_str())
        .json(&body);

    if let Some(bearer_token) = &config.bearer_token {
        req_builder = req_builder.bearer_auth(bearer_token);
    }

    let req = req_builder.build()?;
    let res = config.client.execute(req)?;
    let status = res.status();
    let content = res.text()?;

    if !status.is_client_error() && !status.is_server_error() {
        serde_json::from_str(&content).map_err(Error::from)
    } else {
        let entity: Option<CreateWorkflowError> = serde_json::from_str(&content).ok();
        let error = ResponseContent {
            status,
            content,
            entity,
        };
        Err(Error::Response(error))
    }
}

pub fn delete_workflow(
    config: &Config,
    namespace: &str,
    name: &str,
    delete_options: Option<metav1::DeleteOptions>,
    force: bool,
) -> Result<serde_json::Value, Error<DeleteWorkflowError>> {
    let uri = format!(
        "{}/api/v1/workflows/{namespace}/{name}",
        config.host,
        namespace = super::urlencode(namespace),
        name = super::urlencode(name)
    );

    let mut req_builder = config.client.request(reqwest::Method::DELETE, uri.as_str());

    let delete_options = delete_options.unwrap_or_default();

    if let Some(grace_period) = delete_options.grace_period_seconds {
        req_builder = req_builder.query(&[(
            "deleteOptions.gracePeriodSeconds",
            &grace_period.to_string(),
        )]);
    }

    if let Some(dependants) = delete_options.orphan_dependents {
        req_builder =
            req_builder.query(&[("deleteOptions.orphanDependents", &dependants.to_string())]);
    }
    if let Some(policy) = delete_options.propagation_policy {
        req_builder =
            req_builder.query(&[("deleteOptions.propagationPolicy", &policy.to_string())]);
    }

    let preconditions = delete_options.preconditions.unwrap_or_default();
    if let Some(uid) = preconditions.uid {
        req_builder = req_builder.query(&[("deleteOptions.preconditions.uid", &uid.to_string())]);
    }
    if let Some(version) = preconditions.resource_version {
        req_builder = req_builder.query(&[(
            "deleteOptions.preconditions.resourceVersion",
            &version.to_string(),
        )]);
    }

    if let Some(val) = delete_options.dry_run {
        req_builder = match "multi" {
            "multi" => req_builder.query(
                &val.into_iter()
                    .map(|p| ("deleteOptions.dryRun".to_owned(), p.to_string()))
                    .collect::<Vec<(String, String)>>(),
            ),
            _ => req_builder.query(&[(
                "deleteOptions.dryRun",
                &val.into_iter()
                    .map(|p| p.to_string())
                    .collect::<Vec<String>>()
                    .join(",")
                    .to_string(),
            )]),
        };
    }

    if force {
        req_builder = req_builder.query(&[("force", &force.to_string())]);
    }

    if let Some(bearer_token) = &config.bearer_token {
        req_builder = req_builder.bearer_auth(bearer_token);
    }

    let req = req_builder.build()?;
    let res = config.client.execute(req)?;
    let status = res.status();
    let content = res.text()?;

    if !status.is_client_error() && !status.is_server_error() {
        serde_json::from_str(&content).map_err(Error::from)
    } else {
        let entity: Option<DeleteWorkflowError> = serde_json::from_str(&content).ok();
        let err = ResponseContent {
            status,
            content,
            entity,
        };
        Err(Error::Response(err))
    }
}

pub fn get_workflow(
    config: &Config,
    namespace: &str,
    name: &str,
    resource_version: Option<&str>,
    fields: Option<&str>,
) -> Result<Workflow, Error<GetWorkflowError>> {
    let uri = format!(
        "{}/api/v1/workflows/{namespace}/{name}",
        config.host,
        namespace = super::urlencode(namespace),
        name = super::urlencode(name)
    );

    let mut req_builder = config.client.request(reqwest::Method::GET, uri.as_str());

    if let Some(version) = resource_version {
        req_builder = req_builder.query(&[("getOptions.resourceVersion", &version.to_string())]);
    }
    if let Some(local_var_str) = fields {
        req_builder = req_builder.query(&[("fields", &local_var_str.to_string())]);
    }
    if let Some(bearer_token) = &config.bearer_token {
        req_builder = req_builder.bearer_auth(bearer_token);
    }

    let req = req_builder.build()?;
    let res = config.client.execute(req)?;
    let status = res.status();
    let content = res.text()?;

    if !status.is_client_error() && !status.is_server_error() {
        serde_json::from_str(&content).map_err(Error::from)
    } else {
        let entity: Option<GetWorkflowError> = serde_json::from_str(&content).ok();
        let error = ResponseContent {
            status,
            content,
            entity,
        };
        Err(Error::Response(error))
    }
}

pub fn lint_workflow(
    config: &Config,
    namespace: &str,
    body: LintRequest,
) -> Result<Workflow, Error<LintWorkflowError>> {
    let uri = format!(
        "{}/api/v1/workflows/{namespace}/lint",
        config.host,
        namespace = super::urlencode(namespace)
    );

    let mut req_builder = config
        .client
        .request(reqwest::Method::POST, uri.as_str())
        .json(&body);

    if let Some(bearer_token) = &config.bearer_token {
        req_builder = req_builder.bearer_auth(bearer_token);
    }

    let req = req_builder.build()?;
    let res = config.client.execute(req)?;
    let status = res.status();
    let content = res.text()?;

    if !status.is_client_error() && !status.is_server_error() {
        serde_json::from_str(&content).map_err(Error::from)
    } else {
        let entity: Option<LintWorkflowError> = serde_json::from_str(&content).ok();
        let error = ResponseContent {
            status,
            content,
            entity,
        };
        Err(Error::Response(error))
    }
}

pub fn list_workflows(
    config: &Config,
    namespace: &str,
    list_options: Option<ListOptions>,
    fields: Option<&str>,
    name_filter: Option<&str>,
) -> Result<WorkflowList, Error<ListWorkflowsError>> {
    let uri = format!(
        "{}/api/v1/workflows/{namespace}",
        config.host,
        namespace = super::urlencode(namespace)
    );

    let mut req_builder = config.client.request(reqwest::Method::GET, uri.as_str());

    let list_options = list_options.unwrap_or_default();
    if let Some(val) = list_options.label_selector {
        req_builder = req_builder.query(&[("listOptions.labelSelector", &val.to_string())]);
    }
    if let Some(val) = list_options.field_selector {
        req_builder = req_builder.query(&[("listOptions.fieldSelector", &val.to_string())]);
    }
    if let Some(val) = list_options.watch {
        req_builder = req_builder.query(&[("listOptions.watch", &val.to_string())]);
    }
    if let Some(val) = list_options.allow_watch_bookmarks {
        req_builder = req_builder.query(&[("listOptions.allowWatchBookmarks", &val.to_string())]);
    }
    if let Some(val) = list_options.resource_version {
        req_builder = req_builder.query(&[("listOptions.resourceVersion", &val.to_string())]);
    }
    if let Some(val) = list_options.resource_version_match {
        req_builder = req_builder.query(&[("listOptions.resourceVersionMatch", &val.to_string())]);
    }
    if let Some(val) = list_options.timeout_seconds {
        req_builder = req_builder.query(&[("listOptions.timeoutSeconds", &val.to_string())]);
    }
    if let Some(val) = list_options.limit {
        req_builder = req_builder.query(&[("listOptions.limit", &val.to_string())]);
    }
    if let Some(local_var_str) = list_options.r#continue {
        req_builder = req_builder.query(&[("listOptions.continue", &local_var_str.to_string())]);
    }
    if let Some(val) = list_options.send_initial_events {
        req_builder = req_builder.query(&[("listOptions.sendInitialEvents", &val.to_string())]);
    }
    if let Some(val) = fields {
        req_builder = req_builder.query(&[("fields", &val.to_string())]);
    }
    if let Some(val) = name_filter {
        req_builder = req_builder.query(&[("nameFilter", &val.to_string())]);
    }

    if let Some(bearer_token) = &config.bearer_token {
        req_builder = req_builder.bearer_auth(bearer_token);
    }

    let req = req_builder.build()?;
    let res = config.client.execute(req)?;
    let status = res.status();
    let content = res.text()?;

    if !status.is_client_error() && !status.is_server_error() {
        serde_json::from_str(&content).map_err(Error::from)
    } else {
        let entity: Option<ListWorkflowsError> = serde_json::from_str(&content).ok();
        let error = ResponseContent {
            status,
            content,
            entity,
        };
        Err(Error::Response(error))
    }
}

pub fn submit_workflow(
    config: &Config,
    namespace: &str,
    body: SubmitRequest,
) -> Result<Workflow, Error<SubmitWorkflowError>> {
    let uri = format!(
        "{}/api/v1/workflows/{namespace}/submit",
        config.host,
        namespace = super::urlencode(namespace)
    );

    let mut req_builder = config
        .client
        .request(reqwest::Method::POST, uri.as_str())
        .json(&body);

    if let Some(bearer_token) = &config.bearer_token {
        req_builder = req_builder.bearer_auth(bearer_token);
    }

    let req = req_builder.build()?;
    let res = config.client.execute(req)?;
    let status = res.status();
    let content = res.text()?;

    if !status.is_client_error() && !status.is_server_error() {
        serde_json::from_str(&content).map_err(Error::from)
    } else {
        let entity: Option<SubmitWorkflowError> = serde_json::from_str(&content).ok();
        let error = ResponseContent {
            status,
            content,
            entity,
        };
        Err(Error::Response(error))
    }
}
