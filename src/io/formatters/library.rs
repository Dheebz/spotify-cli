//! Library formatting functions

use serde_json::Value;

use crate::io::common::extract_artist_names;

pub fn format_saved_tracks(items: &[Value]) {
    println!("Saved Tracks:");
    for (i, item) in items.iter().take(20).enumerate() {
        if let Some(track) = item.get("track") {
            let name = track.get("name").and_then(|v| v.as_str()).unwrap_or("Unknown");
            let artists = extract_artist_names(track);
            println!("  {}. {} - {}", i + 1, name, artists);
        }
    }
}

pub fn format_library_check(results: &[Value]) {
    println!("Library Check:");
    for (i, result) in results.iter().enumerate() {
        let saved = result.as_bool().unwrap_or(false);
        let status = if saved { "Saved" } else { "Not saved" };
        println!("  Track {}: {}", i + 1, status);
    }
}
