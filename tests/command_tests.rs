//! Command layer tests with mocked HTTP
//!
//! These tests verify command behavior by mocking the Spotify API.
//! Uses wiremock to simulate API responses.

use serde_json::json;
use wiremock::matchers::{body_string_contains, method, path, query_param};
use wiremock::{Mock, MockServer, ResponseTemplate};

use spotify_cli::http::api::SpotifyApi;
use spotify_cli::endpoints::library::{get_saved_tracks, save_tracks, remove_tracks, check_saved_tracks};
use spotify_cli::endpoints::albums::{get_album, get_users_saved_albums, get_album_tracks, get_new_releases};
use spotify_cli::endpoints::artists::{get_artist, get_artist_top_tracks, get_artists_albums};
use spotify_cli::endpoints::tracks::get_track;
use spotify_cli::endpoints::player::{
    get_playback_state, get_available_devices, get_users_queue, skip_to_next,
    skip_to_previous, pause_playback, start_resume_playback, set_playback_volume,
    get_recently_played_tracks,
};
use spotify_cli::endpoints::playlists::{
    get_current_user_playlists, get_playlist, create_playlist, add_items_to_playlist,
};
use spotify_cli::endpoints::user::{get_current_user, get_users_top_items};
use spotify_cli::endpoints::search;

async fn setup_mock_server() -> (MockServer, SpotifyApi) {
    let mock_server = MockServer::start().await;
    let api = SpotifyApi::with_base_url("test_token".to_string(), mock_server.uri());
    (mock_server, api)
}

// ============================================================================
// Library command tests
// ============================================================================

mod library {
    use super::*;

    #[tokio::test]
    async fn get_saved_tracks_returns_tracks() {
        let (mock_server, api) = setup_mock_server().await;

        Mock::given(method("GET"))
            .and(path("/me/tracks"))
            .and(query_param("limit", "20"))
            .and(query_param("offset", "0"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "items": [
                    {
                        "added_at": "2024-01-01T00:00:00Z",
                        "track": {
                            "id": "track1",
                            "name": "Test Track",
                            "duration_ms": 180000,
                            "uri": "spotify:track:track1",
                            "type": "track",
                            "artists": [{"id": "a1", "name": "Artist", "type": "artist", "uri": "u"}]
                        }
                    }
                ],
                "total": 1,
                "limit": 20,
                "offset": 0
            })))
            .mount(&mock_server)
            .await;

        let result = get_saved_tracks::get_saved_tracks(&api, Some(20), Some(0)).await;
        assert!(result.is_ok());
        let payload = result.unwrap().unwrap();
        assert_eq!(payload["items"][0]["track"]["name"], "Test Track");
    }

    #[tokio::test]
    async fn get_saved_tracks_with_pagination() {
        let (mock_server, api) = setup_mock_server().await;

        Mock::given(method("GET"))
            .and(path("/me/tracks"))
            .and(query_param("limit", "50"))
            .and(query_param("offset", "100"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "items": [],
                "total": 100,
                "limit": 50,
                "offset": 100
            })))
            .mount(&mock_server)
            .await;

        let result = get_saved_tracks::get_saved_tracks(&api, Some(50), Some(100)).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn save_tracks_success() {
        let (mock_server, api) = setup_mock_server().await;

        Mock::given(method("PUT"))
            .and(path("/me/tracks"))
            .and(query_param("ids", "track1,track2"))
            .respond_with(ResponseTemplate::new(204))  // Spotify returns 204 No Content
            .mount(&mock_server)
            .await;

        let ids: Vec<String> = vec!["track1".to_string(), "track2".to_string()];
        let result = save_tracks::save_tracks(&api, &ids).await;
        assert!(result.is_ok(), "save_tracks failed: {:?}", result);
    }

    #[tokio::test]
    async fn remove_tracks_success() {
        let (mock_server, api) = setup_mock_server().await;

        Mock::given(method("DELETE"))
            .and(path("/me/tracks"))
            .and(query_param("ids", "track1"))
            .respond_with(ResponseTemplate::new(204))  // Spotify returns 204 No Content
            .mount(&mock_server)
            .await;

        let ids: Vec<String> = vec!["track1".to_string()];
        let result = remove_tracks::remove_tracks(&api, &ids).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn check_saved_tracks_returns_booleans() {
        let (mock_server, api) = setup_mock_server().await;

        Mock::given(method("GET"))
            .and(path("/me/tracks/contains"))
            .and(query_param("ids", "track1,track2"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!([true, false])))
            .mount(&mock_server)
            .await;

        let ids: Vec<String> = vec!["track1".to_string(), "track2".to_string()];
        let result = check_saved_tracks::check_saved_tracks(&api, &ids).await;
        assert!(result.is_ok());
        let payload = result.unwrap().unwrap();
        assert_eq!(payload, json!([true, false]));
    }
}

