use super::pin::Pin;

/// Calculate fuzzy match score for a pin against a query
/// Returns 0.0 for no match, higher values for better matches
pub fn calculate_fuzzy_score(pin: &Pin, query: &str, query_words: &[&str]) -> f64 {
    let alias_lower = pin.alias.to_lowercase();
    let mut score = 0.0;

    // Exact alias match = highest score
    if alias_lower == query {
        return 100.0;
    }

    // Alias starts with query
    if alias_lower.starts_with(query) {
        score += 50.0;
    }

    // Alias contains query
    if alias_lower.contains(query) {
        score += 30.0;
    }

    // Check each query word
    for word in query_words {
        if alias_lower.contains(word) {
            score += 10.0;
        }

        // Check tags
        for tag in &pin.tags {
            let tag_lower = tag.to_lowercase();
            if tag_lower == *word {
                score += 15.0; // Exact tag match
            } else if tag_lower.contains(word) {
                score += 8.0;
            }
        }
    }

    // Levenshtein distance for typo tolerance on alias
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
