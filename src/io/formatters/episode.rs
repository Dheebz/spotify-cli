//! Episode formatting functions

use serde_json::Value;

use crate::io::common::{format_duration_as, print_table, truncate, DurationFormat};

pub fn format_episode_detail(payload: &Value) {
    let name = payload.get("name").and_then(|v| v.as_str()).unwrap_or("Unknown");
    let show_name = payload
        .get("show")
        .and_then(|s| s.get("name"))
        .and_then(|v| v.as_str())
        .unwrap_or("Unknown");
    let description = payload.get("description").and_then(|v| v.as_str()).unwrap_or("");
    let duration_ms = payload.get("duration_ms").and_then(|v| v.as_u64()).unwrap_or(0);
    let release_date = payload.get("release_date").and_then(|v| v.as_str()).unwrap_or("");
    let explicit = payload.get("explicit").and_then(|v| v.as_bool()).unwrap_or(false);
    let uri = payload.get("uri").and_then(|v| v.as_str()).unwrap_or("");

    println!("{}", name);
    println!("  Show: {}", show_name);
    if !description.is_empty() {
        let desc = if description.len() > 200 {
            format!("{}...", &description[..200])
        } else {
            description.to_string()
        };
        println!("  Description: {}", desc);
    }
    println!("  Duration: {}", format_duration_as(duration_ms, DurationFormat::Long));
    if !release_date.is_empty() {
        println!("  Released: {}", release_date);
    }
    if explicit {
        println!("  Explicit: Yes");
    }
    if !uri.is_empty() {
        println!("  URI: {}", uri);
    }
}

pub fn format_episodes(items: &[Value], message: &str) {
    println!("{}:", message);
    println!();

    let rows: Vec<Vec<String>> = items
        .iter()
        .enumerate()
        .map(|(i, item)| {
            // Handle both direct episode objects and wrapped {"episode": ...} objects
            let episode = item.get("episode").unwrap_or(item);
            let name = episode.get("name").and_then(|v| v.as_str()).unwrap_or("Unknown");
            let show = episode
                .get("show")
                .and_then(|s| s.get("name"))
                .and_then(|v| v.as_str())
                .unwrap_or("Unknown");
            let duration_ms = episode.get("duration_ms").and_then(|v| v.as_u64()).unwrap_or(0);
            vec![
                (i + 1).to_string(),
                truncate(name, 30),
                truncate(show, 20),
                format_duration_as(duration_ms, DurationFormat::Long),
            ]
        })
        .collect();

    print_table("Episodes", &["#", "Name", "Show", "Duration"], &rows, &[3, 30, 20, 10]);
}

