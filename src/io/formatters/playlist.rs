//! Playlist formatting functions

use serde_json::Value;

use crate::io::common::{extract_artist_names, format_number};

pub fn format_playlists(items: &[Value]) {
    println!("Your Playlists:");
    for (i, item) in items.iter().enumerate() {
        let name = item.get("name").and_then(|v| v.as_str()).unwrap_or("Unknown");
        let tracks = item
            .get("tracks")
            .and_then(|t| t.get("total"))
            .and_then(|v| v.as_u64())
            .unwrap_or(0);
        let owner = item
            .get("owner")
            .and_then(|o| o.get("display_name"))
            .and_then(|v| v.as_str())
            .unwrap_or("Unknown");

        println!("  {}. {} ({} tracks) - {}", i + 1, name, tracks, owner);
    }
}

pub fn format_playlist_detail(payload: &Value) {
    let name = payload.get("name").and_then(|v| v.as_str()).unwrap_or("Unknown");
    let owner = payload
        .get("owner")
        .and_then(|o| o.get("display_name"))
        .and_then(|v| v.as_str())
        .unwrap_or("Unknown");
    let description = payload
        .get("description")
        .and_then(|v| v.as_str())
        .filter(|s| !s.is_empty());
    let total = payload
        .get("tracks")
        .and_then(|t| t.get("total"))
        .and_then(|v| v.as_u64())
        .unwrap_or(0);
    let followers = payload
        .get("followers")
        .and_then(|f| f.get("total"))
        .and_then(|v| v.as_u64())
        .unwrap_or(0);

    println!("{}", name);
    println!("  by {} | {} tracks | {} followers", owner, total, format_number(followers));
    if let Some(desc) = description {
        println!("  {}", desc);
    }

    // Show first few tracks if available
    if let Some(tracks) = payload.get("tracks").and_then(|t| t.get("items")).and_then(|i| i.as_array())
        && !tracks.is_empty() {
            println!("\nTracks:");
            for (i, item) in tracks.iter().take(10).enumerate() {
                if let Some(track) = item.get("track") {
                    let track_name = track.get("name").and_then(|v| v.as_str()).unwrap_or("Unknown");
                    let artists = extract_artist_names(track);
                    println!("  {}. {} - {}", i + 1, track_name, artists);
                }
            }
            if tracks.len() > 10 {
                println!("  ... and {} more", total as usize - 10);
            }
        }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn format_playlists_with_items() {
        let items = vec![
            json!({
                "name": "My Playlist",
                "tracks": { "total": 50 },
                "owner": { "display_name": "user123" }
            }),
            json!({
                "name": "Another Playlist",
                "tracks": { "total": 100 },
                "owner": { "display_name": "user456" }
            }),
        ];
        format_playlists(&items);
    }

    #[test]
    fn format_playlists_empty() {
        let items: Vec<Value> = vec![];
        format_playlists(&items);
    }

    #[test]
    fn format_playlists_minimal_data() {
        let items = vec![json!({})];
        format_playlists(&items);
    }

    #[test]
    fn format_playlist_detail_full() {
        let payload = json!({
            "name": "Test Playlist",
            "owner": { "display_name": "Test User" },
            "description": "A great playlist for testing",
            "tracks": {
                "total": 25,
                "items": [
                    { "track": { "name": "Track 1", "artists": [{ "name": "Artist 1" }] } },
                    { "track": { "name": "Track 2", "artists": [{ "name": "Artist 2" }] } }
                ]
            },
            "followers": { "total": 5000 }
        });
        format_playlist_detail(&payload);
    }

    #[test]
    fn format_playlist_detail_minimal() {
        let payload = json!({});
        format_playlist_detail(&payload);
    }

    #[test]
    fn format_playlist_detail_empty_description() {
        let payload = json!({
            "name": "Playlist",
            "description": "",
            "owner": { "display_name": "User" }
        });
        format_playlist_detail(&payload);
    }

    #[test]
    fn format_playlist_detail_many_tracks() {
        let tracks: Vec<Value> = (0..15)
            .map(|i| json!({ "track": { "name": format!("Track {}", i), "artists": [{ "name": "Artist" }] } }))
            .collect();
        let payload = json!({
            "name": "Big Playlist",
            "owner": { "display_name": "User" },
            "tracks": { "total": 100, "items": tracks },
            "followers": { "total": 1000000 }
        });
        format_playlist_detail(&payload);
    }

    #[test]
    fn format_playlist_detail_no_tracks() {
        let payload = json!({
            "name": "Empty Playlist",
            "owner": { "display_name": "User" },
            "tracks": { "total": 0, "items": [] }
        });
        format_playlist_detail(&payload);
    }
}
