/// Track metadata used in playback and library actions.
#[derive(Debug, Clone)]
pub struct Track {
    pub id: String,
    pub name: String,
    pub artists: Vec<String>,
    pub artist_ids: Vec<String>,
    pub album: Option<String>,
    pub album_id: Option<String>,
    pub duration_ms: Option<u32>,
}
