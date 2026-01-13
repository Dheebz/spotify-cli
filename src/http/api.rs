use reqwest::Method;
use serde_json::Value;

use super::client::{HttpClient, HttpError};

const BASE_URL: &str = "https://api.spotify.com/v1";

/// Spotify Web API client - authenticated requests to api.spotify.com/v1
pub struct SpotifyApi {
    http: HttpClient,
    access_token: String,
}

impl SpotifyApi {
    pub fn new(access_token: String) -> Self {
        Self {
            http: HttpClient::new(),
            access_token,
        }
    }

    fn url(path: &str) -> String {
        format!("{}{}", BASE_URL, path)
    }

    /// Core request method - all HTTP methods delegate to this
    async fn request(
        &self,
        method: Method,
        path: &str,
        body: Option<&Value>,
    ) -> Result<Option<Value>, HttpError> {
        let mut req = self
            .http
            .inner()
            .request(method, Self::url(path))
            .header("Authorization", format!("Bearer {}", self.access_token));

        req = match body {
            Some(json) => req.json(json),
            None => req.header("Content-Length", "0"),
        };

        Self::handle_response(req.send().await?).await
    }

    pub async fn get(&self, path: &str) -> Result<Option<Value>, HttpError> {
        // GET requests don't send Content-Length: 0
        let response = self
            .http
            .inner()
            .request(Method::GET, Self::url(path))
            .header("Authorization", format!("Bearer {}", self.access_token))
            .send()
            .await?;
        Self::handle_response(response).await
    }

    pub async fn post(&self, path: &str) -> Result<Option<Value>, HttpError> {
        self.request(Method::POST, path, None).await
    }

    pub async fn post_json(&self, path: &str, body: &Value) -> Result<Option<Value>, HttpError> {
        self.request(Method::POST, path, Some(body)).await
    }

    pub async fn put(&self, path: &str) -> Result<Option<Value>, HttpError> {
        self.request(Method::PUT, path, None).await
    }

    pub async fn put_json(&self, path: &str, body: &Value) -> Result<Option<Value>, HttpError> {
        self.request(Method::PUT, path, Some(body)).await
    }

    pub async fn delete(&self, path: &str) -> Result<Option<Value>, HttpError> {
        self.request(Method::DELETE, path, None).await
    }

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
