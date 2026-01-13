//! Search result filtering and processing helpers

use crate::endpoints::search::SEARCH_RESULT_KEYS;
use crate::storage::pins::PinStore;
use serde_json::Value;

/// Filter out ghost entries (entries without valid IDs)
pub fn filter_ghost_entries(data: &mut Value) {
    for result_type in SEARCH_RESULT_KEYS {
        if let Some(container) = data.get_mut(result_type)
            && let Some(items) = container.get_mut("items")
                && let Some(arr) = items.as_array_mut() {
                    arr.retain(|item| item.get("id").and_then(|v| v.as_str()).is_some());
                }
    }
}

/// Filter to only keep exact matches (name contains query)
pub fn filter_exact_matches(data: &mut Value, query: &str) {
    let query_lower = query.to_lowercase();

    for result_type in SEARCH_RESULT_KEYS {
        if let Some(container) = data.get_mut(result_type)
            && let Some(items) = container.get_mut("items")
                && let Some(arr) = items.as_array_mut() {
                    arr.retain(|item| {
                        item.get("name")
                            .and_then(|v| v.as_str())
                            .map(|name| name.to_lowercase().contains(&query_lower))
                            .unwrap_or(false)
                    });
                }
    }
}

/// Extract the first playable URI from pins or spotify results
pub fn extract_first_uri(pins: &[Value], spotify: &Value) -> Option<String> {
    if let Some(first_pin) = pins.first()
        && let Some(uri) = first_pin.get("uri").and_then(|v| v.as_str()) {
            return Some(uri.to_string());
        }

    for result_type in SEARCH_RESULT_KEYS {
        if let Some(items) = spotify
            .get(result_type)
            .and_then(|t| t.get("items"))
            .and_then(|i| i.as_array())
            && let Some(first) = items.first()
                && let Some(uri) = first.get("uri").and_then(|v| v.as_str()) {
                    return Some(uri.to_string());
                }
    }

    None
}

/// Search pins with fuzzy matching
pub fn search_pins(query: &str) -> Vec<Value> {
    let store = match PinStore::new() {
        Ok(s) => s,
        Err(_) => return vec![],
    };

    let mut results: Vec<_> = store.fuzzy_search(query);
    results.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));

    results
        .into_iter()
        .map(|(pin, score)| {
            serde_json::json!({
                "type": pin.resource_type.as_str(),
                "id": pin.id,
                "alias": pin.alias,
                "tags": pin.tags,
                "uri": pin.uri(),
                "score": score
            })
        })
        .collect()
}
