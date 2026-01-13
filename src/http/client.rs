use reqwest::Client;
use serde::Deserialize;
use thiserror::Error;

/// Spotify API error response structure
#[derive(Debug, Deserialize)]
struct SpotifyErrorResponse {
    error: SpotifyError,
}

#[derive(Debug, Deserialize)]
struct SpotifyError {
    #[allow(dead_code)]
    status: u16,
    message: String,
}

#[derive(Debug, Error)]
pub enum HttpError {
    #[error("Network error: {0}")]
    Network(#[from] reqwest::Error),

    #[error("{message}")]
    Api { status: u16, message: String },

    #[error("Rate limited - try again later")]
    RateLimited,

    #[error("Token expired or invalid")]
    Unauthorized,

    #[error("Access denied")]
    Forbidden,

    #[error("Resource not found")]
    NotFound,
}

impl HttpError {
    /// Create an HttpError from a non-success HTTP response
    pub async fn from_response(response: reqwest::Response) -> Self {
        let status = response.status().as_u16();
        let body = response.text().await.unwrap_or_default();

        // Try to parse Spotify's error format: {"error": {"status": 401, "message": "..."}}
        let message = if let Ok(spotify_err) = serde_json::from_str::<SpotifyErrorResponse>(&body) {
            spotify_err.error.message
        } else if body.len() < 200 && !body.contains('<') {
            // Use raw body if it's short and not HTML
            body
        } else {
            // Fallback to generic message based on status
            match status {
                400 => "Bad request".to_string(),
                401 => "Unauthorized".to_string(),
                403 => "Forbidden".to_string(),
                404 => "Not found".to_string(),
                429 => "Rate limited".to_string(),
                500..=599 => "Spotify server error".to_string(),
                _ => format!("HTTP error {}", status),
            }
        };

        // Return specific error types for common cases
        match status {
            401 => HttpError::Unauthorized,
            403 => HttpError::Forbidden,
            404 => HttpError::NotFound,
            429 => HttpError::RateLimited,
            _ => HttpError::Api { status, message },
        }
    }

    /// Get the HTTP status code for this error
    pub fn status_code(&self) -> u16 {
        match self {
            HttpError::Network(_) => 503,
            HttpError::Api { status, .. } => *status,
            HttpError::RateLimited => 429,
            HttpError::Unauthorized => 401,
            HttpError::Forbidden => 403,
            HttpError::NotFound => 404,
        }
    }

    /// Get a user-friendly error message
    pub fn user_message(&self) -> &str {
        match self {
            HttpError::Network(_) => "Network error - check your connection",
            HttpError::Api { message, .. } => message,
            HttpError::RateLimited => "Too many requests - please wait a moment",
            HttpError::Unauthorized => "Session expired - run: spotify-cli auth refresh",
            HttpError::Forbidden => "You don't have permission for this action",
            HttpError::NotFound => "Resource not found",
        }
    }
}

/// Base HTTP client - pure reqwest wrapper
pub struct HttpClient {
    client: Client,
}

impl HttpClient {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
        }
    }

    pub fn inner(&self) -> &Client {
        &self.client
    }
}

impl Default for HttpClient {
    fn default() -> Self {
        Self::new()
    }
}
