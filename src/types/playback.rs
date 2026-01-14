//! Playback state types from Spotify API.

use serde::{Deserialize, Serialize};

use super::common::ExternalUrls;
use super::track::Track;

/// Current playback state.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlaybackState {
    /// The device currently playing.
    pub device: Option<Device>,
    /// Repeat mode: off, track, context.
    pub repeat_state: Option<String>,
    /// Whether shuffle is on.
    pub shuffle_state: Option<bool>,
    /// Playback context (album, playlist, etc.).
    pub context: Option<PlaybackContext>,
    /// Unix timestamp of when data was fetched.
    pub timestamp: Option<u64>,
    /// Progress into the currently playing track (ms).
    pub progress_ms: Option<u64>,
    /// Whether something is currently playing.
    pub is_playing: bool,
    /// The currently playing track/episode.
    pub item: Option<Track>,
    /// Currently playing type: track, episode, ad, unknown.
    pub currently_playing_type: Option<String>,
    /// Actions available/restricted.
    pub actions: Option<PlaybackActions>,
}

/// Playback context (what's being played from).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlaybackContext {
    /// Context type: album, artist, playlist.
    #[serde(rename = "type")]
    pub context_type: Option<String>,
    /// Spotify URL.
    pub href: Option<String>,
    /// External URLs.
    pub external_urls: Option<ExternalUrls>,
    /// Spotify URI.
    pub uri: Option<String>,
}

/// Available playback actions.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlaybackActions {
    /// Whether interrupting playback is allowed.
    pub interrupting_playback: Option<bool>,
    /// Whether pausing is allowed.
    pub pausing: Option<bool>,
    /// Whether resuming is allowed.
    pub resuming: Option<bool>,
    /// Whether seeking is allowed.
    pub seeking: Option<bool>,
    /// Whether skipping next is allowed.
    pub skipping_next: Option<bool>,
    /// Whether skipping previous is allowed.
    pub skipping_prev: Option<bool>,
    /// Whether toggling repeat context is allowed.
    pub toggling_repeat_context: Option<bool>,
    /// Whether toggling shuffle is allowed.
    pub toggling_shuffle: Option<bool>,
    /// Whether toggling repeat track is allowed.
    pub toggling_repeat_track: Option<bool>,
    /// Whether transferring playback is allowed.
    pub transferring_playback: Option<bool>,
}

/// Device information.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Device {
    /// Device ID.
    pub id: Option<String>,
    /// Whether this is the currently active device.
    pub is_active: bool,
    /// Whether the device is in a private session.
    pub is_private_session: Option<bool>,
    /// Whether controlling this device is restricted.
    pub is_restricted: Option<bool>,
    /// Device name.
    pub name: String,
    /// Device type: computer, smartphone, speaker, etc.
    #[serde(rename = "type")]
    pub device_type: String,
    /// Current volume percentage.
    pub volume_percent: Option<u32>,
    /// Whether the device supports volume control.
    pub supports_volume: Option<bool>,
}

/// Devices response.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DevicesResponse {
    /// List of devices.
    pub devices: Vec<Device>,
}

/// Queue response.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueueResponse {
    /// Currently playing track.
    pub currently_playing: Option<Track>,
    /// Upcoming tracks in the queue.
    pub queue: Vec<Track>,
}

/// Recently played item.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayHistory {
    /// The track that was played.
    pub track: Track,
    /// When the track was played.
    pub played_at: String,
    /// Playback context.
    pub context: Option<PlaybackContext>,
}

/// Recently played response (cursor-paginated).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecentlyPlayedResponse {
    /// URL to the API endpoint.
    pub href: Option<String>,
    /// Maximum number of items.
    pub limit: Option<u32>,
    /// URL to the next page.
    pub next: Option<String>,
    /// Cursors for pagination.
    pub cursors: Option<RecentlyPlayedCursors>,
    /// Total count (may be null).
    pub total: Option<u32>,
    /// The recently played items.
    pub items: Vec<PlayHistory>,
}

/// Cursors for recently played pagination.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecentlyPlayedCursors {
    /// Cursor to the next page.
    pub after: Option<String>,
    /// Cursor to the previous page.
    pub before: Option<String>,
}

