//! Integration tests for spotify-cli commands

use spotify_cli::io::output::{ErrorKind, Response, Status};
use spotify_cli::cli::commands::SearchFilters;
use spotify_cli::storage::pins::{Pin, ResourceType};


#[test]
fn test_success_response_serialization() {
    let resp = Response::success(200, "OK");
    let json = resp.to_json();

    assert!(json.contains(r#""status":"success""#));
    assert!(json.contains(r#""code":200"#));
    assert!(json.contains(r#""message":"OK""#));
    assert!(!json.contains("payload"));
    assert!(!json.contains("error"));
}

#[test]
fn test_success_with_payload_serialization() {
    let payload = serde_json::json!({"track": "Test Track", "artist": "Test Artist"});
    let resp = Response::success_with_payload(200, "Track found", payload);
    let json = resp.to_json();

    assert!(json.contains(r#""status":"success""#));
    assert!(json.contains("payload"));
    assert!(json.contains("Test Track"));
}

#[test]
fn test_error_response_serialization() {
    let resp = Response::err(404, "Not found", ErrorKind::NotFound);
    let json = resp.to_json();

    assert!(json.contains(r#""status":"error""#));
    assert!(json.contains(r#""code":404"#));
    assert!(json.contains(r#""kind":"not_found""#));
}

#[test]
fn test_error_with_details_serialization() {
    let resp = Response::err_with_details(
        500,
        "Storage failed",
        ErrorKind::Storage,
        "Could not write to disk"
    );
    let json = resp.to_json();

    assert!(json.contains(r#""kind":"storage_error""#));
    assert!(json.contains("Could not write to disk"));
}


#[test]
fn test_search_filters_empty() {
    let filters = SearchFilters::default();
    assert!(!filters.has_filters());
    assert_eq!(filters.build_query("test"), "test");
}

#[test]
fn test_search_filters_with_artist() {
    let filters = SearchFilters {
        artist: Some("Radiohead".to_string()),
        ..Default::default()
    };

    assert!(filters.has_filters());
    let query = filters.build_query("creep");
    assert!(query.contains("creep"));
    assert!(query.contains("artist:Radiohead"));
}

#[test]
fn test_search_filters_multiple() {
    let filters = SearchFilters {
        artist: Some("Beatles".to_string()),
        album: Some("Abbey Road".to_string()),
        year: Some("1969".to_string()),
        ..Default::default()
    };

    let query = filters.build_query("");
    assert!(query.contains("artist:Beatles"));
    assert!(query.contains("album:Abbey Road"));
    assert!(query.contains("year:1969"));
}

#[test]
fn test_search_filters_tags() {
    let filters = SearchFilters {
        new: true,
        hipster: true,
        ..Default::default()
    };

    let query = filters.build_query("indie");
    assert!(query.contains("tag:new"));
    assert!(query.contains("tag:hipster"));
}


#[test]
fn test_extract_id_from_url() {
    let url = "https://open.spotify.com/track/4iV5W9uYEdYUVa79Axb7Rh";
    let id = Pin::extract_id(url);
    assert_eq!(id, "4iV5W9uYEdYUVa79Axb7Rh");
}

#[test]
fn test_extract_id_from_url_with_params() {
    let url = "https://open.spotify.com/track/4iV5W9uYEdYUVa79Axb7Rh?si=abc123";
    let id = Pin::extract_id(url);
    assert_eq!(id, "4iV5W9uYEdYUVa79Axb7Rh");
}

#[test]
fn test_extract_id_from_uri() {
    let uri = "spotify:track:4iV5W9uYEdYUVa79Axb7Rh";
    let id = Pin::extract_id(uri);
    assert_eq!(id, "4iV5W9uYEdYUVa79Axb7Rh");
}

#[test]
fn test_extract_id_passthrough() {
    let id = "4iV5W9uYEdYUVa79Axb7Rh";
    let result = Pin::extract_id(id);
    assert_eq!(result, "4iV5W9uYEdYUVa79Axb7Rh");
}


#[test]
fn test_pin_creation() {
    let pin = Pin::new(
        ResourceType::Playlist,
        "test-id".to_string(),
        "my-playlist".to_string(),
        vec!["chill".to_string(), "focus".to_string()],
    );

    assert_eq!(pin.resource_type, ResourceType::Playlist);
    assert_eq!(pin.id, "test-id");
    assert_eq!(pin.alias, "my-playlist");
    assert_eq!(pin.tags.len(), 2);
}

#[test]
fn test_pin_uri_generation() {
    let pin = Pin::new(
        ResourceType::Track,
        "abc123".to_string(),
        "favorite".to_string(),
        vec![],
    );

    assert_eq!(pin.uri(), "spotify:track:abc123");
}

#[test]
fn test_pin_uri_for_each_type() {
    let types = [
        (ResourceType::Track, "spotify:track:id123"),
        (ResourceType::Album, "spotify:album:id123"),
        (ResourceType::Artist, "spotify:artist:id123"),
        (ResourceType::Playlist, "spotify:playlist:id123"),
        (ResourceType::Show, "spotify:show:id123"),
        (ResourceType::Episode, "spotify:episode:id123"),
        (ResourceType::Audiobook, "spotify:audiobook:id123"),
    ];

    for (resource_type, expected_uri) in types {
        let pin = Pin::new(
            resource_type,
            "id123".to_string(),
            "test".to_string(),
            vec![],
        );
        assert_eq!(pin.uri(), expected_uri);
    }
}


#[test]
fn test_error_kind_string_representation() {
    assert_eq!(ErrorKind::Network.as_str(), "network_error");
    assert_eq!(ErrorKind::Api.as_str(), "api_error");
    assert_eq!(ErrorKind::Auth.as_str(), "auth_error");
    assert_eq!(ErrorKind::NotFound.as_str(), "not_found");
    assert_eq!(ErrorKind::Forbidden.as_str(), "forbidden");
    assert_eq!(ErrorKind::RateLimited.as_str(), "rate_limited");
    assert_eq!(ErrorKind::Validation.as_str(), "validation_error");
    assert_eq!(ErrorKind::Storage.as_str(), "storage_error");
    assert_eq!(ErrorKind::Config.as_str(), "config_error");
    assert_eq!(ErrorKind::Player.as_str(), "player_error");
}

#[test]
fn test_response_status_variants() {
    let success = Response::success(200, "OK");
    assert!(matches!(success.status, Status::Success));

    let error = Response::err(500, "Error", ErrorKind::Api);
    assert!(matches!(error.status, Status::Error));
}


#[test]
fn test_resource_type_as_str() {
    assert_eq!(ResourceType::Track.as_str(), "track");
    assert_eq!(ResourceType::Album.as_str(), "album");
    assert_eq!(ResourceType::Artist.as_str(), "artist");
    assert_eq!(ResourceType::Playlist.as_str(), "playlist");
    assert_eq!(ResourceType::Show.as_str(), "show");
    assert_eq!(ResourceType::Episode.as_str(), "episode");
    assert_eq!(ResourceType::Audiobook.as_str(), "audiobook");
}

#[test]
fn test_resource_type_from_str() {
    assert_eq!("track".parse::<ResourceType>().unwrap(), ResourceType::Track);
    assert_eq!("album".parse::<ResourceType>().unwrap(), ResourceType::Album);
    assert_eq!("artist".parse::<ResourceType>().unwrap(), ResourceType::Artist);
    assert_eq!("playlist".parse::<ResourceType>().unwrap(), ResourceType::Playlist);
    assert_eq!("show".parse::<ResourceType>().unwrap(), ResourceType::Show);
    assert_eq!("episode".parse::<ResourceType>().unwrap(), ResourceType::Episode);
    assert_eq!("audiobook".parse::<ResourceType>().unwrap(), ResourceType::Audiobook);
}

#[test]
fn test_resource_type_from_str_invalid() {
    assert!("invalid".parse::<ResourceType>().is_err());
    assert!("".parse::<ResourceType>().is_err());
}


use spotify_cli::types::{Track, Album, Artist, PlaybackState, Device};
use spotify_cli::storage::config::FuzzyConfig;
use spotify_cli::storage::fuzzy::{calculate_score, levenshtein_distance};

#[test]
fn test_track_deserialization() {
    let json = r#"{
        "id": "abc123",
        "name": "Test Track",
        "duration_ms": 210000,
        "uri": "spotify:track:abc123",
        "type": "track",
        "artists": [{"id": "art1", "name": "Test Artist", "type": "artist", "uri": "spotify:artist:art1"}],
        "album": {"id": "alb1", "name": "Test Album", "type": "album", "uri": "spotify:album:alb1"}
    }"#;

    let track: Track = serde_json::from_str(json).expect("Failed to deserialize track");
    assert_eq!(track.id, "abc123");
    assert_eq!(track.name, "Test Track");
    assert_eq!(track.duration_ms, 210000);
    assert_eq!(track.duration_str(), "3:30");
    assert_eq!(track.artist_name(), Some("Test Artist"));
    assert_eq!(track.album_name(), Some("Test Album"));
}

#[test]
fn test_artist_deserialization() {
    let json = r#"{
        "id": "artist123",
        "name": "Famous Artist",
        "type": "artist",
        "uri": "spotify:artist:artist123",
        "genres": ["rock", "alternative"],
        "popularity": 85,
        "followers": {"total": 1000000}
    }"#;

    let artist: Artist = serde_json::from_str(json).expect("Failed to deserialize artist");
    assert_eq!(artist.id, "artist123");
    assert_eq!(artist.name, "Famous Artist");
    assert_eq!(artist.popularity, Some(85));
    assert_eq!(artist.genres.as_ref().map(|g| g.len()), Some(2));
}

#[test]
fn test_album_deserialization() {
    let json = r#"{
        "id": "album123",
        "name": "Great Album",
        "type": "album",
        "uri": "spotify:album:album123",
        "album_type": "album",
        "total_tracks": 12,
        "release_date": "2023-06-15",
        "artists": [{"id": "art1", "name": "The Band", "type": "artist", "uri": "spotify:artist:art1"}]
    }"#;

    let album: Album = serde_json::from_str(json).expect("Failed to deserialize album");
    assert_eq!(album.id, "album123");
    assert_eq!(album.name, "Great Album");
    assert_eq!(album.total_tracks, Some(12));
    assert_eq!(album.artist_name(), Some("The Band"));
    assert_eq!(album.release_year(), Some("2023"));
}

#[test]
fn test_device_deserialization() {
    let json = r#"{
        "id": "device123",
        "is_active": true,
        "name": "My Computer",
        "type": "Computer",
        "volume_percent": 75
    }"#;

    let device: Device = serde_json::from_str(json).expect("Failed to deserialize device");
    assert_eq!(device.name, "My Computer");
    assert!(device.is_active);
    assert_eq!(device.volume_percent, Some(75));
    assert_eq!(device.device_type_display(), "Computer");
}

#[test]
fn test_playback_state_deserialization() {
    let json = r#"{
        "is_playing": true,
        "progress_ms": 45000,
        "item": {
            "id": "track1",
            "name": "Playing Now",
            "duration_ms": 180000,
            "type": "track",
            "uri": "spotify:track:track1",
            "artists": [{"id": "a1", "name": "Artist", "type": "artist", "uri": "spotify:artist:a1"}]
        },
        "device": {
            "id": "dev1",
            "is_active": true,
            "name": "Speaker",
            "type": "Speaker",
            "volume_percent": 50
        }
    }"#;

    let state: PlaybackState = serde_json::from_str(json).expect("Failed to deserialize playback");
    assert!(state.is_playing);
    assert_eq!(state.progress_ms, Some(45000));
    assert_eq!(state.progress_str(), "0:45");
    assert_eq!(state.track_name(), Some("Playing Now"));
    assert_eq!(state.artist_name(), Some("Artist"));
    assert_eq!(state.device_name(), Some("Speaker"));
}


#[test]
fn test_levenshtein_identical_strings() {
    assert_eq!(levenshtein_distance("hello", "hello"), 0);
    assert_eq!(levenshtein_distance("", ""), 0);
}

#[test]
fn test_levenshtein_empty_string() {
    assert_eq!(levenshtein_distance("", "abc"), 3);
    assert_eq!(levenshtein_distance("abc", ""), 3);
}

#[test]
fn test_levenshtein_single_char_diff() {
    assert_eq!(levenshtein_distance("cat", "bat"), 1);
    assert_eq!(levenshtein_distance("cat", "car"), 1);
    assert_eq!(levenshtein_distance("cat", "cats"), 1);
}

#[test]
fn test_levenshtein_multiple_operations() {
    assert_eq!(levenshtein_distance("kitten", "sitting"), 3);
    assert_eq!(levenshtein_distance("saturday", "sunday"), 3);
}

#[test]
fn test_fuzzy_exact_match_case_insensitive() {
    let config = FuzzyConfig::default();
    let score = calculate_score("Radiohead", "radiohead", &config);
    assert_eq!(score, config.exact_match);
}

#[test]
fn test_fuzzy_starts_with_bonus() {
    let config = FuzzyConfig::default();
    let score = calculate_score("Radiohead - Creep", "radio", &config);
    assert!(score >= config.starts_with);
}

#[test]
fn test_fuzzy_contains_bonus() {
    let config = FuzzyConfig::default();
    let score = calculate_score("The Radiohead Album", "radiohead", &config);
    assert!(score >= config.contains);
}

#[test]
fn test_fuzzy_word_match() {
    let config = FuzzyConfig::default();
    let score = calculate_score("Tool - Lateralus", "tool lateralus", &config);
    assert!(score >= config.word_match * 2.0);
}

#[test]
fn test_fuzzy_no_match_low_score() {
    let config = FuzzyConfig::default();
    let score = calculate_score("Completely Different", "xyz", &config);
    assert!(score < config.contains);
}

#[test]
fn test_fuzzy_similarity_threshold() {
    let config = FuzzyConfig::default();
    let score_similar = calculate_score("Radiohed", "radiohead", &config);
    let score_different = calculate_score("xyz", "radiohead", &config);
    assert!(score_similar > score_different);
}

#[test]
fn test_extract_id_various_url_formats() {
    let test_cases = [
        ("https://open.spotify.com/track/123", "123"),
        ("https://open.spotify.com/album/456", "456"),
        ("https://open.spotify.com/playlist/789?si=abc", "789"),
        ("https://open.spotify.com/artist/abc123", "abc123"),
    ];

    for (url, expected) in test_cases {
        assert_eq!(Pin::extract_id(url), expected, "Failed for URL: {}", url);
    }
}

#[test]
fn test_extract_id_various_uri_formats() {
    let test_cases = [
        ("spotify:track:123", "123"),
        ("spotify:album:456", "456"),
        ("spotify:playlist:789", "789"),
        ("spotify:artist:abc", "abc"),
    ];

    for (uri, expected) in test_cases {
        assert_eq!(Pin::extract_id(uri), expected, "Failed for URI: {}", uri);
    }
}

#[test]
fn test_track_duration_formatting() {
    let cases = [
        (0, "0:00"),
        (1000, "0:01"),
        (59000, "0:59"),
        (60000, "1:00"),
        (90000, "1:30"),
        (3600000, "60:00"),
    ];

    for (ms, expected) in cases {
        let json = format!(
            r#"{{"id":"t","name":"T","duration_ms":{},"uri":"u","type":"track","artists":[]}}"#,
            ms
        );
        let track: Track = serde_json::from_str(&json).unwrap();
        assert_eq!(track.duration_str(), expected, "Failed for {}ms", ms);
    }
}

#[test]
fn test_album_release_year_formats() {
    let test_cases = [
        ("2023-06-15", Some("2023")),
        ("2023-06", Some("2023")),
        ("2023", Some("2023")),
    ];

    for (date, expected) in test_cases {
        let json = format!(
            r#"{{"id":"a","name":"A","type":"album","uri":"u","release_date":"{}","artists":[]}}"#,
            date
        );
        let album: Album = serde_json::from_str(&json).unwrap();
        assert_eq!(album.release_year(), expected, "Failed for date: {}", date);
    }
}

#[test]
fn test_playback_state_progress_formatting() {
    let base_json = r#"{"is_playing":true,"item":{"id":"t","name":"T","duration_ms":180000,"uri":"u","type":"track","artists":[]}}"#;

    let cases = [
        (0, "0:00"),
        (30000, "0:30"),
        (65000, "1:05"),
        (180000, "3:00"),
    ];

    for (progress, expected) in cases {
        let json = base_json.replace("is_playing\":true", &format!("is_playing\":true,\"progress_ms\":{}", progress));
        let state: PlaybackState = serde_json::from_str(&json).unwrap();
        assert_eq!(state.progress_str(), expected, "Failed for {}ms", progress);
    }
}

#[test]
fn test_search_filters_all_fields() {
    let filters = SearchFilters {
        artist: Some("Artist".to_string()),
        album: Some("Album".to_string()),
        track: Some("Track".to_string()),
        year: Some("2023".to_string()),
        genre: Some("rock".to_string()),
        isrc: Some("US1234567890".to_string()),
        upc: Some("012345678901".to_string()),
        new: true,
        hipster: true,
    };

    let query = filters.build_query("query");
    assert!(query.contains("query"));
    assert!(query.contains("artist:Artist"));
    assert!(query.contains("album:Album"));
    assert!(query.contains("track:Track"));
    assert!(query.contains("year:2023"));
    assert!(query.contains("genre:rock"));
    assert!(query.contains("isrc:US1234567890"));
    assert!(query.contains("upc:012345678901"));
    assert!(query.contains("tag:new"));
    assert!(query.contains("tag:hipster"));
}

#[test]
fn test_response_json_format() {
    let resp = Response::success_with_payload(
        200,
        "Test",
        serde_json::json!({"key": "value"}),
    );
    let json = resp.to_json();

    let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();
    assert_eq!(parsed["status"], "success");
    assert_eq!(parsed["code"], 200);
    assert_eq!(parsed["message"], "Test");
    assert_eq!(parsed["payload"]["key"], "value");
}
