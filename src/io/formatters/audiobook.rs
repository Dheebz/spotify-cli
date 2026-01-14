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

    // Filter out ghost entries (audiobooks that are no longer available)
    let valid_items: Vec<_> = items
        .iter()
        .filter(|item| {
            let audiobook = item.get("audiobook").unwrap_or(*item);
            // Check if the audiobook has a valid id (non-null)
            audiobook.get("id").and_then(|v| v.as_str()).is_some()
        })
        .collect();

    let rows: Vec<Vec<String>> = valid_items
        .iter()
        .enumerate()
        .map(|(i, item)| {
            // Handle both direct audiobook objects and wrapped objects
            let audiobook = item.get("audiobook").unwrap_or(*item);
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

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn format_audiobook_detail_full() {
        let payload = json!({
            "name": "The Great Audiobook",
            "authors": [{ "name": "Author One" }, { "name": "Author Two" }],
            "narrators": [{ "name": "Narrator One" }],
            "publisher": "Test Publisher",
            "description": "A fascinating audiobook about testing",
            "total_chapters": 25,
            "explicit": true,
            "uri": "spotify:audiobook:abc123"
        });
        format_audiobook_detail(&payload);
    }

    #[test]
    fn format_audiobook_detail_minimal() {
        let payload = json!({});
        format_audiobook_detail(&payload);
    }

    #[test]
    fn format_audiobook_detail_long_description() {
        let long_desc = "A".repeat(300);
        let payload = json!({
            "name": "Audiobook",
            "description": long_desc
        });
        format_audiobook_detail(&payload);
    }

    #[test]
    fn format_audiobook_detail_no_authors() {
        let payload = json!({
            "name": "Anonymous Audiobook",
            "publisher": "Publisher",
            "total_chapters": 10
        });
        format_audiobook_detail(&payload);
    }

    #[test]
    fn format_audiobooks_with_items() {
        let items = vec![
            json!({
                "name": "Audiobook One",
                "authors": [{ "name": "Author A" }],
                "total_chapters": 15
            }),
            json!({
                "name": "Audiobook Two",
                "authors": [{ "name": "Author B" }, { "name": "Author C" }],
                "total_chapters": 30
            }),
        ];
        format_audiobooks(&items, "Your Audiobooks");
    }

    #[test]
    fn format_audiobooks_empty() {
        let items: Vec<Value> = vec![];
        format_audiobooks(&items, "No Audiobooks");
    }

    #[test]
    fn format_audiobooks_wrapped() {
        let items = vec![json!({
            "audiobook": {
                "name": "Wrapped Audiobook",
                "authors": [{ "name": "Author" }],
                "total_chapters": 20
            }
        })];
        format_audiobooks(&items, "Saved Audiobooks");
    }

    #[test]
    fn format_audiobook_chapters_with_items() {
        let items = vec![
            json!({
                "name": "Chapter 1: Introduction",
                "duration_ms": 600000,
                "chapter_number": 1
            }),
            json!({
                "name": "Chapter 2: The Beginning",
                "duration_ms": 1200000,
                "chapter_number": 2
            }),
        ];
        format_audiobook_chapters(&items, "Chapters");
    }

    #[test]
    fn format_audiobook_chapters_empty() {
        let items: Vec<Value> = vec![];
        format_audiobook_chapters(&items, "No Chapters");
    }

    #[test]
    fn format_audiobook_chapters_no_chapter_number() {
        let items = vec![
            json!({ "name": "Prologue", "duration_ms": 300000 }),
            json!({ "name": "Epilogue", "duration_ms": 180000 }),
        ];
        format_audiobook_chapters(&items, "Chapters");
    }
}
