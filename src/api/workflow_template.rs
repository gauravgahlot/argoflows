use k8s_openapi::apimachinery::pkg::apis::meta::v1 as metav1;

use crate::config::Config;
use crate::error::{
    workflow_template::{
        CreateWorkflowTemplateError, DeleteWorkflowTemplateError, GetWorkflowTemplateError,
        ListWorkflowTemplatesError,
    },
    Error,
};
use crate::types::{
    workflow_template::{CreateRequest, WorkflowTemplate, WorkflowTemplateList},
    ListOptions, ResponseContent,
};

use super::*;

pub fn create_workflow_template(
    config: &Config,
    namespace: &str,
    body: CreateRequest,
) -> Result<WorkflowTemplate, Error<CreateWorkflowTemplateError>> {
    let uri = format!(
        "{}/api/v1/workflow-templates/{namespace}",
        config.host,
        namespace = url::encode(namespace)
    );

    let mut req_builder = config.client.request(reqwest::Method::POST, uri.as_str());
    req_builder = req_builder.json(&body);

    let req = req_builder.build()?;
    let resp = config.client.execute(req)?;
    let status = resp.status();
    let content = resp.text()?;

    if !status.is_client_error() && !status.is_server_error() {
        serde_json::from_str(&content).map_err(Error::from)
    } else {
        let entity: Option<CreateWorkflowTemplateError> = serde_json::from_str(&content).ok();
        let error = ResponseContent {
            status,
            content,
            entity,
        };
        Err(Error::Response(error))
    }
}

pub fn delete_workflow_template(
    config: &Config,
    namespace: &str,
    name: &str,
    delete_options: Option<metav1::DeleteOptions>,
) -> Result<serde_json::Value, Error<DeleteWorkflowTemplateError>> {
    let uri = format!(
        "{}/api/v1/workflow-templates/{namespace}/{name}",
        config.host,
        namespace = url::encode(namespace),
        name = url::encode(name)
    );
    let mut req_builder = config.client.request(reqwest::Method::DELETE, uri.as_str());

    let delete_options = delete_options.unwrap_or_default();

    if let Some(ref grace_period) = delete_options.grace_period_seconds {
        req_builder = req_builder.query(&[(
            "deleteOptions.gracePeriodSeconds",
            &grace_period.to_string(),
        )]);
    }

    if let Some(ref dependants) = delete_options.orphan_dependents {
        req_builder =
            req_builder.query(&[("deleteOptions.orphanDependents", &dependants.to_string())]);
    }
    if let Some(ref policy) = delete_options.propagation_policy {
        req_builder =
            req_builder.query(&[("deleteOptions.propagationPolicy", &policy.to_string())]);
    }

    let preconditions = delete_options.preconditions.unwrap_or_default();
    if let Some(ref uid) = preconditions.uid {
        req_builder = req_builder.query(&[("deleteOptions.preconditions.uid", &uid.to_string())]);
    }
    if let Some(ref version) = preconditions.resource_version {
        req_builder = req_builder.query(&[(
            "deleteOptions.preconditions.resourceVersion",
            &version.to_string(),
        )]);
    }

    if let Some(ref val) = delete_options.dry_run {
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

    if let Some(bearer_token) = &config.bearer_token {
        req_builder = req_builder.bearer_auth(bearer_token);
    }

    let req = req_builder.build()?;
    let resp = config.client.execute(req)?;
    let status = resp.status();
    let content = resp.text()?;

    if !status.is_client_error() && !status.is_server_error() {
        serde_json::from_str(&content).map_err(Error::from)
    } else {
        let entity: Option<DeleteWorkflowTemplateError> = serde_json::from_str(&content).ok();
        let error = ResponseContent {
            status,
            content,
            entity: entity,
        };
        Err(Error::Response(error))
    }
}

pub fn get_workflow_template(
    config: &Config,
    namespace: &str,
    name: &str,
    resource_version: Option<&str>,
) -> Result<WorkflowTemplate, Error<GetWorkflowTemplateError>> {
    let uri = format!(
        "{}/api/v1/workflow-templates/{namespace}/{name}",
        config.host,
        namespace = url::encode(namespace),
        name = url::encode(name)
    );

    let mut req_builder = config.client.request(reqwest::Method::GET, uri.as_str());

    if let Some(ref version) = resource_version {
        req_builder = req_builder.query(&[("getOptions.resourceVersion", &version.to_string())]);
    }
    if let Some(bearer_token) = &config.bearer_token {
        req_builder = req_builder.bearer_auth(bearer_token);
    }

    let req = req_builder.build()?;
    let resp = config.client.execute(req)?;
    let status = resp.status();
    let content = resp.text()?;

    if !status.is_client_error() && !status.is_server_error() {
        serde_json::from_str(&content).map_err(Error::from)
    } else {
        let entity: Option<GetWorkflowTemplateError> = serde_json::from_str(&content).ok();
        let error = ResponseContent {
            status,
            content,
            entity,
        };
        Err(Error::Response(error))
    }
}

pub fn list_workflow_templates(
    config: &Config,
    namespace: &str,
    name_pattern: Option<&str>,
    list_options: Option<ListOptions>,
) -> Result<WorkflowTemplateList, Error<ListWorkflowTemplatesError>> {
    let uri = format!(
        "{}/api/v1/workflow-templates/{namespace}",
        config.host,
        namespace = url::encode(namespace)
    );

    let mut req_builder = config.client.request(reqwest::Method::GET, uri.as_str());

    if let Some(pattern) = name_pattern {
        req_builder = req_builder.query(&[("namePattern", pattern.to_string())]);
    }

    let list_options = list_options.unwrap_or_default();
    if let Some(ref val) = list_options.label_selector {
        req_builder = req_builder.query(&[("listOptions.labelSelector", val)]);
    }
    if let Some(ref val) = list_options.field_selector {
        req_builder = req_builder.query(&[("listOptions.fieldSelector", val)]);
    }
    if let Some(ref val) = list_options.watch {
        req_builder = req_builder.query(&[("listOptions.watch", val)]);
    }
    if let Some(ref val) = list_options.allow_watch_bookmarks {
        req_builder = req_builder.query(&[("listOptions.allowWatchBookmarks", val)]);
    }
    if let Some(ref val) = list_options.resource_version {
        req_builder = req_builder.query(&[("listOptions.resourceVersion", val)]);
    }
    if let Some(ref val) = list_options.resource_version_match {
        req_builder = req_builder.query(&[("listOptions.resourceVersionMatch", val)]);
    }
    if let Some(ref val) = list_options.timeout_seconds {
        req_builder = req_builder.query(&[("listOptions.timeoutSeconds", val)]);
    }
    if let Some(ref val) = list_options.limit {
        req_builder = req_builder.query(&[("listOptions.limit", val)]);
    }
    if let Some(ref val) = list_options.r#continue {
        req_builder = req_builder.query(&[("listOptions.continue", val)]);
    }

    if let Some(bearer_token) = &config.bearer_token {
        req_builder = req_builder.bearer_auth(bearer_token);
    }

    let req = req_builder.build()?;
    let resp = config.client.execute(req)?;
    let status = resp.status();
    let content = resp.text()?;

    if !status.is_client_error() && !status.is_server_error() {
        serde_json::from_str(&content).map_err(Error::from)
    } else {
        let entity: Option<ListWorkflowTemplatesError> = serde_json::from_str(&content).ok();
        let error = ResponseContent {
            status,
            content,
            entity,
        };
        Err(Error::Response(error))
    }
}
