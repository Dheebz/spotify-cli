use super::pin::Pin;
use crate::storage::fuzzy::levenshtein_distance;

/// Calculate fuzzy match score for a pin against a query
/// Returns 0.0 for no match, higher values for better matches
pub fn calculate_fuzzy_score(pin: &Pin, query: &str, query_words: &[&str]) -> f64 {
    let alias_lower = pin.alias.to_lowercase();
    let mut score = 0.0;

    if alias_lower == query {
        return 100.0;
    }

    if alias_lower.starts_with(query) {
        score += 50.0;
    }

    if alias_lower.contains(query) {
        score += 30.0;
    }

    for word in query_words {
        if alias_lower.contains(word) {
            score += 10.0;
        }

        for tag in &pin.tags {
            let tag_lower = tag.to_lowercase();
            if tag_lower == *word {
                score += 15.0;
            } else if tag_lower.contains(word) {
                score += 8.0;
            }
        }
    }

    let distance = levenshtein_distance(&alias_lower, query);
    let max_len = alias_lower.len().max(query.len());
    if max_len > 0 {
        let similarity = 1.0 - (distance as f64 / max_len as f64);
        if similarity > 0.6 {
            score += similarity * 20.0;
        }
    }

    score
}
