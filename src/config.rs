#[derive(Debug, Clone)]
pub struct Config {
    pub bearer_token: String,
    pub url: String,
    pub allow_insecure: bool,
    // ... other existing fields
}

#[derive(Default)]
pub struct ConfigBuilder {
    bearer_token: Option<String>,
    url: Option<String>,
    allow_insecure: bool,
    // ... other existing fields
}

impl ConfigBuilder {
    // ... existing methods

    /// Allow insecure HTTP connections (for testing/development)
    pub fn allow_insecure(mut self, allow_insecure: bool) -> Self {
        self.allow_insecure = allow_insecure;
        self
    }

    pub fn build(self) -> Result<Config> {
        let url = self.url.ok_or(anyhow!("URL must be set"))?;
        
        // Validate URL scheme
        if !self.allow_insecure && !url.starts_with("https://") {
            return Err(anyhow!(
                "URL must use HTTPS protocol unless allow_insecure is enabled. \
                Try using .allow_insecure(true) in your ConfigBuilder"
            ));
        }

        Ok(Config {
            bearer_token: self.bearer_token.unwrap_or_default(),
            url,
            allow_insecure: self.allow_insecure,
            // ... other existing fields
        })
    }
}

// Usage Example :

// let cfg = Config::builder()
//     .bearer_token("your-token")
//     .url("http://localhost:2746")  // HTTP URL
//     .allow_insecure(true)  // Enable HTTP connection
//     .build()?;