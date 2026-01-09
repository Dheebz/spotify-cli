#![cfg(feature = "http-tests")]

use httpmock::Method::{DELETE, GET, POST, PUT};
use httpmock::MockServer;
use spotify_cli::cache::metadata::MetadataStore;
use spotify_cli::spotify::auth::{AuthService, AuthToken};
use spotify_cli::spotify::client::SpotifyClient;
use std::fs;
use std::path::PathBuf;

fn temp_dir(name: &str) -> PathBuf {
    let mut path = std::env::temp_dir();
    let stamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    path.push(format!("spotify-cli-http-{name}-{stamp}"));
    fs::create_dir_all(&path).unwrap();
    path
}

fn client_with_token(server: &MockServer) -> SpotifyClient {
    let dir = temp_dir("auth");
    let store = MetadataStore::new(dir.join("metadata.json"));
    let auth = AuthService::new(store);
    auth.login(AuthToken {
        access_token: "token".to_string(),
        refresh_token: None,
        expires_at: None,
        scopes: None,
    })
    .unwrap();
    unsafe {
        std::env::set_var("SPOTIFY_CLI_API_BASE", server.base_url());
    }
    SpotifyClient::new(auth).unwrap()
}

fn teardown_env() {
    unsafe {
        std::env::remove_var("SPOTIFY_CLI_API_BASE");
    }
}

#[test]
fn search_tracks_parses_items() {
    let server = MockServer::start();
    let mock = server.mock(|when, then| {
        when.method(GET)
            .path("/search")
            .query_param("q", "boards")
            .query_param("type", "track")
            .query_param("limit", "1");
        then.status(200).json_body(serde_json::json!({
            "tracks": { "items": [ { "id": "1", "name": "Track", "uri": "uri", "artists": [{ "name": "Artist" }] } ] }
        }));
    });

    let client = client_with_token(&server);
    let results = client
        .search()
        .search(
            "boards",
            spotify_cli::domain::search::SearchType::Track,
            1,
            false,
        )
        .unwrap();
    mock.assert();
    assert_eq!(results.items.len(), 1);
    assert_eq!(results.items[0].name, "Track");
    teardown_env();
}

#[test]
fn playlists_list_parses_items() {
    let server = MockServer::start();
    let mock = server.mock(|when, then| {
        when.method(GET).path("/me/playlists").query_param("limit", "50");
        then.status(200).json_body(serde_json::json!({
            "items": [ { "id": "1", "name": "MyRadar", "owner": { "display_name": "Me" }, "collaborative": false, "public": false } ],
            "next": null
        }));
    });

    let client = client_with_token(&server);
    let playlists = client.playlists().list_all().unwrap();
    mock.assert();
    assert_eq!(playlists.len(), 1);
    assert_eq!(playlists[0].name, "MyRadar");
    teardown_env();
}

#[test]
fn playlist_get_parses_detail() {
    let server = MockServer::start();
    let mock = server.mock(|when, then| {
        when.method(GET).path("/playlists/abc");
        then.status(200).json_body(serde_json::json!({
            "id": "abc",
            "name": "MyRadar",
            "uri": "uri",
            "owner": { "display_name": "Me" },
            "tracks": { "total": 10 },
            "collaborative": false,
            "public": false
        }));
    });

    let client = client_with_token(&server);
    let detail = client.playlists().get("abc").unwrap();
    mock.assert();
    assert_eq!(detail.tracks_total, Some(10));
    teardown_env();
}

#[test]
fn playlist_follow_puts() {
    let server = MockServer::start();
    let mock = server.mock(|when, then| {
        when.method(PUT).path("/playlists/abc/followers");
        then.status(200);
    });
    let client = client_with_token(&server);
    client.playlists().follow("abc").unwrap();
    mock.assert();
    teardown_env();
}

#[test]
fn playlist_unfollow_deletes() {
    let server = MockServer::start();
    let mock = server.mock(|when, then| {
        when.method(DELETE).path("/playlists/abc/followers");
        then.status(200);
    });
    let client = client_with_token(&server);
    client.playlists().unfollow("abc").unwrap();
    mock.assert();
    teardown_env();
}

#[test]
fn playlist_add_posts() {
    let server = MockServer::start();
    let mock = server.mock(|when, then| {
        when.method(POST)
            .path("/playlists/abc/tracks")
            .json_body(serde_json::json!({ "uris": ["spotify:track:1"] }));
        then.status(201)
            .json_body(serde_json::json!({ "snapshot_id": "snap" }));
    });
    let client = client_with_token(&server);
    client
        .playlists()
        .add_tracks("abc", &[String::from("spotify:track:1")])
        .unwrap();
    mock.assert();
    teardown_env();
}

