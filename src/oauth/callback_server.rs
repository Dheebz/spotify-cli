//! Local HTTP server for OAuth callback.
//!
//! Starts a temporary HTTP server on localhost to receive the OAuth callback
//! containing the authorization code after user approval.

use std::time::Duration;
use thiserror::Error;
use tiny_http::{Response, Server};
use url::Url;

/// Default port for the callback server.
pub const DEFAULT_PORT: u16 = 8888;
/// Path where Spotify redirects after authorization.
pub const CALLBACK_PATH: &str = "/callback";

#[derive(Debug, Error)]
pub enum CallbackError {
    #[error("Failed to start server: {0}")]
    ServerStart(String),

    #[error("Timeout waiting for callback")]
    Timeout,

    #[error("Missing authorization code")]
    MissingCode,

    #[error("Authorization denied: {0}")]
    Denied(String),

    #[error("Invalid callback request")]
    InvalidRequest,
}

/// HTTP server that listens for the OAuth callback.
pub struct CallbackServer {
    port: u16,
    timeout: Duration,
}

/// Result from a successful OAuth callback.
pub struct CallbackResult {
    /// The authorization code to exchange for tokens.
    pub code: String,
    /// Optional state parameter for CSRF protection.
    pub state: Option<String>,
}

impl CallbackServer {
    /// Create a new callback server on the given port.
    ///
    /// Default timeout is 5 minutes.
    pub fn new(port: u16) -> Self {
        Self {
            port,
            timeout: Duration::from_secs(300), // 5 minute timeout
        }
    }

    /// Set a custom timeout for waiting for the callback.
    pub fn with_timeout(mut self, timeout: Duration) -> Self {
        self.timeout = timeout;
        self
    }

    /// Get the redirect URI for this server.
    pub fn redirect_uri(&self) -> String {
        format!("http://127.0.0.1:{}{}", self.port, CALLBACK_PATH)
    }

    /// Start the server and wait for the OAuth callback.
    ///
    /// Blocks until callback is received or timeout expires.
    pub fn wait_for_callback(self) -> Result<CallbackResult, CallbackError> {
        let addr = format!("127.0.0.1:{}", self.port);
        let server =
            Server::http(&addr).map_err(|e| CallbackError::ServerStart(e.to_string()))?;

        loop {
            let request = match server.recv_timeout(self.timeout) {
                Ok(Some(req)) => req,
                Ok(None) => return Err(CallbackError::Timeout),
                Err(_) => return Err(CallbackError::Timeout),
            };

            let url_str = format!("http://127.0.0.1{}", request.url());
            let url = Url::parse(&url_str).map_err(|_| CallbackError::InvalidRequest)?;

            if url.path() != CALLBACK_PATH {
                let response = Response::from_string("Not found").with_status_code(404);
                let _ = request.respond(response);
                continue;
            }

            let params: std::collections::HashMap<_, _> = url.query_pairs().collect();

            if let Some(error) = params.get("error") {
                let description = params
                    .get("error_description")
                    .map(|s| s.to_string())
                    .unwrap_or_else(|| error.to_string());

                let response = Response::from_string(error_html(&description))
                    .with_header(
                        "Content-Type: text/html; charset=utf-8"
                            .parse::<tiny_http::Header>()
                            .unwrap(),
                    );
                let _ = request.respond(response);

                return Err(CallbackError::Denied(description));
            }

            let code = params
                .get("code")
                .map(|s| s.to_string())
                .ok_or(CallbackError::MissingCode)?;

            let state = params.get("state").map(|s| s.to_string());

            let response = Response::from_string(success_html()).with_header(
                "Content-Type: text/html; charset=utf-8"
                    .parse::<tiny_http::Header>()
                    .unwrap(),
            );
            let _ = request.respond(response);

            return Ok(CallbackResult { code, state });
        }
    }
}

fn success_html() -> String {
    r#"<!DOCTYPE html>
<html>
<head>
    <title>spotify-cli</title>
    <style>
        body {
            font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, sans-serif;
            display: flex;
            justify-content: center;
            align-items: center;
            height: 100vh;
            margin: 0;
            background: #191414;
            color: #1DB954;
        }
        .container { text-align: center; }
        h1 { font-size: 2rem; margin-bottom: 1rem; }
        p { color: #b3b3b3; }
    </style>
</head>
<body>
    <div class="container">
        <h1>Authenticated!</h1>
        <p>You can close this window and return to your terminal.</p>
    </div>
</body>
</html>"#
        .to_string()
}

fn error_html(message: &str) -> String {
    format!(
        r#"<!DOCTYPE html>
<html>
<head>
    <title>spotify-cli - Error</title>
    <style>
        body {{
            font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, sans-serif;
            display: flex;
            justify-content: center;
            align-items: center;
            height: 100vh;
            margin: 0;
            background: #191414;
            color: #e22134;
        }}
        .container {{ text-align: center; }}
        h1 {{ font-size: 2rem; margin-bottom: 1rem; }}
        p {{ color: #b3b3b3; }}
    </style>
</head>
<body>
    <div class="container">
        <h1>Authentication Failed</h1>
        <p>{}</p>
    </div>
</body>
</html>"#,
        message
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn redirect_uri_uses_correct_format() {
        let server = CallbackServer::new(8888);
        assert_eq!(server.redirect_uri(), "http://127.0.0.1:8888/callback");
    }

    #[test]
    fn can_customize_port() {
        let server = CallbackServer::new(9999);
        assert_eq!(server.redirect_uri(), "http://127.0.0.1:9999/callback");
    }
}
