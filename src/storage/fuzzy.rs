use super::config::FuzzyConfig;

/// Calculate fuzzy match score for a name against a query
/// Returns 0.0 for no match, higher values for better matches
pub fn calculate_score(name: &str, query: &str, config: &FuzzyConfig) -> f64 {
    let name_lower = name.to_lowercase();
    let query_lower = query.to_lowercase();
    let query_words: Vec<&str> = query_lower.split_whitespace().collect();

    let mut score = 0.0;

    if name_lower == query_lower {
        return config.exact_match;
    }

    if name_lower.starts_with(&query_lower) {
        score += config.starts_with;
    }

    if name_lower.contains(&query_lower) {
        score += config.contains;
    }

    for word in &query_words {
        if name_lower.contains(word) {
            score += config.word_match;
        }
    }

    let distance = levenshtein_distance(&name_lower, &query_lower);
    let max_len = name_lower.len().max(query_lower.len());
    if max_len > 0 {
        let similarity = 1.0 - (distance as f64 / max_len as f64);
        if similarity > config.similarity_threshold {
            score += similarity * config.similarity_weight;
        }
    }

    score
}

/// Simple Levenshtein distance implementation
pub fn levenshtein_distance(a: &str, b: &str) -> usize {
    let a_chars: Vec<char> = a.chars().collect();
    let b_chars: Vec<char> = b.chars().collect();
    let a_len = a_chars.len();
    let b_len = b_chars.len();

    if a_len == 0 {
        return b_len;
    }
    if b_len == 0 {
        return a_len;
    }

    let mut matrix = vec![vec![0; b_len + 1]; a_len + 1];

    for (i, row) in matrix.iter_mut().enumerate() {
        row[0] = i;
    }
    for (j, cell) in matrix[0].iter_mut().enumerate() {
        *cell = j;
    }

    for i in 1..=a_len {
        for j in 1..=b_len {
            let cost = if a_chars[i - 1] == b_chars[j - 1] { 0 } else { 1 };
            matrix[i][j] = (matrix[i - 1][j] + 1)
                .min(matrix[i][j - 1] + 1)
                .min(matrix[i - 1][j - 1] + cost);
        }
    }

    matrix[a_len][b_len]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn default_config() -> FuzzyConfig {
        FuzzyConfig::default()
    }

    #[test]
    fn exact_match_highest_score() {
        let config = default_config();
        let score = calculate_score("TOOL", "tool", &config);
        assert_eq!(score, config.exact_match);
    }

    #[test]
    fn starts_with_scores_high() {
        let config = default_config();
        let score = calculate_score("TOOL - Lateralus", "tool", &config);
        assert!(score >= config.starts_with);
    }

    #[test]
    fn no_match_scores_low() {
        let config = default_config();
        let score = calculate_score("Weezer", "tool", &config);
        assert!(score < config.contains);
    }

    #[test]
    fn levenshtein_basic() {
        assert_eq!(levenshtein_distance("kitten", "sitting"), 3);
        assert_eq!(levenshtein_distance("tool", "tool"), 0);
        assert_eq!(levenshtein_distance("", "abc"), 3);
    }

    #[test]
    fn levenshtein_empty_strings() {
        assert_eq!(levenshtein_distance("", ""), 0);
        assert_eq!(levenshtein_distance("abc", ""), 3);
        assert_eq!(levenshtein_distance("", "xyz"), 3);
    }

    #[test]
    fn levenshtein_single_char() {
        assert_eq!(levenshtein_distance("a", "b"), 1);
        assert_eq!(levenshtein_distance("a", "a"), 0);
    }

    #[test]
    fn levenshtein_longer_strings() {
        assert_eq!(levenshtein_distance("hello", "hello"), 0);
        assert_eq!(levenshtein_distance("hello", "hallo"), 1);
        assert_eq!(levenshtein_distance("hello", "world"), 4);
    }

    #[test]
    fn contains_partial_match() {
        let config = default_config();
        let score = calculate_score("My Favorite Tool", "tool", &config);
        // Should get contains + word_match bonus
        assert!(score >= config.contains);
    }

    #[test]
    fn word_match_scoring() {
        let config = default_config();
        let score = calculate_score("rock and roll", "rock roll", &config);
        // Should get word match bonuses for "rock" and "roll"
        assert!(score >= config.word_match * 2.0);
    }

    #[test]
    fn similarity_bonus_applied() {
        let config = default_config();
        // "tools" is very similar to "tool"
        let score = calculate_score("tools", "tool", &config);
        // Should get similarity bonus since they are 80% similar
        assert!(score > 0.0);
    }

    #[test]
    fn case_insensitive_matching() {
        let config = default_config();
        let score1 = calculate_score("TOOL", "tool", &config);
        let score2 = calculate_score("tool", "TOOL", &config);
        let score3 = calculate_score("Tool", "TOOL", &config);
        assert_eq!(score1, score2);
        assert_eq!(score2, score3);
    }

    #[test]
    fn custom_config_values() {
        let config = FuzzyConfig {
            exact_match: 200.0,
            starts_with: 100.0,
            contains: 50.0,
            word_match: 20.0,
            similarity_threshold: 0.5,
            similarity_weight: 30.0,
        };
        let score = calculate_score("test", "test", &config);
        assert_eq!(score, 200.0);
    }

    #[test]
    fn zero_score_for_unrelated() {
        let config = FuzzyConfig {
            exact_match: 100.0,
            starts_with: 50.0,
            contains: 30.0,
            word_match: 10.0,
            similarity_threshold: 0.9, // High threshold
            similarity_weight: 20.0,
        };
        let score = calculate_score("completely different", "xyz", &config);
        // Very different strings with high threshold should score low
        assert!(score < 30.0); // Less than contains
    }
}
