use std::fs;
use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use crate::domain::search::SearchResults;
use crate::error::Result;

/// JSON-backed cache for the last search results.
#[derive(Debug, Clone)]
pub struct SearchStore {
    path: PathBuf,
}

impl SearchStore {
    pub fn new(path: PathBuf) -> Self {
        Self { path }
    }

    pub fn load(&self) -> Result<Option<CachedSearch>> {
        if !self.path.exists() {
            return Ok(None);
        }
        let contents = fs::read_to_string(&self.path)?;
        let cached = serde_json::from_str(&contents)?;
        Ok(Some(cached))
    }

    pub fn save(&self, cached: &CachedSearch) -> Result<()> {
        let payload = serde_json::to_string_pretty(cached)?;
        fs::write(&self.path, payload)?;
        Ok(())
    }
}

/// Cached search entry persisted on disk.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CachedSearch {
    pub query: String,
    pub results: SearchResults,
}

#[cfg(test)]
mod tests {
    use super::{CachedSearch, SearchStore};
    use crate::domain::search::{SearchResults, SearchType};
    use std::fs;
    use std::path::PathBuf;

    fn temp_path(name: &str) -> PathBuf {
        let mut path = std::env::temp_dir();
        let stamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        path.push(format!("spotify-cli-{name}-{stamp}.json"));
        path
    }

    #[test]
    fn search_store_round_trip() {
        let path = temp_path("search");
        let store = SearchStore::new(path.clone());
        let cached = CachedSearch {
            query: "boards".to_string(),
            results: SearchResults {
                kind: SearchType::Track,
                items: Vec::new(),
            },
        };
        store.save(&cached).expect("save");
        let loaded = store.load().expect("load").expect("cached");
        assert_eq!(loaded.query, "boards");
        assert_eq!(loaded.results.kind, SearchType::Track);
        let _ = fs::remove_file(path);
    }
}
