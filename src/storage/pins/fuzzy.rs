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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::storage::pins::resource_type::ResourceType;

    fn make_pin(alias: &str, tags: Vec<&str>) -> Pin {
        Pin::new(
            ResourceType::Track,
            "id123".to_string(),
            alias.to_string(),
            tags.into_iter().map(String::from).collect(),
        )
    }

    #[test]
    fn exact_match_returns_max_score() {
        let pin = make_pin("favorite", vec![]);
        let score = calculate_fuzzy_score(&pin, "favorite", &["favorite"]);
        assert_eq!(score, 100.0);
    }

    #[test]
    fn exact_match_case_insensitive() {
        let pin = make_pin("FAVORITE", vec![]);
        let score = calculate_fuzzy_score(&pin, "favorite", &["favorite"]);
        assert_eq!(score, 100.0);
    }

    #[test]
    fn starts_with_adds_bonus() {
        let pin = make_pin("favorite song", vec![]);
        let score = calculate_fuzzy_score(&pin, "favorite", &["favorite"]);
        assert!(score >= 50.0);
    }

    #[test]
    fn contains_adds_bonus() {
        let pin = make_pin("my favorite track", vec![]);
        let score = calculate_fuzzy_score(&pin, "favorite", &["favorite"]);
        assert!(score >= 30.0);
    }

    #[test]
    fn word_match_adds_bonus() {
        let pin = make_pin("chill vibes", vec![]);
        let words = vec!["chill", "vibes"];
        let score = calculate_fuzzy_score(&pin, "chill vibes", &words);
        // Contains bonus + word match bonuses
        assert!(score > 30.0);
    }

    #[test]
    fn tag_exact_match_adds_bonus() {
        let pin = make_pin("my track", vec!["rock", "chill"]);
        let score = calculate_fuzzy_score(&pin, "rock", &["rock"]);
        // Tag exact match bonus (15.0)
        assert!(score >= 15.0);
    }

    #[test]
    fn tag_contains_adds_bonus() {
        let pin = make_pin("my track", vec!["alternative rock", "chill"]);
        let score = calculate_fuzzy_score(&pin, "rock", &["rock"]);
        // Tag contains bonus (8.0)
        assert!(score >= 8.0);
    }

    #[test]
    fn no_match_returns_low_score() {
        let pin = make_pin("something completely different", vec!["jazz"]);
        let score = calculate_fuzzy_score(&pin, "xyz", &["xyz"]);
        // No match bonuses, possibly some levenshtein similarity
        assert!(score < 30.0);
    }

    #[test]
    fn similar_aliases_get_levenshtein_bonus() {
        let pin = make_pin("favorites", vec![]);
        let score = calculate_fuzzy_score(&pin, "favorite", &["favorite"]);
        // Very similar strings should get bonus
        assert!(score > 0.0);
    }

    #[test]
    fn multiple_tags_matching() {
        let pin = make_pin("playlist", vec!["rock", "chill", "vibes"]);
        let words = vec!["rock", "chill"];
        let score = calculate_fuzzy_score(&pin, "rock chill", &words);
        // Multiple tag matches
        assert!(score >= 30.0);
    }

    #[test]
    fn empty_tags_still_works() {
        let pin = make_pin("my alias", vec![]);
        let score = calculate_fuzzy_score(&pin, "my", &["my"]);
        // Should still calculate based on alias
        assert!(score > 0.0);
    }
}
