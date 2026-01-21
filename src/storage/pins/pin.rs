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
        if url_or_id.contains("open.spotify.com") {
            url_or_id
                .split('/')
                .next_back()
                .unwrap_or(url_or_id)
                .split('?')
                .next()
                .unwrap_or(url_or_id)
                .to_string()
        } else if url_or_id.contains(':') {
            url_or_id
                .split(':')
                .next_back()
                .unwrap_or(url_or_id)
                .to_string()
        } else {
            url_or_id.to_string()
        }
    }

    /// Get the Spotify URI for this pin
    pub fn uri(&self) -> String {
        format!("spotify:{}:{}", self.resource_type.as_str(), self.id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pin_new() {
        let pin = Pin::new(
            ResourceType::Track,
            "track123".to_string(),
            "favorite".to_string(),
            vec!["rock".to_string(), "chill".to_string()],
        );
        assert_eq!(pin.id, "track123");
        assert_eq!(pin.alias, "favorite");
        assert_eq!(pin.tags.len(), 2);
    }

    #[test]
    fn pin_uri_track() {
        let pin = Pin::new(
            ResourceType::Track,
            "abc123".to_string(),
            "test".to_string(),
            vec![],
        );
        assert_eq!(pin.uri(), "spotify:track:abc123");
    }

    #[test]
    fn pin_uri_playlist() {
        let pin = Pin::new(
            ResourceType::Playlist,
            "xyz789".to_string(),
            "test".to_string(),
            vec![],
        );
        assert_eq!(pin.uri(), "spotify:playlist:xyz789");
    }

    #[test]
    fn pin_uri_album() {
        let pin = Pin::new(
            ResourceType::Album,
            "album123".to_string(),
            "test".to_string(),
            vec![],
        );
        assert_eq!(pin.uri(), "spotify:album:album123");
    }

    #[test]
    fn extract_id_from_plain_id() {
        let id = Pin::extract_id("abc123");
        assert_eq!(id, "abc123");
    }

    #[test]
    fn extract_id_from_spotify_uri() {
        let id = Pin::extract_id("spotify:track:abc123");
        assert_eq!(id, "abc123");
    }

    #[test]
    fn extract_id_from_spotify_url() {
        let id = Pin::extract_id("https://open.spotify.com/track/abc123");
        assert_eq!(id, "abc123");
    }

    #[test]
    fn extract_id_from_spotify_url_with_query() {
        let id = Pin::extract_id("https://open.spotify.com/track/abc123?si=xyz");
        assert_eq!(id, "abc123");
    }

    #[test]
    fn pin_serializes_to_json() {
        let pin = Pin::new(
            ResourceType::Track,
            "abc123".to_string(),
            "test_alias".to_string(),
            vec!["tag1".to_string()],
        );
        let json = serde_json::to_value(&pin).unwrap();
        assert_eq!(json["id"], "abc123");
        assert_eq!(json["alias"], "test_alias");
    }

    #[test]
    fn pin_deserializes_from_json() {
        let json = serde_json::json!({
            "resource_type": "track",
            "id": "abc123",
            "alias": "test_alias",
            "tags": ["tag1", "tag2"]
        });
        let pin: Pin = serde_json::from_value(json).unwrap();
        assert_eq!(pin.id, "abc123");
        assert_eq!(pin.alias, "test_alias");
        assert_eq!(pin.tags.len(), 2);
    }
}
