use k8s_openapi::api::core::v1 as corev1;
use k8s_openapi::apimachinery::pkg::apis::meta::v1 as metav1;

use crate::config::Config;
use crate::error::{workflow::*, Error};
use crate::types::LogOptions;
use crate::types::{workflow::*, ListOptions, ResponseContent};

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
    if let Some(val) = fields {
        req_builder = req_builder.query(&[("fields", val.to_string())]);
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
    if let Some(val) = list_options.r#continue {
        req_builder = req_builder.query(&[("listOptions.continue", val.to_string())]);
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

pub fn resubmit_workflow(
    config: &Config,
    namespace: &str,
    name: &str,
    body: ResubmitRequest,
) -> Result<Workflow, Error<ResubmitWorkflowError>> {
    let uri = format!(
        "{}/api/v1/workflows/{namespace}/{name}/resubmit",
        config.host,
        namespace = super::urlencode(namespace),
        name = super::urlencode(name)
    );

    let mut req_builder = config
        .client
        .request(reqwest::Method::PUT, uri.as_str())
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
        let entity: Option<ResubmitWorkflowError> = serde_json::from_str(&content).ok();
        let error = ResponseContent {
            status,
            content,
            entity,
        };
        Err(Error::Response(error))
    }
}

pub fn resume_workflow(
    config: &Config,
    namespace: &str,
    name: &str,
    body: ResumeRequest,
) -> Result<Workflow, Error<ResumeWorkflowError>> {
    let uri = format!(
        "{}/api/v1/workflows/{namespace}/{name}/resume",
        config.host,
        namespace = super::urlencode(namespace),
        name = super::urlencode(name)
    );
    let mut req_builder = config
        .client
        .request(reqwest::Method::PUT, uri.as_str())
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
        let entity: Option<ResumeWorkflowError> = serde_json::from_str(&content).ok();
        let error = ResponseContent {
            status: status,
            content: content,
            entity: entity,
        };
        Err(Error::Response(error))
    }
}

pub fn retry_workflow(
    config: &Config,
    namespace: &str,
    name: &str,
    body: RetryRequest,
) -> Result<Workflow, Error<RetryWorkflowError>> {
    let uri = format!(
        "{}/api/v1/workflows/{namespace}/{name}/retry",
        config.host,
        namespace = super::urlencode(namespace),
        name = super::urlencode(name)
    );
    let mut req_builder = config
        .client
        .request(reqwest::Method::PUT, uri.as_str())
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
        let entity: Option<RetryWorkflowError> = serde_json::from_str(&content).ok();
        let error = ResponseContent {
            status: status,
            content: content,
            entity: entity,
        };
        Err(Error::Response(error))
    }
}

pub fn set_workflow(
    config: &Config,
    namespace: &str,
    name: &str,
    body: SetRequest,
) -> Result<Workflow, Error<SetWorkflowError>> {
    let uri = format!(
        "{}/api/v1/workflows/{namespace}/{name}/set",
        config.host,
        namespace = super::urlencode(namespace),
        name = super::urlencode(name)
    );
    let mut req_builder = config
        .client
        .request(reqwest::Method::PUT, uri.as_str())
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
        let entity: Option<SetWorkflowError> = serde_json::from_str(&content).ok();
        let error = ResponseContent {
            status: status,
            content: content,
            entity: entity,
        };
        Err(Error::Response(error))
    }
}

pub fn stop_workflow(
    config: &Config,
    namespace: &str,
    name: &str,
    body: StopRequest,
) -> Result<Workflow, Error<StopWorkflowError>> {
    let uri = format!(
        "{}/api/v1/workflows/{namespace}/{name}/stop",
        config.host,
        namespace = super::urlencode(namespace),
        name = super::urlencode(name)
    );
    let mut req_builder = config
        .client
        .request(reqwest::Method::PUT, uri.as_str())
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
        let entity: Option<StopWorkflowError> = serde_json::from_str(&content).ok();
        let error = ResponseContent {
            status: status,
            content: content,
            entity: entity,
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

pub fn suspend_workflow(
    config: &Config,
    namespace: &str,
    name: &str,
    body: SuspendRequest,
) -> Result<Workflow, Error<SuspendWorkflowError>> {
    let uri = format!(
        "{}/api/v1/workflows/{namespace}/{name}/suspend",
        config.host,
        namespace = super::urlencode(namespace),
        name = super::urlencode(name)
    );

    let mut req_builder = config
        .client
        .request(reqwest::Method::PUT, uri.as_str())
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
        let entity: Option<SuspendWorkflowError> = serde_json::from_str(&content).ok();
        let error = ResponseContent {
            status,
            content,
            entity,
        };
        Err(Error::Response(error))
    }
}

pub fn terminate_workflow(
    config: &Config,
    namespace: &str,
    name: &str,
    body: TerminateRequest,
) -> Result<Workflow, Error<TerminateWorkflowError>> {
    let uri = format!(
        "{}/api/v1/workflows/{namespace}/{name}/terminate",
        config.host,
        namespace = super::urlencode(namespace),
        name = super::urlencode(name)
    );

    let mut req_builder = config
        .client
        .request(reqwest::Method::PUT, uri.as_str())
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
        let entity: Option<TerminateWorkflowError> = serde_json::from_str(&content).ok();
        let error = ResponseContent {
            status,
            content,
            entity,
        };
        Err(Error::Response(error))
    }
}

