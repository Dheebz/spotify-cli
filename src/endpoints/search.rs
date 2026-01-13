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

    let encoded_query = urlencoding::encode(query);
    let endpoint = Endpoint::Search {
        query: &encoded_query,
        types: &type_str,
        limit: api_limit,
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
