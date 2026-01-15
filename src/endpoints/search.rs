use crate::http::api::SpotifyApi;
use crate::http::client::HttpError;
use crate::http::endpoints::Endpoint;
use serde_json::Value;

/// Search types supported by the Spotify API (singular form for API queries)
pub const SEARCH_TYPES: &[&str] = &[
    "track",
    "artist",
    "album",
    "playlist",
    "show",
    "episode",
    "audiobook",
];

/// Search result keys in API responses (plural form)
pub const SEARCH_RESULT_KEYS: &[&str] = &[
    "tracks",
    "artists",
    "albums",
    "playlists",
    "shows",
    "episodes",
    "audiobooks",
];

/// Search the Spotify catalog
///
/// # Arguments
/// * `client` - Authenticated Spotify client
/// * `query` - Search query
/// * `types` - Optional list of types to search (defaults to all)
/// * `limit` - Optional limit per type (default 20, max 50)
/// * `market` - Optional market (ISO 3166-1 alpha-2 country code) for content availability
///
/// # Market Parameter
///
/// The market parameter is important for podcast/episode searches. Without it,
/// the Spotify API may return incomplete episode data (missing show information).
/// When provided, episodes will include their parent show's name and other metadata.
///
/// # Spotify API Quirk Workaround
///
/// The Spotify Search API has a known bug/quirk where requesting `limit=1`
/// returns different (often incorrect) results compared to `limit=2`.
///
/// For example, searching "tool" with `limit=1` might return "Weezer" as the
/// top artist, but `limit=2` correctly returns "TOOL" first, then "Weezer".
///
/// This appears to be a ranking/relevance calculation issue on Spotify's end
/// where the algorithm behaves differently when only one result is requested.
///
/// **Workaround**: When `limit=1` is requested, we actually fetch `limit=2`
/// from the API and then truncate the results to only return the first item.
/// This ensures consistent and correct "top result" behavior.
///
/// This workaround was implemented on 2025-01-12 after observing the issue.
/// If Spotify fixes this behavior in the future, this workaround can be removed.
pub async fn search(
    client: &SpotifyApi,
    query: &str,
    types: Option<&[&str]>,
    limit: Option<u8>,
    market: Option<&str>,
) -> Result<Option<Value>, HttpError> {
    let type_str = types
        .map(|t| t.join(","))
        .unwrap_or_else(|| SEARCH_TYPES.join(","));

    let requested_limit = limit.unwrap_or(20).min(50);

    // WORKAROUND: Spotify API returns incorrect results when limit=1.
    // We fetch limit=2 instead and truncate the response.
    // See function documentation for full explanation.
    let api_limit = if requested_limit == 1 { 2 } else { requested_limit };
    let needs_truncation = requested_limit == 1;

    let endpoint = Endpoint::Search {
        query,
        types: &type_str,
        limit: api_limit,
        market,
    }.path();

    let response = client.get(&endpoint).await?;

    // If we requested limit=1, truncate all result arrays to single item
    if needs_truncation
        && let Some(mut data) = response {
            truncate_search_results(&mut data);
            return Ok(Some(data));
        }

    Ok(response)
}

/// Truncates all search result arrays to contain only the first item.
///
/// This is part of the limit=1 workaround. The Spotify API response contains
/// multiple result types (tracks, artists, albums, etc.), each with an "items"
/// array. This function truncates each of those arrays to a single element.
fn truncate_search_results(data: &mut Value) {
    for result_type in SEARCH_RESULT_KEYS {
        if let Some(container) = data.get_mut(result_type) {
            if let Some(items) = container.get_mut("items")
                && let Some(arr) = items.as_array_mut() {
                    arr.truncate(1);
                }
            // Also update the limit field to reflect what was actually returned
            if let Some(limit) = container.get_mut("limit") {
                *limit = Value::Number(1.into());
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn search_types_has_expected_types() {
        assert!(SEARCH_TYPES.contains(&"track"));
        assert!(SEARCH_TYPES.contains(&"artist"));
        assert!(SEARCH_TYPES.contains(&"album"));
        assert!(SEARCH_TYPES.contains(&"playlist"));
        assert!(SEARCH_TYPES.contains(&"show"));
        assert!(SEARCH_TYPES.contains(&"episode"));
        assert!(SEARCH_TYPES.contains(&"audiobook"));
    }

    #[test]
    fn search_types_count() {
        assert_eq!(SEARCH_TYPES.len(), 7);
    }

    #[test]
    fn search_result_keys_are_plural() {
        for key in SEARCH_RESULT_KEYS {
            assert!(key.ends_with('s'), "{} should be plural", key);
        }
    }

    #[test]
    fn search_result_keys_count() {
        assert_eq!(SEARCH_RESULT_KEYS.len(), 7);
    }

    #[test]
    fn truncate_search_results_works() {
        let mut data = json!({
            "tracks": {
                "items": [
                    {"name": "track1"},
                    {"name": "track2"},
                    {"name": "track3"}
                ],
                "limit": 3
            },
            "artists": {
                "items": [
                    {"name": "artist1"},
                    {"name": "artist2"}
                ],
                "limit": 2
            }
        });

        truncate_search_results(&mut data);

        let tracks = data["tracks"]["items"].as_array().unwrap();
        assert_eq!(tracks.len(), 1);
        assert_eq!(tracks[0]["name"], "track1");

        let artists = data["artists"]["items"].as_array().unwrap();
        assert_eq!(artists.len(), 1);
        assert_eq!(artists[0]["name"], "artist1");

        assert_eq!(data["tracks"]["limit"], 1);
        assert_eq!(data["artists"]["limit"], 1);
    }

    #[test]
    fn truncate_handles_missing_keys() {
        let mut data = json!({
            "unknown_key": {
                "items": [1, 2, 3]
            }
        });

        truncate_search_results(&mut data);

        // Should not crash and unknown key should be unchanged
        assert_eq!(data["unknown_key"]["items"].as_array().unwrap().len(), 3);
    }

    #[test]
    fn truncate_handles_empty_items() {
        let mut data = json!({
            "tracks": {
                "items": [],
                "limit": 0
            }
        });

        truncate_search_results(&mut data);

        assert_eq!(data["tracks"]["items"].as_array().unwrap().len(), 0);
    }

    #[test]
    fn truncate_handles_single_item() {
        let mut data = json!({
            "albums": {
                "items": [{"name": "album1"}],
                "limit": 1
            }
        });

        truncate_search_results(&mut data);

        assert_eq!(data["albums"]["items"].as_array().unwrap().len(), 1);
    }
}
