//! Player-related formatting functions

use serde_json::Value;

use crate::io::common::{extract_artist_names, format_duration};

pub fn format_player_status(payload: &Value, item: &Value) {
    let is_playing = payload
        .get("is_playing")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);

    let track_name = item.get("name").and_then(|v| v.as_str()).unwrap_or("Unknown");
    let artists = extract_artist_names(item);

    let album = item
        .get("album")
        .and_then(|a| a.get("name"))
        .and_then(|v| v.as_str())
        .unwrap_or("Unknown");

    let progress_ms = payload.get("progress_ms").and_then(|v| v.as_u64()).unwrap_or(0);
    let duration_ms = item.get("duration_ms").and_then(|v| v.as_u64()).unwrap_or(0);

    let progress = format_duration(progress_ms);
    let duration = format_duration(duration_ms);

    let status_icon = if is_playing { "▶" } else { "⏸" };

    println!("{} {} - {}", status_icon, track_name, artists);
    println!("  Album: {}", album);
    println!("  Progress: {} / {}", progress, duration);

    if let Some(device) = payload.get("device") {
        let device_name = device.get("name").and_then(|v| v.as_str()).unwrap_or("Unknown");
        let volume = device.get("volume_percent").and_then(|v| v.as_u64()).unwrap_or(0);
        println!("  Device: {} ({}%)", device_name, volume);
    }

    let shuffle = payload.get("shuffle_state").and_then(|v| v.as_bool()).unwrap_or(false);
    let repeat = payload.get("repeat_state").and_then(|v| v.as_str()).unwrap_or("off");
    if shuffle || repeat != "off" {
        let mut modes = vec![];
        if shuffle {
            modes.push("shuffle");
        }
        if repeat != "off" {
            modes.push(repeat);
        }
        println!("  Mode: {}", modes.join(", "));
    }
}

pub fn format_queue(payload: &Value) {
    if let Some(current) = payload.get("currently_playing")
        && !current.is_null() {
            let name = current.get("name").and_then(|v| v.as_str()).unwrap_or("Unknown");
            let artists = extract_artist_names(current);
            println!("Now Playing: {} - {}", name, artists);
        }

    if let Some(queue) = payload.get("queue").and_then(|q| q.as_array()) {
        if queue.is_empty() {
            println!("Queue is empty.");
        } else {
            println!("Up Next:");
            for (i, track) in queue.iter().take(10).enumerate() {
                let name = track.get("name").and_then(|v| v.as_str()).unwrap_or("Unknown");
                let artists = extract_artist_names(track);
                println!("  {}. {} - {}", i + 1, name, artists);
            }
            if queue.len() > 10 {
                println!("  ... and {} more", queue.len() - 10);
            }
        }
    }
}

pub fn format_devices(devices: &[Value]) {
    if devices.is_empty() {
        println!("No devices available.");
        return;
    }
    println!("Available Devices:");
    for device in devices {
        let name = device.get("name").and_then(|v| v.as_str()).unwrap_or("Unknown");
        let dtype = device.get("type").and_then(|v| v.as_str()).unwrap_or("Unknown");
        let active = device.get("is_active").and_then(|v| v.as_bool()).unwrap_or(false);
        let volume = device.get("volume_percent").and_then(|v| v.as_u64());

        let active_marker = if active { " *" } else { "" };
        let vol_str = volume.map(|v| format!(" ({}%)", v)).unwrap_or_default();

        println!("  [{}] {}{}{}", dtype, name, vol_str, active_marker);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn format_player_status_playing() {
        let payload = json!({
            "is_playing": true,
            "progress_ms": 60000,
            "device": { "name": "My Speaker", "volume_percent": 75 },
            "shuffle_state": false,
            "repeat_state": "off"
        });
        let item = json!({
            "name": "Test Track",
            "artists": [{ "name": "Test Artist" }],
            "album": { "name": "Test Album" },
            "duration_ms": 180000
        });
        format_player_status(&payload, &item);
    }

    #[test]
    fn format_player_status_paused() {
        let payload = json!({
            "is_playing": false,
            "progress_ms": 30000
        });
        let item = json!({
            "name": "Paused Track",
            "artists": [{ "name": "Artist" }],
            "album": { "name": "Album" },
            "duration_ms": 200000
        });
        format_player_status(&payload, &item);
    }

    #[test]
    fn format_player_status_with_shuffle_repeat() {
        let payload = json!({
            "is_playing": true,
            "progress_ms": 0,
            "shuffle_state": true,
            "repeat_state": "track"
        });
        let item = json!({
            "name": "Track",
            "artists": [{ "name": "Artist" }],
            "duration_ms": 120000
        });
        format_player_status(&payload, &item);
    }

    #[test]
    fn format_player_status_minimal() {
        let payload = json!({});
        let item = json!({});
        format_player_status(&payload, &item);
    }

    #[test]
    fn format_queue_with_items() {
        let payload = json!({
            "currently_playing": {
                "name": "Current Song",
                "artists": [{ "name": "Current Artist" }]
            },
            "queue": [
                { "name": "Next Song 1", "artists": [{ "name": "Artist 1" }] },
                { "name": "Next Song 2", "artists": [{ "name": "Artist 2" }] }
            ]
        });
        format_queue(&payload);
    }

    #[test]
    fn format_queue_empty() {
        let payload = json!({
            "currently_playing": null,
            "queue": []
        });
        format_queue(&payload);
    }

    #[test]
    fn format_queue_large() {
        let queue: Vec<Value> = (0..15)
            .map(|i| json!({ "name": format!("Track {}", i), "artists": [{ "name": "Artist" }] }))
            .collect();
        let payload = json!({
            "currently_playing": { "name": "Current", "artists": [{ "name": "Artist" }] },
            "queue": queue
        });
        format_queue(&payload);
    }

    #[test]
    fn format_queue_no_currently_playing() {
        let payload = json!({
            "queue": [
                { "name": "Queued Track", "artists": [{ "name": "Artist" }] }
            ]
        });
        format_queue(&payload);
    }

    #[test]
    fn format_devices_multiple() {
        let devices = vec![
            json!({
                "name": "Living Room Speaker",
                "type": "Speaker",
                "is_active": true,
                "volume_percent": 80
            }),
            json!({
                "name": "Phone",
                "type": "Smartphone",
                "is_active": false,
                "volume_percent": 50
            }),
        ];
        format_devices(&devices);
    }

    #[test]
    fn format_devices_empty() {
        let devices: Vec<Value> = vec![];
        format_devices(&devices);
    }

    #[test]
    fn format_devices_without_volume() {
        let devices = vec![json!({
            "name": "TV",
            "type": "TV",
            "is_active": false
        })];
        format_devices(&devices);
    }

    #[test]
    fn format_devices_minimal() {
        let devices = vec![json!({})];
        format_devices(&devices);
    }
}