// ============================================================================
// Album command tests
// ============================================================================

mod albums {
    use super::*;

    #[tokio::test]
    async fn get_album_returns_album_details() {
        let (mock_server, api) = setup_mock_server().await;

        Mock::given(method("GET"))
            .and(path("/albums/album123"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "id": "album123",
                "name": "Test Album",
                "type": "album",
                "uri": "spotify:album:album123",
                "release_date": "2024-01-01",
                "total_tracks": 12,
                "artists": [{"id": "a1", "name": "Test Artist", "type": "artist", "uri": "u"}]
            })))
            .mount(&mock_server)
            .await;

        let result = get_album::get_album(&api, "album123").await;
        assert!(result.is_ok());
        let payload = result.unwrap().unwrap();
        assert_eq!(payload["name"], "Test Album");
        assert_eq!(payload["total_tracks"], 12);
    }

    #[tokio::test]
    async fn get_album_not_found() {
        let (mock_server, api) = setup_mock_server().await;

        Mock::given(method("GET"))
            .and(path("/albums/notfound"))
            .respond_with(ResponseTemplate::new(404))
            .mount(&mock_server)
            .await;

        let result = get_album::get_album(&api, "notfound").await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn get_saved_albums_returns_list() {
        let (mock_server, api) = setup_mock_server().await;

        Mock::given(method("GET"))
            .and(path("/me/albums"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "items": [
                    {
                        "added_at": "2024-01-01T00:00:00Z",
                        "album": {
                            "id": "album1",
                            "name": "Saved Album",
                            "type": "album",
                            "uri": "spotify:album:album1"
                        }
                    }
                ],
                "total": 1
            })))
            .mount(&mock_server)
            .await;

        let result = get_users_saved_albums::get_users_saved_albums(&api, Some(20), Some(0)).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn get_album_tracks_returns_tracks() {
        let (mock_server, api) = setup_mock_server().await;

        Mock::given(method("GET"))
            .and(path("/albums/album123/tracks"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "items": [
                    {"id": "t1", "name": "Track 1", "track_number": 1, "duration_ms": 180000},
                    {"id": "t2", "name": "Track 2", "track_number": 2, "duration_ms": 200000}
                ],
                "total": 2
            })))
            .mount(&mock_server)
            .await;

        let result = get_album_tracks::get_album_tracks(&api, "album123", Some(20), Some(0)).await;
        assert!(result.is_ok());
        let payload = result.unwrap().unwrap();
        assert_eq!(payload["items"].as_array().unwrap().len(), 2);
    }

    #[tokio::test]
    async fn get_new_releases_returns_albums() {
        let (mock_server, api) = setup_mock_server().await;

        Mock::given(method("GET"))
            .and(path("/browse/new-releases"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "albums": {
                    "items": [
                        {"id": "new1", "name": "New Album", "type": "album"}
                    ],
                    "total": 1
                }
            })))
            .mount(&mock_server)
            .await;

        let result = get_new_releases::get_new_releases(&api, Some(20), Some(0)).await;
        assert!(result.is_ok());
    }
}

// ============================================================================
// Artist command tests
// ============================================================================

mod artists {
    use super::*;

