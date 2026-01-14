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

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn format_chapter_detail_full() {
        let payload = json!({
            "name": "Chapter 1: The Beginning",
            "audiobook": { "name": "Great Audiobook" },
            "description": "In this chapter we explore the beginning",
            "duration_ms": 1800000,
            "chapter_number": 1,
            "uri": "spotify:chapter:abc123"
        });
        format_chapter_detail(&payload);
    }

    #[test]
    fn format_chapter_detail_minimal() {
        let payload = json!({});
        format_chapter_detail(&payload);
    }

    #[test]
    fn format_chapter_detail_long_description() {
        let long_desc = "A".repeat(300);
        let payload = json!({
            "name": "Chapter",
            "audiobook": { "name": "Book" },
            "description": long_desc,
            "duration_ms": 600000
        });
        format_chapter_detail(&payload);
    }

    #[test]
    fn format_chapter_detail_no_chapter_number() {
        let payload = json!({
            "name": "Prologue",
            "audiobook": { "name": "Mystery Book" },
            "duration_ms": 300000
        });
        format_chapter_detail(&payload);
    }

    #[test]
    fn format_chapter_detail_short_duration() {
        let payload = json!({
            "name": "Short Chapter",
            "duration_ms": 45000,
            "chapter_number": 5
        });
        format_chapter_detail(&payload);
    }

    #[test]
    fn format_chapter_detail_no_uri() {
        let payload = json!({
            "name": "Chapter Without URI",
            "audiobook": { "name": "Book" },
            "duration_ms": 900000
        });
        format_chapter_detail(&payload);
    }
}
