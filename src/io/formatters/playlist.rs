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
