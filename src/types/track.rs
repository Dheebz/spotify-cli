//! Track types from Spotify API.

use serde::{Deserialize, Serialize};

use super::album::AlbumSimplified;
use super::artist::ArtistSimplified;
use super::common::{ExternalIds, ExternalUrls, LinkedFrom, Restrictions};

/// Simplified track object (used in album tracks, etc.).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrackSimplified {
    /// Artists who performed the track.
    pub artists: Option<Vec<ArtistSimplified>>,
    /// Markets where the track is available.
    pub available_markets: Option<Vec<String>>,
    /// Disc number.
    pub disc_number: Option<u32>,
    /// Track duration in milliseconds.
    pub duration_ms: u64,
    /// Whether the track has explicit lyrics.
    pub explicit: Option<bool>,
    /// External URLs.
    pub external_urls: Option<ExternalUrls>,
    /// Spotify URL.
    pub href: Option<String>,
    /// Spotify ID.
    pub id: String,
    /// Whether the track is playable in the user's market.
    pub is_playable: Option<bool>,
    /// Linked track info if relinked.
    pub linked_from: Option<LinkedFrom>,
    /// Restrictions if any.
    pub restrictions: Option<Restrictions>,
    /// Track name.
    pub name: String,
    /// Preview URL (30 second preview).
    pub preview_url: Option<String>,
    /// Track number on the disc.
    pub track_number: Option<u32>,
    /// Object type (always "track").
    #[serde(rename = "type")]
    pub item_type: String,
    /// Spotify URI.
    pub uri: String,
    /// Whether the track is a local file.
    pub is_local: Option<bool>,
}

/// Full track object.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Track {
    /// Album containing the track.
    pub album: Option<AlbumSimplified>,
    /// Artists who performed the track.
    pub artists: Option<Vec<ArtistSimplified>>,
    /// Markets where the track is available.
    pub available_markets: Option<Vec<String>>,
    /// Disc number.
    pub disc_number: Option<u32>,
    /// Track duration in milliseconds.
    pub duration_ms: u64,
    /// Whether the track has explicit lyrics.
    pub explicit: Option<bool>,
    /// External IDs (ISRC, etc.).
    pub external_ids: Option<ExternalIds>,
    /// External URLs.
    pub external_urls: Option<ExternalUrls>,
    /// Spotify URL.
    pub href: Option<String>,
    /// Spotify ID.
    pub id: String,
    /// Whether the track is playable.
    pub is_playable: Option<bool>,
    /// Linked track info if relinked.
    pub linked_from: Option<LinkedFrom>,
    /// Restrictions if any.
    pub restrictions: Option<Restrictions>,
    /// Track name.
    pub name: String,
    /// Popularity score (0-100).
    pub popularity: Option<u32>,
    /// Preview URL.
    pub preview_url: Option<String>,
    /// Track number on the disc.
    pub track_number: Option<u32>,
    /// Object type (always "track").
    #[serde(rename = "type")]
    pub item_type: String,
    /// Spotify URI.
    pub uri: String,
    /// Whether the track is a local file.
    pub is_local: Option<bool>,
}

impl Track {
    /// Get the primary artist name.
    pub fn artist_name(&self) -> Option<&str> {
        self.artists
            .as_ref()
            .and_then(|artists| artists.first())
            .map(|a| a.name.as_str())
    }

    /// Get all artist names joined.
    pub fn artist_names(&self) -> String {
        self.artists
            .as_ref()
            .map(|artists| {
                artists
                    .iter()
                    .map(|a| a.name.as_str())
                    .collect::<Vec<_>>()
                    .join(", ")
            })
            .unwrap_or_default()
    }

    /// Get the album name.
    pub fn album_name(&self) -> Option<&str> {
        self.album.as_ref().map(|a| a.name.as_str())
    }

    /// Get duration as MM:SS string.
    pub fn duration_str(&self) -> String {
        let total_secs = self.duration_ms / 1000;
        let mins = total_secs / 60;
        let secs = total_secs % 60;
        format!("{}:{:02}", mins, secs)
    }

    /// Get album image URL.
    pub fn image_url(&self) -> Option<&str> {
        self.album.as_ref().and_then(|a| a.image_url())
    }
}

impl TrackSimplified {
    /// Get the primary artist name.
    pub fn artist_name(&self) -> Option<&str> {
        self.artists
            .as_ref()
            .and_then(|artists| artists.first())
            .map(|a| a.name.as_str())
    }

    /// Get duration as MM:SS string.
    pub fn duration_str(&self) -> String {
        let total_secs = self.duration_ms / 1000;
        let mins = total_secs / 60;
        let secs = total_secs % 60;
        format!("{}:{:02}", mins, secs)
    }
}

/// Saved track (wraps track with added_at timestamp).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SavedTrack {
    /// When the track was saved.
    pub added_at: String,
    /// The track.
    pub track: Track,
}
