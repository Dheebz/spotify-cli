use serde::{Deserialize, Serialize};

/// Playback device metadata.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Device {
    pub id: String,
    pub name: String,
    pub volume_percent: Option<u32>,
}