    #[tokio::test]
    async fn get_artist_returns_details() {
        let (mock_server, api) = setup_mock_server().await;

        Mock::given(method("GET"))
            .and(path("/artists/artist123"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "id": "artist123",
                "name": "Test Artist",
                "type": "artist",
                "uri": "spotify:artist:artist123",
                "genres": ["rock", "alternative"],
                "popularity": 85,
                "followers": {"total": 1000000}
            })))
            .mount(&mock_server)
            .await;

        let result = get_artist::get_artist(&api, "artist123").await;
        assert!(result.is_ok());
        let payload = result.unwrap().unwrap();
        assert_eq!(payload["name"], "Test Artist");
        assert_eq!(payload["popularity"], 85);
    }

    #[tokio::test]
    async fn get_artist_top_tracks_returns_tracks() {
        let (mock_server, api) = setup_mock_server().await;

        Mock::given(method("GET"))
            .and(path("/artists/artist123/top-tracks"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "tracks": [
                    {"id": "t1", "name": "Hit Song", "popularity": 90},
                    {"id": "t2", "name": "Another Hit", "popularity": 85}
                ]
            })))
            .mount(&mock_server)
            .await;

        let result = get_artist_top_tracks::get_artist_top_tracks(&api, "artist123", Some("US")).await;
        assert!(result.is_ok());
        let payload = result.unwrap().unwrap();
        assert_eq!(payload["tracks"].as_array().unwrap().len(), 2);
    }

    #[tokio::test]
    async fn get_artists_albums_returns_albums() {
        let (mock_server, api) = setup_mock_server().await;

        Mock::given(method("GET"))
            .and(path("/artists/artist123/albums"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "items": [
                    {"id": "a1", "name": "Album 1", "album_type": "album"},
                    {"id": "a2", "name": "Album 2", "album_type": "single"}
                ],
                "total": 2
            })))
            .mount(&mock_server)
            .await;

        let result = get_artists_albums::get_artists_albums(&api, "artist123", Some(20), Some(0)).await;
        assert!(result.is_ok());
    }
}

// ============================================================================
// Track command tests
// ============================================================================

mod tracks {
    use super::*;

    #[tokio::test]
    async fn get_track_returns_details() {
        let (mock_server, api) = setup_mock_server().await;

        Mock::given(method("GET"))
            .and(path("/tracks/track123"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "id": "track123",
                "name": "Test Track",
                "duration_ms": 210000,
                "uri": "spotify:track:track123",
                "type": "track",
                "popularity": 75,
                "artists": [{"id": "a1", "name": "Artist", "type": "artist", "uri": "u"}],
                "album": {"id": "alb1", "name": "Album", "type": "album", "uri": "u"}
            })))
            .mount(&mock_server)
            .await;

        let result = get_track::get_track(&api, "track123").await;
        assert!(result.is_ok());
        let payload = result.unwrap().unwrap();
        assert_eq!(payload["name"], "Test Track");
        assert_eq!(payload["duration_ms"], 210000);
    }

    #[tokio::test]
    async fn get_track_not_found() {
        let (mock_server, api) = setup_mock_server().await;

        Mock::given(method("GET"))
            .and(path("/tracks/notfound"))
            .respond_with(ResponseTemplate::new(404))
            .mount(&mock_server)
            .await;

        let result = get_track::get_track(&api, "notfound").await;
        assert!(result.is_err());
    }
}

// ============================================================================
// Player command tests
// ============================================================================

mod player {
    use super::*;

