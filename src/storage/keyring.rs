//! Secure token storage using system keychain.
//!
//! Uses the platform's native credential manager:
//! - macOS: Keychain
//! - Linux: Secret Service (GNOME Keyring, KWallet)
//! - Windows: Windows Credential Manager

use keyring::Entry;
use thiserror::Error;

use crate::oauth::token::Token;

const SERVICE_NAME: &str = "spotify-cli";
const TOKEN_KEY: &str = "oauth_token";

/// Errors that can occur when using keyring storage.
#[derive(Debug, Error)]
pub enum KeyringError {
    #[error("Keyring error: {0}")]
    Keyring(#[from] keyring::Error),

    #[error("Failed to serialize token: {0}")]
    Serialize(#[from] serde_json::Error),

    #[error("Token not found in keyring")]
    NotFound,
}

/// Secure token storage using system keychain.
pub struct KeyringStore {
    entry: Entry,
}

impl KeyringStore {
    /// Create a new keyring store.
    pub fn new() -> Result<Self, KeyringError> {
        let entry = Entry::new(SERVICE_NAME, TOKEN_KEY)?;
        Ok(Self { entry })
    }

    /// Save a token to the keychain.
    pub fn save(&self, token: &Token) -> Result<(), KeyringError> {
        let json = serde_json::to_string(token)?;
        self.entry.set_password(&json)?;
        Ok(())
    }

    /// Load a token from the keychain.
    pub fn load(&self) -> Result<Token, KeyringError> {
        let json = self.entry.get_password().map_err(|e| match e {
            keyring::Error::NoEntry => KeyringError::NotFound,
            other => KeyringError::Keyring(other),
        })?;
        let token = serde_json::from_str(&json)?;
        Ok(token)
    }

    /// Delete the token from the keychain.
    pub fn delete(&self) -> Result<(), KeyringError> {
        match self.entry.delete_credential() {
            Ok(()) => Ok(()),
            Err(keyring::Error::NoEntry) => Ok(()), // Already deleted
            Err(e) => Err(KeyringError::Keyring(e)),
        }
    }

    /// Check if a token exists in the keychain.
    pub fn exists(&self) -> bool {
        self.entry.get_password().is_ok()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn keyring_error_display() {
        let err = KeyringError::NotFound;
        let display = format!("{}", err);
        assert!(display.contains("not found"));
    }

    #[test]
    fn keyring_error_serialize() {
        let json_err = serde_json::from_str::<Token>("invalid").unwrap_err();
        let err = KeyringError::Serialize(json_err);
        let display = format!("{}", err);
        assert!(display.contains("serialize"));
    }
}
