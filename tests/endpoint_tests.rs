//! Integration tests for Spotify API endpoints using mock server
//!
//! These tests verify that endpoint functions correctly:
//! - Build request URLs
//! - Send appropriate HTTP methods
//! - Parse responses
//! - Handle errors

use serde_json::json;
use spotify_cli::http::api::SpotifyApi;
use spotify_cli::http::client::HttpError;
use wiremock::matchers::{header, method, path, path_regex, query_param};
use wiremock::{Mock, MockServer, ResponseTemplate};

/// Create a mock server and API client for testing
async fn setup() -> (MockServer, SpotifyApi) {
    let server = MockServer::start().await;
    let api = SpotifyApi::with_base_url("test_token".to_string(), server.uri());
    (server, api)
}

// =============================================================================
// User Endpoint Tests
// =============================================================================

mod user_endpoints {
    use super::*;
    use spotify_cli::endpoints::user::{
        get_current_user, get_users_profile, get_users_top_items, get_followed_artists,
        follow_artists_or_users, unfollow_artists_or_users, check_if_user_follows_artist_or_users,
    };

    #[tokio::test]
    async fn test_get_current_user() {
        let (server, api) = setup().await;

        Mock::given(method("GET"))
            .and(path("/me"))
            .and(header("Authorization", "Bearer test_token"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "id": "user123",
                "display_name": "Test User",
                "email": "test@example.com",
                "country": "US",
                "product": "premium"
            })))
            .mount(&server)
            .await;

        let result = get_current_user::get_current_user(&api).await;
        assert!(result.is_ok());
        let data = result.unwrap().unwrap();
        assert_eq!(data["id"], "user123");
        assert_eq!(data["display_name"], "Test User");
    }

    #[tokio::test]
    async fn test_get_users_profile() {
        let (server, api) = setup().await;

        Mock::given(method("GET"))
            .and(path("/users/spotify"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "id": "spotify",
                "display_name": "Spotify",
                "type": "user"
            })))
            .mount(&server)
            .await;

        let result = get_users_profile::get_users_profile(&api, "spotify").await;
        assert!(result.is_ok());
        let data = result.unwrap().unwrap();
        assert_eq!(data["id"], "spotify");
    }

    #[tokio::test]
    async fn test_get_users_top_tracks() {
        let (server, api) = setup().await;

        Mock::given(method("GET"))
            .and(path("/me/top/tracks"))
            .and(query_param("time_range", "medium_term"))
            .and(query_param("limit", "20"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "items": [
                    {"id": "track1", "name": "Song 1"},
                    {"id": "track2", "name": "Song 2"}
                ],
                "total": 2
            })))
            .mount(&server)
            .await;

        let result = get_users_top_items::get_users_top_items(&api, "tracks", Some("medium_term"), Some(20), Some(0)).await;
        assert!(result.is_ok());
        let data = result.unwrap().unwrap();
        assert_eq!(data["items"].as_array().unwrap().len(), 2);
    }

    #[tokio::test]
    async fn test_get_followed_artists() {
        let (server, api) = setup().await;

        Mock::given(method("GET"))
            .and(path("/me/following"))
            .and(query_param("type", "artist"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "artists": {
                    "items": [
                        {"id": "artist1", "name": "Artist One"}
                    ]
                }
            })))
            .mount(&server)
            .await;

        let result = get_followed_artists::get_followed_artists(&api, Some(20)).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_follow_artists() {
        let (server, api) = setup().await;

        Mock::given(method("PUT"))
            .and(path("/me/following"))
            .and(query_param("type", "artist"))
            .respond_with(ResponseTemplate::new(204))
            .mount(&server)
            .await;

        let ids = vec!["id1".to_string(), "id2".to_string()];
        let result = follow_artists_or_users::follow_artists_or_users(&api, "artist", &ids).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_unfollow_artists() {
        let (server, api) = setup().await;

        Mock::given(method("DELETE"))
            .and(path("/me/following"))
            .and(query_param("type", "artist"))
            .respond_with(ResponseTemplate::new(204))
            .mount(&server)
            .await;

        let ids = vec!["id1".to_string()];
        let result = unfollow_artists_or_users::unfollow_artists_or_users(&api, "artist", &ids).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_check_following() {
        let (server, api) = setup().await;

        Mock::given(method("GET"))
            .and(path("/me/following/contains"))
            .and(query_param("type", "artist"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!([true, false])))
            .mount(&server)
            .await;

        let ids = vec!["id1".to_string(), "id2".to_string()];
        let result = check_if_user_follows_artist_or_users::check_if_user_follows_artist_or_users(&api, "artist", &ids).await;
        assert!(result.is_ok());
        let data = result.unwrap().unwrap();
        assert!(data[0].as_bool().unwrap());
        assert!(!data[1].as_bool().unwrap());
    }
}

// =============================================================================
// Track Endpoint Tests
// =============================================================================

mod track_endpoints {
    use super::*;
    use spotify_cli::endpoints::tracks::{get_track, get_several_tracks};

    #[tokio::test]
    async fn test_get_track() {
        let (server, api) = setup().await;

        Mock::given(method("GET"))
            .and(path("/tracks/3n3Ppam7vgaVa1iaRUc9Lp"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "id": "3n3Ppam7vgaVa1iaRUc9Lp",
                "name": "Mr. Brightside",
                "artists": [{"name": "The Killers"}],
                "album": {"name": "Hot Fuss"},
                "duration_ms": 222973
            })))
            .mount(&server)
            .await;

        let result = get_track::get_track(&api, "3n3Ppam7vgaVa1iaRUc9Lp").await;
        assert!(result.is_ok());
        let track = result.unwrap().unwrap();
        assert_eq!(track["name"], "Mr. Brightside");
    }

    #[tokio::test]
    async fn test_get_track_not_found() {
        let (server, api) = setup().await;

        Mock::given(method("GET"))
            .and(path("/tracks/invalid"))
            .respond_with(ResponseTemplate::new(404).set_body_json(json!({
                "error": {"status": 404, "message": "Not found"}
            })))
            .mount(&server)
            .await;

        let result = get_track::get_track(&api, "invalid").await;
        assert!(matches!(result, Err(HttpError::NotFound)));
    }

    #[tokio::test]
    async fn test_get_several_tracks() {
        let (server, api) = setup().await;

        Mock::given(method("GET"))
            .and(path("/tracks"))
            .and(query_param("ids", "id1,id2,id3"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "tracks": [
                    {"id": "id1", "name": "Track 1"},
                    {"id": "id2", "name": "Track 2"},
                    {"id": "id3", "name": "Track 3"}
                ]
            })))
            .mount(&server)
            .await;

        let ids = vec!["id1".to_string(), "id2".to_string(), "id3".to_string()];
        let result = get_several_tracks::get_several_tracks(&api, &ids).await;
        assert!(result.is_ok());
        let data = result.unwrap().unwrap();
        assert_eq!(data["tracks"].as_array().unwrap().len(), 3);
    }
}

