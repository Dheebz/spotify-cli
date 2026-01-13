//! Persistent storage for configuration, tokens, and pins.
//!
//! All data is stored in the user's config directory:
//! - Linux/macOS: `~/.config/spotify-cli/`
//! - Windows: `%APPDATA%/spotify-cli/`
//!
//! ## Files
//!
//! - `config.toml` - User configuration (client_id, search settings)
//! - `token.json` - OAuth access and refresh tokens
//! - `pins.json` - Pinned resource shortcuts

pub mod config;
pub mod fuzzy;
pub mod paths;
pub mod pins;
pub mod token_store;
