//! Track formatting functions

use serde_json::Value;

use crate::io::common::{extract_artist_names, format_duration, print_table, truncate};

pub fn format_track_detail(payload: &Value) {
    let name = payload.get("name").and_then(|v| v.as_str()).unwrap_or("Unknown");
    let artists = extract_artist_names(payload);
    let album = payload
        .get("album")
        .and_then(|a| a.get("name"))
        .and_then(|v| v.as_str())
        .unwrap_or("Unknown");
    let duration_ms = payload.get("duration_ms").and_then(|v| v.as_u64()).unwrap_or(0);
    let duration = format_duration(duration_ms);
    let popularity = payload.get("popularity").and_then(|v| v.as_u64()).unwrap_or(0);
    let explicit = payload.get("explicit").and_then(|v| v.as_bool()).unwrap_or(false);
    let track_number = payload.get("track_number").and_then(|v| v.as_u64());
    let uri = payload.get("uri").and_then(|v| v.as_str()).unwrap_or("");

    println!("{}", name);
    println!("  Artist: {}", artists);
    println!("  Album: {}", album);
    println!("  Duration: {}", duration);
    if let Some(track_num) = track_number {
        println!("  Track: #{}", track_num);
    }
    println!("  Popularity: {}%", popularity);
    if explicit {
        println!("  Explicit: Yes");
    }
    if !uri.is_empty() {
        println!("  URI: {}", uri);
    }
}

pub fn format_play_history(items: &[Value]) {
    println!("Recently Played:");
    for (i, item) in items.iter().take(10).enumerate() {
        if let Some(track) = item.get("track") {
            let name = track.get("name").and_then(|v| v.as_str()).unwrap_or("Unknown");
            let artists = extract_artist_names(track);
            println!("  {}. {} - {}", i + 1, name, artists);
        }
    }
}

pub fn format_top_tracks(tracks: &[Value], message: &str) {
    println!("{}:", message);
    println!();
    print_table(
        "Your Top Tracks",
        &["#", "Title", "Artist", "Album"],
        &tracks
            .iter()
            .enumerate()
            .map(|(i, track)| {
                let name = track
                    .get("name")
                    .and_then(|v| v.as_str())
                    .unwrap_or("Unknown");
                let artists = extract_artist_names(track);
                let album = track
                    .get("album")
                    .and_then(|a| a.get("name"))
                    .and_then(|v| v.as_str())
                    .unwrap_or("Unknown");
                vec![
                    (i + 1).to_string(),
                    truncate(name, 25),
                    truncate(&artists, 18),
                    truncate(album, 20),
                ]
            })
            .collect::<Vec<_>>(),
        &[3, 25, 18, 20],
    );
}

pub fn format_artist_top_tracks(tracks: &[Value]) {
    if tracks.is_empty() {
        println!("No top tracks found.");
        return;
    }
    println!("Top Tracks:");
    for (i, track) in tracks.iter().enumerate() {
        let name = track.get("name").and_then(|v| v.as_str()).unwrap_or("Unknown");
        let album = track
            .get("album")
            .and_then(|a| a.get("name"))
            .and_then(|v| v.as_str())
            .unwrap_or("Unknown");
        let popularity = track.get("popularity").and_then(|v| v.as_u64()).unwrap_or(0);
        println!("  {}. {} - {} ({}%)", i + 1, name, album, popularity);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn format_track_detail_full() {
        let payload = json!({
            "name": "Test Track",
            "artists": [{ "name": "Test Artist" }],
            "album": { "name": "Test Album" },
            "duration_ms": 210000,
            "popularity": 75,
            "explicit": true,
            "track_number": 5,
            "uri": "spotify:track:abc123"
        });
        format_track_detail(&payload);
    }

    #[test]
    fn format_track_detail_minimal() {
        let payload = json!({});
        format_track_detail(&payload);
    }

    #[test]
    fn format_track_detail_without_explicit() {
        let payload = json!({
            "name": "Clean Track",
            "artists": [{ "name": "Artist" }],
            "album": { "name": "Album" },
            "duration_ms": 180000,
            "popularity": 50,
            "explicit": false
        });
        format_track_detail(&payload);
    }

    #[test]
    fn format_track_detail_multiple_artists() {
        let payload = json!({
            "name": "Collab Track",
            "artists": [
                { "name": "Artist One" },
                { "name": "Artist Two" },
                { "name": "Artist Three" }
            ],
            "album": { "name": "Album" },
            "duration_ms": 240000
        });
        format_track_detail(&payload);
    }

    #[test]
    fn format_play_history_with_tracks() {
        let items = vec![
            json!({ "track": { "name": "Track 1", "artists": [{ "name": "Artist 1" }] } }),
            json!({ "track": { "name": "Track 2", "artists": [{ "name": "Artist 2" }] } }),
        ];
        format_play_history(&items);
    }

    #[test]
    fn format_play_history_empty() {
        let items: Vec<Value> = vec![];
        format_play_history(&items);
    }

    #[test]
    fn format_play_history_more_than_ten() {
        let items: Vec<Value> = (0..15)
            .map(|i| json!({ "track": { "name": format!("Track {}", i), "artists": [{ "name": "Artist" }] } }))
            .collect();
        format_play_history(&items);
    }

    #[test]
    fn format_top_tracks_with_data() {
        let tracks = vec![
            json!({
                "name": "Popular Song",
                "artists": [{ "name": "Famous Artist" }],
                "album": { "name": "Hit Album" }
            }),
            json!({
                "name": "Another Hit",
                "artists": [{ "name": "Another Artist" }],
                "album": { "name": "Great Album" }
            }),
        ];
        format_top_tracks(&tracks, "Top Tracks This Month");
    }

    #[test]
    fn format_top_tracks_empty() {
        let tracks: Vec<Value> = vec![];
        format_top_tracks(&tracks, "No Tracks Found");
    }

    #[test]
    fn format_artist_top_tracks_with_data() {
        let tracks = vec![
            json!({
                "name": "Best Song",
                "album": { "name": "Greatest Hits" },
                "popularity": 95
            }),
            json!({
                "name": "Second Best",
                "album": { "name": "New Album" },
                "popularity": 80
            }),
        ];
        format_artist_top_tracks(&tracks);
    }

    #[test]
    fn format_artist_top_tracks_empty() {
        let tracks: Vec<Value> = vec![];
        format_artist_top_tracks(&tracks);
    }

    #[test]
    fn format_artist_top_tracks_minimal_data() {
        let tracks = vec![json!({})];
        format_artist_top_tracks(&tracks);
    }
}