    #[tokio::test]
    async fn get_playback_state_returns_state() {
        let (mock_server, api) = setup_mock_server().await;

        Mock::given(method("GET"))
            .and(path("/me/player"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "is_playing": true,
                "progress_ms": 45000,
                "item": {
                    "id": "track1",
                    "name": "Playing Now",
                    "duration_ms": 180000,
                    "type": "track",
                    "uri": "spotify:track:track1",
                    "artists": [{"id": "a1", "name": "Artist", "type": "artist", "uri": "u"}]
                },
                "device": {
                    "id": "dev1",
                    "name": "My Speaker",
                    "type": "Speaker",
                    "is_active": true,
                    "volume_percent": 50
                }
            })))
            .mount(&mock_server)
            .await;

        let result = get_playback_state::get_playback_state(&api).await;
        assert!(result.is_ok());
        let payload = result.unwrap().unwrap();
        assert_eq!(payload["is_playing"], true);
        assert_eq!(payload["item"]["name"], "Playing Now");
    }

    #[tokio::test]
    async fn get_playback_state_no_active_device() {
        let (mock_server, api) = setup_mock_server().await;

        Mock::given(method("GET"))
            .and(path("/me/player"))
            .respond_with(ResponseTemplate::new(204))
            .mount(&mock_server)
            .await;

        let result = get_playback_state::get_playback_state(&api).await;
        assert!(result.is_ok());
        assert!(result.unwrap().is_none());
    }

    #[tokio::test]
    async fn get_devices_returns_list() {
        let (mock_server, api) = setup_mock_server().await;

        Mock::given(method("GET"))
            .and(path("/me/player/devices"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "devices": [
                    {"id": "d1", "name": "Phone", "type": "Smartphone", "is_active": false},
                    {"id": "d2", "name": "Computer", "type": "Computer", "is_active": true}
                ]
            })))
            .mount(&mock_server)
            .await;

        let result = get_available_devices::get_available_devices(&api).await;
        assert!(result.is_ok());
        let payload = result.unwrap().unwrap();
        assert_eq!(payload["devices"].as_array().unwrap().len(), 2);
    }

    #[tokio::test]
    async fn get_queue_returns_queue() {
        let (mock_server, api) = setup_mock_server().await;

        Mock::given(method("GET"))
            .and(path("/me/player/queue"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "currently_playing": {
                    "id": "t1", "name": "Current", "type": "track"
                },
                "queue": [
                    {"id": "t2", "name": "Next Up", "type": "track"},
                    {"id": "t3", "name": "After That", "type": "track"}
                ]
            })))
            .mount(&mock_server)
            .await;

        let result = get_users_queue::get_users_queue(&api).await;
        assert!(result.is_ok());
        let payload = result.unwrap().unwrap();
        assert_eq!(payload["queue"].as_array().unwrap().len(), 2);
    }

    #[tokio::test]
    async fn skip_next_success() {
        let (mock_server, api) = setup_mock_server().await;

        Mock::given(method("POST"))
            .and(path("/me/player/next"))
            .respond_with(ResponseTemplate::new(204))
            .mount(&mock_server)
            .await;

        let result = skip_to_next::skip_to_next(&api).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn skip_previous_success() {
        let (mock_server, api) = setup_mock_server().await;

        Mock::given(method("POST"))
            .and(path("/me/player/previous"))
            .respond_with(ResponseTemplate::new(204))
            .mount(&mock_server)
            .await;

        let result = skip_to_previous::skip_to_previous(&api).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn pause_playback_success() {
        let (mock_server, api) = setup_mock_server().await;

        Mock::given(method("PUT"))
            .and(path("/me/player/pause"))
            .respond_with(ResponseTemplate::new(204))
            .mount(&mock_server)
            .await;

        let result = pause_playback::pause_playback(&api).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn start_playback_success() {
        let (mock_server, api) = setup_mock_server().await;

        Mock::given(method("PUT"))
            .and(path("/me/player/play"))
            .respond_with(ResponseTemplate::new(204))
            .mount(&mock_server)
            .await;

        let result = start_resume_playback::start_resume_playback(&api, None, None).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn start_playback_with_context() {
        let (mock_server, api) = setup_mock_server().await;

        Mock::given(method("PUT"))
            .and(path("/me/player/play"))
            .respond_with(ResponseTemplate::new(204))
            .mount(&mock_server)
            .await;

        let result = start_resume_playback::start_resume_playback(
            &api,
            Some("spotify:playlist:123"),
            None
        ).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn start_playback_with_uris() {
        let (mock_server, api) = setup_mock_server().await;

        Mock::given(method("PUT"))
            .and(path("/me/player/play"))
            .respond_with(ResponseTemplate::new(204))
            .mount(&mock_server)
            .await;

        let uris = vec!["spotify:track:123".to_string()];
        let result = start_resume_playback::start_resume_playback(
            &api,
            None,
            Some(&uris)
        ).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn set_volume_success() {
        let (mock_server, api) = setup_mock_server().await;

        Mock::given(method("PUT"))
            .and(path("/me/player/volume"))
            .and(query_param("volume_percent", "75"))
            .respond_with(ResponseTemplate::new(204))
            .mount(&mock_server)
            .await;

        let result = set_playback_volume::set_playback_volume(&api, 75).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn get_recently_played_returns_history() {
        let (mock_server, api) = setup_mock_server().await;

        Mock::given(method("GET"))
            .and(path("/me/player/recently-played"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "items": [
                    {
                        "played_at": "2024-01-01T12:00:00Z",
                        "track": {"id": "t1", "name": "Recent Track", "type": "track"}
                    }
                ]
            })))
            .mount(&mock_server)
            .await;

        let result = get_recently_played_tracks::get_recently_played_tracks(&api).await;
        assert!(result.is_ok());
    }
}

// ============================================================================
// Playlist command tests
// ============================================================================

mod playlists {
    use super::*;

    #[tokio::test]
    async fn get_user_playlists_returns_list() {
        let (mock_server, api) = setup_mock_server().await;

        Mock::given(method("GET"))
            .and(path("/me/playlists"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "items": [
                    {"id": "p1", "name": "My Playlist", "owner": {"id": "user1"}},
                    {"id": "p2", "name": "Another Playlist", "owner": {"id": "user1"}}
                ],
                "total": 2
            })))
            .mount(&mock_server)
            .await;

        let result = get_current_user_playlists::get_current_user_playlists(&api, Some(20), Some(0)).await;
        assert!(result.is_ok());
        let payload = result.unwrap().unwrap();
        assert_eq!(payload["items"].as_array().unwrap().len(), 2);
    }

    #[tokio::test]
    async fn get_playlist_returns_details() {
        let (mock_server, api) = setup_mock_server().await;

        Mock::given(method("GET"))
            .and(path("/playlists/playlist123"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "id": "playlist123",
                "name": "Test Playlist",
                "description": "A test playlist",
                "owner": {"id": "user1", "display_name": "Test User"},
                "tracks": {"total": 50}
            })))
            .mount(&mock_server)
            .await;

        let result = get_playlist::get_playlist(&api, "playlist123").await;
        assert!(result.is_ok());
        let payload = result.unwrap().unwrap();
        assert_eq!(payload["name"], "Test Playlist");
    }

    #[tokio::test]
    async fn create_playlist_returns_new_playlist() {
        let (mock_server, api) = setup_mock_server().await;

        Mock::given(method("POST"))
            .and(path("/users/user123/playlists"))
            .respond_with(ResponseTemplate::new(201).set_body_json(json!({
                "id": "newplaylist",
                "name": "New Playlist",
                "uri": "spotify:playlist:newplaylist"
            })))
            .mount(&mock_server)
            .await;

        let result = create_playlist::create_playlist(
            &api,
            "user123",
            "New Playlist",
            None,
            false
        ).await;
        assert!(result.is_ok());
        let payload = result.unwrap().unwrap();
        assert_eq!(payload["id"], "newplaylist");
    }

    #[tokio::test]
    async fn create_public_playlist() {
        let (mock_server, api) = setup_mock_server().await;

        Mock::given(method("POST"))
            .and(path("/users/user123/playlists"))
            .respond_with(ResponseTemplate::new(201).set_body_json(json!({
                "id": "publicplaylist",
                "name": "Public Playlist",
                "public": true
            })))
            .mount(&mock_server)
            .await;

        let result = create_playlist::create_playlist(
            &api,
            "user123",
            "Public Playlist",
            Some("A public playlist"),
            true
        ).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn add_items_to_playlist_returns_snapshot() {
        let (mock_server, api) = setup_mock_server().await;

        Mock::given(method("POST"))
            .and(path("/playlists/playlist123/tracks"))
            .respond_with(ResponseTemplate::new(201).set_body_json(json!({
                "snapshot_id": "abc123"
            })))
            .mount(&mock_server)
            .await;

        let uris: Vec<String> = vec!["spotify:track:t1".to_string(), "spotify:track:t2".to_string()];
        let result = add_items_to_playlist::add_items_to_playlist(
            &api,
            "playlist123",
            &uris,
            None
        ).await;
        assert!(result.is_ok());
    }
}

// ============================================================================
// User command tests
// ============================================================================

mod user {
    use super::*;

    #[tokio::test]
    async fn get_current_user_returns_profile() {
        let (mock_server, api) = setup_mock_server().await;

        Mock::given(method("GET"))
            .and(path("/me"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "id": "user123",
                "display_name": "Test User",
                "email": "test@example.com",
                "country": "US",
                "product": "premium",
                "followers": {"total": 100}
            })))
            .mount(&mock_server)
            .await;

        let result = get_current_user::get_current_user(&api).await;
        assert!(result.is_ok());
        let payload = result.unwrap().unwrap();
        assert_eq!(payload["display_name"], "Test User");
        assert_eq!(payload["product"], "premium");
    }

    #[tokio::test]
    async fn get_top_tracks_returns_list() {
        let (mock_server, api) = setup_mock_server().await;

        Mock::given(method("GET"))
            .and(path("/me/top/tracks"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "items": [
                    {"id": "t1", "name": "Top Track 1", "type": "track"},
                    {"id": "t2", "name": "Top Track 2", "type": "track"}
                ],
                "total": 2
            })))
            .mount(&mock_server)
            .await;

        let result = get_users_top_items::get_users_top_items(&api, "tracks", None, Some(20), Some(0)).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn get_top_artists_returns_list() {
        let (mock_server, api) = setup_mock_server().await;

        Mock::given(method("GET"))
            .and(path("/me/top/artists"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "items": [
                    {"id": "a1", "name": "Top Artist", "type": "artist"}
                ],
                "total": 1
            })))
            .mount(&mock_server)
            .await;

        let result = get_users_top_items::get_users_top_items(&api, "artists", Some("medium_term"), Some(20), Some(0)).await;
        assert!(result.is_ok());
    }
}

