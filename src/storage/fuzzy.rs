use super::config::FuzzyConfig;

/// Calculate fuzzy match score for a name against a query
/// Returns 0.0 for no match, higher values for better matches
pub fn calculate_score(name: &str, query: &str, config: &FuzzyConfig) -> f64 {
    let name_lower = name.to_lowercase();
    let query_lower = query.to_lowercase();
    let query_words: Vec<&str> = query_lower.split_whitespace().collect();

    let mut score = 0.0;

    // Exact match = highest score
    if name_lower == query_lower {
        return config.exact_match;
    }

    // Name starts with query
    if name_lower.starts_with(&query_lower) {
        score += config.starts_with;
    }

    // Name contains query
    if name_lower.contains(&query_lower) {
        score += config.contains;
    }

    // Check each query word
    for word in &query_words {
        if name_lower.contains(word) {
            score += config.word_match;
        }
    }

    // Levenshtein distance for typo tolerance
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
}
