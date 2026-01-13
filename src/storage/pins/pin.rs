use serde::{Deserialize, Serialize};

use super::resource_type::ResourceType;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Pin {
    pub resource_type: ResourceType,
    pub id: String,
    pub alias: String,
    pub tags: Vec<String>,
}

impl Pin {
    pub fn new(resource_type: ResourceType, id: String, alias: String, tags: Vec<String>) -> Self {
        Self {
            resource_type,
            id,
            alias,
            tags,
        }
    }

    /// Extract Spotify ID from a URL or return the input if it's already an ID
    pub fn extract_id(url_or_id: &str) -> String {
        // Handle URLs like:
        // https://open.spotify.com/playlist/37i9dQZEVXbsdW9lIOtMPR?si=abc123
        // spotify:playlist:37i9dQZEVXbsdW9lIOtMPR
        if url_or_id.contains("open.spotify.com") {
            // URL format
            url_or_id
                .split('/')
                .next_back()
                .unwrap_or(url_or_id)
                .split('?')
                .next()
                .unwrap_or(url_or_id)
                .to_string()
        } else if url_or_id.contains(':') {
            // URI format (spotify:type:id)
            url_or_id
                .split(':')
                .next_back()
                .unwrap_or(url_or_id)
                .to_string()
        } else {
            // Already an ID
            url_or_id.to_string()
        }
    }

    /// Get the Spotify URI for this pin
    pub fn uri(&self) -> String {
        format!("spotify:{}:{}", self.resource_type.as_str(), self.id)
    }
}
