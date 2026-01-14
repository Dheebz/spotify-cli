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

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn format_episode_detail_full() {
        let payload = json!({
            "name": "Episode Title",
            "show": { "name": "Podcast Name" },
            "description": "A detailed description of this episode",
            "duration_ms": 3600000,
            "release_date": "2024-01-15",
            "explicit": true,
            "uri": "spotify:episode:abc123"
        });
        format_episode_detail(&payload);
    }

    #[test]
    fn format_episode_detail_minimal() {
        let payload = json!({});
        format_episode_detail(&payload);
    }

    #[test]
    fn format_episode_detail_long_description() {
        let long_desc = "A".repeat(300);
        let payload = json!({
            "name": "Episode",
            "show": { "name": "Show" },
            "description": long_desc,
            "duration_ms": 1800000
        });
        format_episode_detail(&payload);
    }

    #[test]
    fn format_episode_detail_not_explicit() {
        let payload = json!({
            "name": "Family Episode",
            "show": { "name": "Family Show" },
            "explicit": false,
            "duration_ms": 900000
        });
        format_episode_detail(&payload);
    }

    #[test]
    fn format_episodes_with_items() {
        let items = vec![
            json!({
                "name": "Episode One",
                "show": { "name": "Podcast A" },
                "duration_ms": 3600000
            }),
            json!({
                "name": "Episode Two",
                "show": { "name": "Podcast B" },
                "duration_ms": 1800000
            }),
        ];
        format_episodes(&items, "Recent Episodes");
    }

    #[test]
    fn format_episodes_empty() {
        let items: Vec<Value> = vec![];
        format_episodes(&items, "No Episodes");
    }

    #[test]
    fn format_episodes_wrapped() {
        let items = vec![json!({
            "episode": {
                "name": "Wrapped Episode",
                "show": { "name": "Show" },
                "duration_ms": 2700000
            }
        })];
        format_episodes(&items, "Saved Episodes");
    }

    #[test]
    fn format_episodes_minimal() {
        let items = vec![json!({})];
        format_episodes(&items, "Episodes");
    }
}
