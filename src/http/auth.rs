//! Spotify authentication endpoints.
//!
//! Handles token exchange and refresh via accounts.spotify.com.

use thiserror::Error;

use super::client::HttpClient;
use crate::constants::SPOTIFY_AUTH_BASE_URL;

/// Errors from authentication requests.
#[derive(Debug, Error)]
pub enum AuthError {
    #[error("Request failed: {0}")]
    Request(#[from] reqwest::Error),

    #[error("Token exchange failed ({status}): {message}")]
    TokenExchange { status: u16, message: String },
}

/// Spotify authentication client.
///
/// Handles token operations via accounts.spotify.com/api/token.
pub struct SpotifyAuth {
    http: HttpClient,
}

impl SpotifyAuth {
    /// Create a new authentication client.
    pub fn new() -> Self {
        Self {
            http: HttpClient::new(),
        }
    }

    /// Build a URL for the Spotify accounts endpoint.
    pub fn url(path: &str) -> String {
        format!("{}{}", SPOTIFY_AUTH_BASE_URL, path)
    }

    /// Exchange authorization code for tokens (PKCE flow)
    pub async fn exchange_code(
        &self,
        client_id: &str,
        code: &str,
        redirect_uri: &str,
        code_verifier: &str,
    ) -> Result<serde_json::Value, AuthError> {
        let params = [
            ("grant_type", "authorization_code"),
            ("code", code),
            ("redirect_uri", redirect_uri),
            ("client_id", client_id),
            ("code_verifier", code_verifier),
        ];

        self.token_request(&params).await
    }

    /// Refresh an access token
    pub async fn refresh_token(
        &self,
        client_id: &str,
        refresh_token: &str,
    ) -> Result<serde_json::Value, AuthError> {
        let params = [
            ("grant_type", "refresh_token"),
            ("refresh_token", refresh_token),
            ("client_id", client_id),
        ];

        self.token_request(&params).await
    }

    async fn token_request(&self, params: &[(&str, &str)]) -> Result<serde_json::Value, AuthError> {
        let response = self
            .http
            .inner()
            .post(Self::url("/api/token"))
            .form(params)
            .send()
            .await?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            return Err(AuthError::TokenExchange {
                status: status.as_u16(),
                message: body,
            });
        }

        let json: serde_json::Value = response.json().await?;
        Ok(json)
    }
}

impl Default for SpotifyAuth {
    fn default() -> Self {
        Self::new()
    }
}
