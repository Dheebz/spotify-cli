//! Artist formatting functions

use serde_json::Value;

use crate::io::common::{format_number, print_table, truncate};

pub fn format_artist_detail(payload: &Value) {
    let name = payload.get("name").and_then(|v| v.as_str()).unwrap_or("Unknown");
    let followers = payload
        .get("followers")
        .and_then(|f| f.get("total"))
        .and_then(|v| v.as_u64())
        .unwrap_or(0);
    let genres = payload
        .get("genres")
        .and_then(|v| v.as_array())
        .map(|arr| {
            arr.iter()
                .filter_map(|g| g.as_str())
                .collect::<Vec<_>>()
                .join(", ")
        })
        .unwrap_or_default();
    let popularity = payload.get("popularity").and_then(|v| v.as_u64()).unwrap_or(0);
    let uri = payload.get("uri").and_then(|v| v.as_str()).unwrap_or("");

    println!("{}", name);
    println!("  Followers: {}", format_number(followers));
    println!("  Popularity: {}%", popularity);
    if !genres.is_empty() {
        println!("  Genres: {}", genres);
    }
    if !uri.is_empty() {
        println!("  URI: {}", uri);
    }
}

pub fn format_top_artists(artists: &[Value], message: &str) {
    println!("{}:", message);
    println!();
    print_table(
        "Your Top Artists",
        &["#", "Name", "Genres", "Popularity"],
        &artists
            .iter()
            .enumerate()
            .map(|(i, artist)| {
                let name = artist
                    .get("name")
                    .and_then(|v| v.as_str())
                    .unwrap_or("Unknown");
                let genres = artist
                    .get("genres")
                    .and_then(|v| v.as_array())
                    .map(|arr| {
                        arr.iter()
                            .take(2)
                            .filter_map(|g| g.as_str())
                            .collect::<Vec<_>>()
                            .join(", ")
                    })
                    .unwrap_or_default();
                let popularity = artist
                    .get("popularity")
                    .and_then(|v| v.as_u64())
                    .map(|p| format!("{}%", p))
                    .unwrap_or_else(|| "-".to_string());
                vec![
                    (i + 1).to_string(),
                    truncate(name, 25),
                    truncate(&genres, 25),
                    popularity,
                ]
            })
            .collect::<Vec<_>>(),
        &[3, 25, 25, 10],
    );
}
