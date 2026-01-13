//! Authenticated Spotify Web API client.
//!
//! All requests are made with Bearer token authentication to api.spotify.com/v1.
//! Includes automatic retry with exponential backoff for rate limiting.

use reqwest::Method;
use serde_json::Value;
use std::time::{Duration, Instant};
use tokio::time::sleep;
use tracing::{debug, info, trace, warn};

use super::client::{HttpClient, HttpError};

const BASE_URL: &str = "https://api.spotify.com/v1";
const MAX_RETRIES: u32 = 3;

/// Spotify Web API client.
///
/// Makes authenticated HTTP requests to the Spotify API.
/// All methods require a valid access token.
pub struct SpotifyApi {
    http: HttpClient,
    access_token: String,
}

impl SpotifyApi {
    /// Create a new API client with the given access token.
    pub fn new(access_token: String) -> Self {
        Self {
            http: HttpClient::new(),
            access_token,
        }
    }

    /// Build a full API URL from a path.
    fn url(path: &str) -> String {
        format!("{}{}", BASE_URL, path)
    }

    /// Core request method - all HTTP methods delegate to this.
    /// Includes automatic retry with exponential backoff for rate limiting.
    async fn request(
        &self,
        method: Method,
        path: &str,
        body: Option<&Value>,
    ) -> Result<Option<Value>, HttpError> {
        let mut retries = 0;

        loop {
            debug!(method = %method, path = %path, retry = retries, "API request");
            trace!(body = ?body, "Request body");

            let mut req = self
                .http
                .inner()
                .request(method.clone(), Self::url(path))
                .header("Authorization", format!("Bearer {}", self.access_token));

            req = match body {
                Some(json) => req.json(json),
                None => req.header("Content-Length", "0"),
            };

            let start = Instant::now();
            let response = req.send().await?;
            let elapsed_ms = start.elapsed().as_millis();

            // Log response details
            let status = response.status().as_u16();
            let rate_limit = response
                .headers()
                .get("x-ratelimit-remaining")
                .and_then(|v| v.to_str().ok())
                .and_then(|s| s.parse::<u32>().ok());

            info!(
                method = %method,
                path = %path,
                status = status,
                elapsed_ms = elapsed_ms,
                rate_limit_remaining = ?rate_limit,
                "API response"
            );

            let result = Self::handle_response(response).await;

            match &result {
                Err(HttpError::RateLimited { retry_after_secs }) if retries < MAX_RETRIES => {
                    let wait_secs = *retry_after_secs;
                    warn!(
                        method = %method,
                        path = %path,
                        retry = retries + 1,
                        wait_secs = wait_secs,
                        "Rate limited, retrying"
                    );
                    sleep(Duration::from_secs(wait_secs)).await;
                    retries += 1;
                    continue;
                }
                _ => return result,
            }
        }
    }

    /// Make a GET request with automatic retry for rate limiting.
    pub async fn get(&self, path: &str) -> Result<Option<Value>, HttpError> {
        let mut retries = 0;

        loop {
            debug!(method = "GET", path = %path, retry = retries, "API request");

            // GET requests don't send Content-Length: 0
            let start = Instant::now();
            let response = self
                .http
                .inner()
                .request(Method::GET, Self::url(path))
                .header("Authorization", format!("Bearer {}", self.access_token))
                .send()
                .await?;
            let elapsed_ms = start.elapsed().as_millis();

            // Log response details
            let status = response.status().as_u16();
            let rate_limit = response
                .headers()
                .get("x-ratelimit-remaining")
                .and_then(|v| v.to_str().ok())
                .and_then(|s| s.parse::<u32>().ok());

            info!(
                method = "GET",
                path = %path,
                status = status,
                elapsed_ms = elapsed_ms,
                rate_limit_remaining = ?rate_limit,
                "API response"
            );

            let result = Self::handle_response(response).await;

            match &result {
                Err(HttpError::RateLimited { retry_after_secs }) if retries < MAX_RETRIES => {
                    let wait_secs = *retry_after_secs;
                    warn!(
                        method = "GET",
                        path = %path,
                        retry = retries + 1,
                        wait_secs = wait_secs,
                        "Rate limited, retrying"
                    );
                    sleep(Duration::from_secs(wait_secs)).await;
                    retries += 1;
                    continue;
                }
                _ => return result,
            }
        }
    }

    /// Make a POST request without body.
    pub async fn post(&self, path: &str) -> Result<Option<Value>, HttpError> {
        self.request(Method::POST, path, None).await
    }

    /// Make a POST request with JSON body.
    pub async fn post_json(&self, path: &str, body: &Value) -> Result<Option<Value>, HttpError> {
        self.request(Method::POST, path, Some(body)).await
    }

    /// Make a PUT request without body.
    pub async fn put(&self, path: &str) -> Result<Option<Value>, HttpError> {
        self.request(Method::PUT, path, None).await
    }

    /// Make a PUT request with JSON body.
    pub async fn put_json(&self, path: &str, body: &Value) -> Result<Option<Value>, HttpError> {
        self.request(Method::PUT, path, Some(body)).await
    }

    /// Make a DELETE request without body.
    pub async fn delete(&self, path: &str) -> Result<Option<Value>, HttpError> {
        self.request(Method::DELETE, path, None).await
    }

    /// Make a DELETE request with JSON body.
    pub async fn delete_json(&self, path: &str, body: &Value) -> Result<Option<Value>, HttpError> {
        self.request(Method::DELETE, path, Some(body)).await
    }

    async fn handle_response(response: reqwest::Response) -> Result<Option<Value>, HttpError> {
        let status = response.status();

        if status == reqwest::StatusCode::NO_CONTENT {
            return Ok(None);
        }

        if !status.is_success() {
            return Err(HttpError::from_response(response).await);
        }

        let json: Value = response.json().await?;
        Ok(Some(json))
    }
}
