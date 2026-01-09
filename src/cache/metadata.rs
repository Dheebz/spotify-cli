use std::fs;
use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use crate::domain::settings::Settings;
use crate::error::Result;

/// Persistent metadata for auth credentials and settings.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Metadata {
    pub auth: Option<AuthTokenCache>,
    pub client: Option<ClientIdentity>,
    #[serde(default)]
    pub settings: Settings,
}

/// Cached OAuth token fields stored locally.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthTokenCache {
    pub access_token: String,
    pub refresh_token: Option<String>,
    pub expires_at: Option<u64>,
    #[serde(default)]
    pub granted_scopes: Option<Vec<String>>,
}

/// Stored client identity (client id) for refresh flows.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClientIdentity {
    pub client_id: String,
}

/// JSON-backed metadata store.
#[derive(Debug, Clone)]
pub struct MetadataStore {
    path: PathBuf,
}

impl MetadataStore {
    pub fn new(path: PathBuf) -> Self {
        Self { path }
    }

    pub fn load(&self) -> Result<Metadata> {
        if !self.path.exists() {
            return Ok(Metadata::default());
        }

        let contents = fs::read_to_string(&self.path)?;
        let metadata = serde_json::from_str(&contents)?;
        Ok(metadata)
    }

    pub fn save(&self, metadata: &Metadata) -> Result<()> {
        let payload = serde_json::to_string_pretty(metadata)?;
        fs::write(&self.path, payload)?;
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let _ = fs::set_permissions(&self.path, fs::Permissions::from_mode(0o600));
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::{AuthTokenCache, Metadata, MetadataStore};
    use crate::domain::settings::Settings;
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
    fn metadata_store_round_trip() {
        let path = temp_path("metadata-store");
        let store = MetadataStore::new(path.clone());
        let metadata = Metadata {
            auth: Some(AuthTokenCache {
                access_token: "token".to_string(),
                refresh_token: None,
                expires_at: Some(1),
                granted_scopes: Some(vec!["user-read-private".to_string()]),
            }),
            client: None,
            settings: Settings {
                country: Some("AU".to_string()),
                user_name: Some("Me".to_string()),
            },
        };
        store.save(&metadata).expect("save");
        let loaded = store.load().expect("load");
        assert_eq!(loaded.settings.country.as_deref(), Some("AU"));
        assert_eq!(loaded.settings.user_name.as_deref(), Some("Me"));
        let _ = fs::remove_file(path);
    }
}
