//! Configuration file handling.
//!
//! Loads and parses the TOML configuration file from the user's config directory.

use serde::{Deserialize, Serialize};
use std::fs;
use thiserror::Error;

use super::paths;

/// Errors that can occur when loading configuration.
#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("Config file not found at {0}")]
    NotFound(String),

    #[error("Failed to read config: {0}")]
    Read(#[from] std::io::Error),

    #[error("Failed to parse config: {0}")]
    Parse(#[from] toml::de::Error),

    #[error("Failed to serialize config: {0}")]
    Serialize(#[from] toml::ser::Error),

    #[error("Could not determine config path: {0}")]
    Path(#[from] paths::PathError),

    #[error("Missing required field: {0}")]
    MissingField(String),
}

/// Spotify API credentials and core settings.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpotifyConfig {
    /// Spotify Developer App client ID.
    pub client_id: String,
    /// Token storage backend (keyring or file)
    #[serde(default)]
    pub token_storage: TokenStorageBackend,
}

/// Fuzzy search scoring configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FuzzyConfig {
    /// Score for exact name match
    #[serde(default = "default_exact_match")]
    pub exact_match: f64,
    /// Score for name starts with query
    #[serde(default = "default_starts_with")]
    pub starts_with: f64,
    /// Score for name contains query
    #[serde(default = "default_contains")]
    pub contains: f64,
    /// Score for each word match
    #[serde(default = "default_word_match")]
    pub word_match: f64,
    /// Minimum similarity threshold for Levenshtein matching (0.0-1.0)
    #[serde(default = "default_similarity_threshold")]
    pub similarity_threshold: f64,
    /// Weight multiplier for Levenshtein similarity bonus
    #[serde(default = "default_similarity_weight")]
    pub similarity_weight: f64,
}

fn default_exact_match() -> f64 { 100.0 }
fn default_starts_with() -> f64 { 50.0 }
fn default_contains() -> f64 { 30.0 }
fn default_word_match() -> f64 { 10.0 }
fn default_similarity_threshold() -> f64 { 0.6 }
fn default_similarity_weight() -> f64 { 20.0 }

impl Default for FuzzyConfig {
    fn default() -> Self {
        Self {
            exact_match: default_exact_match(),
            starts_with: default_starts_with(),
            contains: default_contains(),
            word_match: default_word_match(),
            similarity_threshold: default_similarity_threshold(),
            similarity_weight: default_similarity_weight(),
        }
    }
}

/// Search behavior configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchConfig {
    /// Include fuzzy scores in results
    #[serde(default = "default_show_scores")]
    pub show_scores: bool,
    /// Sort results by fuzzy score (default: false, preserves Spotify's order)
    #[serde(default = "default_sort_by_score")]
    pub sort_by_score: bool,
    /// Fuzzy matching configuration
    #[serde(default)]
    pub fuzzy: FuzzyConfig,
}

fn default_show_scores() -> bool { true }
fn default_sort_by_score() -> bool { false }

impl Default for SearchConfig {
    fn default() -> Self {
        Self {
            show_scores: default_show_scores(),
            sort_by_score: default_sort_by_score(),
            fuzzy: FuzzyConfig::default(),
        }
    }
}

/// Token storage backend options.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum TokenStorageBackend {
    /// Use system keychain (default, most secure)
    #[default]
    Keyring,
    /// Use file-based storage (fallback)
    File,
}

/// Root configuration structure.
///
/// Loaded from `~/.config/spotify-cli/config.toml`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    #[serde(rename = "spotify-cli")]
    pub spotify_cli: SpotifyConfig,
    #[serde(default)]
    pub search: SearchConfig,
}

impl Config {
    /// Load configuration from the default config file.
    ///
    /// Returns error if file doesn't exist or client_id is missing.
    pub fn load() -> Result<Self, ConfigError> {
        let path = paths::config_file()?;

        if !path.exists() {
            return Err(ConfigError::NotFound(path.display().to_string()));
        }

        let contents = fs::read_to_string(&path)?;
        let config: Config = toml::from_str(&contents)?;

        if config.spotify_cli.client_id.is_empty() {
            return Err(ConfigError::MissingField("client_id".to_string()));
        }

        Ok(config)
    }

    pub fn client_id(&self) -> &str {
        &self.spotify_cli.client_id
    }