// =============================================================================
// Album Endpoint Tests
// =============================================================================

mod album_endpoints {
    use super::*;
    use spotify_cli::endpoints::albums::{
        get_album, get_album_tracks, get_new_releases,
        save_albums_for_current_user, remove_users_saved_albums,
        check_users_saved_albums,
    };

    #[tokio::test]
    async fn test_get_album() {
        let (server, api) = setup().await;

        Mock::given(method("GET"))
            .and(path("/albums/4aawyAB9vmqN3uQ7FjRGTy"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "id": "4aawyAB9vmqN3uQ7FjRGTy",
                "name": "Global Warming",
                "artists": [{"name": "Pitbull"}],
                "release_date": "2012-11-16",
                "total_tracks": 18
            })))
            .mount(&server)
            .await;

        let result = get_album::get_album(&api, "4aawyAB9vmqN3uQ7FjRGTy").await;
        assert!(result.is_ok());
        let album = result.unwrap().unwrap();
        assert_eq!(album["name"], "Global Warming");
    }

    #[tokio::test]
    async fn test_get_album_tracks() {
        let (server, api) = setup().await;

        Mock::given(method("GET"))
            .and(path_regex(r"/albums/.*/tracks"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "items": [
                    {"id": "t1", "name": "Track 1", "track_number": 1},
                    {"id": "t2", "name": "Track 2", "track_number": 2}
                ],
                "total": 2
            })))
            .mount(&server)
            .await;

        let result = get_album_tracks::get_album_tracks(&api, "album123", Some(20), Some(0)).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_get_new_releases() {
        let (server, api) = setup().await;

        Mock::given(method("GET"))
            .and(path("/browse/new-releases"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "albums": {
                    "items": [
                        {"id": "new1", "name": "New Album 1"},
                        {"id": "new2", "name": "New Album 2"}
                    ]
                }
            })))
            .mount(&server)
            .await;

        let result = get_new_releases::get_new_releases(&api, Some(20), Some(0)).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_save_albums() {
        let (server, api) = setup().await;

        Mock::given(method("PUT"))
            .and(path("/me/albums"))
            .and(query_param("ids", "album1"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({})))
            .mount(&server)
            .await;

        let ids = vec!["album1".to_string()];
        let result = save_albums_for_current_user::save_albums(&api, &ids).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_remove_saved_albums() {
        let (server, api) = setup().await;

        Mock::given(method("DELETE"))
            .and(path("/me/albums"))
            .and(query_param("ids", "album1"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({})))
            .mount(&server)
            .await;

        let ids = vec!["album1".to_string()];
        let result = remove_users_saved_albums::remove_albums(&api, &ids).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_check_saved_albums() {
        let (server, api) = setup().await;

        Mock::given(method("GET"))
            .and(path("/me/albums/contains"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!([true])))
            .mount(&server)
            .await;

        let ids = vec!["album1".to_string()];
        let result = check_users_saved_albums::check_saved_albums(&api, &ids).await;
        assert!(result.is_ok());
    }
}

// =============================================================================
// Artist Endpoint Tests
// =============================================================================

mod artist_endpoints {
    use super::*;
    use spotify_cli::endpoints::artists::{
        get_artist, get_artist_top_tracks, get_artists_albums,
        get_artists_related_artists,
    };

    #[tokio::test]
    async fn test_get_artist() {
        let (server, api) = setup().await;

        Mock::given(method("GET"))
            .and(path("/artists/0OdUWJ0sBjDrqHygGUXeCF"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "id": "0OdUWJ0sBjDrqHygGUXeCF",
                "name": "Band of Horses",
                "genres": ["indie rock", "rock"],
                "popularity": 59,
                "followers": {"total": 500000}
            })))
            .mount(&server)
            .await;

        let result = get_artist::get_artist(&api, "0OdUWJ0sBjDrqHygGUXeCF").await;
        assert!(result.is_ok());
        let artist = result.unwrap().unwrap();
        assert_eq!(artist["name"], "Band of Horses");
    }

    #[tokio::test]
    async fn test_get_artist_top_tracks() {
        let (server, api) = setup().await;

        Mock::given(method("GET"))
            .and(path_regex(r"/artists/.*/top-tracks"))
            .and(query_param("market", "US"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "tracks": [
                    {"id": "t1", "name": "Top Song 1", "popularity": 80},
                    {"id": "t2", "name": "Top Song 2", "popularity": 75}
                ]
            })))
            .mount(&server)
            .await;

        let result = get_artist_top_tracks::get_artist_top_tracks(&api, "artist123", Some("US")).await;
        assert!(result.is_ok());
        let data = result.unwrap().unwrap();
        assert_eq!(data["tracks"].as_array().unwrap().len(), 2);
    }

    #[tokio::test]
    async fn test_get_artists_albums() {
        let (server, api) = setup().await;

        Mock::given(method("GET"))
            .and(path_regex(r"/artists/.*/albums"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "items": [
                    {"id": "a1", "name": "Album 1", "release_date": "2020"},
                    {"id": "a2", "name": "Album 2", "release_date": "2018"}
                ],
                "total": 2
            })))
            .mount(&server)
            .await;

        let result = get_artists_albums::get_artists_albums(&api, "artist123", Some(20), Some(0)).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_get_related_artists() {
        let (server, api) = setup().await;

        Mock::given(method("GET"))
            .and(path_regex(r"/artists/.*/related-artists"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "artists": [
                    {"id": "rel1", "name": "Similar Artist 1"},
                    {"id": "rel2", "name": "Similar Artist 2"}
                ]
            })))
            .mount(&server)
            .await;

        let result = get_artists_related_artists::get_artists_related_artists(&api, "artist123").await;
        assert!(result.is_ok());
    }
}

