//! Common utility functions for formatting output

use serde_json::Value;

/// Duration format variants
pub enum DurationFormat {
    /// Short format: "3:45" or "1:02:30" for tracks
    Short,
    /// Long format: "1h 23m" or "45m" for episodes
    Long,
    /// Long format with seconds: "1h 23m" or "5m 30s" or "45s" for chapters
    LongWithSeconds,
}

/// Format milliseconds as duration string with specified format
pub fn format_duration_as(ms: u64, format: DurationFormat) -> String {
    let total_secs = ms / 1000;
    let hours = total_secs / 3600;
    let mins = (total_secs % 3600) / 60;
    let secs = total_secs % 60;

    match format {
        DurationFormat::Short => {
            if hours > 0 {
                format!("{}:{:02}:{:02}", hours, mins, secs)
            } else {
                format!("{}:{:02}", mins, secs)
            }
        }
        DurationFormat::Long => {
            if hours > 0 {
                format!("{}h {}m", hours, mins)
            } else {
                format!("{}m", mins)
            }
        }
        DurationFormat::LongWithSeconds => {
            if hours > 0 {
                format!("{}h {}m", hours, mins)
            } else if mins > 0 {
                format!("{}m {}s", mins, secs)
            } else {
                format!("{}s", secs)
            }
        }
    }
}

/// Format milliseconds as mm:ss duration string (short format)
/// Convenience wrapper for backward compatibility
pub fn format_duration(ms: u64) -> String {
    format_duration_as(ms, DurationFormat::Short)
}

/// Truncate string to max length with ellipsis
pub fn truncate(s: &str, max: usize) -> String {
    if s.chars().count() <= max {
        s.to_string()
    } else {
        format!("{}...", s.chars().take(max - 3).collect::<String>())
    }
}

/// Format large numbers with K/M suffix
pub fn format_number(n: u64) -> String {
    if n >= 1_000_000 {
        format!("{:.1}M", n as f64 / 1_000_000.0)
    } else if n >= 1_000 {
        format!("{:.1}K", n as f64 / 1_000.0)
    } else {
        n.to_string()
    }
}

/// Get fuzzy score from an item
pub fn get_score(item: &Value) -> i64 {
    item.get("fuzzy_score")
        .and_then(|v| v.as_f64())
        .map(|s| s as i64)
        .unwrap_or(0)
}

/// Extract artist names from an item's "artists" array
pub fn extract_artist_names(item: &Value) -> String {
    item.get("artists")
        .and_then(|v| v.as_array())
        .map(|arr| {
            arr.iter()
                .filter_map(|a| a.get("name").and_then(|n| n.as_str()))
                .collect::<Vec<_>>()
                .join(", ")
        })
        .unwrap_or_else(|| "Unknown".to_string())
}

