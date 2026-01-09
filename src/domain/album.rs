use serde::{Deserialize, Serialize};

/// Album metadata plus track listing.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Album {
    pub id: String,
    pub name: String,
    pub uri: String,
    pub artists: Vec<String>,
    pub release_date: Option<String>,
    pub total_tracks: Option<u32>,
    pub tracks: Vec<AlbumTrack>,
    pub duration_ms: Option<u64>,
}

/// Album track entry for album info output.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlbumTrack {
    pub name: String,
    pub duration_ms: u32,
    pub track_number: u32,
}
