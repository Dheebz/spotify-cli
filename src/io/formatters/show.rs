//! Show (podcast) formatting functions

use serde_json::Value;

use crate::io::common::{format_duration_as, print_table, truncate, DurationFormat};

pub fn format_show_detail(payload: &Value) {
    let name = payload.get("name").and_then(|v| v.as_str()).unwrap_or("Unknown");
    let publisher = payload.get("publisher").and_then(|v| v.as_str()).unwrap_or("Unknown");
    let description = payload.get("description").and_then(|v| v.as_str()).unwrap_or("");
    let total_episodes = payload.get("total_episodes").and_then(|v| v.as_u64()).unwrap_or(0);
    let explicit = payload.get("explicit").and_then(|v| v.as_bool()).unwrap_or(false);
    let uri = payload.get("uri").and_then(|v| v.as_str()).unwrap_or("");

    println!("{}", name);
    println!("  Publisher: {}", publisher);
    if !description.is_empty() {
        let desc = if description.len() > 200 {
            format!("{}...", &description[..200])
        } else {
            description.to_string()
        };
        println!("  Description: {}", desc);
    }
    println!("  Total Episodes: {}", total_episodes);
    if explicit {
        println!("  Explicit: Yes");
    }
    if !uri.is_empty() {
        println!("  URI: {}", uri);
    }
}

pub fn format_shows(items: &[Value], message: &str) {
    println!("{}:", message);
    println!();

    let rows: Vec<Vec<String>> = items
        .iter()
        .enumerate()
        .map(|(i, item)| {
            // Handle both direct show objects and wrapped {"show": ...} objects
            let show = item.get("show").unwrap_or(item);
            let name = show.get("name").and_then(|v| v.as_str()).unwrap_or("Unknown");
            let publisher = show.get("publisher").and_then(|v| v.as_str()).unwrap_or("Unknown");
            let episodes = show.get("total_episodes").and_then(|v| v.as_u64()).unwrap_or(0);
            vec![
                (i + 1).to_string(),
                truncate(name, 30),
                truncate(publisher, 20),
                episodes.to_string(),
            ]
        })
        .collect();

    print_table("Shows", &["#", "Name", "Publisher", "Episodes"], &rows, &[3, 30, 20, 8]);
}

pub fn format_show_episodes(items: &[Value], message: &str) {
    println!("{}:", message);
    println!();

    let rows: Vec<Vec<String>> = items
        .iter()
        .enumerate()
        .map(|(i, item)| {
            let name = item.get("name").and_then(|v| v.as_str()).unwrap_or("Unknown");
            let duration_ms = item.get("duration_ms").and_then(|v| v.as_u64()).unwrap_or(0);
            let duration = format_duration_as(duration_ms, DurationFormat::Long);
            let release = item.get("release_date").and_then(|v| v.as_str()).unwrap_or("");
            vec![
                (i + 1).to_string(),
                truncate(name, 35),
                duration,
                release.to_string(),
            ]
        })
        .collect();

    print_table("Episodes", &["#", "Name", "Duration", "Released"], &rows, &[3, 35, 10, 12]);
}

