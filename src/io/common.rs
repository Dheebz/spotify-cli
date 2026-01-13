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
