//! OAuth token types and expiry handling.

use serde::{Deserialize, Serialize};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

/// OAuth token with expiry tracking.
///
/// Stores both access and refresh tokens, along with the absolute expiry timestamp.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Token {
    /// Bearer token for API requests.
    pub access_token: String,
    /// Token type (always "Bearer" for Spotify).
    pub token_type: String,
    /// Space-separated list of granted scopes.
    pub scope: String,
    /// Unix timestamp when the access token expires.
    pub expires_at: u64,
    /// Token used to obtain new access tokens.
    pub refresh_token: Option<String>,
}

/// Raw token response from Spotify's token endpoint.
#[derive(Debug, Deserialize)]
pub struct SpotifyTokenResponse {
    pub access_token: String,
    pub token_type: String,
    pub scope: String,
    /// Seconds until token expires.
    pub expires_in: u64,
    pub refresh_token: Option<String>,
}

impl Token {
    /// Create a token from Spotify's API response.
    ///
    /// Converts the relative `expires_in` to an absolute timestamp.
    pub fn from_response(response: SpotifyTokenResponse) -> Self {
        let expires_at = current_timestamp() + response.expires_in;

        Self {
            access_token: response.access_token,
            token_type: response.token_type,
            scope: response.scope,
            expires_at,
            refresh_token: response.refresh_token,
        }
    }

    /// Check if the token is expired or about to expire.
    ///
    /// Returns true if the token expires within 60 seconds.
    pub fn is_expired(&self) -> bool {
        let now = current_timestamp();
        let buffer = 60; // Consider expired 60 seconds early

        now + buffer >= self.expires_at
    }

    /// Get seconds until the token expires.
    ///
    /// Returns negative value if already expired.
    pub fn seconds_until_expiry(&self) -> i64 {
        let now = current_timestamp();
        self.expires_at as i64 - now as i64
    }
}

fn current_timestamp() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or(Duration::ZERO)
        .as_secs()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_token(expires_in: u64) -> Token {
        let response = SpotifyTokenResponse {
            access_token: "test_access".to_string(),
            token_type: "Bearer".to_string(),
            scope: "user-read-playback-state".to_string(),
            expires_in,
            refresh_token: Some("test_refresh".to_string()),
        };

        Token::from_response(response)
    }

    #[test]
    fn fresh_token_is_not_expired() {
        let token = make_token(3600);
        assert!(!token.is_expired());
    }

    #[test]
    fn token_expiring_soon_is_expired() {
        let token = make_token(30);
        assert!(token.is_expired());
    }

    #[test]
    fn seconds_until_expiry_is_positive_for_fresh_token() {
        let token = make_token(3600);
        assert!(token.seconds_until_expiry() > 0);
    }
}
