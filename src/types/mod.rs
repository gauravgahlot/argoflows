use reqwest;

#[derive(Debug)]
pub struct ResponseContent<T> {
    pub status: reqwest::StatusCode,
    pub content: String,
    pub entity: Option<T>,
}

pub mod http;

pub mod info;

pub mod metrics;

pub mod sync;
