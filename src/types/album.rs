//! Album types from Spotify API.

use serde::{Deserialize, Serialize};

use super::artist::ArtistSimplified;
use super::common::{Copyright, ExternalIds, ExternalUrls, Image, Paginated, Restrictions};
use super::track::TrackSimplified;

/// Album type enumeration.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum AlbumType {
    Album,
    Single,
    Compilation,
}

/// Simplified album object (used in nested contexts).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlbumSimplified {
    /// Album type.
    pub album_type: Option<String>,
    /// Total number of tracks.
    pub total_tracks: Option<u32>,
    /// Markets where the album is available.
    pub available_markets: Option<Vec<String>>,
    /// External URLs.
    pub external_urls: Option<ExternalUrls>,
    /// Spotify URL.
    pub href: Option<String>,
    /// Spotify ID.
    pub id: String,
    /// Album cover images.
    pub images: Option<Vec<Image>>,
    /// Album name.
    pub name: String,
    /// Release date (YYYY, YYYY-MM, or YYYY-MM-DD).
    pub release_date: Option<String>,
    /// Release date precision.
    pub release_date_precision: Option<String>,
    /// Restrictions if any.
    pub restrictions: Option<Restrictions>,
    /// Object type (always "album").
    #[serde(rename = "type")]
    pub item_type: String,
    /// Spotify URI.
    pub uri: String,
    /// Artists on the album.
    pub artists: Option<Vec<ArtistSimplified>>,
}

/// Full album object.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Album {
    /// Album type.
    pub album_type: Option<String>,
    /// Total number of tracks.
    pub total_tracks: Option<u32>,
    /// Markets where the album is available.
    pub available_markets: Option<Vec<String>>,
    /// External URLs.
    pub external_urls: Option<ExternalUrls>,
    /// Spotify URL.
    pub href: Option<String>,
    /// Spotify ID.
    pub id: String,
    /// Album cover images.
    pub images: Option<Vec<Image>>,
    /// Album name.
    pub name: String,
    /// Release date.
    pub release_date: Option<String>,
    /// Release date precision.
    pub release_date_precision: Option<String>,
    /// Restrictions if any.
    pub restrictions: Option<Restrictions>,
    /// Object type.
    #[serde(rename = "type")]
    pub item_type: String,
    /// Spotify URI.
    pub uri: String,
    /// Artists on the album.
    pub artists: Option<Vec<ArtistSimplified>>,
    /// Album tracks (paginated).
    pub tracks: Option<Paginated<TrackSimplified>>,
    /// Copyright information.
    pub copyrights: Option<Vec<Copyright>>,
    /// External IDs.
    pub external_ids: Option<ExternalIds>,
    /// Genres (may be empty).
    pub genres: Option<Vec<String>>,
    /// Label name.
    pub label: Option<String>,
    /// Popularity score (0-100).
    pub popularity: Option<u32>,
}

impl Album {
    /// Get the largest image URL if available.
    pub fn image_url(&self) -> Option<&str> {
        self.images
            .as_ref()
            .and_then(|imgs| imgs.first())
            .map(|img| img.url.as_str())
    }

    /// Get the primary artist name.
    pub fn artist_name(&self) -> Option<&str> {
        self.artists
            .as_ref()
            .and_then(|artists| artists.first())
            .map(|a| a.name.as_str())
    }

    /// Get release year.
    pub fn release_year(&self) -> Option<&str> {
        self.release_date.as_ref().map(|d| &d[..4])
    }
}

impl AlbumSimplified {
    /// Get the largest image URL if available.
    pub fn image_url(&self) -> Option<&str> {
        self.images
            .as_ref()
            .and_then(|imgs| imgs.first())
            .map(|img| img.url.as_str())
    }

    /// Get the primary artist name.
    pub fn artist_name(&self) -> Option<&str> {
        self.artists
            .as_ref()
            .and_then(|artists| artists.first())
            .map(|a| a.name.as_str())
    }
}