#[test]
fn devices_list_parses_items() {
    let server = MockServer::start();
    let mock = server.mock(|when, then| {
        when.method(GET).path("/me/player/devices");
        then.status(200).json_body(serde_json::json!({
            "devices": [ { "id": "1", "name": "Office", "volume_percent": 50 } ]
        }));
    });

    let client = client_with_token(&server);
    let devices = client.devices().list().unwrap();
    mock.assert();
    assert_eq!(devices.len(), 1);
    teardown_env();
}

#[test]
fn devices_set_active_puts() {
    let server = MockServer::start();
    let mock = server.mock(|when, then| {
        when.method(PUT)
            .path("/me/player")
            .json_body(serde_json::json!({ "device_ids": ["1"], "play": true }));
        then.status(204);
    });
    let client = client_with_token(&server);
    client.devices().set_active("1").unwrap();
    mock.assert();
    teardown_env();
}

#[test]
fn playback_status_parses_track() {
    let server = MockServer::start();
    let mock = server.mock(|when, then| {
        when.method(GET).path("/me/player");
        then.status(200).json_body(serde_json::json!({
            "is_playing": true,
            "progress_ms": 1000,
            "item": {
                "id": "t1",
                "name": "Song",
                "duration_ms": 2000,
                "album": { "name": "Album" },
                "artists": [ { "name": "Artist" } ]
            },
            "device": { "id": "d1", "name": "Speaker", "volume_percent": 80 }
        }));
    });

    let client = client_with_token(&server);
    let status = client.playback().status().unwrap();
    mock.assert();
    assert!(status.is_playing);
    assert!(status.track.is_some());
    teardown_env();
}

#[test]
fn playback_control_puts() {
    let server = MockServer::start();
    let mock = server.mock(|when, then| {
        when.method(PUT).path("/me/player/pause");
        then.status(204);
    });
    let client = client_with_token(&server);
    client.playback().pause().unwrap();
    mock.assert();
    teardown_env();
}

#[test]
fn albums_get_parses_tracks() {
    let server = MockServer::start();
    let album_mock = server.mock(|when, then| {
        when.method(GET).path("/albums/abc");
        then.status(200).json_body(serde_json::json!({
            "id": "abc",
            "name": "Album",
            "uri": "uri",
            "release_date": "2024-01-01",
            "total_tracks": 1,
            "artists": [ { "name": "Artist" } ]
        }));
    });
    let tracks_mock = server.mock(|when, then| {
        when.method(GET)
            .path("/albums/abc/tracks")
            .query_param("limit", "50");
        then.status(200).json_body(serde_json::json!({
            "items": [ { "name": "Track", "duration_ms": 1000, "track_number": 1 } ],
            "next": null
        }));
    });

    let client = client_with_token(&server);
    let album = client.albums().get("abc").unwrap();
    album_mock.assert();
    tracks_mock.assert();
    assert_eq!(album.tracks.len(), 1);
    teardown_env();
}

#[test]
fn artists_get_parses_artist() {
    let server = MockServer::start();
    let mock = server.mock(|when, then| {
        when.method(GET).path("/artists/abc");
        then.status(200).json_body(serde_json::json!({
            "id": "abc",
            "name": "Artist",
            "uri": "uri",
            "genres": ["alt"],
            "followers": { "total": 5 }
        }));
    });
    let client = client_with_token(&server);
    let artist = client.artists().get("abc").unwrap();
    mock.assert();
    assert_eq!(artist.followers, Some(5));
    teardown_env();
}

#[test]
fn track_like_puts() {
    let server = MockServer::start();
    let mock = server.mock(|when, then| {
        when.method(PUT)
            .path("/me/tracks")
            .query_param("ids", "abc");
        then.status(200);
    });
    let client = client_with_token(&server);
    client.track().like("abc").unwrap();
    mock.assert();
    teardown_env();
}

#[test]
fn track_unlike_deletes() {
    let server = MockServer::start();
    let mock = server.mock(|when, then| {
        when.method(DELETE)
            .path("/me/tracks")
            .query_param("ids", "abc");
        then.status(200);
    });
    let client = client_with_token(&server);
    client.track().unlike("abc").unwrap();
    mock.assert();
    teardown_env();
}