// =============================================================================
// Player Endpoint Tests
// =============================================================================

mod player_endpoints {
    use super::*;
    use spotify_cli::endpoints::player::{
        get_playback_state, get_available_devices, get_currently_playing_track,
        get_users_queue, get_recently_played_tracks, pause_playback, skip_to_next,
        skip_to_previous, seek_to_position, set_playback_volume, toggle_playback_shuffle,
        set_repeat_mode, add_item_to_playback_queue,
    };

    #[tokio::test]
    async fn test_get_playback_state() {
        let (server, api) = setup().await;

        Mock::given(method("GET"))
            .and(path("/me/player"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "is_playing": true,
                "progress_ms": 50000,
                "item": {"id": "track1", "name": "Current Song"},
                "device": {"id": "device1", "name": "My Speaker"}
            })))
            .mount(&server)
            .await;

        let result = get_playback_state::get_playback_state(&api).await;
        assert!(result.is_ok());
        let state = result.unwrap().unwrap();
        assert!(state["is_playing"].as_bool().unwrap());
    }

    #[tokio::test]
    async fn test_get_playback_state_no_active_device() {
        let (server, api) = setup().await;

        Mock::given(method("GET"))
            .and(path("/me/player"))
            .respond_with(ResponseTemplate::new(204))
            .mount(&server)
            .await;

        let result = get_playback_state::get_playback_state(&api).await;
        assert!(result.is_ok());
        assert!(result.unwrap().is_none());
    }

    #[tokio::test]
    async fn test_get_available_devices() {
        let (server, api) = setup().await;

        Mock::given(method("GET"))
            .and(path("/me/player/devices"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "devices": [
                    {"id": "d1", "name": "Laptop", "type": "Computer", "is_active": true},
                    {"id": "d2", "name": "Phone", "type": "Smartphone", "is_active": false}
                ]
            })))
            .mount(&server)
            .await;

        let result = get_available_devices::get_available_devices(&api).await;
        assert!(result.is_ok());
        let data = result.unwrap().unwrap();
        assert_eq!(data["devices"].as_array().unwrap().len(), 2);
    }

    #[tokio::test]
    async fn test_get_currently_playing() {
        let (server, api) = setup().await;

        Mock::given(method("GET"))
            .and(path("/me/player/currently-playing"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "is_playing": true,
                "item": {"id": "track1", "name": "Now Playing"}
            })))
            .mount(&server)
            .await;

        let result = get_currently_playing_track::get_currently_playing_track(&api).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_get_queue() {
        let (server, api) = setup().await;

        Mock::given(method("GET"))
            .and(path("/me/player/queue"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "currently_playing": {"id": "current", "name": "Current Track"},
                "queue": [
                    {"id": "q1", "name": "Next Up 1"},
                    {"id": "q2", "name": "Next Up 2"}
                ]
            })))
            .mount(&server)
            .await;

        let result = get_users_queue::get_users_queue(&api).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_pause_playback() {
        let (server, api) = setup().await;

        Mock::given(method("PUT"))
            .and(path("/me/player/pause"))
            .respond_with(ResponseTemplate::new(204))
            .mount(&server)
            .await;

        let result = pause_playback::pause_playback(&api).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_skip_next() {
        let (server, api) = setup().await;

        Mock::given(method("POST"))
            .and(path("/me/player/next"))
            .respond_with(ResponseTemplate::new(204))
            .mount(&server)
            .await;

        let result = skip_to_next::skip_to_next(&api).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_skip_previous() {
        let (server, api) = setup().await;

        Mock::given(method("POST"))
            .and(path("/me/player/previous"))
            .respond_with(ResponseTemplate::new(204))
            .mount(&server)
            .await;

        let result = skip_to_previous::skip_to_previous(&api).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_seek_to_position() {
        let (server, api) = setup().await;

        Mock::given(method("PUT"))
            .and(path("/me/player/seek"))
            .and(query_param("position_ms", "30000"))
            .respond_with(ResponseTemplate::new(204))
            .mount(&server)
            .await;

        let result = seek_to_position::seek_to_position(&api, 30000).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_set_volume() {
        let (server, api) = setup().await;

        Mock::given(method("PUT"))
            .and(path("/me/player/volume"))
            .and(query_param("volume_percent", "50"))
            .respond_with(ResponseTemplate::new(204))
            .mount(&server)
            .await;

        let result = set_playback_volume::set_playback_volume(&api, 50).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_set_shuffle() {
        let (server, api) = setup().await;

        Mock::given(method("PUT"))
            .and(path("/me/player/shuffle"))
            .and(query_param("state", "true"))
            .respond_with(ResponseTemplate::new(204))
            .mount(&server)
            .await;

        let result = toggle_playback_shuffle::toggle_playback_shuffle(&api, true).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_set_repeat() {
        let (server, api) = setup().await;

        Mock::given(method("PUT"))
            .and(path("/me/player/repeat"))
            .and(query_param("state", "track"))
            .respond_with(ResponseTemplate::new(204))
            .mount(&server)
            .await;

        let result = set_repeat_mode::set_repeat_mode(&api, "track").await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_add_to_queue() {
        let (server, api) = setup().await;

        Mock::given(method("POST"))
            .and(path("/me/player/queue"))
            .respond_with(ResponseTemplate::new(204))
            .mount(&server)
            .await;

        let result = add_item_to_playback_queue::add_item_to_playback_queue(&api, "spotify:track:123").await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_get_recently_played() {
        let (server, api) = setup().await;

        Mock::given(method("GET"))
            .and(path("/me/player/recently-played"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "items": [
                    {"track": {"id": "t1", "name": "Recent 1"}, "played_at": "2024-01-01T12:00:00Z"},
                    {"track": {"id": "t2", "name": "Recent 2"}, "played_at": "2024-01-01T11:00:00Z"}
                ]
            })))
            .mount(&server)
            .await;

        let result = get_recently_played_tracks::get_recently_played_tracks(&api).await;
        assert!(result.is_ok());
    }
}

// =============================================================================
// Playlist Endpoint Tests
// =============================================================================

mod playlist_endpoints {
    use super::*;
    use spotify_cli::endpoints::playlists::{
        get_playlist, get_current_user_playlists, create_playlist,
        add_items_to_playlist, remove_items_from_playlist, change_playlist_details,
        follow_playlist, unfollow_playlist, get_featured_playlists,
    };

    #[tokio::test]
    async fn test_get_playlist() {
        let (server, api) = setup().await;

        Mock::given(method("GET"))
            .and(path("/playlists/37i9dQZF1DXcBWIGoYBM5M"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "id": "37i9dQZF1DXcBWIGoYBM5M",
                "name": "Today's Top Hits",
                "owner": {"id": "spotify"},
                "tracks": {"total": 50}
            })))
            .mount(&server)
            .await;

        let result = get_playlist::get_playlist(&api, "37i9dQZF1DXcBWIGoYBM5M").await;
        assert!(result.is_ok());
        let playlist = result.unwrap().unwrap();
        assert_eq!(playlist["name"], "Today's Top Hits");
    }

    #[tokio::test]
    async fn test_get_current_user_playlists() {
        let (server, api) = setup().await;

        Mock::given(method("GET"))
            .and(path("/me/playlists"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "items": [
                    {"id": "p1", "name": "My Playlist 1"},
                    {"id": "p2", "name": "My Playlist 2"}
                ],
                "total": 2
            })))
            .mount(&server)
            .await;

        let result = get_current_user_playlists::get_current_user_playlists(&api, Some(20), Some(0)).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_create_playlist() {
        let (server, api) = setup().await;

        Mock::given(method("POST"))
            .and(path("/users/user123/playlists"))
            .respond_with(ResponseTemplate::new(201).set_body_json(json!({
                "id": "new_playlist",
                "name": "My New Playlist",
                "public": false
            })))
            .mount(&server)
            .await;

        let result = create_playlist::create_playlist(&api, "user123", "My New Playlist", None, false).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_add_items_to_playlist() {
        let (server, api) = setup().await;

        Mock::given(method("POST"))
            .and(path_regex(r"/playlists/.*/tracks"))
            .respond_with(ResponseTemplate::new(201).set_body_json(json!({
                "snapshot_id": "abc123"
            })))
            .mount(&server)
            .await;

        let uris = vec!["spotify:track:1".to_string(), "spotify:track:2".to_string()];
        let result = add_items_to_playlist::add_items_to_playlist(
            &api, "playlist123", &uris, None
        ).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_remove_items_from_playlist() {
        let (server, api) = setup().await;

        Mock::given(method("DELETE"))
            .and(path_regex(r"/playlists/.*/tracks"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "snapshot_id": "def456"
            })))
            .mount(&server)
            .await;

        let uris = vec!["spotify:track:1".to_string()];
        let result = remove_items_from_playlist::remove_items_from_playlist(
            &api, "playlist123", &uris
        ).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_change_playlist_details() {
        let (server, api) = setup().await;

        Mock::given(method("PUT"))
            .and(path("/playlists/playlist123"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({})))
            .mount(&server)
            .await;

        let result = change_playlist_details::change_playlist_details(
            &api, "playlist123", Some("New Name"), Some("New Description"), Some(true)
        ).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_follow_playlist() {
        let (server, api) = setup().await;

        Mock::given(method("PUT"))
            .and(path("/playlists/playlist123/followers"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({})))
            .mount(&server)
            .await;

        let result = follow_playlist::follow_playlist(&api, "playlist123", Some(false)).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_unfollow_playlist() {
        let (server, api) = setup().await;

        Mock::given(method("DELETE"))
            .and(path("/playlists/playlist123/followers"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({})))
            .mount(&server)
            .await;

        let result = unfollow_playlist::unfollow_playlist(&api, "playlist123").await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_get_featured_playlists() {
        let (server, api) = setup().await;

        Mock::given(method("GET"))
            .and(path("/browse/featured-playlists"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "message": "Monday morning music",
                "playlists": {
                    "items": [{"id": "f1", "name": "Morning Coffee"}]
                }
            })))
            .mount(&server)
            .await;

        let result = get_featured_playlists::get_featured_playlists(&api, Some(20), Some(0)).await;
        assert!(result.is_ok());
    }
}

// =============================================================================
// Search Endpoint Tests
// =============================================================================

mod search_endpoints {
    use super::*;
    use spotify_cli::endpoints::search;

    #[tokio::test]
    async fn test_search_tracks() {
        let (server, api) = setup().await;

        Mock::given(method("GET"))
            .and(path("/search"))
            .and(query_param("type", "track"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "tracks": {
                    "items": [
                        {"id": "t1", "name": "Search Result 1"},
                        {"id": "t2", "name": "Search Result 2"}
                    ],
                    "total": 100
                }
            })))
            .mount(&server)
            .await;

        let result = search::search(&api, "test query", Some(&["track"]), Some(20)).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_search_multiple_types() {
        let (server, api) = setup().await;

        Mock::given(method("GET"))
            .and(path("/search"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "tracks": {"items": [], "total": 0},
                "artists": {"items": [], "total": 0},
                "albums": {"items": [], "total": 0}
            })))
            .mount(&server)
            .await;

        let result = search::search(&api, "beatles", Some(&["track", "artist", "album"]), Some(20)).await;
        assert!(result.is_ok());
    }
}

// =============================================================================
// Library Endpoint Tests
// =============================================================================

mod library_endpoints {
    use super::*;
    use spotify_cli::endpoints::library::{
        get_saved_tracks, save_tracks, remove_tracks, check_saved_tracks,
    };

    #[tokio::test]
    async fn test_get_saved_tracks() {
        let (server, api) = setup().await;

        Mock::given(method("GET"))
            .and(path("/me/tracks"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "items": [
                    {"track": {"id": "t1", "name": "Liked Song 1"}},
                    {"track": {"id": "t2", "name": "Liked Song 2"}}
                ],
                "total": 100
            })))
            .mount(&server)
            .await;

        let result = get_saved_tracks::get_saved_tracks(&api, Some(20), Some(0)).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_save_tracks() {
        let (server, api) = setup().await;

        Mock::given(method("PUT"))
            .and(path("/me/tracks"))
            .and(query_param("ids", "track1,track2"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({})))
            .mount(&server)
            .await;

        let ids = vec!["track1".to_string(), "track2".to_string()];
        let result = save_tracks::save_tracks(&api, &ids).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_remove_tracks() {
        let (server, api) = setup().await;

        Mock::given(method("DELETE"))
            .and(path("/me/tracks"))
            .and(query_param("ids", "track1"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({})))
            .mount(&server)
            .await;

        let ids = vec!["track1".to_string()];
        let result = remove_tracks::remove_tracks(&api, &ids).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_check_saved_tracks() {
        let (server, api) = setup().await;

        Mock::given(method("GET"))
            .and(path("/me/tracks/contains"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!([true, false, true])))
            .mount(&server)
            .await;

        let ids = vec!["t1".to_string(), "t2".to_string(), "t3".to_string()];
        let result = check_saved_tracks::check_saved_tracks(&api, &ids).await;
        assert!(result.is_ok());
        let data = result.unwrap().unwrap();
        assert!(data[0].as_bool().unwrap());
        assert!(!data[1].as_bool().unwrap());
    }
}

// =============================================================================
// Category Endpoint Tests
// =============================================================================

mod category_endpoints {
    use super::*;
    use spotify_cli::endpoints::categories::{
        get_single_browse_category, get_several_browse_categories, get_category_playlists,
    };

    #[tokio::test]
    async fn test_get_category() {
        let (server, api) = setup().await;

        Mock::given(method("GET"))
            .and(path("/browse/categories/pop"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "id": "pop",
                "name": "Pop",
                "icons": [{"url": "https://example.com/pop.jpg"}]
            })))
            .mount(&server)
            .await;

        let result = get_single_browse_category::get_single_browse_category(&api, "pop").await;
        assert!(result.is_ok());
        let category = result.unwrap().unwrap();
        assert_eq!(category["name"], "Pop");
    }

    #[tokio::test]
    async fn test_get_categories() {
        let (server, api) = setup().await;

        Mock::given(method("GET"))
            .and(path("/browse/categories"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "categories": {
                    "items": [
                        {"id": "pop", "name": "Pop"},
                        {"id": "rock", "name": "Rock"},
                        {"id": "hiphop", "name": "Hip Hop"}
                    ]
                }
            })))
            .mount(&server)
            .await;

        let result = get_several_browse_categories::get_several_browse_categories(&api, Some(20), Some(0)).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_get_category_playlists() {
        let (server, api) = setup().await;

        Mock::given(method("GET"))
            .and(path_regex(r"/browse/categories/.*/playlists"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "playlists": {
                    "items": [
                        {"id": "p1", "name": "Top Pop"},
                        {"id": "p2", "name": "Pop Rising"}
                    ]
                }
            })))
            .mount(&server)
            .await;

        let result = get_category_playlists::get_category_playlists(&api, "pop", Some(20), Some(0)).await;
        assert!(result.is_ok());
    }
}

