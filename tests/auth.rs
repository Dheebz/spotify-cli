use std::fs;
use std::time::{SystemTime, UNIX_EPOCH};

use spotify_cli::cache::metadata::MetadataStore;
use spotify_cli::cache::metadata::{AuthTokenCache, Metadata};
use spotify_cli::spotify::auth::{AuthService, AuthToken};

fn temp_path(name: &str) -> std::path::PathBuf {
    let stamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("time")
        .as_nanos();
    std::env::temp_dir().join(format!("spotify-cli-{name}-{stamp}"))
}

#[test]
fn auth_login_status() {
    let dir = temp_path("auth");
    fs::create_dir_all(&dir).expect("create dir");
    let path = dir.join("metadata.json");
    let store = MetadataStore::new(path);
    let auth = AuthService::new(store);

    unsafe {
        std::env::set_var("SPOTIFY_CLI_SKIP_PROFILE", "1");
    }

    let status = auth.status().expect("status");
    assert!(!status.logged_in);

    auth.login(AuthToken {
        access_token: "token".to_string(),
        refresh_token: Some("refresh".to_string()),
        expires_at: Some(10),
        scopes: Some(vec!["user-read-private".to_string()]),
    })
    .expect("login");

    let status = auth.status().expect("status");
    assert!(status.logged_in);
    assert_eq!(status.expires_at, Some(10));
}

#[test]
fn auth_scopes_reports_missing() {
    let dir = temp_path("auth-scopes");
    fs::create_dir_all(&dir).expect("create dir");
    let path = dir.join("metadata.json");
    let store = MetadataStore::new(path);

    let metadata = Metadata {
        auth: Some(AuthTokenCache {
            access_token: "token".to_string(),
            refresh_token: None,
            expires_at: None,
            granted_scopes: Some(vec!["user-read-private".to_string()]),
        }),
        client: None,
        settings: Default::default(),
    };
    store.save(&metadata).expect("save metadata");

    let auth = AuthService::new(store);
    let scopes = auth.scopes().expect("scopes");
    assert!(scopes.required.contains(&"user-read-private".to_string()));
    assert!(scopes.missing.len() + scopes.granted.unwrap_or_default().len() >= 1);
}
