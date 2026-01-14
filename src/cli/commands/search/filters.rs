//! Search result filtering and processing helpers

use crate::endpoints::search::SEARCH_RESULT_KEYS;
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

/// Extract the first playable URI from spotify results or pins
/// Prioritizes Spotify results since the user is explicitly searching
pub fn extract_first_uri(pins: &[Value], spotify: &Value) -> Option<String> {
    // Prioritize Spotify search results - the user is searching for something specific
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

    // Fall back to pins if no Spotify results
    if let Some(first_pin) = pins.first()
        && let Some(uri) = first_pin.get("uri").and_then(|v| v.as_str()) {
            return Some(uri.to_string());
        }

    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn filter_ghost_entries_removes_invalid_entries() {
        let mut data = json!({
            "tracks": {
                "items": [
                    {"id": "valid", "name": "Track 1"},
                    {"name": "Ghost Track"},
                    {"id": null, "name": "Null ID Track"},
                    {"id": "also_valid", "name": "Track 2"}
                ]
            }
        });

        filter_ghost_entries(&mut data);

        let items = data["tracks"]["items"].as_array().unwrap();
        assert_eq!(items.len(), 2);
        assert_eq!(items[0]["id"], "valid");
        assert_eq!(items[1]["id"], "also_valid");
    }

    #[test]
    fn filter_ghost_entries_preserves_valid_entries() {
        let mut data = json!({
            "artists": {
                "items": [
                    {"id": "a1", "name": "Artist 1"},
                    {"id": "a2", "name": "Artist 2"}
                ]
            }
        });

        filter_ghost_entries(&mut data);

        let items = data["artists"]["items"].as_array().unwrap();
        assert_eq!(items.len(), 2);
    }

    #[test]
    fn filter_ghost_entries_handles_empty_items() {
        let mut data = json!({
            "albums": {
                "items": []
            }
        });

        filter_ghost_entries(&mut data);

        let items = data["albums"]["items"].as_array().unwrap();
        assert_eq!(items.len(), 0);
    }

    #[test]
    fn filter_ghost_entries_handles_missing_result_type() {
        let mut data = json!({
            "unknown": {
                "items": [{"id": null}]
            }
        });

        filter_ghost_entries(&mut data);

        // Should not crash and unknown key should be unchanged
        assert!(data.get("unknown").is_some());
    }

    #[test]
    fn filter_exact_matches_filters_by_name() {
        let mut data = json!({
            "tracks": {
                "items": [
                    {"id": "t1", "name": "Hello World"},
                    {"id": "t2", "name": "Goodbye"},
                    {"id": "t3", "name": "Hello Again"}
                ]
            }
        });

        filter_exact_matches(&mut data, "Hello");

        let items = data["tracks"]["items"].as_array().unwrap();
        assert_eq!(items.len(), 2);
        assert!(items[0]["name"].as_str().unwrap().contains("Hello"));
        assert!(items[1]["name"].as_str().unwrap().contains("Hello"));
    }

    #[test]
    fn filter_exact_matches_case_insensitive() {
        let mut data = json!({
            "artists": {
                "items": [
                    {"id": "a1", "name": "THE BEATLES"},
                    {"id": "a2", "name": "Beetle"},
                    {"id": "a3", "name": "Beatles Cover Band"}
                ]
            }
        });

        filter_exact_matches(&mut data, "beatles");

        let items = data["artists"]["items"].as_array().unwrap();
        assert_eq!(items.len(), 2);
    }

    #[test]
    fn filter_exact_matches_handles_missing_name() {
        let mut data = json!({
            "albums": {
                "items": [
                    {"id": "a1", "name": "Album One"},
                    {"id": "a2"}
                ]
            }
        });

        filter_exact_matches(&mut data, "Album");

        let items = data["albums"]["items"].as_array().unwrap();
        assert_eq!(items.len(), 1);
    }

    #[test]
    fn extract_first_uri_prioritizes_spotify_results() {
        let pins = vec![json!({"uri": "spotify:track:pin123"})];
        let spotify = json!({
            "tracks": {
                "items": [{"uri": "spotify:track:api123"}]
            }
        });

        // Spotify results take priority since user is explicitly searching
        let uri = extract_first_uri(&pins, &spotify);
        assert_eq!(uri, Some("spotify:track:api123".to_string()));
    }

    #[test]
    fn extract_first_uri_falls_back_to_pins() {
        let pins = vec![json!({"uri": "spotify:track:pin123"})];
        let spotify = json!({
            "tracks": {
                "items": []
            }
        });

        // Falls back to pins when no Spotify results
        let uri = extract_first_uri(&pins, &spotify);
        assert_eq!(uri, Some("spotify:track:pin123".to_string()));
    }

    #[test]
    fn extract_first_uri_from_spotify_when_no_pins() {
        let pins: Vec<Value> = vec![];
        let spotify = json!({
            "tracks": {
                "items": [{"uri": "spotify:track:api123"}]
            }
        });

        let uri = extract_first_uri(&pins, &spotify);
        assert_eq!(uri, Some("spotify:track:api123".to_string()));
    }

    #[test]
    fn extract_first_uri_from_spotify_when_pin_has_no_uri() {
        let pins = vec![json!({"name": "no uri"})];
        let spotify = json!({
            "artists": {
                "items": [{"uri": "spotify:artist:art123"}]
            }
        });

        let uri = extract_first_uri(&pins, &spotify);
        assert_eq!(uri, Some("spotify:artist:art123".to_string()));
    }

    #[test]
    fn extract_first_uri_returns_none_when_empty() {
        let pins: Vec<Value> = vec![];
        let spotify = json!({});

        let uri = extract_first_uri(&pins, &spotify);
        assert!(uri.is_none());
    }

    #[test]
    fn extract_first_uri_tries_multiple_result_types() {
        let pins: Vec<Value> = vec![];
        let spotify = json!({
            "tracks": {
                "items": []
            },
            "albums": {
                "items": [{"uri": "spotify:album:alb123"}]
            }
        });

        let uri = extract_first_uri(&pins, &spotify);
        assert_eq!(uri, Some("spotify:album:alb123".to_string()));
    }
}
