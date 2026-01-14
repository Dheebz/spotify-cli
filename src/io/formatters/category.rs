//! Category formatting functions

use serde_json::Value;

use crate::io::common::print_table;

pub fn format_categories(items: &[Value]) {
    let rows: Vec<Vec<String>> = items
        .iter()
        .enumerate()
        .map(|(i, cat)| {
            let name = cat.get("name").and_then(|v| v.as_str()).unwrap_or("Unknown");
            let id = cat.get("id").and_then(|v| v.as_str()).unwrap_or("");
            vec![(i + 1).to_string(), name.to_string(), id.to_string()]
        })
        .collect();
    print_table("Categories", &["#", "Name", "ID"], &rows, &[3, 30, 20]);
}

pub fn format_category_detail(payload: &Value) {
    let name = payload.get("name").and_then(|v| v.as_str()).unwrap_or("Unknown");
    let id = payload.get("id").and_then(|v| v.as_str()).unwrap_or("");

    println!("{}", name);
    println!("  ID: {}", id);

    if let Some(icons) = payload.get("icons").and_then(|i| i.as_array())
        && let Some(first) = icons.first()
            && let Some(url) = first.get("url").and_then(|v| v.as_str()) {
                println!("  Icon: {}", url);
            }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn format_categories_with_items() {
        let items = vec![
            json!({ "name": "Pop", "id": "pop" }),
            json!({ "name": "Rock", "id": "rock" }),
            json!({ "name": "Hip-Hop", "id": "hiphop" }),
        ];
        format_categories(&items);
    }

    #[test]
    fn format_categories_empty() {
        let items: Vec<Value> = vec![];
        format_categories(&items);
    }

    #[test]
    fn format_categories_minimal() {
        let items = vec![json!({})];
        format_categories(&items);
    }

    #[test]
    fn format_category_detail_full() {
        let payload = json!({
            "name": "Top Lists",
            "id": "toplists",
            "icons": [{ "url": "https://example.com/icon.png" }]
        });
        format_category_detail(&payload);
    }

    #[test]
    fn format_category_detail_minimal() {
        let payload = json!({});
        format_category_detail(&payload);
    }

    #[test]
    fn format_category_detail_no_icons() {
        let payload = json!({
            "name": "Mood",
            "id": "mood"
        });
        format_category_detail(&payload);
    }

    #[test]
    fn format_category_detail_empty_icons() {
        let payload = json!({
            "name": "Focus",
            "id": "focus",
            "icons": []
        });
        format_category_detail(&payload);
    }
}
