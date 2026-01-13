use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use thiserror::Error;

use super::fuzzy::calculate_fuzzy_score;
use super::pin::Pin;
use super::resource_type::ResourceType;
use super::PINS_FILE;
use crate::storage::paths;

#[derive(Debug, Error)]
pub enum PinError {
    #[error("Failed to get config directory: {0}")]
    PathError(#[from] paths::PathError),
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
    #[error("JSON error: {0}")]
    JsonError(#[from] serde_json::Error),
    #[error("Pin not found: {0}")]
    NotFound(String),
    #[error("Pin already exists with alias: {0}")]
    AlreadyExists(String),
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct PinStore {
    pins: Vec<Pin>,
    #[serde(skip)]
    path: Option<PathBuf>,
}

impl PinStore {
    pub fn new() -> Result<Self, PinError> {
        let config_dir = paths::config_dir()?;
        let path = config_dir.join(PINS_FILE);

        let mut store = if path.exists() {
            let contents = fs::read_to_string(&path)?;
            serde_json::from_str(&contents)?
        } else {
            PinStore::default()
        };

        store.path = Some(path);
        Ok(store)
    }

    fn save(&self) -> Result<(), PinError> {
        if let Some(path) = &self.path {
            if let Some(parent) = path.parent() {
                fs::create_dir_all(parent)?;
            }
            let contents = serde_json::to_string_pretty(&self)?;
            fs::write(path, contents)?;
        }
        Ok(())
    }

    pub fn add(&mut self, pin: Pin) -> Result<(), PinError> {
        // Check for duplicate alias
        if self.pins.iter().any(|p| p.alias.to_lowercase() == pin.alias.to_lowercase()) {
            return Err(PinError::AlreadyExists(pin.alias));
        }
        self.pins.push(pin);
        self.save()
    }

    pub fn remove(&mut self, alias_or_id: &str) -> Result<Pin, PinError> {
        let lower = alias_or_id.to_lowercase();
        let idx = self
            .pins
            .iter()
            .position(|p| p.alias.to_lowercase() == lower || p.id == alias_or_id)
            .ok_or_else(|| PinError::NotFound(alias_or_id.to_string()))?;

        let removed = self.pins.remove(idx);
        self.save()?;
        Ok(removed)
    }

    pub fn list(&self, filter_type: Option<ResourceType>) -> Vec<&Pin> {
        self.pins
            .iter()
            .filter(|p| filter_type.is_none_or(|t| p.resource_type == t))
            .collect()
    }

    pub fn find_by_alias(&self, alias: &str) -> Option<&Pin> {
        let lower = alias.to_lowercase();
        self.pins.iter().find(|p| p.alias.to_lowercase() == lower)
    }

    pub fn all(&self) -> &[Pin] {
        &self.pins
    }

    /// Fuzzy search pins by query matching against alias and tags
    /// Returns pins with a relevance score (higher = better match)
    pub fn fuzzy_search(&self, query: &str) -> Vec<(&Pin, f64)> {
        let query_lower = query.to_lowercase();
        let query_words: Vec<&str> = query_lower.split_whitespace().collect();

        self.pins
            .iter()
            .filter_map(|pin| {
                let score = calculate_fuzzy_score(pin, &query_lower, &query_words);
                if score > 0.0 {
                    Some((pin, score))
                } else {
                    None
                }
            })
            .collect()
    }
}
