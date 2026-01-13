//! Response types and output functions

use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::format_registry::format_payload;
use crate::http::client::HttpError;

/// Type-safe error categories
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ErrorKind {
    /// Network/connectivity issues
    Network,
    /// Spotify API returned an error
    Api,
    /// Authentication required or failed
    Auth,
    /// Resource not found
    NotFound,
    /// Permission denied
    Forbidden,
    /// Rate limited by Spotify
    RateLimited,
    /// Invalid input from user
    Validation,
    /// Local storage error
    Storage,
    /// Configuration error
    Config,
    /// Player-specific error
    Player,
}

impl ErrorKind {
    pub fn as_str(&self) -> &'static str {
        match self {
            ErrorKind::Network => "network_error",
            ErrorKind::Api => "api_error",
            ErrorKind::Auth => "auth_error",
            ErrorKind::NotFound => "not_found",
            ErrorKind::Forbidden => "forbidden",
            ErrorKind::RateLimited => "rate_limited",
            ErrorKind::Validation => "validation_error",
            ErrorKind::Storage => "storage_error",
            ErrorKind::Config => "config_error",
            ErrorKind::Player => "player_error",
        }
    }
}

impl std::fmt::Display for ErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Status {
    Success,
    Error,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Response {
    pub status: Status,
    pub code: u16,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payload: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<ErrorDetail>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorDetail {
    pub kind: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<String>,
}

impl Response {
    pub fn success(code: u16, message: impl Into<String>) -> Self {
        Self {
            status: Status::Success,
            code,
            message: message.into(),
            payload: None,
            error: None,
        }
    }

    pub fn success_with_payload(code: u16, message: impl Into<String>, payload: Value) -> Self {
        Self {
            status: Status::Success,
            code,
            message: message.into(),
            payload: Some(payload),
            error: None,
        }
    }

    /// Create an error response with ErrorKind
    pub fn err(code: u16, message: impl Into<String>, kind: ErrorKind) -> Self {
        Self {
            status: Status::Error,
            code,
            message: message.into(),
            payload: None,
            error: Some(ErrorDetail {
                kind: kind.to_string(),
                details: None,
            }),
        }
    }

    /// Create an error response with ErrorKind and details
    pub fn err_with_details(
        code: u16,
        message: impl Into<String>,
        kind: ErrorKind,
        details: impl Into<String>,
    ) -> Self {
        Self {
            status: Status::Error,
            code,
            message: message.into(),
            payload: None,
            error: Some(ErrorDetail {
                kind: kind.to_string(),
                details: Some(details.into()),
            }),
        }
    }

    /// Create a Response from an HttpError, preserving the original status code
    pub fn from_http_error(err: &HttpError, context: &str) -> Self {
        let kind = match err {
            HttpError::Network(_) => ErrorKind::Network,
            HttpError::Unauthorized => ErrorKind::Auth,
            HttpError::Forbidden => ErrorKind::Forbidden,
            HttpError::NotFound => ErrorKind::NotFound,
            HttpError::RateLimited => ErrorKind::RateLimited,
            HttpError::Api { .. } => ErrorKind::Api,
        };

        Self {
            status: Status::Error,
            code: err.status_code(),
            message: context.to_string(),
            payload: None,
            error: Some(ErrorDetail {
                kind: kind.to_string(),
                details: Some(err.user_message().to_string()),
            }),
        }
    }

    // Legacy methods for backward compatibility
    pub fn error(code: u16, message: impl Into<String>, kind: impl Into<String>) -> Self {
        Self {
            status: Status::Error,
            code,
            message: message.into(),
            payload: None,
            error: Some(ErrorDetail {
                kind: kind.into(),
                details: None,
            }),
        }
    }

    pub fn error_with_details(
        code: u16,
        message: impl Into<String>,
        kind: impl Into<String>,
        details: impl Into<String>,
    ) -> Self {
        Self {
            status: Status::Error,
            code,
            message: message.into(),
            payload: None,
            error: Some(ErrorDetail {
                kind: kind.into(),
                details: Some(details.into()),
            }),
        }
    }

    pub fn to_json(&self) -> String {
        serde_json::to_string(self).unwrap_or_else(|_| {
            r#"{"status":"error","code":500,"message":"Failed to serialize response"}"#.to_string()
        })
    }
}

// ============================================================================
// Error Response Macros (updated to use ErrorKind)
// ============================================================================

/// Create an API error response from HttpError (preserves status code)
#[macro_export]
macro_rules! api_error {
    ($ctx:expr, $err:expr) => {
        $crate::io::output::Response::from_http_error(&$err, $ctx)
    };
}

/// Create a storage error response (500, ErrorKind::Storage)
#[macro_export]
macro_rules! storage_error {
    ($msg:expr, $err:expr) => {
        $crate::io::output::Response::err_with_details(
            500,
            $msg,
            $crate::io::output::ErrorKind::Storage,
            $err.to_string(),
        )
    };
}

/// Create an auth error response (401, ErrorKind::Auth)
#[macro_export]
macro_rules! auth_error {
    ($msg:expr, $err:expr) => {
        $crate::io::output::Response::err_with_details(
            401,
            $msg,
            $crate::io::output::ErrorKind::Auth,
            $err.to_string(),
        )
    };
}

pub fn print_json(response: &Response) {
    println!("{}", response.to_json());
}

pub fn print_human(response: &Response) {
    match &response.status {
        Status::Error => {
            eprintln!("Error: {}", response.message);
            if let Some(err) = &response.error
                && let Some(details) = &err.details {
                    eprintln!("  {}", details);
                }
        }
        Status::Success => {
            if let Some(payload) = &response.payload {
                format_payload(payload, &response.message);
            } else {
                println!("{}", response.message);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn success_response_serializes() {
        let resp = Response::success(200, "OK");
        let json = resp.to_json();
        assert!(json.contains(r#""status":"success""#));
        assert!(json.contains(r#""code":200"#));
    }

    #[test]
    fn error_response_includes_error_detail() {
        let resp = Response::err(401, "Unauthorized", ErrorKind::Auth);
        let json = resp.to_json();
        assert!(json.contains(r#""status":"error""#));
        assert!(json.contains(r#""kind":"auth_error""#));
    }

    #[test]
    fn payload_skipped_when_none() {
        let resp = Response::success(200, "OK");
        let json = resp.to_json();
        assert!(!json.contains("payload"));
    }

    #[test]
    fn payload_included_when_present() {
        let payload = serde_json::json!({"track": "test"});
        let resp = Response::success_with_payload(200, "OK", payload);
        let json = resp.to_json();
        assert!(json.contains("payload"));
        assert!(json.contains("track"));
    }

    #[test]
    fn error_kind_serializes_to_snake_case() {
        assert_eq!(ErrorKind::NotFound.as_str(), "not_found");
        assert_eq!(ErrorKind::RateLimited.as_str(), "rate_limited");
        assert_eq!(ErrorKind::Api.as_str(), "api_error");
    }
}