// ============================================================================
// Search command tests
// ============================================================================

mod search_tests {
    use super::*;

    #[tokio::test]
    async fn search_tracks_returns_results() {
        let (mock_server, api) = setup_mock_server().await;

        Mock::given(method("GET"))
            .and(path("/search"))
            .and(query_param("type", "track"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "tracks": {
                    "items": [
                        {"id": "t1", "name": "Found Track", "type": "track"}
                    ],
                    "total": 1
                }
            })))
            .mount(&mock_server)
            .await;

        let types: &[&str] = &["track"];
        let result = search::search(&api, "test query", Some(types), Some(20), None).await;
        assert!(result.is_ok());
        let payload = result.unwrap().unwrap();
        assert!(!payload["tracks"]["items"].as_array().unwrap().is_empty());
    }

    #[tokio::test]
    async fn search_multiple_types_returns_results() {
        let (mock_server, api) = setup_mock_server().await;

        Mock::given(method("GET"))
            .and(path("/search"))
            .and(query_param("type", "track,album,artist"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "tracks": {"items": [{"id": "t1", "name": "Track", "type": "track"}], "total": 1},
                "albums": {"items": [{"id": "a1", "name": "Album", "type": "album"}], "total": 1},
                "artists": {"items": [{"id": "ar1", "name": "Artist", "type": "artist"}], "total": 1}
            })))
            .mount(&mock_server)
            .await;

        let types: &[&str] = &["track", "album", "artist"];
        let result = search::search(&api, "query", Some(types), Some(20), None).await;
        assert!(result.is_ok());
        let payload = result.unwrap().unwrap();
        assert!(payload["tracks"].is_object());
        assert!(payload["albums"].is_object());
        assert!(payload["artists"].is_object());
    }

    #[tokio::test]
    async fn search_with_no_results() {
        let (mock_server, api) = setup_mock_server().await;

        Mock::given(method("GET"))
            .and(path("/search"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "tracks": {"items": [], "total": 0}
            })))
            .mount(&mock_server)
            .await;

        let types: &[&str] = &["track"];
        let result = search::search(&api, "xyznonexistent", Some(types), Some(20), None).await;
        assert!(result.is_ok());
        let payload = result.unwrap().unwrap();
        assert_eq!(payload["tracks"]["items"].as_array().unwrap().len(), 0);
    }
}