// =============================================================================
// Show/Podcast Endpoint Tests
// =============================================================================

mod show_endpoints {
    use super::*;
    use spotify_cli::endpoints::shows::{
        get_show, get_show_episodes, save_shows_for_current_user,
        remove_users_saved_shows,
    };

    #[tokio::test]
    async fn test_get_show() {
        let (server, api) = setup().await;

        Mock::given(method("GET"))
            .and(path("/shows/show123"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "id": "show123",
                "name": "The Daily",
                "publisher": "The New York Times",
                "total_episodes": 500
            })))
            .mount(&server)
            .await;

        let result = get_show::get_show(&api, "show123").await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_get_show_episodes() {
        let (server, api) = setup().await;

        Mock::given(method("GET"))
            .and(path_regex(r"/shows/.*/episodes"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "items": [
                    {"id": "ep1", "name": "Episode 1"},
                    {"id": "ep2", "name": "Episode 2"}
                ]
            })))
            .mount(&server)
            .await;

        let result = get_show_episodes::get_show_episodes(&api, "show123", Some(20), Some(0)).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_save_shows() {
        let (server, api) = setup().await;

        Mock::given(method("PUT"))
            .and(path("/me/shows"))
            .and(query_param("ids", "show1"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({})))
            .mount(&server)
            .await;

        let ids = vec!["show1".to_string()];
        let result = save_shows_for_current_user::save_shows(&api, &ids).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_remove_saved_shows() {
        let (server, api) = setup().await;

        Mock::given(method("DELETE"))
            .and(path("/me/shows"))
            .and(query_param("ids", "show1"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({})))
            .mount(&server)
            .await;

        let ids = vec!["show1".to_string()];
        let result = remove_users_saved_shows::remove_shows(&api, &ids).await;
        assert!(result.is_ok());
    }
}

