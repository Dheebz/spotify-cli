//! Pin search helpers

use crate::storage::pins::PinStore;
use serde_json::Value;

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
