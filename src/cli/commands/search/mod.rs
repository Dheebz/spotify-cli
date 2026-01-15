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

use crate::endpoints::episodes::get_several_episodes;
use crate::endpoints::search;
use crate::endpoints::user::get_current_user;
use crate::http::api::SpotifyApi;
use crate::io::output::{ErrorKind, Response};
use crate::storage::config::Config;
use serde_json::Value;

use super::{with_client, SearchFilters};
use filters::{extract_first_uri, filter_exact_matches, filter_ghost_entries};
use pins::search_pins;
use playback::play_uri;
use scoring::add_fuzzy_scores;

/// Enrich episodes with show information by fetching full episode details.
/// Uses a show cache to avoid duplicating show data for episodes from the same show.
async fn enrich_episodes(client: &SpotifyApi, results: &mut Value) {
    let episodes = match results
        .get("episodes")
        .and_then(|e| e.get("items"))
        .and_then(|i| i.as_array())
    {
        Some(eps) => eps,
        None => return,
    };

    // Collect IDs of episodes missing show info
    let ids: Vec<String> = episodes
        .iter()
        .filter(|ep| ep.get("show").is_none() || ep.get("show").unwrap().is_null())
        .filter_map(|ep| ep.get("id").and_then(|id| id.as_str()).map(String::from))
        .collect();

    if ids.is_empty() {
        return;
    }

    // Fetch full episode details in one batch call
    let full_episodes = match get_several_episodes::get_several_episodes(client, &ids).await {
        Ok(Some(data)) => data,
        _ => return,
    };

    // Build a cache of show_id -> show info (each unique show stored once)
    let mut show_cache: std::collections::HashMap<String, Value> = std::collections::HashMap::new();
    // Map episode_id -> show_id for lookup
    let mut episode_show_map: std::collections::HashMap<String, String> = std::collections::HashMap::new();

    if let Some(eps) = full_episodes.get("episodes").and_then(|e| e.as_array()) {
        for ep in eps {
            if let (Some(ep_id), Some(show)) = (
                ep.get("id").and_then(|id| id.as_str()),
                ep.get("show"),
            ) {
                if let Some(show_id) = show.get("id").and_then(|id| id.as_str()) {
                    // Cache show by its ID (only store once per unique show)
                    show_cache.entry(show_id.to_string()).or_insert_with(|| show.clone());
                    // Map episode to its show
                    episode_show_map.insert(ep_id.to_string(), show_id.to_string());
                }
            }
        }
    }

    // Merge show info back into search results using the cache
    if let Some(items) = results
        .get_mut("episodes")
        .and_then(|e| e.get_mut("items"))
        .and_then(|i| i.as_array_mut())
    {
        for ep in items.iter_mut() {
            if let Some(ep_id) = ep.get("id").and_then(|id| id.as_str()) {
                if let Some(show_id) = episode_show_map.get(ep_id) {
                    if let Some(show) = show_cache.get(show_id) {
                        ep.as_object_mut().map(|obj| obj.insert("show".to_string(), show.clone()));
                    }
                }
            }
        }
    }
}

pub async fn search_command(
    query: &str,
    types: &[String],
    limit: u8,
    pins_only: bool,
    exact: bool,
    filters: SearchFilters,
    play: bool,
    sort: bool,
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
        // Use --sort flag or fall back to config setting
        let sort_by_score = sort || config.as_ref().map(|c| c.sort_by_score()).unwrap_or(false);

        // Fetch user's market for proper podcast/episode results
        let market = match get_current_user::get_current_user(&client).await {
            Ok(Some(user)) => user
                .get("country")
                .and_then(|c| c.as_str())
                .map(String::from),
            _ => None,
        };

        match search::search(&client, &full_query, Some(&type_strs), Some(limit), market.as_deref()).await {
            Ok(Some(mut spotify_results)) => {
                filter_ghost_entries(&mut spotify_results);

                // Enrich episodes with show info (search API returns simplified objects)
                enrich_episodes(&client, &mut spotify_results).await;

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
