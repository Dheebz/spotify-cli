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

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn format_artist_detail_full() {
        let payload = json!({
            "name": "Test Artist",
            "followers": { "total": 5000000 },
            "genres": ["rock", "alternative rock", "indie"],
            "popularity": 85,
            "uri": "spotify:artist:abc123"
        });
        format_artist_detail(&payload);
    }

    #[test]
    fn format_artist_detail_minimal() {
        let payload = json!({});
        format_artist_detail(&payload);
    }

    #[test]
    fn format_artist_detail_no_genres() {
        let payload = json!({
            "name": "New Artist",
            "followers": { "total": 100 },
            "popularity": 10
        });
        format_artist_detail(&payload);
    }

    #[test]
    fn format_artist_detail_empty_genres() {
        let payload = json!({
            "name": "Artist",
            "genres": [],
            "followers": { "total": 500 }
        });
        format_artist_detail(&payload);
    }

    #[test]
    fn format_top_artists_with_data() {
        let artists = vec![
            json!({
                "name": "Artist One",
                "genres": ["pop", "dance", "electronic"],
                "popularity": 90
            }),
            json!({
                "name": "Artist Two",
                "genres": ["rock"],
                "popularity": 75
            }),
        ];
        format_top_artists(&artists, "Your Top Artists This Month");
    }

    #[test]
    fn format_top_artists_empty() {
        let artists: Vec<Value> = vec![];
        format_top_artists(&artists, "No Top Artists");
    }

    #[test]
    fn format_top_artists_minimal() {
        let artists = vec![json!({})];
        format_top_artists(&artists, "Artists");
    }

    #[test]
    fn format_top_artists_many_genres() {
        let artists = vec![json!({
            "name": "Multi-Genre Artist",
            "genres": ["genre1", "genre2", "genre3", "genre4", "genre5"],
            "popularity": 60
        })];
        format_top_artists(&artists, "Artists");
    }
}
