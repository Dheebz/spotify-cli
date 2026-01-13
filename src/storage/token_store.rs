//! OAuth token persistence.
//!
//! Stores tokens in JSON format in the user's config directory.
//! Tokens are automatically saved after successful authentication
//! and loaded on subsequent CLI invocations.

use std::fs;
use std::path::PathBuf;
use thiserror::Error;

use super::paths;
use crate::oauth::token::Token;

/// Errors that can occur when storing/loading tokens.
#[derive(Debug, Error)]
pub enum TokenStoreError {
    #[error("Could not determine config directory: {0}")]
    Path(#[from] paths::PathError),

    #[error("Failed to create directory: {0}")]
    CreateDir(#[from] std::io::Error),

    #[error("Failed to serialize token: {0}")]
    Serialize(#[from] serde_json::Error),

    #[error("Token not found")]
    NotFound,
}

/// Token storage manager.
///
/// Handles reading and writing OAuth tokens to disk.
pub struct TokenStore {
    path: PathBuf,
}

impl TokenStore {
    /// Create a new token store using the default path.
    pub fn new() -> Result<Self, TokenStoreError> {
        let path = paths::token_file()?;
        Ok(Self { path })
    }

    /// Save a token to disk.
    ///
    /// Creates the parent directory if it doesn't exist.
    pub fn save(&self, token: &Token) -> Result<(), TokenStoreError> {
        if let Some(parent) = self.path.parent() {
            fs::create_dir_all(parent)?;
        }

        let json = serde_json::to_string_pretty(token)?;
        fs::write(&self.path, json)?;

        Ok(())
    }

    /// Load a token from disk.
    ///
    /// Returns `NotFound` error if no token file exists.
    pub fn load(&self) -> Result<Token, TokenStoreError> {
        if !self.path.exists() {
            return Err(TokenStoreError::NotFound);
        }

        let json = fs::read_to_string(&self.path)?;
        let token = serde_json::from_str(&json)?;

        Ok(token)
    }

    /// Delete the stored token.
    pub fn delete(&self) -> Result<(), TokenStoreError> {
        if self.path.exists() {
            fs::remove_file(&self.path)?;
        }

        Ok(())
    }

    /// Check if a token file exists.
    pub fn exists(&self) -> bool {
        self.path.exists()
    }

    /// Get the path to the token file.
    pub fn path(&self) -> &PathBuf {
        &self.path
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::oauth::token::SpotifyTokenResponse;
    use std::env;

    const TOKEN_FILE: &str = "token.json";

    fn temp_store() -> TokenStore {
        let temp_dir = env::temp_dir().join(format!("spotify-cli-test-{}", rand::random::<u64>()));
        TokenStore {
            path: temp_dir.join(TOKEN_FILE),
        }
    }

    fn make_token() -> Token {
        Token::from_response(SpotifyTokenResponse {
            access_token: "test_access".to_string(),
            token_type: "Bearer".to_string(),
            scope: "user-read-playback-state".to_string(),
            expires_in: 3600,
            refresh_token: Some("test_refresh".to_string()),
        })
    }

    #[test]
    fn save_and_load_token() {
        let store = temp_store();
        let token = make_token();

        store.save(&token).unwrap();
        let loaded = store.load().unwrap();

        assert_eq!(loaded.access_token, token.access_token);
        assert_eq!(loaded.refresh_token, token.refresh_token);

        store.delete().unwrap();
    }

    #[test]
    fn load_nonexistent_returns_not_found() {
        let store = temp_store();
        let result = store.load();

        assert!(matches!(result, Err(TokenStoreError::NotFound)));
    }

    #[test]
    fn exists_returns_false_when_no_token() {
        let store = temp_store();
        assert!(!store.exists());
    }

    #[test]
    fn exists_returns_true_after_save() {
        let store = temp_store();
        let token = make_token();

        store.save(&token).unwrap();
        assert!(store.exists());

        store.delete().unwrap();
    }
}