/// Print a formatted table with header, columns, and rows
pub fn print_table(header: &str, cols: &[&str], rows: &[Vec<String>], col_widths: &[usize]) {
    println!("\n{}:", header);

    print!("  ");
    for (i, col) in cols.iter().enumerate() {
        if i == cols.len() - 1 {
            print!("{:>width$}", col, width = col_widths[i]);
        } else {
            print!("{:<width$}  ", col, width = col_widths[i]);
        }
    }
    println!();

    print!("  ");
    for (i, &w) in col_widths.iter().enumerate() {
        if i == col_widths.len() - 1 {
            print!("{}", "-".repeat(w));
        } else {
            print!("{}  ", "-".repeat(w));
        }
    }
    println!();

    for row in rows {
        print!("  ");
        for (i, cell) in row.iter().enumerate() {
            if i == row.len() - 1 {
                print!("{:>width$}", cell, width = col_widths[i]);
            } else {
                print!("{:<width$}  ", cell, width = col_widths[i]);
            }
        }
        println!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn format_duration_short_minutes_only() {
        assert_eq!(format_duration(0), "0:00");
        assert_eq!(format_duration(1000), "0:01");
        assert_eq!(format_duration(60000), "1:00");
        assert_eq!(format_duration(210000), "3:30");
    }

    #[test]
    fn format_duration_short_with_hours() {
        assert_eq!(format_duration(3600000), "1:00:00");
        assert_eq!(format_duration(3661000), "1:01:01");
        assert_eq!(format_duration(7200000), "2:00:00");
    }

    #[test]
    fn format_duration_as_short() {
        assert_eq!(format_duration_as(180000, DurationFormat::Short), "3:00");
        assert_eq!(format_duration_as(3661000, DurationFormat::Short), "1:01:01");
    }

    #[test]
    fn format_duration_as_long() {
        assert_eq!(format_duration_as(60000, DurationFormat::Long), "1m");
        assert_eq!(format_duration_as(3660000, DurationFormat::Long), "1h 1m");
        assert_eq!(format_duration_as(7200000, DurationFormat::Long), "2h 0m");
    }

    #[test]
    fn format_duration_as_long_with_seconds() {
        assert_eq!(format_duration_as(30000, DurationFormat::LongWithSeconds), "30s");
        assert_eq!(format_duration_as(90000, DurationFormat::LongWithSeconds), "1m 30s");
        assert_eq!(format_duration_as(3661000, DurationFormat::LongWithSeconds), "1h 1m");
    }

    #[test]
    fn truncate_short_string() {
        assert_eq!(truncate("hello", 10), "hello");
        assert_eq!(truncate("short", 10), "short");
    }

    #[test]
    fn truncate_long_string() {
        assert_eq!(truncate("hello world", 8), "hello...");
        assert_eq!(truncate("a very long string", 10), "a very ...");
    }

    #[test]
    fn truncate_exact_length() {
        assert_eq!(truncate("hello", 5), "hello");
    }

    #[test]
    fn format_number_small() {
        assert_eq!(format_number(0), "0");
        assert_eq!(format_number(999), "999");
    }

    #[test]
    fn format_number_thousands() {
        assert_eq!(format_number(1000), "1.0K");
        assert_eq!(format_number(1500), "1.5K");
        assert_eq!(format_number(10000), "10.0K");
        assert_eq!(format_number(999999), "1000.0K");
    }

    #[test]
    fn format_number_millions() {
        assert_eq!(format_number(1000000), "1.0M");
        assert_eq!(format_number(1500000), "1.5M");
        assert_eq!(format_number(10000000), "10.0M");
    }

    #[test]
    fn get_score_present() {
        let item = json!({ "fuzzy_score": 75.5 });
        assert_eq!(get_score(&item), 75);
    }

    #[test]
    fn get_score_missing() {
        let item = json!({ "name": "test" });
        assert_eq!(get_score(&item), 0);
    }

    #[test]
    fn get_score_non_numeric() {
        let item = json!({ "fuzzy_score": "not a number" });
        assert_eq!(get_score(&item), 0);
    }

    #[test]
    fn extract_artist_names_single() {
        let item = json!({
            "artists": [{ "name": "Artist One" }]
        });
        assert_eq!(extract_artist_names(&item), "Artist One");
    }

    #[test]
    fn extract_artist_names_multiple() {
        let item = json!({
            "artists": [
                { "name": "Artist One" },
                { "name": "Artist Two" }
            ]
        });
        assert_eq!(extract_artist_names(&item), "Artist One, Artist Two");
    }

    #[test]
    fn extract_artist_names_empty_array() {
        let item = json!({ "artists": [] });
        assert_eq!(extract_artist_names(&item), "");
    }

    #[test]
    fn extract_artist_names_missing() {
        let item = json!({ "name": "Track" });
        assert_eq!(extract_artist_names(&item), "Unknown");
    }

    #[test]
    fn extract_artist_names_null() {
        let item = json!({ "artists": null });
        assert_eq!(extract_artist_names(&item), "Unknown");
    }
}
