//! Event polling and broadcasting
//!
//! Polls Spotify for playback state changes and broadcasts events to subscribers.

use std::time::Duration;

use serde_json::Value;
use tokio::sync::broadcast;
use tokio::time::interval;
use tracing::{debug, trace, warn};

use crate::cli::commands::get_authenticated_client;
use crate::endpoints::player::get_playback_state;

use super::protocol::RpcNotification;

/// Event types that clients can subscribe to
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EventType {
    TrackChanged,
    PlaybackStateChanged,
    VolumeChanged,
    ShuffleChanged,
    RepeatChanged,
    DeviceChanged,
}

/// Playback state snapshot for change detection
#[derive(Debug, Clone, Default)]
struct PlaybackSnapshot {
    track_id: Option<String>,
    is_playing: bool,
    volume: u8,
    shuffle: bool,
    repeat: String,
    device_id: Option<String>,
}

impl PlaybackSnapshot {
    fn from_json(state: &Value) -> Self {
        Self {
            track_id: state
                .get("item")
                .and_then(|i| i.get("id"))
                .and_then(|v| v.as_str())
                .map(String::from),
            is_playing: state
                .get("is_playing")
                .and_then(|v| v.as_bool())
                .unwrap_or(false),
            volume: state
                .get("device")
                .and_then(|d| d.get("volume_percent"))
                .and_then(|v| v.as_u64())
                .unwrap_or(0) as u8,
            shuffle: state
                .get("shuffle_state")
                .and_then(|v| v.as_bool())
                .unwrap_or(false),
            repeat: state
                .get("repeat_state")
                .and_then(|v| v.as_str())
                .unwrap_or("off")
                .to_string(),
            device_id: state
                .get("device")
                .and_then(|d| d.get("id"))
                .and_then(|v| v.as_str())
                .map(String::from),
        }
    }
}

/// Event poller that monitors Spotify playback state
pub struct EventPoller {
    event_tx: broadcast::Sender<RpcNotification>,
    poll_interval: Duration,
}

impl EventPoller {
    pub fn new(event_tx: broadcast::Sender<RpcNotification>) -> Self {
        Self {
            event_tx,
            poll_interval: Duration::from_secs(2),
        }
    }

    /// Set the polling interval
    pub fn with_interval(mut self, interval: Duration) -> Self {
        self.poll_interval = interval;
        self
    }

    /// Run the event polling loop
    pub async fn run(&self) {
        let mut interval = interval(self.poll_interval);
        let mut last_state = PlaybackSnapshot::default();

        loop {
            interval.tick().await;

            match self.poll_playback_state().await {
                Some(current) => {
                    self.detect_and_broadcast_changes(&last_state, &current).await;
                    last_state = current;
                }
                None => {
                    trace!("No playback state available");
                }
            }
        }
    }

    async fn poll_playback_state(&self) -> Option<PlaybackSnapshot> {
        let client = match get_authenticated_client().await {
            Ok(c) => c,
            Err(_) => {
                trace!("Not authenticated, skipping poll");
                return None;
            }
        };

        match get_playback_state::get_playback_state(&client).await {
            Ok(Some(state)) => Some(PlaybackSnapshot::from_json(&state)),
            Ok(None) => None,
            Err(e) => {
                warn!(error = %e, "Failed to poll playback state");
                None
            }
        }
    }

    async fn detect_and_broadcast_changes(
        &self,
        old: &PlaybackSnapshot,
        new: &PlaybackSnapshot,
    ) {
        // Track changed
        if old.track_id != new.track_id {
            debug!(old = ?old.track_id, new = ?new.track_id, "Track changed");
            self.broadcast("event.trackChanged", serde_json::json!({
                "track_id": new.track_id,
            }));
        }

        // Playback state changed (play/pause)
        if old.is_playing != new.is_playing {
            debug!(is_playing = new.is_playing, "Playback state changed");
            self.broadcast("event.playbackStateChanged", serde_json::json!({
                "is_playing": new.is_playing,
            }));
        }

        // Volume changed
        if old.volume != new.volume {
            debug!(volume = new.volume, "Volume changed");
            self.broadcast("event.volumeChanged", serde_json::json!({
                "volume": new.volume,
            }));
        }

        // Shuffle changed
        if old.shuffle != new.shuffle {
            debug!(shuffle = new.shuffle, "Shuffle changed");
            self.broadcast("event.shuffleChanged", serde_json::json!({
                "shuffle": new.shuffle,
            }));
        }

        // Repeat changed
        if old.repeat != new.repeat {
            debug!(repeat = %new.repeat, "Repeat changed");
            self.broadcast("event.repeatChanged", serde_json::json!({
                "repeat": new.repeat,
            }));
        }

        // Device changed
        if old.device_id != new.device_id {
            debug!(device = ?new.device_id, "Device changed");
            self.broadcast("event.deviceChanged", serde_json::json!({
                "device_id": new.device_id,
            }));
        }
    }

    fn broadcast(&self, method: &str, params: Value) {
        let notification = RpcNotification::new(method, Some(params));
        // Ignore send errors - no subscribers is fine
        let _ = self.event_tx.send(notification);
    }
}
