use reqwest::Client;
use thiserror::Error;
use tracing::{info, warn, error};

#[derive(Error, Debug)]
pub enum GitHubError {
    #[error("http error: {0}")]
    Http(#[from] reqwest::Error),
    #[error("api error: {0}")]
    Api(String),
    #[error("rate limit exceeded")]
    RateLimited,
    #[error("not found")]
    NotFound,
    #[error("timeout")]
    Timeout,
}

pub struct GitHubClient {
    client: Client,
    token: String,
}

impl GitHubClient {
    pub fn new(token: String) -> Self {
        let client = Client::builder()
            .user_agent("tinyiothub-marketplace/1.0")
            .build()
            .expect("reqwest client must build");
        Self { client, token }
    }

    pub async fn fetch_index_json(
        &self,
        repo: &str,
        branch: &str,
        path: &str,
    ) -> Result<Vec<serde_json::Value>, GitHubError> {
        let url = format!(
            "https://raw.githubusercontent.com/{}/{}/{}/index.json",
            repo, branch, path
        );

        info!("Fetching index.json from {}", url);

        let response = self.client
            .get(&url)
            .header("Authorization", format!("Bearer {}", self.token))
            .header("Accept", "application/json")
            .send()
            .await?;

        if response.status() == 404 {
            return Err(GitHubError::NotFound);
        }

        if response.status() == 403 {
            // Check if rate limited
            if let Some(remaining) = response.headers().get("x-ratelimit-remaining") {
                if remaining.to_str().unwrap_or("0") == "0" {
                    return Err(GitHubError::RateLimited);
                }
            }
        }

        if !response.status().is_success() {
            return Err(GitHubError::Api(format!("HTTP {}", response.status())));
        }

        let items: Vec<serde_json::Value> = response.json().await?;

        info!("Fetched {} items from {}/{}/{}", items.len(), repo, branch, path);

        Ok(items)
    }

    pub async fn fetch_raw_content(&self, url: &str) -> Result<Vec<u8>, GitHubError> {
        let response = self.client
            .get(url)
            .header("Authorization", format!("Bearer {}", self.token))
            .send()
            .await?;

        if response.status() == 404 {
            return Err(GitHubError::NotFound);
        }

        if response.status() == 403 {
            return Err(GitHubError::RateLimited);
        }

        if !response.status().is_success() {
            return Err(GitHubError::Api(format!("HTTP {}", response.status())));
        }

        let bytes = response.bytes().await?;
        Ok(bytes.to_vec())
    }

    /// Exponential backoff retry with jitter
    /// delay = min(cap, initial * 2^attempt + jitter(0, delay/2))
    pub async fn fetch_with_retry<F, Fut, T>(
        &self,
        fetch_fn: F,
        max_retries: usize,
    ) -> Result<T, GitHubError>
    where
        F: Fn() -> Fut,
        Fut: std::future::Future<Output = Result<T, GitHubError>>,
    {
        let mut attempt = 0;
        let initial_delay = std::time::Duration::from_secs(1);
        let cap_delay = std::time::Duration::from_secs(30);

        loop {
            match fetch_fn().await {
                Ok(result) => return Ok(result),
                Err(e) => {
                    if attempt >= max_retries {
                        error!("Max retries ({}) exceeded: {}", max_retries, e);
                        return Err(e);
                    }

                    // Check if error is retryable
                    match &e {
                        GitHubError::RateLimited | GitHubError::Timeout => {}
                        GitHubError::NotFound => return Err(e),
                        _ => {
                            // Exponential backoff for other errors
                            let delay_secs = {
                                let exp = (initial_delay.as_secs() * 2_u64.pow(attempt as u32))
                                    .min(cap_delay.as_secs());
                                // Add jitter: [0, delay/2]
                                let jitter = (rand_simple() * exp as f64 / 2.0) as u64;
                                exp + jitter
                            };

                            warn!("Retryable error: {}, attempt {}/{}, waiting {}s",
                                e, attempt + 1, max_retries, delay_secs);

                            tokio::time::sleep(std::time::Duration::from_secs(delay_secs)).await;
                            attempt += 1;
                            continue;
                        }
                    }
                }
            }
        }
    }
}

// Simple random for jitter (0.0 to 1.0)
fn rand_simple() -> f64 {
    use std::time::{SystemTime, UNIX_EPOCH};
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .subsec_nanos();
    (nanos as f64) / (u32::MAX as f64)
}
