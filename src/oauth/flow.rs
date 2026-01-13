//! OAuth 2.0 Authorization Code flow with PKCE.
//!
//! Orchestrates the full authentication flow: browser authorization, callback handling,
//! and token exchange.

use thiserror::Error;
use url::Url;

use super::callback_server::{CallbackError, CallbackResult, CallbackServer, DEFAULT_PORT};
use super::pkce::PkceChallenge;
use super::token::{SpotifyTokenResponse, Token};
use crate::http::auth::SpotifyAuth;

const AUTHORIZE_ENDPOINT: &str = "/authorize";

#[derive(Debug, Error)]
pub enum OAuthError {
    #[error("Callback error: {0}")]
    Callback(#[from] CallbackError),

    #[error("Auth error: {0}")]
    Auth(#[from] crate::http::auth::AuthError),

    #[error("Failed to open browser: {0}")]
    Browser(String),

    #[error("Failed to parse token response")]
    TokenParse,
}

/// OAuth flow configuration and execution.
///
/// Handles the complete OAuth 2.0 Authorization Code flow with PKCE.
pub struct OAuthFlow {
    client_id: String,
    redirect_uri: String,
    scopes: Vec<String>,
    port: u16,
}

impl OAuthFlow {
    /// Create a new OAuth flow with the given Spotify client ID.
    ///
    /// Uses default scopes and port 8888 for the callback server.
    pub fn new(client_id: String) -> Self {
        let port = DEFAULT_PORT;
        let redirect_uri = format!("http://127.0.0.1:{}/callback", port);

        Self {
            client_id,
            redirect_uri,
            scopes: default_scopes(),
            port,
        }
    }

    /// Override the default scopes.
    pub fn with_scopes(mut self, scopes: Vec<String>) -> Self {
        self.scopes = scopes;
        self
    }

    /// Override the default callback port.
    pub fn with_port(mut self, port: u16) -> Self {
        self.port = port;
        self.redirect_uri = format!("http://127.0.0.1:{}/callback", port);
        self
    }

    /// Execute the full OAuth flow.
    ///
    /// 1. Generates PKCE challenge
    /// 2. Opens browser to Spotify authorization page
    /// 3. Waits for callback with authorization code
    /// 4. Exchanges code for tokens
    pub async fn authenticate(&self) -> Result<Token, OAuthError> {
        let pkce = PkceChallenge::generate();

        let auth_url = self.build_auth_url(&pkce);

        open_browser(&auth_url)?;

        let callback_result = self.wait_for_callback()?;

        let token = self
            .exchange_code(&callback_result.code, &pkce.verifier)
            .await?;

        Ok(token)
    }

    /// Refresh an expired access token using a refresh token.
    pub async fn refresh(&self, refresh_token: &str) -> Result<Token, OAuthError> {
        let auth = SpotifyAuth::new();

        let response = auth.refresh_token(&self.client_id, refresh_token).await?;

        let token_response: SpotifyTokenResponse =
            serde_json::from_value(response).map_err(|_| OAuthError::TokenParse)?;

        Ok(Token::from_response(token_response))
    }

    fn build_auth_url(&self, pkce: &PkceChallenge) -> String {
        let mut url = Url::parse(&SpotifyAuth::url(AUTHORIZE_ENDPOINT)).unwrap();

        url.query_pairs_mut()
            .append_pair("client_id", &self.client_id)
            .append_pair("response_type", "code")
            .append_pair("redirect_uri", &self.redirect_uri)
            .append_pair("scope", &self.scopes.join(" "))
            .append_pair("code_challenge_method", "S256")
            .append_pair("code_challenge", &pkce.challenge);

        url.to_string()
    }

    fn wait_for_callback(&self) -> Result<CallbackResult, OAuthError> {
        let server = CallbackServer::new(self.port);
        let result = server.wait_for_callback()?;
        Ok(result)
    }

    async fn exchange_code(&self, code: &str, verifier: &str) -> Result<Token, OAuthError> {
        let auth = SpotifyAuth::new();

        let response = auth
            .exchange_code(&self.client_id, code, &self.redirect_uri, verifier)
            .await?;

        let token_response: SpotifyTokenResponse =
            serde_json::from_value(response).map_err(|_| OAuthError::TokenParse)?;

        Ok(Token::from_response(token_response))
    }
}

fn default_scopes() -> Vec<String> {
    vec![
        "user-read-playback-state".to_string(),
        "user-modify-playback-state".to_string(),
        "user-read-currently-playing".to_string(),
        "user-library-read".to_string(),
        "user-library-modify".to_string(),
        "playlist-read-private".to_string(),
        "playlist-read-collaborative".to_string(),
        "playlist-modify-private".to_string(),
        "playlist-modify-public".to_string(),
        "user-read-private".to_string(),
        "user-read-email".to_string(),
        "user-top-read".to_string(),
        "user-read-recently-played".to_string(),
        "user-follow-read".to_string(),
        "user-follow-modify".to_string(),
    ]
}

fn open_browser(url: &str) -> Result<(), OAuthError> {
    #[cfg(target_os = "macos")]
    {
        std::process::Command::new("open")
            .arg(url)
            .spawn()
            .map_err(|e| OAuthError::Browser(e.to_string()))?;
    }

    #[cfg(target_os = "linux")]
    {
        std::process::Command::new("xdg-open")
            .arg(url)
            .spawn()
            .map_err(|e| OAuthError::Browser(e.to_string()))?;
    }

    #[cfg(target_os = "windows")]
    {
        std::process::Command::new("cmd")
            .args(["/C", "start", "", url])
            .spawn()
            .map_err(|e| OAuthError::Browser(e.to_string()))?;
    }

    Ok(())
}
