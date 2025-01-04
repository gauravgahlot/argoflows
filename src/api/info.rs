use crate::config::Config;
use crate::error::{info::GetVersionError, Error};
use crate::types::{info::Version, ResponseContent};

pub fn get_version(config: &Config) -> Result<Version, Error<GetVersionError>> {
    let uri = format!("{}/api/v1/version", config.base_path);

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
