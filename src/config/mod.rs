use reqwest::blocking as reqwest;

const DEFAULT_BASE_PATH: &str = "https://localhost:2746";

#[derive(Debug, Clone)]
pub struct Config {
    pub base_path: String,
    pub client: reqwest::Client,
    pub bearer_token: Option<String>,
}

impl Config {
    pub fn new(bearer_token: String) -> Self {
        Config {
            base_path: DEFAULT_BASE_PATH.to_owned(),
            bearer_token: Some(bearer_token),
            client: reqwest::Client::builder()
                .danger_accept_invalid_certs(true)
                .build()
                .expect("failed to create a reqwest client"),
        }
    }
}
