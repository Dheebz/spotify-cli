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
