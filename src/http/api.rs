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
use crate::constants::{MAX_API_RETRIES, SPOTIFY_API_BASE_URL};

/// Spotify Web API client.
///
/// Makes authenticated HTTP requests to the Spotify API.
/// All methods require a valid access token.
pub struct SpotifyApi {
    http: HttpClient,
    access_token: String,
    base_url: String,
}

impl SpotifyApi {
    /// Create a new API client with the given access token.
    pub fn new(access_token: String) -> Self {
        Self {
            http: HttpClient::new(),
            access_token,
            base_url: SPOTIFY_API_BASE_URL.to_string(),
        }
    }

    /// Create a new API client with a custom base URL.
    ///
    /// Useful for testing with mock servers or connecting to alternative endpoints.
    pub fn with_base_url(access_token: String, base_url: String) -> Self {
        Self {
            http: HttpClient::new(),
            access_token,
            base_url,
        }
    }

    /// Build a full API URL from a path.
    fn url(&self, path: &str) -> String {
        format!("{}{}", self.base_url, path)
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
                .request(method.clone(), self.url(path))
                .header("Authorization", format!("Bearer {}", self.access_token));

            req = match body {
                Some(json) => req.json(json),
                // GET requests don't need Content-Length, other methods need it for empty body
                None if method != Method::GET => req.header("Content-Length", "0"),
                None => req,
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
                Err(HttpError::RateLimited { retry_after_secs }) if retries < MAX_API_RETRIES => {
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

    /// Make a GET request.
    pub async fn get(&self, path: &str) -> Result<Option<Value>, HttpError> {
        self.request(Method::GET, path, None).await
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

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use wiremock::matchers::{header, method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    async fn setup_mock_server() -> (MockServer, SpotifyApi) {
        let mock_server = MockServer::start().await;
        let api = SpotifyApi::with_base_url("test_token".to_string(), mock_server.uri());
        (mock_server, api)
    }

    #[tokio::test]
    async fn get_request_returns_json() {
        let (mock_server, api) = setup_mock_server().await;

        Mock::given(method("GET"))
            .and(path("/me"))
            .and(header("Authorization", "Bearer test_token"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "id": "user123",
                "display_name": "Test User"
            })))
            .mount(&mock_server)
            .await;

        let result = api.get("/me").await.unwrap();
        assert!(result.is_some());
        let payload = result.unwrap();
        assert_eq!(payload["id"], "user123");
        assert_eq!(payload["display_name"], "Test User");
    }

    #[tokio::test]
    async fn get_request_handles_204_no_content() {
        let (mock_server, api) = setup_mock_server().await;

        Mock::given(method("GET"))
            .and(path("/empty"))
            .respond_with(ResponseTemplate::new(204))
            .mount(&mock_server)
            .await;

        let result = api.get("/empty").await.unwrap();
        assert!(result.is_none());
    }

    #[tokio::test]
    async fn get_request_handles_401_unauthorized() {
        let (mock_server, api) = setup_mock_server().await;

        Mock::given(method("GET"))
            .and(path("/protected"))
            .respond_with(ResponseTemplate::new(401).set_body_json(json!({
                "error": {
                    "status": 401,
                    "message": "Invalid access token"
                }
            })))
            .mount(&mock_server)
            .await;

        let result = api.get("/protected").await;
        assert!(matches!(result, Err(HttpError::Unauthorized)));
    }

    #[tokio::test]
    async fn get_request_handles_404_not_found() {
        let (mock_server, api) = setup_mock_server().await;

        Mock::given(method("GET"))
            .and(path("/missing"))
            .respond_with(ResponseTemplate::new(404))
            .mount(&mock_server)
            .await;

        let result = api.get("/missing").await;
        assert!(matches!(result, Err(HttpError::NotFound)));
    }

    #[tokio::test]
    async fn post_request_sends_empty_body() {
        let (mock_server, api) = setup_mock_server().await;

        Mock::given(method("POST"))
            .and(path("/player/next"))
            .and(header("Content-Length", "0"))
            .respond_with(ResponseTemplate::new(204))
            .mount(&mock_server)
            .await;

        let result = api.post("/player/next").await.unwrap();
        assert!(result.is_none());
    }

    #[tokio::test]
    async fn post_json_sends_body() {
        let (mock_server, api) = setup_mock_server().await;

        Mock::given(method("POST"))
            .and(path("/playlists"))
            .and(header("Authorization", "Bearer test_token"))
            .respond_with(ResponseTemplate::new(201).set_body_json(json!({
                "id": "playlist123"
            })))
            .mount(&mock_server)
            .await;

        let body = json!({"name": "My Playlist"});
        let result = api.post_json("/playlists", &body).await.unwrap();
        assert!(result.is_some());
        assert_eq!(result.unwrap()["id"], "playlist123");
    }

    #[tokio::test]
    async fn put_request_works() {
        let (mock_server, api) = setup_mock_server().await;

        Mock::given(method("PUT"))
            .and(path("/me/player/play"))
            .respond_with(ResponseTemplate::new(204))
            .mount(&mock_server)
            .await;

        let result = api.put("/me/player/play").await.unwrap();
        assert!(result.is_none());
    }

    #[tokio::test]
    async fn delete_request_works() {
        let (mock_server, api) = setup_mock_server().await;

        Mock::given(method("DELETE"))
            .and(path("/playlists/123/tracks"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "snapshot_id": "abc123"
            })))
            .mount(&mock_server)
            .await;

        let result = api.delete("/playlists/123/tracks").await.unwrap();
        assert!(result.is_some());
    }

    #[tokio::test]
    async fn handles_api_error_with_message() {
        let (mock_server, api) = setup_mock_server().await;

        Mock::given(method("GET"))
            .and(path("/error"))
            .respond_with(ResponseTemplate::new(400).set_body_json(json!({
                "error": {
                    "status": 400,
                    "message": "Invalid market code"
                }
            })))
            .mount(&mock_server)
            .await;

        let result = api.get("/error").await;
        match result {
            Err(HttpError::Api { status, message }) => {
                assert_eq!(status, 400);
                assert_eq!(message, "Invalid market code");
            }
            _ => panic!("Expected Api error"),
        }
    }

    #[tokio::test]
    async fn url_building() {
        let api = SpotifyApi::with_base_url("token".to_string(), "https://api.example.com".to_string());
        assert_eq!(api.url("/me"), "https://api.example.com/me");
        assert_eq!(api.url("/tracks/123"), "https://api.example.com/tracks/123");
    }
}
