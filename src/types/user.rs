//! User types from Spotify API.

use serde::{Deserialize, Serialize};

use super::common::{ExternalUrls, Followers, Image};

/// Public user profile (limited information).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserPublic {
    /// Display name.
    pub display_name: Option<String>,
    /// External URLs.
    pub external_urls: Option<ExternalUrls>,
    /// Follower information.
    pub followers: Option<Followers>,
    /// Spotify URL.
    pub href: Option<String>,
    /// Spotify user ID.
    pub id: String,
    /// User profile images.
    pub images: Option<Vec<Image>>,
    /// Object type (always "user").
    #[serde(rename = "type")]
    pub item_type: String,
    /// Spotify URI.
    pub uri: String,
}

/// Private user profile (current user with full details).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserPrivate {
    /// Country (ISO 3166-1 alpha-2).
    pub country: Option<String>,
    /// Display name.
    pub display_name: Option<String>,
    /// Email address.
    pub email: Option<String>,
    /// Explicit content settings.
    pub explicit_content: Option<ExplicitContent>,
    /// External URLs.
    pub external_urls: Option<ExternalUrls>,
    /// Follower information.
    pub followers: Option<Followers>,
    /// Spotify URL.
    pub href: Option<String>,
    /// Spotify user ID.
    pub id: String,
    /// User profile images.
    pub images: Option<Vec<Image>>,
    /// Product type (premium, free, etc.).
    pub product: Option<String>,
    /// Object type.
    #[serde(rename = "type")]
    pub item_type: String,
    /// Spotify URI.
    pub uri: String,
}

/// Explicit content filter settings.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExplicitContent {
    /// Whether to filter explicit content.
    pub filter_enabled: Option<bool>,
    /// Whether filter is locked (can't be changed).
    pub filter_locked: Option<bool>,
}

impl UserPrivate {
    /// Check if user has premium subscription.
    pub fn is_premium(&self) -> bool {
        self.product.as_deref() == Some("premium")
    }

    /// Get the largest image URL if available.
    pub fn image_url(&self) -> Option<&str> {
        self.images
            .as_ref()
            .and_then(|imgs| imgs.first())
            .map(|img| img.url.as_str())
    }
}

impl UserPublic {
    /// Get the largest image URL if available.
    pub fn image_url(&self) -> Option<&str> {
        self.images
            .as_ref()
            .and_then(|imgs| imgs.first())
            .map(|img| img.url.as_str())
    }
}

/// Top items response (tracks or artists).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TopTracksResponse {
    /// URL to the API endpoint.
    pub href: Option<String>,
    /// Maximum number of items.
    pub limit: Option<u32>,
    /// URL to the next page.
    pub next: Option<String>,
    /// Offset of items returned.
    pub offset: Option<u32>,
    /// URL to the previous page.
    pub previous: Option<String>,
    /// Total number of items.
    pub total: Option<u32>,
    /// The top tracks.
    pub items: Vec<super::track::Track>,
}

/// Top artists response.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TopArtistsResponse {
    /// URL to the API endpoint.
    pub href: Option<String>,
    /// Maximum number of items.
    pub limit: Option<u32>,
    /// URL to the next page.
    pub next: Option<String>,
    /// Offset of items returned.
    pub offset: Option<u32>,
    /// URL to the previous page.
    pub previous: Option<String>,
    /// Total number of items.
    pub total: Option<u32>,
    /// The top artists.
    pub items: Vec<super::artist::Artist>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn user_public_deserializes() {
        let json = json!({
            "id": "user123",
            "type": "user",
            "uri": "spotify:user:user123",
            "display_name": "John Doe"
        });
        let user: UserPublic = serde_json::from_value(json).unwrap();
        assert_eq!(user.id, "user123");
        assert_eq!(user.display_name, Some("John Doe".to_string()));
    }

    #[test]
    fn user_public_image_url() {
        let json = json!({
            "id": "user123",
            "type": "user",
            "uri": "spotify:user:user123",
            "images": [{"url": "https://profile.jpg", "height": 300, "width": 300}]
        });
        let user: UserPublic = serde_json::from_value(json).unwrap();
        assert_eq!(user.image_url(), Some("https://profile.jpg"));
    }

    #[test]
    fn user_public_image_url_none_when_empty() {
        let json = json!({
            "id": "user123",
            "type": "user",
            "uri": "spotify:user:user123"
        });
        let user: UserPublic = serde_json::from_value(json).unwrap();
        assert!(user.image_url().is_none());
    }

    #[test]
    fn user_private_deserializes() {
        let json = json!({
            "id": "user123",
            "type": "user",
            "uri": "spotify:user:user123",
            "email": "user@example.com",
            "product": "premium",
            "country": "US"
        });
        let user: UserPrivate = serde_json::from_value(json).unwrap();
        assert_eq!(user.id, "user123");
        assert_eq!(user.email, Some("user@example.com".to_string()));
        assert!(user.is_premium());
    }

    #[test]
    fn user_private_is_premium_false_for_free() {
        let json = json!({
            "id": "user123",
            "type": "user",
            "uri": "spotify:user:user123",
            "product": "free"
        });
        let user: UserPrivate = serde_json::from_value(json).unwrap();
        assert!(!user.is_premium());
    }

    #[test]
    fn user_private_is_premium_false_when_none() {
        let json = json!({
            "id": "user123",
            "type": "user",
            "uri": "spotify:user:user123"
        });
        let user: UserPrivate = serde_json::from_value(json).unwrap();
        assert!(!user.is_premium());
    }

    #[test]
    fn explicit_content_deserializes() {
        let json = json!({
            "filter_enabled": true,
            "filter_locked": false
        });
        let explicit: ExplicitContent = serde_json::from_value(json).unwrap();
        assert_eq!(explicit.filter_enabled, Some(true));
        assert_eq!(explicit.filter_locked, Some(false));
    }

    #[test]
    fn top_tracks_response_deserializes() {
        let json = json!({
            "items": [],
            "total": 50,
            "limit": 20,
            "offset": 0
        });
        let resp: TopTracksResponse = serde_json::from_value(json).unwrap();
        assert!(resp.items.is_empty());
        assert_eq!(resp.total, Some(50));
    }

    #[test]
    fn top_artists_response_deserializes() {
        let json = json!({
            "items": [],
            "total": 50,
            "limit": 20,
            "offset": 0
        });
        let resp: TopArtistsResponse = serde_json::from_value(json).unwrap();
        assert!(resp.items.is_empty());
        assert_eq!(resp.total, Some(50));
    }
}
