//! Album formatting functions

use serde_json::Value;

use crate::io::common::{extract_artist_names, format_duration};

pub fn format_album_detail(payload: &Value) {
    let name = payload.get("name").and_then(|v| v.as_str()).unwrap_or("Unknown");
    let artists = extract_artist_names(payload);
    let album_type = payload.get("album_type").and_then(|v| v.as_str()).unwrap_or("album");
    let release_date = payload.get("release_date").and_then(|v| v.as_str()).unwrap_or("Unknown");
    let total_tracks = payload.get("total_tracks").and_then(|v| v.as_u64()).unwrap_or(0);
    let popularity = payload.get("popularity").and_then(|v| v.as_u64()).unwrap_or(0);
    let uri = payload.get("uri").and_then(|v| v.as_str()).unwrap_or("");

    println!("{} ({})", name, album_type);
    println!("  Artist: {}", artists);
    println!("  Released: {}", release_date);
    println!("  Tracks: {}", total_tracks);
    println!("  Popularity: {}%", popularity);
    if !uri.is_empty() {
        println!("  URI: {}", uri);
    }

    // Show tracks
    if let Some(tracks) = payload.get("tracks").and_then(|t| t.get("items")).and_then(|i| i.as_array())
        && !tracks.is_empty() {
            println!("\nTracklist:");
            for track in tracks.iter() {
                let track_num = track.get("track_number").and_then(|v| v.as_u64()).unwrap_or(0);
                let track_name = track.get("name").and_then(|v| v.as_str()).unwrap_or("Unknown");
                let duration_ms = track.get("duration_ms").and_then(|v| v.as_u64()).unwrap_or(0);
                let duration = format_duration(duration_ms);
                println!("  {}. {} [{}]", track_num, track_name, duration);
            }
        }
}
