//! Artist types from Spotify API.

use serde::{Deserialize, Serialize};

use super::common::{ExternalUrls, Followers, Image};

/// Simplified artist object (used in nested contexts).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArtistSimplified {
    /// External URLs for the artist.
    pub external_urls: Option<ExternalUrls>,
    /// Spotify URL for the artist.
    pub href: Option<String>,
    /// Spotify ID.
    pub id: String,
    /// Artist name.
    pub name: String,
    /// Object type (always "artist").
    #[serde(rename = "type")]
    pub item_type: String,
    /// Spotify URI.
    pub uri: String,
}

/// Full artist object.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Artist {
    /// External URLs.
    pub external_urls: Option<ExternalUrls>,
    /// Follower information.
    pub followers: Option<Followers>,
    /// Genres associated with the artist.
    pub genres: Option<Vec<String>>,
    /// Spotify URL.
    pub href: Option<String>,
    /// Spotify ID.
    pub id: String,
    /// Artist images.
    pub images: Option<Vec<Image>>,
    /// Artist name.
    pub name: String,
    /// Popularity score (0-100).
    pub popularity: Option<u32>,
    /// Object type (always "artist").
    #[serde(rename = "type")]
    pub item_type: String,
    /// Spotify URI.
    pub uri: String,
}

impl Artist {
    /// Get the largest image URL if available.
    pub fn image_url(&self) -> Option<&str> {
        self.images
            .as_ref()
            .and_then(|imgs| imgs.first())
            .map(|img| img.url.as_str())
    }
}

/// Response for artist's top tracks.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArtistTopTracksResponse {
    /// List of top tracks.
    pub tracks: Vec<super::track::Track>,
}

/// Response for related artists.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RelatedArtistsResponse {
    /// List of related artists.
    pub artists: Vec<Artist>,
}

/// Response wrapper for followed artists (cursor-paginated).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FollowedArtistsResponse {
    /// The artists container.
    pub artists: FollowedArtistsCursored,
}

/// Cursor-paginated artists list.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FollowedArtistsCursored {
    /// URL to the API endpoint.
    pub href: Option<String>,
    /// Maximum number of items.
    pub limit: Option<u32>,
    /// URL to the next page.
    pub next: Option<String>,
    /// Cursors for pagination.
    pub cursors: Option<super::common::Cursors>,
    /// Total count.
    pub total: Option<u32>,
    /// The followed artists.
    pub items: Vec<Artist>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn artist_simplified_deserializes() {
        let json = json!({
            "id": "abc123",
            "name": "Test Artist",
            "type": "artist",
            "uri": "spotify:artist:abc123"
        });
        let artist: ArtistSimplified = serde_json::from_value(json).unwrap();
        assert_eq!(artist.id, "abc123");
        assert_eq!(artist.name, "Test Artist");
    }

    #[test]
    fn artist_full_deserializes() {
        let json = json!({
            "id": "abc123",
            "name": "Test Artist",
            "type": "artist",
            "uri": "spotify:artist:abc123",
            "genres": ["rock", "alternative"],
            "popularity": 75,
            "followers": {"total": 1000000}
        });
        let artist: Artist = serde_json::from_value(json).unwrap();
        assert_eq!(artist.id, "abc123");
        assert_eq!(artist.popularity, Some(75));
        assert_eq!(artist.genres.as_ref().unwrap().len(), 2);
    }

    #[test]
    fn artist_image_url_returns_first_image() {
        let json = json!({
            "id": "abc123",
            "name": "Test Artist",
            "type": "artist",
            "uri": "spotify:artist:abc123",
            "images": [
                {"url": "https://large.jpg", "height": 640, "width": 640},
                {"url": "https://medium.jpg", "height": 300, "width": 300}
            ]
        });
        let artist: Artist = serde_json::from_value(json).unwrap();
        assert_eq!(artist.image_url(), Some("https://large.jpg"));
    }

    #[test]
    fn artist_image_url_returns_none_when_no_images() {
        let json = json!({
            "id": "abc123",
            "name": "Test Artist",
            "type": "artist",
            "uri": "spotify:artist:abc123"
        });
        let artist: Artist = serde_json::from_value(json).unwrap();
        assert!(artist.image_url().is_none());
    }

    #[test]
    fn artist_top_tracks_response_deserializes() {
        let json = json!({
            "tracks": []
        });
        let resp: ArtistTopTracksResponse = serde_json::from_value(json).unwrap();
        assert!(resp.tracks.is_empty());
    }

    #[test]
    fn related_artists_response_deserializes() {
        let json = json!({
            "artists": []
        });
        let resp: RelatedArtistsResponse = serde_json::from_value(json).unwrap();
        assert!(resp.artists.is_empty());
    }

    #[test]
    fn followed_artists_response_deserializes() {
        let json = json!({
            "artists": {
                "items": [],
                "limit": 20,
                "total": 0
            }
        });
        let resp: FollowedArtistsResponse = serde_json::from_value(json).unwrap();
        assert!(resp.artists.items.is_empty());
    }
}
