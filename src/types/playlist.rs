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

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn playlist_simplified_deserializes() {
        let json = json!({
            "id": "playlist123",
            "name": "My Playlist",
            "type": "playlist",
            "uri": "spotify:playlist:playlist123",
            "tracks": {"total": 50}
        });
        let playlist: PlaylistSimplified = serde_json::from_value(json).unwrap();
        assert_eq!(playlist.id, "playlist123");
        assert_eq!(playlist.name, "My Playlist");
        assert_eq!(playlist.track_count(), 50);
    }

    #[test]
    fn playlist_simplified_track_count_zero_when_none() {
        let json = json!({
            "id": "playlist123",
            "name": "My Playlist",
            "type": "playlist",
            "uri": "spotify:playlist:playlist123"
        });
        let playlist: PlaylistSimplified = serde_json::from_value(json).unwrap();
        assert_eq!(playlist.track_count(), 0);
    }

    #[test]
    fn playlist_simplified_image_url() {
        let json = json!({
            "id": "playlist123",
            "name": "My Playlist",
            "type": "playlist",
            "uri": "spotify:playlist:playlist123",
            "images": [{"url": "https://cover.jpg", "height": 300, "width": 300}]
        });
        let playlist: PlaylistSimplified = serde_json::from_value(json).unwrap();
        assert_eq!(playlist.image_url(), Some("https://cover.jpg"));
    }

    #[test]
    fn playlist_full_deserializes() {
        let json = json!({
            "id": "playlist123",
            "name": "My Playlist",
            "type": "playlist",
            "uri": "spotify:playlist:playlist123",
            "owner": {"id": "user123", "type": "user", "uri": "spotify:user:user123"},
            "tracks": {"href": "https://api.spotify.com/v1/playlists/playlist123/tracks", "items": [], "total": 0, "limit": 100, "offset": 0},
            "followers": {"total": 500}
        });
        let playlist: Playlist = serde_json::from_value(json).unwrap();
        assert_eq!(playlist.id, "playlist123");
        assert_eq!(playlist.owner_name(), Some("user123"));
    }

    #[test]
    fn playlist_owner_name_prefers_display_name() {
        let json = json!({
            "id": "playlist123",
            "name": "My Playlist",
            "type": "playlist",
            "uri": "spotify:playlist:playlist123",
            "owner": {
                "id": "user123",
                "display_name": "John Doe",
                "type": "user",
                "uri": "spotify:user:user123"
            }
        });
        let playlist: Playlist = serde_json::from_value(json).unwrap();
        assert_eq!(playlist.owner_name(), Some("John Doe"));
    }

    #[test]
    fn playlist_track_deserializes() {
        let json = json!({
            "added_at": "2024-01-15T10:30:00Z",
            "is_local": false,
            "track": null
        });
        let track: PlaylistTrack = serde_json::from_value(json).unwrap();
        assert!(track.track.is_none());
        assert_eq!(track.is_local, Some(false));
    }

    #[test]
    fn playlist_tracks_ref_deserializes() {
        let json = json!({
            "href": "https://api.spotify.com/v1/playlists/123/tracks",
            "total": 42
        });
        let tracks_ref: PlaylistTracksRef = serde_json::from_value(json).unwrap();
        assert_eq!(tracks_ref.total, 42);
    }

    #[test]
    fn featured_playlists_deserializes() {
        let json = json!({
            "message": "Featured today",
            "playlists": {
                "href": "https://api.spotify.com/v1/browse/featured-playlists",
                "items": [],
                "total": 0,
                "limit": 20,
                "offset": 0
            }
        });
        let featured: FeaturedPlaylists = serde_json::from_value(json).unwrap();
        assert_eq!(featured.message, Some("Featured today".to_string()));
    }

    #[test]
    fn category_playlists_deserializes() {
        let json = json!({
            "playlists": {
                "href": "https://api.spotify.com/v1/browse/categories/pop/playlists",
                "items": [],
                "total": 0,
                "limit": 20,
                "offset": 0
            }
        });
        let category: CategoryPlaylists = serde_json::from_value(json).unwrap();
        assert!(category.playlists.items.is_empty());
    }
}
