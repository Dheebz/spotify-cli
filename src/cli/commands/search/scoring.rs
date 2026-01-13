//! Fuzzy scoring helpers for search results

use crate::endpoints::search::SEARCH_RESULT_KEYS;
use crate::storage::config::FuzzyConfig;
use crate::storage::fuzzy;
use serde_json::Value;

/// Add fuzzy scores to search results and optionally sort by score
pub fn add_fuzzy_scores(data: &mut Value, query: &str, config: &FuzzyConfig, sort: bool) {
    for result_type in SEARCH_RESULT_KEYS {
        if let Some(container) = data.get_mut(result_type)
            && let Some(items) = container.get_mut("items")
                && let Some(arr) = items.as_array_mut() {
                    for item in arr.iter_mut() {
                        let name_score = item
                            .get("name")
                            .and_then(|v| v.as_str())
                            .map(|n| fuzzy::calculate_score(n, query, config))
                            .unwrap_or(0.0);

                        let artist_score = item
                            .get("artists")
                            .and_then(|v| v.as_array())
                            .map(|artists| {
                                artists
                                    .iter()
                                    .filter_map(|a| a.get("name").and_then(|n| n.as_str()))
                                    .map(|n| fuzzy::calculate_score(n, query, config))
                                    .fold(0.0_f64, |acc, s| acc.max(s))
                            })
                            .unwrap_or(0.0);

                        let owner_score = item
                            .get("owner")
                            .and_then(|o| o.get("display_name"))
                            .and_then(|v| v.as_str())
                            .map(|n| fuzzy::calculate_score(n, query, config))
                            .unwrap_or(0.0);

                        let score = name_score.max(artist_score).max(owner_score);

                        if let Some(obj) = item.as_object_mut() {
                            obj.insert(
                                "fuzzy_score".to_string(),
                                serde_json::Value::Number(
                                    serde_json::Number::from_f64(score)
                                        .unwrap_or_else(|| 0.into()),
                                ),
                            );
                        }
                    }

                    if sort {
                        arr.sort_by(|a, b| {
                            let score_a =
                                a.get("fuzzy_score").and_then(|v| v.as_f64()).unwrap_or(0.0);
                            let score_b =
                                b.get("fuzzy_score").and_then(|v| v.as_f64()).unwrap_or(0.0);
                            score_b
                                .partial_cmp(&score_a)
                                .unwrap_or(std::cmp::Ordering::Equal)
                        });
                    }
                }
    }
}
