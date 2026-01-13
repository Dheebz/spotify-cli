//! Audiobook formatting functions

use serde_json::Value;

use crate::io::common::{format_duration_as, print_table, truncate, DurationFormat};

pub fn format_audiobook_detail(payload: &Value) {
    let name = payload.get("name").and_then(|v| v.as_str()).unwrap_or("Unknown");
    let authors: Vec<&str> = payload
        .get("authors")
        .and_then(|a| a.as_array())
        .map(|arr| arr.iter().filter_map(|a| a.get("name").and_then(|n| n.as_str())).collect())
        .unwrap_or_default();
    let narrators: Vec<&str> = payload
        .get("narrators")
        .and_then(|a| a.as_array())
        .map(|arr| arr.iter().filter_map(|n| n.get("name").and_then(|v| v.as_str())).collect())
        .unwrap_or_default();
    let publisher = payload.get("publisher").and_then(|v| v.as_str()).unwrap_or("Unknown");
    let description = payload.get("description").and_then(|v| v.as_str()).unwrap_or("");
    let total_chapters = payload.get("total_chapters").and_then(|v| v.as_u64()).unwrap_or(0);
    let explicit = payload.get("explicit").and_then(|v| v.as_bool()).unwrap_or(false);
    let uri = payload.get("uri").and_then(|v| v.as_str()).unwrap_or("");

    println!("{}", name);
    if !authors.is_empty() {
        println!("  Author: {}", authors.join(", "));
    }
    if !narrators.is_empty() {
        println!("  Narrator: {}", narrators.join(", "));
    }
    println!("  Publisher: {}", publisher);
    if !description.is_empty() {
        let desc = if description.len() > 200 {
            format!("{}...", &description[..200])
        } else {
            description.to_string()
        };
        println!("  Description: {}", desc);
    }
    println!("  Total Chapters: {}", total_chapters);
    if explicit {
        println!("  Explicit: Yes");
    }
    if !uri.is_empty() {
        println!("  URI: {}", uri);
    }
}

pub fn format_audiobooks(items: &[Value], message: &str) {
    println!("{}:", message);
    println!();

    let rows: Vec<Vec<String>> = items
        .iter()
        .enumerate()
        .map(|(i, item)| {
            // Handle both direct audiobook objects and wrapped objects
            let audiobook = item.get("audiobook").unwrap_or(item);
            let name = audiobook.get("name").and_then(|v| v.as_str()).unwrap_or("Unknown");
            let authors: Vec<&str> = audiobook
                .get("authors")
                .and_then(|a| a.as_array())
                .map(|arr| arr.iter().filter_map(|a| a.get("name").and_then(|n| n.as_str())).collect())
                .unwrap_or_default();
            let chapters = audiobook.get("total_chapters").and_then(|v| v.as_u64()).unwrap_or(0);
            vec![
                (i + 1).to_string(),
                truncate(name, 30),
                truncate(&authors.join(", "), 20),
                chapters.to_string(),
            ]
        })
        .collect();

    print_table("Audiobooks", &["#", "Name", "Author", "Chapters"], &rows, &[3, 30, 20, 8]);
}

pub fn format_audiobook_chapters(items: &[Value], message: &str) {
    println!("{}:", message);
    println!();

    let rows: Vec<Vec<String>> = items
        .iter()
        .enumerate()
        .map(|(i, item)| {
            let name = item.get("name").and_then(|v| v.as_str()).unwrap_or("Unknown");
            let duration_ms = item.get("duration_ms").and_then(|v| v.as_u64()).unwrap_or(0);
            let chapter_number = item.get("chapter_number").and_then(|v| v.as_u64());
            vec![
                chapter_number.map(|n| n.to_string()).unwrap_or_else(|| (i + 1).to_string()),
                truncate(name, 40),
                format_duration_as(duration_ms, DurationFormat::Long),
            ]
        })
        .collect();

    print_table("Chapters", &["#", "Name", "Duration"], &rows, &[3, 40, 10]);
}

