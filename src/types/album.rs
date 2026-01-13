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