    pub fn fuzzy(&self) -> &FuzzyConfig {
        &self.search.fuzzy
    }

    pub fn show_scores(&self) -> bool {
        self.search.show_scores
    }

    pub fn sort_by_score(&self) -> bool {
        self.search.sort_by_score
    }

    pub fn token_storage(&self) -> TokenStorageBackend {
        self.spotify_cli.token_storage
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_valid_config() {
        let toml = r#"
[spotify-cli]
client_id = "abc123"
"#;

        let config: Config = toml::from_str(toml).unwrap();
        assert_eq!(config.client_id(), "abc123");
    }

    #[test]
    fn missing_client_id_fails() {
        let toml = r#"
[spotify-cli]
client_id = ""
"#;

        let config: Config = toml::from_str(toml).unwrap();
        // The parse succeeds, but load() would fail with MissingField
        assert!(config.client_id().is_empty());
    }

    #[test]
    fn fuzzy_config_default_values() {
        let fuzzy = FuzzyConfig::default();
        assert_eq!(fuzzy.exact_match, 100.0);
        assert_eq!(fuzzy.starts_with, 50.0);
        assert_eq!(fuzzy.contains, 30.0);
        assert_eq!(fuzzy.word_match, 10.0);
        assert_eq!(fuzzy.similarity_threshold, 0.6);
        assert_eq!(fuzzy.similarity_weight, 20.0);
    }

    #[test]
    fn search_config_default_values() {
        let search = SearchConfig::default();
        assert!(search.show_scores);
        assert!(!search.sort_by_score);
    }

    #[test]
    fn config_with_search_settings() {
        let toml = r#"
[spotify-cli]
client_id = "abc123"

[search]
show_scores = false
sort_by_score = true
"#;

        let config: Config = toml::from_str(toml).unwrap();
        assert!(!config.show_scores());
        assert!(config.sort_by_score());
    }

    #[test]
    fn config_with_fuzzy_settings() {
        let toml = r#"
[spotify-cli]
client_id = "abc123"

[search.fuzzy]
exact_match = 200.0
starts_with = 100.0
"#;

        let config: Config = toml::from_str(toml).unwrap();
        assert_eq!(config.fuzzy().exact_match, 200.0);
        assert_eq!(config.fuzzy().starts_with, 100.0);
        // Defaults should still apply for unset fields
        assert_eq!(config.fuzzy().contains, 30.0);
    }

    #[test]
    fn config_defaults_when_search_section_missing() {
        let toml = r#"
[spotify-cli]
client_id = "abc123"
"#;

        let config: Config = toml::from_str(toml).unwrap();
        // Default search config values
        assert!(config.show_scores());
        assert!(!config.sort_by_score());
    }

    #[test]
    fn config_error_display() {
        let err = ConfigError::NotFound("/path/to/config".to_string());
        assert!(err.to_string().contains("/path/to/config"));

        let err = ConfigError::MissingField("client_id".to_string());
        assert!(err.to_string().contains("client_id"));
    }

    #[test]
    fn spotify_config_deserializes() {
        let toml = r#"client_id = "test_client_id""#;
        let config: SpotifyConfig = toml::from_str(toml).unwrap();
        assert_eq!(config.client_id, "test_client_id");
    }

    #[test]
    fn token_storage_defaults_to_keyring() {
        let toml = r#"
[spotify-cli]
client_id = "abc123"
"#;
        let config: Config = toml::from_str(toml).unwrap();
        assert_eq!(config.token_storage(), TokenStorageBackend::Keyring);
    }

    #[test]
    fn token_storage_file_option() {
        let toml = r#"
[spotify-cli]
client_id = "abc123"

token_storage = "file"
"#;
        let config: Config = toml::from_str(toml).unwrap();
        assert_eq!(config.token_storage(), TokenStorageBackend::File);
    }

    #[test]
    fn token_storage_keyring_option() {
        let toml = r#"
[spotify-cli]
client_id = "abc123"

token_storage = "keyring"
"#;
        let config: Config = toml::from_str(toml).unwrap();
        assert_eq!(config.token_storage(), TokenStorageBackend::Keyring);
    }

    #[test]
    fn token_storage_backend_default() {
        let backend = TokenStorageBackend::default();
        assert_eq!(backend, TokenStorageBackend::Keyring);
    }
}
