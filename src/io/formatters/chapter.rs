//! Chapter formatting functions

use serde_json::Value;

use crate::io::common::{format_duration_as, DurationFormat};

pub fn format_chapter_detail(payload: &Value) {
    let name = payload.get("name").and_then(|v| v.as_str()).unwrap_or("Unknown");
    let audiobook_name = payload
        .get("audiobook")
        .and_then(|a| a.get("name"))
        .and_then(|v| v.as_str())
        .unwrap_or("Unknown");
    let description = payload.get("description").and_then(|v| v.as_str()).unwrap_or("");
    let duration_ms = payload.get("duration_ms").and_then(|v| v.as_u64()).unwrap_or(0);
    let chapter_number = payload.get("chapter_number").and_then(|v| v.as_u64());
    let uri = payload.get("uri").and_then(|v| v.as_str()).unwrap_or("");

    println!("{}", name);
    println!("  Audiobook: {}", audiobook_name);
    if let Some(num) = chapter_number {
        println!("  Chapter: {}", num);
    }
    println!("  Duration: {}", format_duration_as(duration_ms, DurationFormat::LongWithSeconds));
    if !description.is_empty() {
        let desc = if description.len() > 200 {
            format!("{}...", &description[..200])
        } else {
            description.to_string()
        };
        println!("  Description: {}", desc);
    }
    if !uri.is_empty() {
        println!("  URI: {}", uri);
    }
}

