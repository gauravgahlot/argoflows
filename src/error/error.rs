use std::error;
use std::fmt;

use reqwest;
use serde::{Deserialize, Serialize};
use serde_json;
use serde_with::serde_as;

use crate::types::ResponseContent;

#[derive(Debug)]
pub enum Error<T> {
    Reqwest(reqwest::Error),
    Serde(serde_json::Error),
    Io(std::io::Error),
    Response(ResponseContent<T>),
}

impl<T> fmt::Display for Error<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (module, e) = match self {
            Error::Reqwest(e) => ("reqwest", e.to_string()),
            Error::Serde(e) => ("serde", e.to_string()),
            Error::Io(e) => ("IO", e.to_string()),
            Error::Response(e) => ("response", format!("status code {}", e.status)),
        };
        write!(f, "error in {}: {}", module, e)
    }
}

impl<T: fmt::Debug> error::Error for Error<T> {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        Some(match self {
            Error::Reqwest(e) => e,
            Error::Serde(e) => e,
            Error::Io(e) => e,
            Error::Response(_) => return None,
        })
    }
}

impl<T> From<serde_json::Error> for Error<T> {
    fn from(e: serde_json::Error) -> Self {
        Error::Serde(e)
    }
}

impl<T> From<reqwest::Error> for Error<T> {
    fn from(e: reqwest::Error) -> Self {
        Error::Reqwest(e)
    }
}

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct GatewayRuntimeError {
    #[serde(rename = "code", skip_serializing_if = "Option::is_none")]
    pub code: Option<i32>,

    #[serde(rename = "details", skip_serializing_if = "Option::is_none")]
    pub details: Option<Vec<GoogleProtobufAny>>,

    #[serde(rename = "error", skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,

    #[serde(rename = "message", skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct GatewayRuntimeStreamError {
    #[serde(rename = "details", skip_serializing_if = "Option::is_none")]
    pub details: Option<Vec<GoogleProtobufAny>>,

    #[serde(rename = "grpc_code", skip_serializing_if = "Option::is_none")]
    pub grpc_code: Option<i32>,

    #[serde(rename = "http_code", skip_serializing_if = "Option::is_none")]
    pub http_code: Option<i32>,

    #[serde(rename = "http_status", skip_serializing_if = "Option::is_none")]
    pub http_status: Option<String>,

    #[serde(rename = "message", skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

#[serde_as]
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct GoogleProtobufAny {
    #[serde(rename = "type_url", skip_serializing_if = "Option::is_none")]
    pub type_url: Option<String>,

    #[serde_as(as = "Option<serde_with::base64::Base64>")]
    #[serde(rename = "value", skip_serializing_if = "Option::is_none")]
    pub value: Option<Vec<u8>>,
}
