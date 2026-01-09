use serde::{Deserialize, Serialize};

/// Artist metadata for info output.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Artist {
    pub id: String,
    pub name: String,
    pub uri: String,
    pub genres: Vec<String>,
    pub followers: Option<u64>,
}
