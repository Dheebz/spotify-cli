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
    /// Returns true if the token expires within the buffer period.
    pub fn is_expired(&self) -> bool {
        use crate::constants::TOKEN_EXPIRY_BUFFER_SECS;
        let now = current_timestamp();

        now + TOKEN_EXPIRY_BUFFER_SECS >= self.expires_at
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

    #[test]
    fn token_from_response_sets_fields() {
        let response = SpotifyTokenResponse {
            access_token: "access123".to_string(),
            token_type: "Bearer".to_string(),
            scope: "scope1 scope2".to_string(),
            expires_in: 3600,
            refresh_token: Some("refresh456".to_string()),
        };

        let token = Token::from_response(response);
        assert_eq!(token.access_token, "access123");
        assert_eq!(token.token_type, "Bearer");
        assert_eq!(token.scope, "scope1 scope2");
        assert_eq!(token.refresh_token, Some("refresh456".to_string()));
    }

    #[test]
    fn token_from_response_without_refresh_token() {
        let response = SpotifyTokenResponse {
            access_token: "access123".to_string(),
            token_type: "Bearer".to_string(),
            scope: "scope1".to_string(),
            expires_in: 3600,
            refresh_token: None,
        };

        let token = Token::from_response(response);
        assert!(token.refresh_token.is_none());
    }

    #[test]
    fn token_serializes_to_json() {
        let token = make_token(3600);
        let json = serde_json::to_value(&token).unwrap();
        assert!(json.get("access_token").is_some());
        assert!(json.get("token_type").is_some());
        assert!(json.get("scope").is_some());
        assert!(json.get("expires_at").is_some());
    }

    #[test]
    fn token_deserializes_from_json() {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        let json = serde_json::json!({
            "access_token": "access123",
            "token_type": "Bearer",
            "scope": "user-read-playback-state",
            "expires_at": now + 3600,
            "refresh_token": "refresh456"
        });

        let token: Token = serde_json::from_value(json).unwrap();
        assert_eq!(token.access_token, "access123");
        assert!(!token.is_expired());
    }

    #[test]
    fn expired_token_is_expired() {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        let token = Token {
            access_token: "expired".to_string(),
            token_type: "Bearer".to_string(),
            scope: "scope".to_string(),
            expires_at: now - 100, // Already expired
            refresh_token: None,
        };

        assert!(token.is_expired());
    }

    #[test]
    fn seconds_until_expiry_negative_for_expired() {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        let token = Token {
            access_token: "expired".to_string(),
            token_type: "Bearer".to_string(),
            scope: "scope".to_string(),
            expires_at: now - 100,
            refresh_token: None,
        };

        assert!(token.seconds_until_expiry() < 0);
    }
}
