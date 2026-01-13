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
