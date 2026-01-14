//! Persistent storage for configuration, tokens, and pins.
//!
//! All data is stored in the user's config directory:
//! - Linux/macOS: `~/.config/spotify-cli/`
//! - Windows: `%APPDATA%/spotify-cli/`
//!
//! ## Files
//!
//! - `config.toml` - User configuration (client_id, search settings)
//! - `token.json` - OAuth access and refresh tokens (fallback)
//! - `pins.json` - Pinned resource shortcuts
//!
//! ## Secure Storage
//!
//! Tokens can be stored securely in the system keychain:
//! - macOS: Keychain
//! - Linux: Secret Service (GNOME Keyring, KWallet)
//! - Windows: Windows Credential Manager

pub mod config;
pub mod fuzzy;
pub mod keyring;
pub mod paths;
pub mod pins;
pub mod token_store;
pub mod unified_token;
