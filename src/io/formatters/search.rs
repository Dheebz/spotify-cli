//! Search result formatting functions

use serde_json::Value;

use crate::io::common::{extract_artist_names, format_number, get_score, print_table, truncate};

pub fn format_search_results(payload: &Value) {
    let mut has_results = false;

    if let Some(pins) = payload.get("pins")
        && let Some(arr) = pins.as_array()
            && !arr.is_empty() {
                has_results = true;
                println!("Pinned:");
                for pin in arr.iter().take(5) {
                    let alias = pin.get("alias").and_then(|v| v.as_str()).unwrap_or("Unknown");
                    let rtype = pin.get("type").and_then(|v| v.as_str()).unwrap_or("?");
                    println!("  [{}] {}", rtype, alias);
                }
            }

    if let Some(spotify) = payload.get("spotify") {
        format_spotify_search(spotify, &mut has_results);
    } else {
        format_spotify_search(payload, &mut has_results);
    }

    if !has_results {
        println!("No results found.");
    }
}

pub fn format_spotify_search(payload: &Value, has_results: &mut bool) {
    if let Some(tracks) = payload.get("tracks").and_then(|t| t.get("items")).and_then(|i| i.as_array())
        && !tracks.is_empty() {
            *has_results = true;
            let rows: Vec<Vec<String>> = tracks.iter().map(|track| {
                let name = track.get("name").and_then(|v| v.as_str()).unwrap_or("Unknown");
                let artists = extract_artist_names(track);
                vec![
                    truncate(name, 30),
                    truncate(&artists, 20),
                    get_score(track).to_string(),
                ]
            }).collect();
            print_table("Tracks", &["Title", "Artist", "Score"], &rows, &[30, 20, 5]);
        }

    if let Some(albums) = payload.get("albums").and_then(|t| t.get("items")).and_then(|i| i.as_array())
        && !albums.is_empty() {
            *has_results = true;
            let rows: Vec<Vec<String>> = albums.iter().map(|album| {
                let name = album.get("name").and_then(|v| v.as_str()).unwrap_or("Unknown");
                let artists = extract_artist_names(album);
                vec![
                    truncate(name, 30),
                    truncate(&artists, 20),
                    get_score(album).to_string(),
                ]
            }).collect();
            print_table("Albums", &["Title", "Artist", "Score"], &rows, &[30, 20, 5]);
        }

    if let Some(artists) = payload.get("artists").and_then(|t| t.get("items")).and_then(|i| i.as_array())
        && !artists.is_empty() {
            *has_results = true;
            let rows: Vec<Vec<String>> = artists.iter().map(|artist| {
                let name = artist.get("name").and_then(|v| v.as_str()).unwrap_or("Unknown");
                let followers = artist
                    .get("followers")
                    .and_then(|f| f.get("total"))
                    .and_then(|v| v.as_u64())
                    .map(format_number)
                    .unwrap_or_else(|| "-".to_string());
                vec![
                    truncate(name, 30),
                    followers,
                    get_score(artist).to_string(),
                ]
            }).collect();
            print_table("Artists", &["Name", "Followers", "Score"], &rows, &[30, 12, 5]);
        }

    if let Some(playlists) = payload.get("playlists").and_then(|t| t.get("items")).and_then(|i| i.as_array()) {
        let valid: Vec<_> = playlists
            .iter()
            .filter(|p| p.get("id").and_then(|v| v.as_str()).is_some())
            .collect();

        if !valid.is_empty() {
            *has_results = true;
            let rows: Vec<Vec<String>> = valid.iter().map(|playlist| {
                let name = playlist
                    .get("name")
                    .and_then(|v| v.as_str())
                    .filter(|s| !s.is_empty())
                    .unwrap_or("[Untitled]");
                let owner = playlist
                    .get("owner")
                    .and_then(|o| o.get("display_name"))
                    .and_then(|v| v.as_str())
                    .unwrap_or_else(|| {
                        playlist
                            .get("owner")
                            .and_then(|o| o.get("id"))
                            .and_then(|v| v.as_str())
                            .unwrap_or("Unknown")
                    });
                vec![
                    truncate(name, 35),
                    truncate(owner, 15),
                    get_score(playlist).to_string(),
                ]
            }).collect();
            print_table("Playlists", &["Name", "Owner", "Score"], &rows, &[35, 15, 5]);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn format_search_results_with_tracks() {
        let payload = json!({
            "tracks": {
                "items": [
                    { "name": "Track 1", "artists": [{ "name": "Artist 1" }], "fuzzy_score": 90.0 },
                    { "name": "Track 2", "artists": [{ "name": "Artist 2" }], "fuzzy_score": 85.0 }
                ]
            }
        });
        format_search_results(&payload);
    }

    #[test]
    fn format_search_results_with_pins() {
        let payload = json!({
            "pins": [
                { "alias": "my favorite", "type": "track" },
                { "alias": "chill playlist", "type": "playlist" }
            ]
        });
        format_search_results(&payload);
    }

    #[test]
    fn format_search_results_empty() {
        let payload = json!({});
        format_search_results(&payload);
    }

    #[test]
    fn format_search_results_no_matches() {
        let payload = json!({
            "tracks": { "items": [] },
            "albums": { "items": [] },
            "artists": { "items": [] },
            "playlists": { "items": [] }
        });
        format_search_results(&payload);
    }

    #[test]
    fn format_spotify_search_tracks() {
        let payload = json!({
            "tracks": {
                "items": [
                    { "name": "Song One", "artists": [{ "name": "Artist" }] },
                    { "name": "Song Two", "artists": [{ "name": "Band" }] }
                ]
            }
        });
        let mut has_results = false;
        format_spotify_search(&payload, &mut has_results);
        assert!(has_results);
    }

    #[test]
    fn format_spotify_search_albums() {
        let payload = json!({
            "albums": {
                "items": [
                    { "name": "Album One", "artists": [{ "name": "Artist" }] }
                ]
            }
        });
        let mut has_results = false;
        format_spotify_search(&payload, &mut has_results);
        assert!(has_results);
    }

    #[test]
    fn format_spotify_search_artists() {
        let payload = json!({
            "artists": {
                "items": [
                    { "name": "Artist One", "followers": { "total": 1000000 } },
                    { "name": "Artist Two" }
                ]
            }
        });
        let mut has_results = false;
        format_spotify_search(&payload, &mut has_results);
        assert!(has_results);
    }

    #[test]
    fn format_spotify_search_playlists() {
        let payload = json!({
            "playlists": {
                "items": [
                    {
                        "id": "pl123",
                        "name": "My Playlist",
                        "owner": { "display_name": "user123" }
                    },
                    {
                        "id": "pl456",
                        "name": "",
                        "owner": { "id": "user456" }
                    }
                ]
            }
        });
        let mut has_results = false;
        format_spotify_search(&payload, &mut has_results);
        assert!(has_results);
    }

    #[test]
    fn format_spotify_search_playlists_without_id() {
        let payload = json!({
            "playlists": {
                "items": [
                    { "name": "No ID Playlist" }
                ]
            }
        });
        let mut has_results = false;
        format_spotify_search(&payload, &mut has_results);
        assert!(!has_results);
    }

    #[test]
    fn format_spotify_search_all_types() {
        let payload = json!({
            "tracks": { "items": [{ "name": "Track", "artists": [{ "name": "Artist" }] }] },
            "albums": { "items": [{ "name": "Album", "artists": [{ "name": "Artist" }] }] },
            "artists": { "items": [{ "name": "Artist", "followers": { "total": 500 } }] },
            "playlists": { "items": [{ "id": "pl1", "name": "Playlist", "owner": { "display_name": "user" } }] }
        });
        let mut has_results = false;
        format_spotify_search(&payload, &mut has_results);
        assert!(has_results);
    }

    #[test]
    fn format_spotify_search_empty() {
        let payload = json!({});
        let mut has_results = false;
        format_spotify_search(&payload, &mut has_results);
        assert!(!has_results);
    }

    #[test]
    fn format_search_results_nested_spotify() {
        let payload = json!({
            "spotify": {
                "tracks": {
                    "items": [
                        { "name": "Nested Track", "artists": [{ "name": "Artist" }] }
                    ]
                }
            }
        });
        format_search_results(&payload);
    }
}
