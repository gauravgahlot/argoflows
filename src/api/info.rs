use crate::config::Config;
use crate::error::{
    info::{CollectEventError, GetInfoError, GetUserInfoError, GetVersionError},
    Error,
};
use crate::types::{
    info::{CollectEventRequest, Info, UserInfo, Version},
    ResponseContent,
};

pub fn collect_event(
    config: &Config,
    body: CollectEventRequest,
) -> Result<serde_json::Value, Error<CollectEventError>> {
    let uri = format!("{}/api/v1/tracking/event", config.host);
    let mut req_builder = config.client.request(reqwest::Method::POST, uri.as_str());

    if let Some(bearer_token) = &config.bearer_token {
        req_builder = req_builder.bearer_auth(bearer_token);
    }

    req_builder = req_builder.json(&body);
    let req = req_builder.build()?;
    let res = config.client.execute(req)?;
    let status = res.status();
    let content = res.text()?;

    if !status.is_client_error() && !status.is_server_error() {
        serde_json::from_str(&content).map_err(Error::from)
    } else {
        let entity: Option<CollectEventError> = serde_json::from_str(&content).ok();
        let err = ResponseContent {
            status,
            content,
            entity,
        };

        Err(Error::Response(err))
    }
}

pub fn get_info(config: &Config) -> Result<Info, Error<GetInfoError>> {
    let uri = format!("{}/api/v1/info", config.host);
    let mut req_builder = config.client.request(reqwest::Method::GET, uri.as_str());

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
        let entity: Option<GetInfoError> = serde_json::from_str(&content).ok();
        let err = ResponseContent {
            status,
            content,
            entity,
        };

        Err(Error::Response(err))
    }
}

pub fn get_user_info(config: &Config) -> Result<UserInfo, Error<GetUserInfoError>> {
    let uri = format!("{}/api/v1/userinfo", config.host);
    let mut req_builder = config.client.request(reqwest::Method::GET, uri.as_str());

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
        let entity: Option<GetUserInfoError> = serde_json::from_str(&content).ok();
        let err = ResponseContent {
            status,
            content,
            entity,
        };

        Err(Error::Response(err))
    }
}

pub fn get_version(config: &Config) -> Result<Version, Error<GetVersionError>> {
    let uri = format!("{}/api/v1/version", config.host);

    let mut req_builder = config.client.request(reqwest::Method::GET, uri.as_str());

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
        let entity: Option<GetVersionError> = serde_json::from_str(&content).ok();
        let error = ResponseContent {
            status,
            content,
            entity,
        };
        Err(Error::Response(error))
    }
}