impl PlaybackState {
    /// Get the currently playing track name.
    pub fn track_name(&self) -> Option<&str> {
        self.item.as_ref().map(|t| t.name.as_str())
    }

    /// Get the currently playing artist name.
    pub fn artist_name(&self) -> Option<&str> {
        self.item.as_ref().and_then(|t| t.artist_name())
    }

    /// Get progress as MM:SS string.
    pub fn progress_str(&self) -> String {
        let ms = self.progress_ms.unwrap_or(0);
        let total_secs = ms / 1000;
        let mins = total_secs / 60;
        let secs = total_secs % 60;
        format!("{}:{:02}", mins, secs)
    }

    /// Get duration as MM:SS string.
    pub fn duration_str(&self) -> String {
        self.item
            .as_ref()
            .map(|t| t.duration_str())
            .unwrap_or_else(|| "0:00".to_string())
    }

    /// Get the device name.
    pub fn device_name(&self) -> Option<&str> {
        self.device.as_ref().map(|d| d.name.as_str())
    }
}

impl Device {
    /// Get a display-friendly device type.
    pub fn device_type_display(&self) -> &str {
        match self.device_type.as_str() {
            "Computer" => "Computer",
            "Smartphone" => "Phone",
            "Speaker" => "Speaker",
            "TV" => "TV",
            "AVR" => "Receiver",
            "STB" => "Set-top Box",
            "AudioDongle" => "Audio Dongle",
            "GameConsole" => "Game Console",
            "CastVideo" => "Cast",
            "CastAudio" => "Cast",
            "Automobile" => "Car",
            "Tablet" => "Tablet",
            _ => &self.device_type,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn device_deserializes() {
        let json = json!({
            "id": "device123",
            "is_active": true,
            "name": "Living Room Speaker",
            "type": "Speaker",
            "volume_percent": 75
        });
        let device: Device = serde_json::from_value(json).unwrap();
        assert_eq!(device.id, Some("device123".to_string()));
        assert!(device.is_active);
        assert_eq!(device.name, "Living Room Speaker");
        assert_eq!(device.device_type, "Speaker");
    }

    #[test]
    fn device_type_display_known_types() {
        let make_device = |t: &str| Device {
            id: None,
            is_active: false,
            is_private_session: None,
            is_restricted: None,
            name: "Test".to_string(),
            device_type: t.to_string(),
            volume_percent: None,
            supports_volume: None,
        };

        assert_eq!(make_device("Computer").device_type_display(), "Computer");
        assert_eq!(make_device("Smartphone").device_type_display(), "Phone");
        assert_eq!(make_device("Speaker").device_type_display(), "Speaker");
        assert_eq!(make_device("TV").device_type_display(), "TV");
        assert_eq!(make_device("AVR").device_type_display(), "Receiver");
        assert_eq!(make_device("Automobile").device_type_display(), "Car");
        assert_eq!(make_device("Tablet").device_type_display(), "Tablet");
    }

    #[test]
    fn device_type_display_unknown() {
        let device = Device {
            id: None,
            is_active: false,
            is_private_session: None,
            is_restricted: None,
            name: "Test".to_string(),
            device_type: "NewDeviceType".to_string(),
            volume_percent: None,
            supports_volume: None,
        };
        assert_eq!(device.device_type_display(), "NewDeviceType");
    }

    #[test]
    fn devices_response_deserializes() {
        let json = json!({
            "devices": [
                {"id": "dev1", "is_active": true, "name": "Device 1", "type": "Computer"},
                {"id": "dev2", "is_active": false, "name": "Device 2", "type": "Speaker"}
            ]
        });
        let resp: DevicesResponse = serde_json::from_value(json).unwrap();
        assert_eq!(resp.devices.len(), 2);
    }

    #[test]
    fn playback_context_deserializes() {
        let json = json!({
            "type": "playlist",
            "uri": "spotify:playlist:abc123",
            "href": "https://api.spotify.com/v1/playlists/abc123"
        });
        let ctx: PlaybackContext = serde_json::from_value(json).unwrap();
        assert_eq!(ctx.context_type, Some("playlist".to_string()));
    }

    #[test]
    fn playback_actions_deserializes() {
        let json = json!({
            "pausing": true,
            "resuming": true,
            "seeking": true,
            "skipping_next": true,
            "skipping_prev": false
        });
        let actions: PlaybackActions = serde_json::from_value(json).unwrap();
        assert_eq!(actions.pausing, Some(true));
        assert_eq!(actions.skipping_prev, Some(false));
    }

    #[test]
    fn playback_state_deserializes() {
        let json = json!({
            "is_playing": true,
            "progress_ms": 60000,
            "repeat_state": "off",
            "shuffle_state": false
        });
        let state: PlaybackState = serde_json::from_value(json).unwrap();
        assert!(state.is_playing);
        assert_eq!(state.progress_ms, Some(60000));
        assert_eq!(state.repeat_state, Some("off".to_string()));
    }

    #[test]
    fn playback_state_track_name() {
        let json = json!({
            "is_playing": true,
            "item": {
                "id": "track1",
                "name": "Test Track",
                "type": "track",
                "uri": "spotify:track:track1",
                "duration_ms": 200000
            }
        });
        let state: PlaybackState = serde_json::from_value(json).unwrap();
        assert_eq!(state.track_name(), Some("Test Track"));
    }

    #[test]
    fn playback_state_track_name_none() {
        let json = json!({
            "is_playing": false
        });
        let state: PlaybackState = serde_json::from_value(json).unwrap();
        assert!(state.track_name().is_none());
    }

    #[test]
    fn playback_state_progress_str() {
        let json = json!({
            "is_playing": true,
            "progress_ms": 125000  // 2:05
        });
        let state: PlaybackState = serde_json::from_value(json).unwrap();
        assert_eq!(state.progress_str(), "2:05");
    }

    #[test]
    fn playback_state_progress_str_zero() {
        let json = json!({
            "is_playing": false
        });
        let state: PlaybackState = serde_json::from_value(json).unwrap();
        assert_eq!(state.progress_str(), "0:00");
    }

    #[test]
    fn playback_state_duration_str() {
        let json = json!({
            "is_playing": true,
            "item": {
                "id": "track1",
                "name": "Test",
                "type": "track",
                "uri": "spotify:track:track1",
                "duration_ms": 210000  // 3:30
            }
        });
        let state: PlaybackState = serde_json::from_value(json).unwrap();
        assert_eq!(state.duration_str(), "3:30");
    }

    #[test]
    fn playback_state_duration_str_no_item() {
        let json = json!({
            "is_playing": false
        });
        let state: PlaybackState = serde_json::from_value(json).unwrap();
        assert_eq!(state.duration_str(), "0:00");
    }

    #[test]
    fn playback_state_device_name() {
        let json = json!({
            "is_playing": true,
            "device": {
                "id": "dev1",
                "is_active": true,
                "name": "My Computer",
                "type": "Computer"
            }
        });
        let state: PlaybackState = serde_json::from_value(json).unwrap();
        assert_eq!(state.device_name(), Some("My Computer"));
    }

    #[test]
    fn queue_response_deserializes() {
        let json = json!({
            "currently_playing": null,
            "queue": []
        });
        let queue: QueueResponse = serde_json::from_value(json).unwrap();
        assert!(queue.currently_playing.is_none());
        assert!(queue.queue.is_empty());
    }

    #[test]
    fn play_history_deserializes() {
        let json = json!({
            "track": {
                "id": "track1",
                "name": "Recent Track",
                "type": "track",
                "uri": "spotify:track:track1",
                "duration_ms": 180000
            },
            "played_at": "2024-01-15T10:30:00Z"
        });
        let history: PlayHistory = serde_json::from_value(json).unwrap();
        assert_eq!(history.track.name, "Recent Track");
        assert_eq!(history.played_at, "2024-01-15T10:30:00Z");
    }

    #[test]
    fn recently_played_response_deserializes() {
        let json = json!({
            "items": [],
            "limit": 20
        });
        let resp: RecentlyPlayedResponse = serde_json::from_value(json).unwrap();
        assert!(resp.items.is_empty());
        assert_eq!(resp.limit, Some(20));
    }

    #[test]
    fn recently_played_cursors_deserializes() {
        let json = json!({
            "after": "1234567890",
            "before": "0987654321"
        });
        let cursors: RecentlyPlayedCursors = serde_json::from_value(json).unwrap();
        assert_eq!(cursors.after, Some("1234567890".to_string()));
        assert_eq!(cursors.before, Some("0987654321".to_string()));
    }
}