// =============================================================================
// Episode Endpoint Tests
// =============================================================================

mod episode_endpoints {
    use super::*;
    use spotify_cli::endpoints::episodes::{
        get_episode, save_episodes_for_current_user,
    };

    #[tokio::test]
    async fn test_get_episode() {
        let (server, api) = setup().await;

        Mock::given(method("GET"))
            .and(path("/episodes/ep123"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "id": "ep123",
                "name": "Episode Title",
                "duration_ms": 3600000,
                "release_date": "2024-01-15"
            })))
            .mount(&server)
            .await;

        let result = get_episode::get_episode(&api, "ep123").await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_save_episodes() {
        let (server, api) = setup().await;

        Mock::given(method("PUT"))
            .and(path("/me/episodes"))
            .and(query_param("ids", "ep1,ep2"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({})))
            .mount(&server)
            .await;

        let ids = vec!["ep1".to_string(), "ep2".to_string()];
        let result = save_episodes_for_current_user::save_episodes(&api, &ids).await;
        assert!(result.is_ok());
    }
}

// =============================================================================
// Audiobook Endpoint Tests
// =============================================================================

mod audiobook_endpoints {
    use super::*;
    use spotify_cli::endpoints::audiobooks::{
        get_audiobook, get_audiobook_chapters,
        save_audiobooks_for_current_user,
    };

