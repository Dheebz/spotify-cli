use serde::{Deserialize, Serialize};

/// Local shortcut for a playlist URL.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PinnedPlaylist {
    pub name: String,
    pub url: String,
}
