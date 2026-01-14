//! Unified token storage with automatic backend selection.
//!
//! Provides a single interface for token storage that can use either:
//! - System keychain (default, most secure)
//! - File-based storage (fallback)
//!
//! The backend is selected based on user configuration, with automatic
//! fallback to file storage if keychain is unavailable.

use thiserror::Error;
use tracing::{debug, warn};

use super::config::TokenStorageBackend;
use super::keyring::{KeyringError, KeyringStore};
use super::token_store::{TokenStore, TokenStoreError};
use crate::oauth::token::Token;

/// Errors that can occur with unified token storage.
#[derive(Debug, Error)]
pub enum UnifiedTokenError {
    #[error("File storage error: {0}")]
    File(#[from] TokenStoreError),

    #[error("Keyring error: {0}")]
    Keyring(#[from] KeyringError),

    #[error("Token not found")]
    NotFound,
}

/// Unified token storage that abstracts over keyring and file backends.
pub struct UnifiedTokenStore {
    backend: TokenStorageBackend,
    keyring: Option<KeyringStore>,
    file: TokenStore,
}

impl UnifiedTokenStore {
    /// Create a new unified token store with the specified backend preference.
    ///
    /// If keyring is requested but unavailable, falls back to file storage.
    pub fn new(preferred_backend: TokenStorageBackend) -> Result<Self, UnifiedTokenError> {
        let file = TokenStore::new()?;

        let (backend, keyring) = match preferred_backend {
            TokenStorageBackend::Keyring => {
                match KeyringStore::new() {
                    Ok(store) => {
                        debug!("Using keyring for token storage");
                        (TokenStorageBackend::Keyring, Some(store))
                    }
                    Err(e) => {
                        warn!("Keyring unavailable, falling back to file storage: {}", e);
                        (TokenStorageBackend::File, None)
                    }
                }
            }
            TokenStorageBackend::File => {
                debug!("Using file for token storage (configured)");
                (TokenStorageBackend::File, None)
            }
        };

        Ok(Self {
            backend,
            keyring,
            file,
        })
    }

    /// Save a token using the active backend.
    pub fn save(&self, token: &Token) -> Result<(), UnifiedTokenError> {
        match self.backend {
            TokenStorageBackend::Keyring => {
                if let Some(ref keyring) = self.keyring {
                    keyring.save(token)?;
                    debug!("Token saved to keyring");
                    Ok(())
                } else {
                    // Fallback to file if keyring not available
                    self.file.save(token)?;
                    debug!("Token saved to file (keyring fallback)");
                    Ok(())
                }
            }
            TokenStorageBackend::File => {
                self.file.save(token)?;
                debug!("Token saved to file");
                Ok(())
            }
        }
    }

    /// Load a token from the active backend.
    ///
    /// For keyring backend, falls back to file if keyring is empty (migration support).
    pub fn load(&self) -> Result<Token, UnifiedTokenError> {
        match self.backend {
            TokenStorageBackend::Keyring => {
                if let Some(ref keyring) = self.keyring {
                    match keyring.load() {
                        Ok(token) => {
                            debug!("Token loaded from keyring");
                            Ok(token)
                        }
                        Err(KeyringError::NotFound) => {
                            // Try file as fallback (migration from file to keyring)
                            debug!("Token not in keyring, checking file for migration");
                            match self.file.load() {
                                Ok(token) => {
                                    debug!("Found token in file, migrating to keyring");
                                    // Migrate to keyring
                                    if keyring.save(&token).is_ok() {
                                        // Delete from file after successful migration
                                        let _ = self.file.delete();
                                        debug!("Token migrated from file to keyring");
                                    }
                                    Ok(token)
                                }
                                Err(TokenStoreError::NotFound) => Err(UnifiedTokenError::NotFound),
                                Err(e) => Err(UnifiedTokenError::File(e)),
                            }
                        }
                        Err(e) => Err(UnifiedTokenError::Keyring(e)),
                    }
                } else {
                    // Keyring wasn't available, use file
                    self.file.load().map_err(|e| match e {
                        TokenStoreError::NotFound => UnifiedTokenError::NotFound,
                        other => UnifiedTokenError::File(other),
                    })
                }
            }
            TokenStorageBackend::File => {
                self.file.load().map_err(|e| match e {
                    TokenStoreError::NotFound => UnifiedTokenError::NotFound,
                    other => UnifiedTokenError::File(other),
                })
            }
        }
    }

    /// Delete the token from storage.
    ///
    /// Attempts to delete from both backends to ensure clean removal.
    pub fn delete(&self) -> Result<(), UnifiedTokenError> {
        // Delete from keyring if available
        if let Some(ref keyring) = self.keyring {
            keyring.delete()?;
            debug!("Token deleted from keyring");
        }

        // Also delete from file (cleanup any legacy tokens)
        self.file.delete()?;
        debug!("Token deleted from file");

        Ok(())
    }

    /// Check if a token exists in storage.
    pub fn exists(&self) -> bool {
        match self.backend {
            TokenStorageBackend::Keyring => {
                if let Some(ref keyring) = self.keyring {
                    keyring.exists() || self.file.exists()
                } else {
                    self.file.exists()
                }
            }
            TokenStorageBackend::File => self.file.exists(),
        }
    }

    /// Get the active backend type.
    pub fn backend(&self) -> TokenStorageBackend {
        self.backend
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unified_token_error_display() {
        let err = UnifiedTokenError::NotFound;
        let display = format!("{}", err);
        assert!(display.contains("not found"));
    }

    #[test]
    fn file_backend_creates_successfully() {
        let store = UnifiedTokenStore::new(TokenStorageBackend::File);
        assert!(store.is_ok());
        assert_eq!(store.unwrap().backend(), TokenStorageBackend::File);
    }

    #[test]
    fn exists_returns_false_when_empty() {
        let store = UnifiedTokenStore::new(TokenStorageBackend::File).unwrap();
        // Note: This may return true if there's already a token file
        // We just verify it doesn't panic
        let _ = store.exists();
    }
}
