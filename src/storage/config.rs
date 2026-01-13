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

/// Spotify API credentials.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpotifyConfig {
    /// Spotify Developer App client ID.
    pub client_id: String,
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
}