/// Saved album (wraps album with added_at timestamp).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SavedAlbum {
    /// When the album was saved.
    pub added_at: String,
    /// The album.
    pub album: Album,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn album_type_deserializes() {
        let json = json!("album");
        let album_type: AlbumType = serde_json::from_value(json).unwrap();
        assert_eq!(album_type, AlbumType::Album);

        let json = json!("single");
        let album_type: AlbumType = serde_json::from_value(json).unwrap();
        assert_eq!(album_type, AlbumType::Single);

        let json = json!("compilation");
        let album_type: AlbumType = serde_json::from_value(json).unwrap();
        assert_eq!(album_type, AlbumType::Compilation);
    }

    #[test]
    fn album_simplified_deserializes() {
        let json = json!({
            "id": "album123",
            "name": "Test Album",
            "type": "album",
            "uri": "spotify:album:album123",
            "album_type": "album",
            "total_tracks": 12,
            "release_date": "2024-01-15"
        });
        let album: AlbumSimplified = serde_json::from_value(json).unwrap();
        assert_eq!(album.id, "album123");
        assert_eq!(album.name, "Test Album");
        assert_eq!(album.total_tracks, Some(12));
    }

    #[test]
    fn album_simplified_image_url() {
        let json = json!({
            "id": "album123",
            "name": "Test Album",
            "type": "album",
            "uri": "spotify:album:album123",
            "images": [{"url": "https://cover.jpg", "height": 640, "width": 640}]
        });
        let album: AlbumSimplified = serde_json::from_value(json).unwrap();
        assert_eq!(album.image_url(), Some("https://cover.jpg"));
    }

    #[test]
    fn album_simplified_image_url_none() {
        let json = json!({
            "id": "album123",
            "name": "Test Album",
            "type": "album",
            "uri": "spotify:album:album123"
        });
        let album: AlbumSimplified = serde_json::from_value(json).unwrap();
        assert!(album.image_url().is_none());
    }

    #[test]
    fn album_simplified_artist_name() {
        let json = json!({
            "id": "album123",
            "name": "Test Album",
            "type": "album",
            "uri": "spotify:album:album123",
            "artists": [{"id": "artist1", "name": "Test Artist", "type": "artist", "uri": "spotify:artist:artist1"}]
        });
        let album: AlbumSimplified = serde_json::from_value(json).unwrap();
        assert_eq!(album.artist_name(), Some("Test Artist"));
    }

    #[test]
    fn album_simplified_artist_name_none() {
        let json = json!({
            "id": "album123",
            "name": "Test Album",
            "type": "album",
            "uri": "spotify:album:album123"
        });
        let album: AlbumSimplified = serde_json::from_value(json).unwrap();
        assert!(album.artist_name().is_none());
    }

    #[test]
    fn album_full_deserializes() {
        let json = json!({
            "id": "album123",
            "name": "Test Album",
            "type": "album",
            "uri": "spotify:album:album123",
            "album_type": "album",
            "total_tracks": 12,
            "release_date": "2024-01-15",
            "popularity": 75,
            "label": "Test Label",
            "genres": ["rock", "alternative"]
        });
        let album: Album = serde_json::from_value(json).unwrap();
        assert_eq!(album.id, "album123");
        assert_eq!(album.popularity, Some(75));
        assert_eq!(album.label, Some("Test Label".to_string()));
    }

    #[test]
    fn album_image_url() {
        let json = json!({
            "id": "album123",
            "name": "Test Album",
            "type": "album",
            "uri": "spotify:album:album123",
            "images": [{"url": "https://large.jpg", "height": 640, "width": 640}]
        });
        let album: Album = serde_json::from_value(json).unwrap();
        assert_eq!(album.image_url(), Some("https://large.jpg"));
    }

    #[test]
    fn album_artist_name() {
        let json = json!({
            "id": "album123",
            "name": "Test Album",
            "type": "album",
            "uri": "spotify:album:album123",
            "artists": [{"id": "artist1", "name": "Primary Artist", "type": "artist", "uri": "spotify:artist:artist1"}]
        });
        let album: Album = serde_json::from_value(json).unwrap();
        assert_eq!(album.artist_name(), Some("Primary Artist"));
    }

    #[test]
    fn album_release_year() {
        let json = json!({
            "id": "album123",
            "name": "Test Album",
            "type": "album",
            "uri": "spotify:album:album123",
            "release_date": "2024-01-15"
        });
        let album: Album = serde_json::from_value(json).unwrap();
        assert_eq!(album.release_year(), Some("2024"));
    }

    #[test]
    fn album_release_year_none() {
        let json = json!({
            "id": "album123",
            "name": "Test Album",
            "type": "album",
            "uri": "spotify:album:album123"
        });
        let album: Album = serde_json::from_value(json).unwrap();
        assert!(album.release_year().is_none());
    }

    #[test]
    fn saved_album_deserializes() {
        let json = json!({
            "added_at": "2024-01-15T10:30:00Z",
            "album": {
                "id": "album123",
                "name": "Test Album",
                "type": "album",
                "uri": "spotify:album:album123"
            }
        });
        let saved: SavedAlbum = serde_json::from_value(json).unwrap();
        assert_eq!(saved.added_at, "2024-01-15T10:30:00Z");
        assert_eq!(saved.album.id, "album123");
    }
}