// ============================================================================
// Error handling tests
// ============================================================================

mod error_handling {
    use super::*;
    use spotify_cli::http::client::HttpError;

    #[tokio::test]
    async fn unauthorized_error() {
        let (mock_server, api) = setup_mock_server().await;

        Mock::given(method("GET"))
            .and(path("/me"))
            .respond_with(ResponseTemplate::new(401).set_body_json(json!({
                "error": {"status": 401, "message": "Invalid access token"}
            })))
            .mount(&mock_server)
            .await;

        let result = get_current_user::get_current_user(&api).await;
        assert!(matches!(result, Err(HttpError::Unauthorized)));
    }

    #[tokio::test]
    async fn forbidden_error() {
        let (mock_server, api) = setup_mock_server().await;

        Mock::given(method("GET"))
            .and(path("/me/player"))
            .respond_with(ResponseTemplate::new(403).set_body_json(json!({
                "error": {"status": 403, "message": "Forbidden"}
            })))
            .mount(&mock_server)
            .await;

        let result = get_playback_state::get_playback_state(&api).await;
        assert!(matches!(result, Err(HttpError::Forbidden)));
    }

    #[tokio::test]
    async fn rate_limited_error() {
        let (mock_server, api) = setup_mock_server().await;

        Mock::given(method("GET"))
            .and(path("/me"))
            .respond_with(
                ResponseTemplate::new(429)
                    .insert_header("Retry-After", "5")
                    .set_body_json(json!({
                        "error": {"status": 429, "message": "Rate limited"}
                    }))
            )
            .expect(4) // Will retry 3 times
            .mount(&mock_server)
            .await;

        let result = get_current_user::get_current_user(&api).await;
        assert!(matches!(result, Err(HttpError::RateLimited { .. })));
    }

