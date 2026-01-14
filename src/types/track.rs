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

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn track_simplified_deserializes() {
        let json = json!({
            "id": "track123",
            "name": "Test Song",
            "type": "track",
            "uri": "spotify:track:track123",
            "duration_ms": 210000,
            "track_number": 5,
            "disc_number": 1
        });
        let track: TrackSimplified = serde_json::from_value(json).unwrap();
        assert_eq!(track.id, "track123");
        assert_eq!(track.name, "Test Song");
        assert_eq!(track.duration_ms, 210000);
    }

    #[test]
    fn track_simplified_artist_name() {
        let json = json!({
            "id": "track123",
            "name": "Test Song",
            "type": "track",
            "uri": "spotify:track:track123",
            "duration_ms": 210000,
            "artists": [{"id": "artist1", "name": "Test Artist", "type": "artist", "uri": "spotify:artist:artist1"}]
        });
        let track: TrackSimplified = serde_json::from_value(json).unwrap();
        assert_eq!(track.artist_name(), Some("Test Artist"));
    }

    #[test]
    fn track_simplified_artist_name_none() {
        let json = json!({
            "id": "track123",
            "name": "Test Song",
            "type": "track",
            "uri": "spotify:track:track123",
            "duration_ms": 210000
        });
        let track: TrackSimplified = serde_json::from_value(json).unwrap();
        assert!(track.artist_name().is_none());
    }

    #[test]
    fn track_simplified_duration_str() {
        let json = json!({
            "id": "track123",
            "name": "Test Song",
            "type": "track",
            "uri": "spotify:track:track123",
            "duration_ms": 210000  // 3:30
        });
        let track: TrackSimplified = serde_json::from_value(json).unwrap();
        assert_eq!(track.duration_str(), "3:30");
    }

    #[test]
    fn track_full_deserializes() {
        let json = json!({
            "id": "track123",
            "name": "Test Song",
            "type": "track",
            "uri": "spotify:track:track123",
            "duration_ms": 210000,
            "popularity": 75,
            "explicit": true
        });
        let track: Track = serde_json::from_value(json).unwrap();
        assert_eq!(track.id, "track123");
        assert_eq!(track.popularity, Some(75));
        assert_eq!(track.explicit, Some(true));
    }

    #[test]
    fn track_artist_name() {
        let json = json!({
            "id": "track123",
            "name": "Test Song",
            "type": "track",
            "uri": "spotify:track:track123",
            "duration_ms": 210000,
            "artists": [{"id": "artist1", "name": "Primary Artist", "type": "artist", "uri": "spotify:artist:artist1"}]
        });
        let track: Track = serde_json::from_value(json).unwrap();
        assert_eq!(track.artist_name(), Some("Primary Artist"));
    }

    #[test]
    fn track_artist_names_multiple() {
        let json = json!({
            "id": "track123",
            "name": "Test Song",
            "type": "track",
            "uri": "spotify:track:track123",
            "duration_ms": 210000,
            "artists": [
                {"id": "artist1", "name": "Artist One", "type": "artist", "uri": "spotify:artist:artist1"},
                {"id": "artist2", "name": "Artist Two", "type": "artist", "uri": "spotify:artist:artist2"}
            ]
        });
        let track: Track = serde_json::from_value(json).unwrap();
        assert_eq!(track.artist_names(), "Artist One, Artist Two");
    }

    #[test]
    fn track_artist_names_empty() {
        let json = json!({
            "id": "track123",
            "name": "Test Song",
            "type": "track",
            "uri": "spotify:track:track123",
            "duration_ms": 210000
        });
        let track: Track = serde_json::from_value(json).unwrap();
        assert_eq!(track.artist_names(), "");
    }

    #[test]
    fn track_album_name() {
        let json = json!({
            "id": "track123",
            "name": "Test Song",
            "type": "track",
            "uri": "spotify:track:track123",
            "duration_ms": 210000,
            "album": {"id": "album1", "name": "Test Album", "type": "album", "uri": "spotify:album:album1"}
        });
        let track: Track = serde_json::from_value(json).unwrap();
        assert_eq!(track.album_name(), Some("Test Album"));
    }

    #[test]
    fn track_album_name_none() {
        let json = json!({
            "id": "track123",
            "name": "Test Song",
            "type": "track",
            "uri": "spotify:track:track123",
            "duration_ms": 210000
        });
        let track: Track = serde_json::from_value(json).unwrap();
        assert!(track.album_name().is_none());
    }

    #[test]
    fn track_duration_str() {
        let json = json!({
            "id": "track123",
            "name": "Test Song",
            "type": "track",
            "uri": "spotify:track:track123",
            "duration_ms": 185000  // 3:05
        });
        let track: Track = serde_json::from_value(json).unwrap();
        assert_eq!(track.duration_str(), "3:05");
    }

    #[test]
    fn track_image_url() {
        let json = json!({
            "id": "track123",
            "name": "Test Song",
            "type": "track",
            "uri": "spotify:track:track123",
            "duration_ms": 210000,
            "album": {
                "id": "album1",
                "name": "Test Album",
                "type": "album",
                "uri": "spotify:album:album1",
                "images": [{"url": "https://cover.jpg", "height": 640, "width": 640}]
            }
        });
        let track: Track = serde_json::from_value(json).unwrap();
        assert_eq!(track.image_url(), Some("https://cover.jpg"));
    }

    #[test]
    fn saved_track_deserializes() {
        let json = json!({
            "added_at": "2024-01-15T10:30:00Z",
            "track": {
                "id": "track123",
                "name": "Test Song",
                "type": "track",
                "uri": "spotify:track:track123",
                "duration_ms": 210000
            }
        });
        let saved: SavedTrack = serde_json::from_value(json).unwrap();
        assert_eq!(saved.added_at, "2024-01-15T10:30:00Z");
        assert_eq!(saved.track.id, "track123");
    }
}
