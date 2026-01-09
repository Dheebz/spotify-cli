use std::fs;
use std::time::{SystemTime, UNIX_EPOCH};

use spotify_cli::cache::metadata::{AuthTokenCache, Metadata, MetadataStore};
use spotify_cli::domain::settings::Settings;

fn temp_path(name: &str) -> std::path::PathBuf {
    let stamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("time")
        .as_nanos();
    std::env::temp_dir().join(format!("spotify-cli-{name}-{stamp}"))
}

#[test]
fn metadata_round_trip() {
    let dir = temp_path("metadata");
    fs::create_dir_all(&dir).expect("create dir");
    let path = dir.join("metadata.json");
    let store = MetadataStore::new(path);

    let metadata = Metadata {
        auth: Some(AuthTokenCache {
            access_token: "token".to_string(),
            refresh_token: Some("refresh".to_string()),
            expires_at: Some(1234),
            granted_scopes: None,
        }),
        client: None,
        settings: Settings::default(),
    };

    store.save(&metadata).expect("save metadata");
    let loaded = store.load().expect("load metadata");

    assert!(loaded.auth.is_some());
    let auth = loaded.auth.expect("auth");
    assert_eq!(auth.access_token, "token");
    assert_eq!(auth.refresh_token.as_deref(), Some("refresh"));
    assert_eq!(auth.expires_at, Some(1234));
}
