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

    /// Create a PinStore with a custom path (for testing)
    #[cfg(test)]
    pub fn with_path(path: PathBuf) -> Result<Self, PinError> {
        let mut store = if path.exists() {
            let contents = fs::read_to_string(&path)?;
            serde_json::from_str(&contents)?
        } else {
            PinStore::default()
        };
        store.path = Some(path);
        Ok(store)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::tempdir;

    fn make_pin(alias: &str, id: &str, resource_type: ResourceType, tags: Vec<&str>) -> Pin {
        Pin::new(
            resource_type,
            id.to_string(),
            alias.to_string(),
            tags.into_iter().map(String::from).collect(),
        )
    }

    #[test]
    fn pin_store_add_and_list() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("pins.json");
        let mut store = PinStore::with_path(path).unwrap();

        let pin = make_pin("favorite", "track123", ResourceType::Track, vec![]);
        store.add(pin).unwrap();

        let pins = store.list(None);
        assert_eq!(pins.len(), 1);
        assert_eq!(pins[0].alias, "favorite");
    }

    #[test]
    fn pin_store_add_duplicate_fails() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("pins.json");
        let mut store = PinStore::with_path(path).unwrap();

        let pin1 = make_pin("favorite", "track123", ResourceType::Track, vec![]);
        let pin2 = make_pin("FAVORITE", "track456", ResourceType::Track, vec![]);

        store.add(pin1).unwrap();
        let result = store.add(pin2);

        assert!(matches!(result, Err(PinError::AlreadyExists(_))));
    }

    #[test]
    fn pin_store_remove_by_alias() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("pins.json");
        let mut store = PinStore::with_path(path).unwrap();

        store.add(make_pin("favorite", "track123", ResourceType::Track, vec![])).unwrap();
        store.add(make_pin("chill", "track456", ResourceType::Track, vec![])).unwrap();

        let removed = store.remove("favorite").unwrap();
        assert_eq!(removed.alias, "favorite");
        assert_eq!(store.list(None).len(), 1);
    }

    #[test]
    fn pin_store_remove_by_id() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("pins.json");
        let mut store = PinStore::with_path(path).unwrap();

        store.add(make_pin("favorite", "track123", ResourceType::Track, vec![])).unwrap();

        let removed = store.remove("track123").unwrap();
        assert_eq!(removed.id, "track123");
    }

    #[test]
    fn pin_store_remove_not_found() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("pins.json");
        let mut store = PinStore::with_path(path).unwrap();

        let result = store.remove("nonexistent");
        assert!(matches!(result, Err(PinError::NotFound(_))));
    }

    #[test]
    fn pin_store_list_with_filter() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("pins.json");
        let mut store = PinStore::with_path(path).unwrap();

        store.add(make_pin("track1", "t1", ResourceType::Track, vec![])).unwrap();
        store.add(make_pin("playlist1", "p1", ResourceType::Playlist, vec![])).unwrap();
        store.add(make_pin("track2", "t2", ResourceType::Track, vec![])).unwrap();

        let tracks = store.list(Some(ResourceType::Track));
        assert_eq!(tracks.len(), 2);

        let playlists = store.list(Some(ResourceType::Playlist));
        assert_eq!(playlists.len(), 1);
    }

    #[test]
    fn pin_store_find_by_alias() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("pins.json");
        let mut store = PinStore::with_path(path).unwrap();

        store.add(make_pin("MyFavorite", "track123", ResourceType::Track, vec![])).unwrap();

        let found = store.find_by_alias("myfavorite");
        assert!(found.is_some());
        assert_eq!(found.unwrap().id, "track123");
    }

    #[test]
    fn pin_store_find_by_alias_not_found() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("pins.json");
        let store = PinStore::with_path(path).unwrap();

        let found = store.find_by_alias("nonexistent");
        assert!(found.is_none());
    }

    #[test]
    fn pin_store_all() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("pins.json");
        let mut store = PinStore::with_path(path).unwrap();

        store.add(make_pin("pin1", "id1", ResourceType::Track, vec![])).unwrap();
        store.add(make_pin("pin2", "id2", ResourceType::Album, vec![])).unwrap();

        assert_eq!(store.all().len(), 2);
    }

    #[test]
    fn pin_store_fuzzy_search() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("pins.json");
        let mut store = PinStore::with_path(path).unwrap();

        store.add(make_pin("favorite song", "t1", ResourceType::Track, vec!["rock"])).unwrap();
        store.add(make_pin("chill vibes", "t2", ResourceType::Track, vec!["chill"])).unwrap();
        store.add(make_pin("workout mix", "p1", ResourceType::Playlist, vec!["gym"])).unwrap();

        let results = store.fuzzy_search("favorite");
        assert!(!results.is_empty());
        assert_eq!(results[0].0.alias, "favorite song");
    }

    #[test]
    fn pin_store_fuzzy_search_by_tag() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("pins.json");
        let mut store = PinStore::with_path(path).unwrap();

        store.add(make_pin("some track", "t1", ResourceType::Track, vec!["rock", "metal"])).unwrap();

        let results = store.fuzzy_search("rock");
        assert!(!results.is_empty());
    }

    #[test]
    fn pin_store_persistence() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("pins.json");

        // Create and add pins
        {
            let mut store = PinStore::with_path(path.clone()).unwrap();
            store.add(make_pin("persistent", "id123", ResourceType::Track, vec![])).unwrap();
        }

        // Reload and verify
        {
            let store = PinStore::with_path(path).unwrap();
            assert_eq!(store.all().len(), 1);
            assert_eq!(store.all()[0].alias, "persistent");
        }
    }

    #[test]
    fn pin_store_loads_existing_file() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("pins.json");

        // Write a pins file manually
        let mut file = fs::File::create(&path).unwrap();
        writeln!(file, r#"{{"pins":[{{"resource_type":"track","id":"abc","alias":"test","tags":[]}}]}}"#).unwrap();

        let store = PinStore::with_path(path).unwrap();
        assert_eq!(store.all().len(), 1);
        assert_eq!(store.all()[0].id, "abc");
    }

    #[test]
    fn pin_error_display() {
        let err = PinError::NotFound("test".to_string());
        assert!(err.to_string().contains("test"));

        let err = PinError::AlreadyExists("alias".to_string());
        assert!(err.to_string().contains("alias"));
    }
}
