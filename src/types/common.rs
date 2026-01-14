//! Common types shared across multiple Spotify API responses.

use serde::{Deserialize, Serialize};

/// Image object returned by Spotify.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Image {
    /// Image URL.
    pub url: String,
    /// Image width in pixels (may be null).
    pub width: Option<u32>,
    /// Image height in pixels (may be null).
    pub height: Option<u32>,
}

/// External URLs for a Spotify object.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExternalUrls {
    /// Spotify URL for the object.
    pub spotify: Option<String>,
}

/// External IDs (ISRC, EAN, UPC).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExternalIds {
    /// International Standard Recording Code.
    pub isrc: Option<String>,
    /// International Article Number.
    pub ean: Option<String>,
    /// Universal Product Code.
    pub upc: Option<String>,
}

/// Follower information.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Followers {
    /// Spotify URL for followers (always null per API docs).
    pub href: Option<String>,
    /// Total number of followers.
    pub total: u32,
}

/// Copyright information.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Copyright {
    /// Copyright text.
    pub text: String,
    /// Copyright type: C = copyright, P = performance copyright.
    #[serde(rename = "type")]
    pub copyright_type: String,
}

/// Restriction information.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Restrictions {
    /// Reason for restriction: market, product, explicit.
    pub reason: String,
}

/// Paginated response wrapper.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Paginated<T> {
    /// URL to the API endpoint for this page.
    pub href: String,
    /// Maximum number of items in the response.
    pub limit: u32,
    /// URL to the next page (null if last page).
    pub next: Option<String>,
    /// Offset of the items returned.
    pub offset: u32,
    /// URL to the previous page (null if first page).
    pub previous: Option<String>,
    /// Total number of items available.
    pub total: u32,
    /// The requested items.
    pub items: Vec<T>,
}

/// Cursored response wrapper (for endpoints using cursor pagination).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Cursored<T> {
    /// URL to the API endpoint.
    pub href: String,
    /// Maximum number of items.
    pub limit: u32,
    /// URL to the next page.
    pub next: Option<String>,
    /// Cursors for pagination.
    pub cursors: Option<Cursors>,
    /// Total number of items (may be null).
    pub total: Option<u32>,
    /// The requested items.
    pub items: Vec<T>,
}

/// Cursor positions for cursor-based pagination.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Cursors {
    /// Cursor to the next page.
    pub after: Option<String>,
    /// Cursor to the previous page.
    pub before: Option<String>,
}

/// Resume point for podcasts/audiobooks.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResumePoint {
    /// Whether playback has been fully completed.
    pub fully_played: bool,
    /// Position in milliseconds where playback was paused.
    pub resume_position_ms: u64,
}