    #[tokio::test]
    async fn api_error_with_message() {
        let (mock_server, api) = setup_mock_server().await;

        Mock::given(method("PUT"))
            .and(path("/me/player/volume"))
            .respond_with(ResponseTemplate::new(400).set_body_json(json!({
                "error": {"status": 400, "message": "Invalid volume percentage"}
            })))
            .mount(&mock_server)
            .await;

        let result = set_playback_volume::set_playback_volume(&api, 150).await;
        match result {
            Err(HttpError::Api { status, message }) => {
                assert_eq!(status, 400);
                assert_eq!(message, "Invalid volume percentage");
            }
            _ => panic!("Expected Api error"),
        }
    }

    #[tokio::test]
    async fn not_found_error() {
        let (mock_server, api) = setup_mock_server().await;

        Mock::given(method("GET"))
            .and(path("/tracks/nonexistent"))
            .respond_with(ResponseTemplate::new(404).set_body_json(json!({
                "error": {"status": 404, "message": "Track not found"}
            })))
            .mount(&mock_server)
            .await;

        let result = get_track::get_track(&api, "nonexistent").await;
        assert!(matches!(result, Err(HttpError::NotFound)));
    }
}

// ============================================================================
// Auth command tests
// ============================================================================

mod auth {
    use super::*;
    use spotify_cli::http::auth::SpotifyAuth;

