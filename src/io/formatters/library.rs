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

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn format_saved_tracks_with_items() {
        let items = vec![
            json!({ "track": { "name": "Track 1", "artists": [{ "name": "Artist 1" }] } }),
            json!({ "track": { "name": "Track 2", "artists": [{ "name": "Artist 2" }] } }),
        ];
        format_saved_tracks(&items);
    }

    #[test]
    fn format_saved_tracks_empty() {
        let items: Vec<Value> = vec![];
        format_saved_tracks(&items);
    }

    #[test]
    fn format_saved_tracks_more_than_twenty() {
        let items: Vec<Value> = (0..25)
            .map(|i| json!({ "track": { "name": format!("Track {}", i), "artists": [{ "name": "Artist" }] } }))
            .collect();
        format_saved_tracks(&items);
    }

    #[test]
    fn format_saved_tracks_no_track_field() {
        let items = vec![json!({ "name": "Direct Item" })];
        format_saved_tracks(&items);
    }

    #[test]
    fn format_library_check_mixed() {
        let results = vec![
            json!(true),
            json!(false),
            json!(true),
        ];
        format_library_check(&results);
    }

    #[test]
    fn format_library_check_empty() {
        let results: Vec<Value> = vec![];
        format_library_check(&results);
    }

    #[test]
    fn format_library_check_all_saved() {
        let results = vec![json!(true), json!(true)];
        format_library_check(&results);
    }

    #[test]
    fn format_library_check_none_saved() {
        let results = vec![json!(false), json!(false)];
        format_library_check(&results);
    }

    #[test]
    fn format_library_check_non_bool() {
        let results = vec![json!("invalid"), json!(null)];
        format_library_check(&results);
    }
}