/// Linked track (relinked due to regional restrictions).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LinkedFrom {
    /// External URLs.
    pub external_urls: Option<ExternalUrls>,
    /// Spotify URL.
    pub href: Option<String>,
    /// Spotify ID.
    pub id: Option<String>,
    /// Object type.
    #[serde(rename = "type")]
    pub item_type: Option<String>,
    /// Spotify URI.
    pub uri: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn image_deserializes() {
        let json = json!({
            "url": "https://image.jpg",
            "width": 640,
            "height": 480
        });
        let image: Image = serde_json::from_value(json).unwrap();
        assert_eq!(image.url, "https://image.jpg");
        assert_eq!(image.width, Some(640));
        assert_eq!(image.height, Some(480));
    }

    #[test]
    fn image_deserializes_with_null_dimensions() {
        let json = json!({
            "url": "https://image.jpg"
        });
        let image: Image = serde_json::from_value(json).unwrap();
        assert!(image.width.is_none());
        assert!(image.height.is_none());
    }

    #[test]
    fn external_urls_deserializes() {
        let json = json!({
            "spotify": "https://open.spotify.com/track/123"
        });
        let urls: ExternalUrls = serde_json::from_value(json).unwrap();
        assert_eq!(urls.spotify, Some("https://open.spotify.com/track/123".to_string()));
    }

    #[test]
    fn external_ids_deserializes() {
        let json = json!({
            "isrc": "USRC12345678",
            "ean": "1234567890123",
            "upc": "012345678905"
        });
        let ids: ExternalIds = serde_json::from_value(json).unwrap();
        assert_eq!(ids.isrc, Some("USRC12345678".to_string()));
        assert_eq!(ids.ean, Some("1234567890123".to_string()));
        assert_eq!(ids.upc, Some("012345678905".to_string()));
    }

    #[test]
    fn followers_deserializes() {
        let json = json!({
            "total": 1000000
        });
        let followers: Followers = serde_json::from_value(json).unwrap();
        assert_eq!(followers.total, 1000000);
        assert!(followers.href.is_none());
    }

    #[test]
    fn copyright_deserializes() {
        let json = json!({
            "text": "(C) 2024 Test Records",
            "type": "C"
        });
        let copyright: Copyright = serde_json::from_value(json).unwrap();
        assert_eq!(copyright.text, "(C) 2024 Test Records");
        assert_eq!(copyright.copyright_type, "C");
    }

    #[test]
    fn restrictions_deserializes() {
        let json = json!({
            "reason": "market"
        });
        let restrictions: Restrictions = serde_json::from_value(json).unwrap();
        assert_eq!(restrictions.reason, "market");
    }

    #[test]
    fn paginated_deserializes() {
        let json = json!({
            "href": "https://api.spotify.com/v1/me/tracks",
            "limit": 20,
            "offset": 0,
            "total": 100,
            "items": [1, 2, 3]
        });
        let paginated: Paginated<i32> = serde_json::from_value(json).unwrap();
        assert_eq!(paginated.limit, 20);
        assert_eq!(paginated.total, 100);
        assert_eq!(paginated.items, vec![1, 2, 3]);
    }

    #[test]
    fn paginated_with_next_prev() {
        let json = json!({
            "href": "https://api.spotify.com/v1/me/tracks?offset=20",
            "limit": 20,
            "offset": 20,
            "total": 100,
            "next": "https://api.spotify.com/v1/me/tracks?offset=40",
            "previous": "https://api.spotify.com/v1/me/tracks?offset=0",
            "items": []
        });
        let paginated: Paginated<i32> = serde_json::from_value(json).unwrap();
        assert!(paginated.next.is_some());
        assert!(paginated.previous.is_some());
    }

    #[test]
    fn cursored_deserializes() {
        let json = json!({
            "href": "https://api.spotify.com/v1/me/following",
            "limit": 20,
            "total": 50,
            "items": ["a", "b"],
            "cursors": {"after": "cursor123", "before": "cursor000"}
        });
        let cursored: Cursored<String> = serde_json::from_value(json).unwrap();
        assert_eq!(cursored.limit, 20);
        assert_eq!(cursored.items.len(), 2);
        assert!(cursored.cursors.is_some());
    }

    #[test]
    fn cursors_deserializes() {
        let json = json!({
            "after": "next_cursor",
            "before": "prev_cursor"
        });
        let cursors: Cursors = serde_json::from_value(json).unwrap();
        assert_eq!(cursors.after, Some("next_cursor".to_string()));
        assert_eq!(cursors.before, Some("prev_cursor".to_string()));
    }

    #[test]
    fn resume_point_deserializes() {
        let json = json!({
            "fully_played": false,
            "resume_position_ms": 120000
        });
        let resume: ResumePoint = serde_json::from_value(json).unwrap();
        assert!(!resume.fully_played);
        assert_eq!(resume.resume_position_ms, 120000);
    }

    #[test]
    fn linked_from_deserializes() {
        let json = json!({
            "id": "track123",
            "type": "track",
            "uri": "spotify:track:track123",
            "href": "https://api.spotify.com/v1/tracks/track123"
        });
        let linked: LinkedFrom = serde_json::from_value(json).unwrap();
        assert_eq!(linked.id, Some("track123".to_string()));
        assert_eq!(linked.item_type, Some("track".to_string()));
    }
}