    #[tokio::test]
    async fn test_get_audiobook() {
        let (server, api) = setup().await;

        Mock::given(method("GET"))
            .and(path("/audiobooks/book123"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "id": "book123",
                "name": "The Great Gatsby",
                "authors": [{"name": "F. Scott Fitzgerald"}],
                "total_chapters": 9
            })))
            .mount(&server)
            .await;

        let result = get_audiobook::get_audiobook(&api, "book123").await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_get_audiobook_chapters() {
        let (server, api) = setup().await;

        Mock::given(method("GET"))
            .and(path_regex(r"/audiobooks/.*/chapters"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "items": [
                    {"id": "ch1", "name": "Chapter 1"},
                    {"id": "ch2", "name": "Chapter 2"}
                ]
            })))
            .mount(&server)
            .await;

        let result = get_audiobook_chapters::get_audiobook_chapters(&api, "book123", Some(20), Some(0)).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_save_audiobooks() {
        let (server, api) = setup().await;

        Mock::given(method("PUT"))
            .and(path("/me/audiobooks"))
            .and(query_param("ids", "book1"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({})))
            .mount(&server)
            .await;

        let ids = vec!["book1".to_string()];
        let result = save_audiobooks_for_current_user::save_audiobooks(&api, &ids).await;
        assert!(result.is_ok());
    }
}

