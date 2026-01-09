use crate::domain::{device::Device, track::Track};

/// Playback context for the current player session.
#[derive(Debug, Clone)]
pub struct PlaybackContext {
    pub kind: String,
    pub uri: String,
}

/// Playback status from the Spotify player endpoint.
#[derive(Debug, Clone)]
pub struct PlayerStatus {
    pub is_playing: bool,
    pub track: Option<Track>,
    pub device: Option<Device>,
    pub context: Option<PlaybackContext>,
    pub progress_ms: Option<u32>,
    pub repeat_state: Option<String>,
    pub shuffle_state: Option<bool>,
}
