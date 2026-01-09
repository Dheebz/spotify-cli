use serde::{Deserialize, Serialize};

/// Minimal playlist representation for listing and selection.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Playlist {
    pub id: String,
    pub name: String,
    pub owner: Option<String>,
    #[serde(default)]
    pub collaborative: bool,
    #[serde(default)]
    pub public: Option<bool>,
}

/// Detailed playlist metadata for info commands.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlaylistDetail {
    pub id: String,
    pub name: String,
    pub uri: String,
    pub owner: Option<String>,
    pub tracks_total: Option<u32>,
    #[serde(default)]
    pub collaborative: bool,
    #[serde(default)]
    pub public: Option<bool>,
}