// =============================================================================
// Markets Endpoint Tests
// =============================================================================

mod markets_endpoints {
    use super::*;
    use spotify_cli::endpoints::markets::get_available_markets;

    #[tokio::test]
    async fn test_get_markets() {
        let (server, api) = setup().await;

        Mock::given(method("GET"))
            .and(path("/markets"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "markets": ["US", "GB", "DE", "FR", "JP", "AU"]
            })))
            .mount(&server)
            .await;

        let result = get_available_markets::get_available_markets(&api).await;
        assert!(result.is_ok());
        let data = result.unwrap().unwrap();
        assert!(!data["markets"].as_array().unwrap().is_empty());
    }
}

// =============================================================================
// Error Handling Tests
// =============================================================================

mod error_handling {
    use super::*;
    use spotify_cli::endpoints::tracks::get_track;

    #[tokio::test]
    async fn test_unauthorized_error() {
        let (server, api) = setup().await;

        Mock::given(method("GET"))
            .and(path("/tracks/test"))
            .respond_with(ResponseTemplate::new(401).set_body_json(json!({
                "error": {"status": 401, "message": "Invalid access token"}
            })))
            .mount(&server)
            .await;

        let result = get_track::get_track(&api, "test").await;
        assert!(matches!(result, Err(HttpError::Unauthorized)));
    }

