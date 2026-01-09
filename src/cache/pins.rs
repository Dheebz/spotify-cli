use std::fs;
use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use crate::domain::pin::PinnedPlaylist;
use crate::error::Result;

/// JSON-backed pin store for local playlist shortcuts.
#[derive(Debug, Clone)]
pub struct PinStore {
    path: PathBuf,
}

impl PinStore {
    pub fn new(path: PathBuf) -> Self {
        Self { path }
    }

    pub fn load(&self) -> Result<Pins> {
        if !self.path.exists() {
            return Ok(Pins::default());
        }
        let contents = fs::read_to_string(&self.path)?;
        let pins = serde_json::from_str(&contents)?;
        Ok(pins)
    }

    pub fn save(&self, pins: &Pins) -> Result<()> {
        let payload = serde_json::to_string_pretty(pins)?;
        fs::write(&self.path, payload)?;
        Ok(())
    }

    pub fn add(&self, name: String, url: String) -> Result<()> {
        let mut pins = self.load()?;
        let lower = name.to_lowercase();
        if let Some(existing) = pins
            .items
            .iter_mut()
            .find(|item| item.name.to_lowercase() == lower)
        {
            existing.url = url;
            existing.name = name;
        } else {
            pins.items.push(PinnedPlaylist { name, url });
        }
        self.save(&pins)
    }

    pub fn remove(&self, name: &str) -> Result<bool> {
        let mut pins = self.load()?;
        let before = pins.items.len();
        let lower = name.to_lowercase();
        pins.items.retain(|item| item.name.to_lowercase() != lower);
        let removed = pins.items.len() != before;
        if removed {
            self.save(&pins)?;
        }
        Ok(removed)
    }
}

/// Pin collection payload.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Pins {
    pub items: Vec<PinnedPlaylist>,
}

#[cfg(test)]
mod tests {
    use super::PinStore;
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
    fn pin_store_add_and_remove() {
        let path = temp_path("pins");
        let store = PinStore::new(path.clone());

        store
            .add(
                "Release Radar".to_string(),
                "https://example.com".to_string(),
            )
            .unwrap();
        let loaded = store.load().unwrap();
        assert_eq!(loaded.items.len(), 1);

        let removed = store.remove("Release Radar").unwrap();
        assert!(removed);
        let loaded = store.load().unwrap();
        assert!(loaded.items.is_empty());

        let _ = fs::remove_file(path);
    }
}
