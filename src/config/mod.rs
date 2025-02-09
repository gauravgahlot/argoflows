use ::reqwest::Error;
use reqwest::blocking as reqwest;

const DEFAULT_HOST: &str = "https://localhost:2746";

/// A `ConfigBuilder` can be used to create a `Config` with custom options.
#[derive(Debug, Default)]
pub struct ConfigBuilder {
    accept_invalid_certs: bool,
    allow_insecure: bool, // New flag to allow HTTP connections
    bearer_token: Option<String>,
    host: String,
}

impl ConfigBuilder {
    /// Constructs a new `ConfigBuilder`.
    pub fn new() -> Self {
        ConfigBuilder {
            host: String::from(DEFAULT_HOST),
            ..Default::default()
        }
    }

    /// Returns a `Config` that uses this `ConfigBuilder` options.
    pub fn build(self) -> Result<Config, Error> {
        let mut builder = reqwest::Client::builder();

        if self.accept_invalid_certs {
            builder = builder.danger_accept_invalid_certs(true);
        }

          // Allow HTTP connection if explicitly enabled
        let host = if self.allow_insecure && self.host.starts_with("https://") {
            self.host.replacen("https://", "http://", 1)
        } else {
            self.host
        };

        let client = builder.build()?;
        Ok(Config {
            host: self.host,
            bearer_token: self.bearer_token,
            client,
        })
    }

    /// Sets the `host`, the client submits the request to.
    pub fn host(mut self, host: &str) -> Self {
        self.host = String::from(host);
        self
    }

    /// Sets the `bearer_token` to be sent in the request header.
    pub fn bearer_token(mut self, token: &str) -> Self {
        self.bearer_token = Some(String::from(token));
        self
    }

    /// Controls the use of certificate validation.
    /// Defaults to `false`.
    pub fn danger_accept_invalid_certs(mut self, allow: bool) -> Self {
        self.accept_invalid_certs = allow;
        self
    }

    /// Allows connecting to an Argo Server over HTTP (insecure connection).
    pub fn allow_insecure(mut self, allow: bool) -> Self {
        self.allow_insecure = allow;
        self
    }
}

/// `Config` defines how the client connects with the Argo server.
///
/// The `Config` has various configuration values to tweak, but the defaults
/// are set to what is usually the most commonly desired value. To tweak the
/// `Config`, use `Config::builder()`.
#[derive(Debug, Default, Clone)]
pub struct Config {
    pub host: String,
    pub bearer_token: Option<String>,
    pub client: reqwest::Client,
}

impl Config {
    /// Constructs a new `Config`.
    pub fn new() -> Self {
        Config {
            host: String::from(DEFAULT_HOST),
            client: reqwest::Client::new(),
            ..Default::default()
        }
    }

    /// Creates a `ConfigBuilder` to build the `Config`.
    ///
    /// This is the same as `ConfigBuilder::new()`.
    pub fn builder() -> ConfigBuilder {
        ConfigBuilder::new()
    }
}