    #[tokio::test]
    async fn test_rate_limit_error() {
        let (server, api) = setup().await;

        Mock::given(method("GET"))
            .and(path("/tracks/test"))
            .respond_with(
                ResponseTemplate::new(429)
                    .append_header("retry-after", "30")
                    .set_body_json(json!({
                        "error": {"status": 429, "message": "Rate limit exceeded"}
                    }))
            )
            .mount(&server)
            .await;

        let result = get_track::get_track(&api, "test").await;
        assert!(matches!(result, Err(HttpError::RateLimited { .. })));
    }

    #[tokio::test]
    async fn test_server_error() {
        let (server, api) = setup().await;

        Mock::given(method("GET"))
            .and(path("/tracks/test"))
            .respond_with(ResponseTemplate::new(500).set_body_json(json!({
                "error": {"status": 500, "message": "Internal server error"}
            })))
            .mount(&server)
            .await;

        let result = get_track::get_track(&api, "test").await;
        assert!(matches!(result, Err(HttpError::Api { status: 500, .. })));
    }

    #[tokio::test]
    async fn test_bad_request_error() {
        let (server, api) = setup().await;

        Mock::given(method("GET"))
            .and(path("/tracks/test"))
            .respond_with(ResponseTemplate::new(400).set_body_json(json!({
                "error": {"status": 400, "message": "Bad request"}
            })))
            .mount(&server)
            .await;

        let result = get_track::get_track(&api, "test").await;
        assert!(matches!(result, Err(HttpError::Api { status: 400, .. })));
    }

    #[tokio::test]
    async fn test_forbidden_error() {
        let (server, api) = setup().await;

        Mock::given(method("GET"))
            .and(path("/tracks/test"))
            .respond_with(ResponseTemplate::new(403).set_body_json(json!({
                "error": {"status": 403, "message": "Forbidden"}
            })))
            .mount(&server)
            .await;

        let result = get_track::get_track(&api, "test").await;
        assert!(matches!(result, Err(HttpError::Forbidden)));
    }
}