pub fn watch_events(
    config: &Config,
    namespace: &str,
    list_options: Option<ListOptions>,
) -> Result<corev1::Event, Error<WatchEventsError>> {
    let uri = format!(
        "{}/api/v1/stream/events/{namespace}",
        config.host,
        namespace = super::urlencode(namespace)
    );

    let mut req_builder = config.client.request(reqwest::Method::GET, uri.as_str());

    let list_options = list_options.unwrap_or_default();
    if let Some(val) = list_options.label_selector {
        req_builder = req_builder.query(&[("listOptions.labelSelector", val.to_string())]);
    }
    if let Some(val) = list_options.field_selector {
        req_builder = req_builder.query(&[("listOptions.fieldSelector", val.to_string())]);
    }
    if let Some(val) = list_options.watch {
        req_builder = req_builder.query(&[("listOptions.watch", val.to_string())]);
    }
    if let Some(val) = list_options.allow_watch_bookmarks {
        req_builder = req_builder.query(&[("listOptions.allowWatchBookmarks", val.to_string())]);
    }
    if let Some(val) = list_options.resource_version {
        req_builder = req_builder.query(&[("listOptions.resourceVersion", val.to_string())]);
    }
    if let Some(val) = list_options.resource_version_match {
        req_builder = req_builder.query(&[("listOptions.resourceVersionMatch", val.to_string())]);
    }
    if let Some(val) = list_options.timeout_seconds {
        req_builder = req_builder.query(&[("listOptions.timeoutSeconds", val.to_string())]);
    }
    if let Some(val) = list_options.limit {
        req_builder = req_builder.query(&[("listOptions.limit", val.to_string())]);
    }
    if let Some(val) = list_options.r#continue {
        req_builder = req_builder.query(&[("listOptions.continue", val.to_string())]);
    }
    if let Some(val) = list_options.send_initial_events {
        req_builder = req_builder.query(&[("listOptions.sendInitialEvents", val.to_string())]);
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
        let entity: Option<WatchEventsError> = serde_json::from_str(&content).ok();
        let error = ResponseContent {
            status,
            content,
            entity,
        };
        Err(Error::Response(error))
    }
}

pub fn watch_workflows(
    config: &Config,
    namespace: &str,
    list_options: Option<ListOptions>,
    fields: Option<&str>,
) -> Result<StreamWatchEvent, Error<WatchWorkflowsError>> {
    let uri = format!(
        "{}/api/v1/workflow-events/{namespace}",
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
    if let Some(val) = list_options.r#continue {
        req_builder = req_builder.query(&[("listOptions.continue", &val.to_string())]);
    }
    if let Some(val) = list_options.send_initial_events {
        req_builder = req_builder.query(&[("listOptions.sendInitialEvents", &val.to_string())]);
    }
    if let Some(val) = fields {
        req_builder = req_builder.query(&[("fields", &val.to_string())]);
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
        let entity: Option<WatchWorkflowsError> = serde_json::from_str(&content).ok();
        let error = ResponseContent {
            status,
            content,
            entity,
        };
        Err(Error::Response(error))
    }
}

pub fn workflow_logs(
    config: &Config,
    namespace: &str,
    name: &str,
    pod_name: Option<&str>,
    log_options: Option<LogOptions>,
    grep: Option<&str>,
    selector: Option<&str>,
) -> Result<StreamLogEntry, Error<WorkflowLogsError>> {
    let uri = format!(
        "{}/api/v1/workflows/{namespace}/{name}/log",
        config.host,
        namespace = super::urlencode(namespace),
        name = super::urlencode(name)
    );

    let mut req_builder = config.client.request(reqwest::Method::GET, uri.as_str());

    let log_options = log_options.unwrap_or_default();
    if let Some(val) = pod_name {
        req_builder = req_builder.query(&[("podName", &val.to_string())]);
    }
    if let Some(val) = log_options.container {
        req_builder = req_builder.query(&[("logOptions.container", &val.to_string())]);
    }
    if let Some(val) = log_options.follow {
        req_builder = req_builder.query(&[("logOptions.follow", &val.to_string())]);
    }
    if let Some(val) = log_options.previous {
        req_builder = req_builder.query(&[("logOptions.previous", &val.to_string())]);
    }
    if let Some(val) = log_options.since_seconds {
        req_builder = req_builder.query(&[("logOptions.sinceSeconds", &val.to_string())]);
    }
    if let Some(val) = log_options.since_time_period_seconds {
        req_builder = req_builder.query(&[("logOptions.sinceTime.seconds", &val.to_string())]);
    }
    if let Some(val) = log_options.since_time_period_nanos {
        req_builder = req_builder.query(&[("logOptions.sinceTime.nanos", &val.to_string())]);
    }
    if let Some(val) = log_options.timestamps {
        req_builder = req_builder.query(&[("logOptions.timestamps", &val.to_string())]);
    }
    if let Some(val) = log_options.tail_lines {
        req_builder = req_builder.query(&[("logOptions.tailLines", &val.to_string())]);
    }
    if let Some(val) = log_options.limit_bytes {
        req_builder = req_builder.query(&[("logOptions.limitBytes", &val.to_string())]);
    }
    if let Some(val) = log_options.insecure_skip_tls_verify_backend {
        req_builder =
            req_builder.query(&[("logOptions.insecureSkipTLSVerifyBackend", &val.to_string())]);
    }

    if let Some(val) = grep {
        req_builder = req_builder.query(&[("grep", &val.to_string())]);
    }

    if let Some(val) = selector {
        req_builder = req_builder.query(&[("selector", &val.to_string())]);
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
        let entity: Option<WorkflowLogsError> = serde_json::from_str(&content).ok();
        let error = ResponseContent {
            status,
            content,
            entity,
        };
        Err(Error::Response(error))
    }
}
