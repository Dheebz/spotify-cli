use std::path::PathBuf;
use thiserror::Error;

const APP_DIR: &str = "spotify-cli";

#[derive(Debug, Error)]
pub enum PathError {
    #[error("Could not determine home directory")]
    NoHomeDir,
}

pub fn config_dir() -> Result<PathBuf, PathError> {
    #[cfg(target_os = "windows")]
    {
        dirs::config_dir()
            .map(|p| p.join(APP_DIR))
            .ok_or(PathError::NoHomeDir)
    }

    #[cfg(not(target_os = "windows"))]
    {
        // Use XDG (~/.config) for both Linux and macOS
        dirs::home_dir()
            .map(|p| p.join(".config").join(APP_DIR))
            .ok_or(PathError::NoHomeDir)
    }
}

pub fn config_file() -> Result<PathBuf, PathError> {
    config_dir().map(|p| p.join("config.toml"))
}

pub fn token_file() -> Result<PathBuf, PathError> {
    config_dir().map(|p| p.join("token.json"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn config_dir_ends_with_app_name() {
        let dir = config_dir().unwrap();
        assert!(dir.ends_with(APP_DIR));
    }

    #[test]
    fn config_file_is_toml() {
        let path = config_file().unwrap();
        assert_eq!(path.extension().unwrap(), "toml");
    }

    #[test]
    fn token_file_is_json() {
        let path = token_file().unwrap();
        assert_eq!(path.extension().unwrap(), "json");
    }

    #[cfg(not(target_os = "windows"))]
    #[test]
    fn unix_uses_xdg_config() {
        let dir = config_dir().unwrap();
        assert!(dir.to_string_lossy().contains(".config"));
    }
}
