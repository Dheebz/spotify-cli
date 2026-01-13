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
