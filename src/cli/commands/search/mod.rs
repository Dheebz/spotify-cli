//! Search command modules
//!
//! This module is organized into submodules:
//! - `filters` - Result filtering (ghost entries, exact matches, URI extraction)
//! - `pins` - Pin search with fuzzy matching
//! - `playback` - Playback helpers for search results
//! - `scoring` - Fuzzy scoring for Spotify results

mod filters;
mod pins;
mod playback;
mod scoring;

use crate::endpoints::search;
use crate::io::output::{ErrorKind, Response};
use crate::storage::config::Config;

use super::{with_client, SearchFilters};
use filters::{extract_first_uri, filter_exact_matches, filter_ghost_entries};
use pins::search_pins;
use playback::play_uri;
use scoring::add_fuzzy_scores;

pub async fn search_command(
    query: &str,
    types: &[String],
    limit: u8,
    pins_only: bool,
    exact: bool,
    filters: SearchFilters,
    play: bool,
) -> Response {
    // Build the full query with filters
    let full_query = filters.build_query(query);

    // Validate that we have something to search for
    if full_query.is_empty() {
        return Response::err(
            400,
            "Search query is empty. Provide a query or use filters (--artist, --album, etc.)",
            ErrorKind::Validation,
        );
    }

    // First, search pins with fuzzy matching (uses base query only)
    let pin_results = search_pins(query);

    if pins_only {
        return Response::success_with_payload(
            200,
            format!("Found {} pinned result(s)", pin_results.len()),
            serde_json::json!({
                "pins": pin_results,
                "spotify": null
            }),
        );
    }

    // Prepare data for closure
    let query = query.to_string();
    let types = types.to_vec();

    with_client(|client| async move {
        let type_strs: Vec<&str> = if types.is_empty() {
            search::SEARCH_TYPES.to_vec()
        } else {
            types.iter().map(|s| s.as_str()).collect()
        };

        // Load config for fuzzy settings
        let config = Config::load().ok();
        let fuzzy_config = config
            .as_ref()
            .map(|c| c.fuzzy().clone())
            .unwrap_or_default();
        let sort_by_score = config.as_ref().map(|c| c.sort_by_score()).unwrap_or(false);

        match search::search(&client, &full_query, Some(&type_strs), Some(limit)).await {
            Ok(Some(mut spotify_results)) => {
                filter_ghost_entries(&mut spotify_results);

                if exact {
                    filter_exact_matches(&mut spotify_results, &query);
                }

                add_fuzzy_scores(&mut spotify_results, &query, &fuzzy_config, sort_by_score);

                if play {
                    if let Some(uri) = extract_first_uri(&pin_results, &spotify_results) {
                        return play_uri(&client, &uri).await;
                    } else {
                        return Response::err(404, "No results to play", ErrorKind::NotFound);
                    }
                }

                Response::success_with_payload(
                    200,
                    format!("Found {} pinned + Spotify results", pin_results.len()),
                    serde_json::json!({
                        "pins": pin_results,
                        "spotify": spotify_results
                    }),
                )
            }
            Ok(None) => {
                if play && !pin_results.is_empty()
                    && let Some(uri) = extract_first_uri(&pin_results, &serde_json::json!({})) {
                        return play_uri(&client, &uri).await;
                    }
                Response::success_with_payload(
                    200,
                    format!("Found {} pinned result(s)", pin_results.len()),
                    serde_json::json!({
                        "pins": pin_results,
                        "spotify": {}
                    }),
                )
            }
            Err(e) => Response::from_http_error(&e, "Search failed"),
        }
    })
    .await
}
