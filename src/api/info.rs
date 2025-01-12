use crate::config::Config;
use crate::error::{
    info::{GetInfoError, GetVersionError, UserInfoError},
    Error,
};
use crate::types::{
    info::{InfoResponse, UserInfoResponse, Version},
    ResponseContent,
};

pub fn get_info(config: &Config) -> Result<InfoResponse, Error<GetInfoError>> {
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

pub fn get_user_info(config: &Config) -> Result<UserInfoResponse, Error<UserInfoError>> {
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
        let entity: Option<UserInfoError> = serde_json::from_str(&content).ok();
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
