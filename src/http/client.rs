//! Low-level HTTP client and error types.
//!
//! Provides a thin wrapper around reqwest with Spotify-specific error handling.

use reqwest::Client;
use serde::Deserialize;
use thiserror::Error;

/// Spotify API error response structure.
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

/// HTTP errors from Spotify API requests.
#[derive(Debug, Error)]
pub enum HttpError {
    #[error("Network error: {0}")]
    Network(#[from] reqwest::Error),

    #[error("{message}")]
    Api { status: u16, message: String },

    #[error("Rate limited - retry after {retry_after_secs} seconds")]
    RateLimited { retry_after_secs: u64 },

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

        // Extract Retry-After header before consuming response body
        let retry_after = response
            .headers()
            .get("retry-after")
            .and_then(|v| v.to_str().ok())
            .and_then(|s| s.parse::<u64>().ok())
            .unwrap_or(1); // Default to 1 second if not specified

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
            429 => HttpError::RateLimited { retry_after_secs: retry_after },
            _ => HttpError::Api { status, message },
        }
    }

    /// Get the retry-after duration if this is a rate limit error
    pub fn retry_after(&self) -> Option<u64> {
        match self {
            HttpError::RateLimited { retry_after_secs } => Some(*retry_after_secs),
            _ => None,
        }
    }

    /// Get the HTTP status code for this error
    pub fn status_code(&self) -> u16 {
        match self {
            HttpError::Network(_) => 503,
            HttpError::Api { status, .. } => *status,
            HttpError::RateLimited { .. } => 429,
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
            HttpError::RateLimited { .. } => "Too many requests - please wait a moment",
            HttpError::Unauthorized => "Session expired - run: spotify-cli auth refresh",
            HttpError::Forbidden => "You don't have permission for this action",
            HttpError::NotFound => "Resource not found",
        }
    }
}

/// Base HTTP client wrapper.
///
/// Thin wrapper around reqwest::Client used by SpotifyApi and SpotifyAuth.
pub struct HttpClient {
    client: Client,
}

impl HttpClient {
    /// Create a new HTTP client.
    pub fn new() -> Self {
        Self {
            client: Client::new(),
        }
    }

    /// Get the underlying reqwest client for making requests.
    pub fn inner(&self) -> &Client {
        &self.client
    }
}

impl Default for HttpClient {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn http_error_status_codes() {
        assert_eq!(HttpError::Unauthorized.status_code(), 401);
        assert_eq!(HttpError::Forbidden.status_code(), 403);
        assert_eq!(HttpError::NotFound.status_code(), 404);
        assert_eq!(HttpError::RateLimited { retry_after_secs: 5 }.status_code(), 429);
        assert_eq!(
            HttpError::Api {
                status: 500,
                message: "Server error".to_string()
            }
            .status_code(),
            500
        );
    }

    #[test]
    fn http_error_user_messages() {
        assert_eq!(HttpError::Unauthorized.user_message(), "Session expired - run: spotify-cli auth refresh");
        assert_eq!(HttpError::Forbidden.user_message(), "You don't have permission for this action");
        assert_eq!(HttpError::NotFound.user_message(), "Resource not found");
        assert_eq!(HttpError::RateLimited { retry_after_secs: 5 }.user_message(), "Too many requests - please wait a moment");
        assert_eq!(
            HttpError::Api {
                status: 500,
                message: "Custom error".to_string()
            }
            .user_message(),
            "Custom error"
        );
    }

    #[test]
    fn http_error_retry_after() {
        assert_eq!(HttpError::RateLimited { retry_after_secs: 30 }.retry_after(), Some(30));
        assert_eq!(HttpError::Unauthorized.retry_after(), None);
        assert_eq!(HttpError::NotFound.retry_after(), None);
    }

    #[test]
    fn http_error_display() {
        assert_eq!(format!("{}", HttpError::Unauthorized), "Token expired or invalid");
        assert_eq!(format!("{}", HttpError::Forbidden), "Access denied");
        assert_eq!(format!("{}", HttpError::NotFound), "Resource not found");
        assert_eq!(
            format!("{}", HttpError::RateLimited { retry_after_secs: 10 }),
            "Rate limited - retry after 10 seconds"
        );
        assert_eq!(
            format!("{}", HttpError::Api { status: 400, message: "Bad request".to_string() }),
            "Bad request"
        );
    }

    #[test]
    fn http_client_default() {
        let client = HttpClient::default();
        // Just verify it creates successfully
        let _ = client.inner();
    }

    #[test]
    fn http_client_new() {
        let client = HttpClient::new();
        // Just verify it creates and inner() works
        let _ = client.inner();
    }

    #[test]
    fn http_error_api_various_statuses() {
        let statuses = [400, 402, 405, 500, 502, 503];
        for status in statuses {
            let err = HttpError::Api { status, message: "test".to_string() };
            assert_eq!(err.status_code(), status);
        }
    }

    #[test]
    fn http_error_is_debug() {
        let err = HttpError::Unauthorized;
        let debug = format!("{:?}", err);
        assert!(debug.contains("Unauthorized"));
    }

    #[test]
    fn http_error_api_user_message() {
        let err = HttpError::Api { status: 400, message: "test msg".to_string() };
        assert_eq!(err.user_message(), "test msg");
    }

    #[test]
    fn http_error_display_for_all_variants() {
        // Test display for all constructible variants
        let api_err = HttpError::Api { status: 500, message: "Server error".to_string() };
        assert_eq!(format!("{}", api_err), "Server error");

        let rate_err = HttpError::RateLimited { retry_after_secs: 30 };
        assert!(format!("{}", rate_err).contains("30"));
    }

    #[test]
    fn spotify_error_response_deserialization() {
        let json = r#"{"error": {"status": 400, "message": "Bad request"}}"#;
        let err: SpotifyErrorResponse = serde_json::from_str(json).unwrap();
        assert_eq!(err.error.message, "Bad request");
    }
}
