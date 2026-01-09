use serde::{Deserialize, Serialize};

/// Search result kinds supported by Spotify search.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SearchType {
    All,
    Track,
    Album,
    Artist,
    Playlist,
}

/// Normalized search item across Spotify result types.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchItem {
    pub id: String,
    pub name: String,
    pub uri: String,
    /// Item kind for mixed searches.
    pub kind: SearchType,
    /// Artist names for track/album results.
    pub artists: Vec<String>,
    /// Album name for track results.
    pub album: Option<String>,
    /// Track duration in milliseconds for track results.
    pub duration_ms: Option<u32>,
    /// Owner display name for playlist results.
    pub owner: Option<String>,
    /// Optional fuzzy score, 0.0..=1.0.
    pub score: Option<f32>,
}

/// Aggregated search results with a kind discriminator.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResults {
    pub kind: SearchType,
    pub items: Vec<SearchItem>,
}