    #[tokio::test]
    async fn refresh_token_success() {
        let mock_server = MockServer::start().await;

        Mock::given(method("POST"))
            .and(path("/api/token"))
            .and(body_string_contains("grant_type=refresh_token"))
            .and(body_string_contains("refresh_token=test_refresh_token"))
            .and(body_string_contains("client_id=test_client"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "access_token": "new_access_token",
                "token_type": "Bearer",
                "scope": "user-read-playback-state",
                "expires_in": 3600,
                "refresh_token": "new_refresh_token"
            })))
            .mount(&mock_server)
            .await;

        let auth = SpotifyAuth::with_base_url(mock_server.uri());
        let result = auth.refresh_token("test_client", "test_refresh_token").await;

        assert!(result.is_ok());
        let token = result.unwrap();
        assert_eq!(token["access_token"], "new_access_token");
        assert_eq!(token["expires_in"], 3600);
    }

    #[tokio::test]
    async fn refresh_token_invalid_token() {
        let mock_server = MockServer::start().await;

        Mock::given(method("POST"))
            .and(path("/api/token"))
            .respond_with(ResponseTemplate::new(400).set_body_json(json!({
                "error": "invalid_grant",
                "error_description": "Refresh token revoked"
            })))
            .mount(&mock_server)
            .await;

        let auth = SpotifyAuth::with_base_url(mock_server.uri());
        let result = auth.refresh_token("test_client", "invalid_token").await;

        assert!(result.is_err());
    }

    #[tokio::test]
    async fn exchange_code_success() {
        let mock_server = MockServer::start().await;

        Mock::given(method("POST"))
            .and(path("/api/token"))
            .and(body_string_contains("grant_type=authorization_code"))
            .and(body_string_contains("code=auth_code"))
            .and(body_string_contains("client_id=test_client"))
            .and(body_string_contains("code_verifier=test_verifier"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "access_token": "access_token_123",
                "token_type": "Bearer",
                "scope": "user-read-playback-state user-library-read",
                "expires_in": 3600,
                "refresh_token": "refresh_token_123"
            })))
            .mount(&mock_server)
            .await;

        let auth = SpotifyAuth::with_base_url(mock_server.uri());
        let result = auth.exchange_code(
            "test_client",
            "auth_code",
            "http://localhost:8888/callback",
            "test_verifier"
        ).await;

        assert!(result.is_ok());
        let token = result.unwrap();
        assert_eq!(token["access_token"], "access_token_123");
        assert_eq!(token["refresh_token"], "refresh_token_123");
    }

    #[tokio::test]
    async fn exchange_code_invalid_code() {
        let mock_server = MockServer::start().await;

        Mock::given(method("POST"))
            .and(path("/api/token"))
            .respond_with(ResponseTemplate::new(400).set_body_json(json!({
                "error": "invalid_grant",
                "error_description": "Authorization code expired"
            })))
            .mount(&mock_server)
            .await;

        let auth = SpotifyAuth::with_base_url(mock_server.uri());
        let result = auth.exchange_code(
            "test_client",
            "expired_code",
            "http://localhost:8888/callback",
            "test_verifier"
        ).await;

        assert!(result.is_err());
    }

    #[tokio::test]
    async fn exchange_code_invalid_verifier() {
        let mock_server = MockServer::start().await;

        Mock::given(method("POST"))
            .and(path("/api/token"))
            .respond_with(ResponseTemplate::new(400).set_body_json(json!({
                "error": "invalid_grant",
                "error_description": "code_verifier was incorrect"
            })))
            .mount(&mock_server)
            .await;

        let auth = SpotifyAuth::with_base_url(mock_server.uri());
        let result = auth.exchange_code(
            "test_client",
            "auth_code",
            "http://localhost:8888/callback",
            "wrong_verifier"
        ).await;

        assert!(result.is_err());
    }

    #[tokio::test]
    async fn refresh_token_server_error() {
        let mock_server = MockServer::start().await;

        Mock::given(method("POST"))
            .and(path("/api/token"))
            .respond_with(ResponseTemplate::new(500).set_body_string("Internal Server Error"))
            .mount(&mock_server)
            .await;

        let auth = SpotifyAuth::with_base_url(mock_server.uri());
        let result = auth.refresh_token("test_client", "refresh_token").await;

        assert!(result.is_err());
    }

    #[tokio::test]
    async fn refresh_token_returns_new_refresh_token() {
        let mock_server = MockServer::start().await;

        Mock::given(method("POST"))
            .and(path("/api/token"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "access_token": "new_access",
                "token_type": "Bearer",
                "scope": "user-read-playback-state",
                "expires_in": 3600,
                "refresh_token": "rotated_refresh_token"
            })))
            .mount(&mock_server)
            .await;

        let auth = SpotifyAuth::with_base_url(mock_server.uri());
        let result = auth.refresh_token("test_client", "old_refresh_token").await;

        assert!(result.is_ok());
        let token = result.unwrap();
        assert_eq!(token["refresh_token"], "rotated_refresh_token");
    }

    #[tokio::test]
    async fn refresh_token_without_new_refresh() {
        let mock_server = MockServer::start().await;

        // Sometimes Spotify doesn't return a new refresh token
        Mock::given(method("POST"))
            .and(path("/api/token"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "access_token": "new_access",
                "token_type": "Bearer",
                "scope": "user-read-playback-state",
                "expires_in": 3600
            })))
            .mount(&mock_server)
            .await;

        let auth = SpotifyAuth::with_base_url(mock_server.uri());
        let result = auth.refresh_token("test_client", "refresh_token").await;

        assert!(result.is_ok());
        let token = result.unwrap();
        assert!(token.get("refresh_token").is_none());
    }
}
