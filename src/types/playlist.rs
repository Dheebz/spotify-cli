//! Playlist types from Spotify API.

use serde::{Deserialize, Serialize};

use super::common::{ExternalUrls, Followers, Image, Paginated};
use super::track::Track;
use super::user::UserPublic;

/// Simplified playlist object.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlaylistSimplified {
    /// Whether the playlist is collaborative.
    pub collaborative: Option<bool>,
    /// Playlist description.
    pub description: Option<String>,
    /// External URLs.
    pub external_urls: Option<ExternalUrls>,
    /// Spotify URL.
    pub href: Option<String>,
    /// Spotify ID.
    pub id: String,
    /// Playlist cover images.
    pub images: Option<Vec<Image>>,
    /// Playlist name.
    pub name: String,
    /// Playlist owner.
    pub owner: Option<UserPublic>,
    /// Whether the playlist is public.
    pub public: Option<bool>,
    /// Snapshot ID (version identifier).
    pub snapshot_id: Option<String>,
    /// Tracks summary (href and total only in simplified).
    pub tracks: Option<PlaylistTracksRef>,
    /// Object type (always "playlist").
    #[serde(rename = "type")]
    pub item_type: String,
    /// Spotify URI.
    pub uri: String,
}

/// Reference to playlist tracks (used in simplified playlist).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlaylistTracksRef {
    /// URL to fetch tracks.
    pub href: Option<String>,
    /// Total number of tracks.
    pub total: u32,
}

/// Full playlist object.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Playlist {
    /// Whether the playlist is collaborative.
    pub collaborative: Option<bool>,
    /// Playlist description.
    pub description: Option<String>,
    /// External URLs.
    pub external_urls: Option<ExternalUrls>,
    /// Follower information.
    pub followers: Option<Followers>,
    /// Spotify URL.
    pub href: Option<String>,
    /// Spotify ID.
    pub id: String,
    /// Playlist cover images.
    pub images: Option<Vec<Image>>,
    /// Playlist name.
    pub name: String,
    /// Playlist owner.
    pub owner: Option<UserPublic>,
    /// Whether the playlist is public.
    pub public: Option<bool>,
    /// Snapshot ID.
    pub snapshot_id: Option<String>,
    /// Playlist tracks (paginated).
    pub tracks: Option<Paginated<PlaylistTrack>>,
    /// Object type.
    #[serde(rename = "type")]
    pub item_type: String,
    /// Spotify URI.
    pub uri: String,
}

/// Track in a playlist (wraps track with metadata).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlaylistTrack {
    /// When the track was added.
    pub added_at: Option<String>,
    /// User who added the track.
    pub added_by: Option<UserPublic>,
    /// Whether the track is a local file.
    pub is_local: Option<bool>,
    /// The track (can be null for deleted tracks).
    pub track: Option<Track>,
}

impl Playlist {
    /// Get the largest image URL if available.
    pub fn image_url(&self) -> Option<&str> {
        self.images
            .as_ref()
            .and_then(|imgs| imgs.first())
            .map(|img| img.url.as_str())
    }

    /// Get the owner's display name.
    pub fn owner_name(&self) -> Option<&str> {
        self.owner
            .as_ref()
            .and_then(|o| o.display_name.as_deref().or(Some(o.id.as_str())))
    }

    /// Get track count.
    pub fn track_count(&self) -> u32 {
        self.tracks.as_ref().map(|t| t.total).unwrap_or(0)
    }
}

impl PlaylistSimplified {
    /// Get the largest image URL if available.
    pub fn image_url(&self) -> Option<&str> {
        self.images
            .as_ref()
            .and_then(|imgs| imgs.first())
            .map(|img| img.url.as_str())
    }

    /// Get track count.
    pub fn track_count(&self) -> u32 {
        self.tracks.as_ref().map(|t| t.total).unwrap_or(0)
    }
}

/// Featured playlists response.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeaturedPlaylists {
    /// Message from Spotify.
    pub message: Option<String>,
    /// Paginated playlists.
    pub playlists: Paginated<PlaylistSimplified>,
}

/// Category playlists response.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CategoryPlaylists {
    /// Paginated playlists.
    pub playlists: Paginated<PlaylistSimplified>,
}
