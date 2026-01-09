use std::fs;
use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use crate::domain::playlist::Playlist;
use crate::error::Result;

/// JSON-backed playlist cache store.
#[derive(Debug, Clone)]
pub struct PlaylistCache {
    path: PathBuf,
}

impl PlaylistCache {
    pub fn new(path: PathBuf) -> Self {
        Self { path }
    }

    pub fn load(&self) -> Result<Option<CacheSnapshot<Playlist>>> {
        if !self.path.exists() {
            return Ok(None);
        }
        let contents = fs::read_to_string(&self.path)?;
        let snapshot = serde_json::from_str(&contents)?;
        Ok(Some(snapshot))
    }

    pub fn save(&self, snapshot: &CacheSnapshot<Playlist>) -> Result<()> {
        let payload = serde_json::to_string_pretty(snapshot)?;
        fs::write(&self.path, payload)?;
        Ok(())
    }
}

/// Snapshot wrapper for cached items.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheSnapshot<T> {
    pub updated_at: u64,
    pub items: Vec<T>,
}

#[cfg(test)]
mod tests {
    use super::{CacheSnapshot, PlaylistCache};
    use crate::domain::playlist::Playlist;
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
    fn playlist_cache_round_trip() {
        let path = temp_path("playlists");
        let cache = PlaylistCache::new(path.clone());
        let snapshot = CacheSnapshot {
            updated_at: 7,
            items: vec![Playlist {
                id: "1".to_string(),
                name: "MyRadar".to_string(),
                owner: Some("Me".to_string()),
                collaborative: false,
                public: Some(false),
            }],
        };
        cache.save(&snapshot).expect("save");
        let loaded = cache.load().expect("load").expect("snapshot");
        assert_eq!(loaded.updated_at, 7);
        assert_eq!(loaded.items.len(), 1);
        assert_eq!(loaded.items[0].name, "MyRadar");
        let _ = fs::remove_file(path);
    }
}
