//! Cache storage for devices, playlists, pins, search results, and metadata.
use std::env;
use std::fs;
use std::path::{Path, PathBuf};

use crate::cache::devices::DeviceCache;
use crate::cache::metadata::MetadataStore;
use crate::cache::playlists::PlaylistCache;
use crate::error::Result;

pub mod devices;
pub mod metadata;
pub mod pins;
pub mod playlists;
pub mod search;

#[derive(Debug, Clone)]
pub struct Cache {
    root: PathBuf,
}

impl Cache {
    pub fn new() -> Result<Self> {
        let root = default_root()?;
        Ok(Self { root })
    }

    pub fn ensure_dirs(&self) -> Result<()> {
        fs::create_dir_all(self.root())?;
        Ok(())
    }

    pub fn root(&self) -> &Path {
        &self.root
    }

    pub fn metadata_store(&self) -> MetadataStore {
        MetadataStore::new(self.root.join("metadata.json"))
    }

    pub fn device_cache(&self) -> DeviceCache {
        DeviceCache::new(self.root.join("devices.json"))
    }

    pub fn playlist_cache(&self) -> PlaylistCache {
        PlaylistCache::new(self.root.join("playlists.json"))
    }

    pub fn pin_store(&self) -> pins::PinStore {
        pins::PinStore::new(self.root.join("pins.json"))
    }

    pub fn search_store(&self) -> search::SearchStore {
        search::SearchStore::new(self.root.join("search.json"))
    }
}

fn default_root() -> Result<PathBuf> {
    if let Ok(custom) = env::var("SPOTIFY_CLI_CACHE_DIR") {
        return Ok(PathBuf::from(custom));
    }

    if let Ok(xdg_cache) = env::var("XDG_CACHE_HOME") {
        return Ok(PathBuf::from(xdg_cache).join("spotify-cli"));
    }

    if cfg!(windows) {
        if let Ok(local) = env::var("LOCALAPPDATA") {
            return Ok(PathBuf::from(local).join("spotify-cli"));
        }

        if let Ok(roaming) = env::var("APPDATA") {
            return Ok(PathBuf::from(roaming).join("spotify-cli"));
        }

        if let Ok(profile) = env::var("USERPROFILE") {
            return Ok(PathBuf::from(profile).join(".cache").join("spotify-cli"));
        }
    }

    let home = env::var("HOME")?;
    Ok(PathBuf::from(home).join(".cache").join("spotify-cli"))
}

#[cfg(test)]
mod tests {
    use super::default_root;
    use std::sync::Mutex;

    static ENV_LOCK: Mutex<()> = Mutex::new(());

    fn set_env(key: &str, value: &str) {
        unsafe {
            std::env::set_var(key, value);
        }
    }

    fn remove_env(key: &str) {
        unsafe {
            std::env::remove_var(key);
        }
    }

    fn restore_env(key: &str, value: Option<String>) {
        match value {
            Some(value) => set_env(key, &value),
            None => remove_env(key),
        }
    }

    #[test]
    fn default_root_uses_custom_dir() {
        let _lock = ENV_LOCK.lock().unwrap();
        let prev_cache = std::env::var("SPOTIFY_CLI_CACHE_DIR").ok();
        let prev_xdg = std::env::var("XDG_CACHE_HOME").ok();
        set_env("SPOTIFY_CLI_CACHE_DIR", "/tmp/custom-cache");
        remove_env("XDG_CACHE_HOME");
        let root = default_root().expect("root");
        assert_eq!(root.to_string_lossy(), "/tmp/custom-cache");
        restore_env("SPOTIFY_CLI_CACHE_DIR", prev_cache);
        restore_env("XDG_CACHE_HOME", prev_xdg);
    }

    #[test]
    #[cfg(not(windows))]
    fn default_root_uses_xdg_cache() {
        let _lock = ENV_LOCK.lock().unwrap();
        let prev_cache = std::env::var("SPOTIFY_CLI_CACHE_DIR").ok();
        let prev_xdg = std::env::var("XDG_CACHE_HOME").ok();
        remove_env("SPOTIFY_CLI_CACHE_DIR");
        set_env("XDG_CACHE_HOME", "/tmp/xdg-cache");
        let root = default_root().expect("root");
        assert_eq!(root.to_string_lossy(), "/tmp/xdg-cache/spotify-cli");
        restore_env("SPOTIFY_CLI_CACHE_DIR", prev_cache);
        restore_env("XDG_CACHE_HOME", prev_xdg);
    }

    #[test]
    #[cfg(not(windows))]
    fn default_root_uses_home_cache() {
        let _lock = ENV_LOCK.lock().unwrap();
        let prev_cache = std::env::var("SPOTIFY_CLI_CACHE_DIR").ok();
        let prev_xdg = std::env::var("XDG_CACHE_HOME").ok();
        let prev_home = std::env::var("HOME").ok();
        remove_env("SPOTIFY_CLI_CACHE_DIR");
        remove_env("XDG_CACHE_HOME");
        set_env("HOME", "/tmp/home");
        let root = default_root().expect("root");
        assert_eq!(root.to_string_lossy(), "/tmp/home/.cache/spotify-cli");
        restore_env("SPOTIFY_CLI_CACHE_DIR", prev_cache);
        restore_env("XDG_CACHE_HOME", prev_xdg);
        restore_env("HOME", prev_home);
    }
}
