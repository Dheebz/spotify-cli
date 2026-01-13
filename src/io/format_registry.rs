//! Formatter registry for type-safe payload formatting
//!
//! This module replaces the brittle if-else chain in dispatcher.rs with a
//! registry pattern where each formatter declares its own matching rules.

use serde_json::Value;
use std::sync::LazyLock;

use super::formatters;

/// Trait for payload formatters
pub trait PayloadFormatter: Send + Sync {
    /// Unique identifier for this formatter (for debugging)
    fn name(&self) -> &'static str;

    /// Check if this formatter can handle the payload
    fn matches(&self, payload: &Value) -> bool;

    /// Format and print the payload
    fn format(&self, payload: &Value, message: &str);
}

/// Registry holding all formatters in priority order
pub struct FormatterRegistry {
    formatters: Vec<Box<dyn PayloadFormatter>>,
}

impl FormatterRegistry {
    pub fn new() -> Self {
        let mut registry = Self {
            formatters: Vec::new(),
        };

        // Register formatters in priority order (most specific first)
        // The order matters: more specific matchers should come before generic ones

        // Player and queue
        registry.register(Box::new(PlayerStatusFormatter));
        registry.register(Box::new(QueueFormatter));
        registry.register(Box::new(DevicesFormatter));

        // Search results (before individual resource types)
        registry.register(Box::new(CombinedSearchFormatter));
        registry.register(Box::new(SpotifySearchFormatter));

        // Pins
        registry.register(Box::new(PinsFormatter));

        // Browse categories
        registry.register(Box::new(CategoryListFormatter));
        registry.register(Box::new(CategoryDetailFormatter));

        // Single resource details (specific first)
        registry.register(Box::new(PlaylistDetailFormatter));
        registry.register(Box::new(TrackDetailFormatter));
        registry.register(Box::new(AlbumDetailFormatter));
        registry.register(Box::new(ArtistDetailFormatter));
        registry.register(Box::new(UserProfileFormatter));
        registry.register(Box::new(ShowDetailFormatter));
        registry.register(Box::new(EpisodeDetailFormatter));
        registry.register(Box::new(AudiobookDetailFormatter));
        registry.register(Box::new(ChapterDetailFormatter));

        // Lists with items arrays
        registry.register(Box::new(PlaylistsFormatter));
        registry.register(Box::new(SavedTracksFormatter));
        registry.register(Box::new(PlayHistoryFormatter));
        registry.register(Box::new(SavedShowsFormatter));
        registry.register(Box::new(ShowEpisodesFormatter));
        registry.register(Box::new(SavedEpisodesFormatter));
        registry.register(Box::new(SavedAudiobooksFormatter));
        registry.register(Box::new(AudiobookChaptersFormatter));
        registry.register(Box::new(TopTracksFormatter));
        registry.register(Box::new(TopArtistsFormatter));

        // Artist top tracks (different structure)
        registry.register(Box::new(ArtistTopTracksFormatter));

        // Library check (array of booleans)
        registry.register(Box::new(LibraryCheckFormatter));

        registry
    }

    fn register(&mut self, formatter: Box<dyn PayloadFormatter>) {
        self.formatters.push(formatter);
    }

    /// Format the payload using the first matching formatter
    pub fn format(&self, payload: &Value, message: &str) {
        for formatter in &self.formatters {
            if formatter.matches(payload) {
                formatter.format(payload, message);
                return;
            }
        }
        // Fallback: just print the message
        println!("{}", message);
    }
}

impl Default for FormatterRegistry {
    fn default() -> Self {
        Self::new()
    }
}

// Global registry instance
pub static REGISTRY: LazyLock<FormatterRegistry> = LazyLock::new(FormatterRegistry::new);

/// Format a payload using the global registry
pub fn format_payload(payload: &Value, message: &str) {
    REGISTRY.format(payload, message);
}

// ============================================================================
// Formatter Implementations
// ============================================================================

// --- Player ---

struct PlayerStatusFormatter;
impl PayloadFormatter for PlayerStatusFormatter {
    fn name(&self) -> &'static str {
        "player_status"
    }
    fn matches(&self, payload: &Value) -> bool {
        payload.get("item").is_some()
    }
    fn format(&self, payload: &Value, _message: &str) {
        if let Some(item) = payload.get("item") {
            formatters::format_player_status(payload, item);
        }
    }
}

struct QueueFormatter;
impl PayloadFormatter for QueueFormatter {
    fn name(&self) -> &'static str {
        "queue"
    }
    fn matches(&self, payload: &Value) -> bool {
        payload.get("currently_playing").is_some() && payload.get("queue").is_some()
    }
    fn format(&self, payload: &Value, _message: &str) {
        formatters::format_queue(payload);
    }
}

struct DevicesFormatter;
impl PayloadFormatter for DevicesFormatter {
    fn name(&self) -> &'static str {
        "devices"
    }
    fn matches(&self, payload: &Value) -> bool {
        payload.get("devices").is_some()
    }
    fn format(&self, payload: &Value, _message: &str) {
        if let Some(devices) = payload.get("devices").and_then(|d| d.as_array()) {
            formatters::format_devices(devices);
        }
    }
}

// --- Search ---

struct CombinedSearchFormatter;
impl PayloadFormatter for CombinedSearchFormatter {
    fn name(&self) -> &'static str {
        "combined_search"
    }
    fn matches(&self, payload: &Value) -> bool {
        // Search results from combined pins + spotify search
        payload.get("spotify").is_some()
    }
    fn format(&self, payload: &Value, _message: &str) {
        formatters::format_search_results(payload);
    }
}

struct SpotifySearchFormatter;
impl PayloadFormatter for SpotifySearchFormatter {
    fn name(&self) -> &'static str {
        "spotify_search"
    }
    fn matches(&self, payload: &Value) -> bool {
        // Pure Spotify search results have nested structure
        // Exclude single resource responses (playlist has "owner", album has "album_type")
        let is_playlist = payload.get("owner").is_some();
        let is_album = payload.get("album_type").is_some();
        !is_playlist
            && !is_album
            && ((payload.get("tracks").is_some()
                && payload["tracks"].get("items").is_some())
                || (payload.get("albums").is_some()
                    && payload["albums"].get("items").is_some())
                || (payload.get("artists").is_some()
                    && payload["artists"].get("items").is_some())
                || (payload.get("playlists").is_some()
                    && payload["playlists"].get("items").is_some()))
    }
    fn format(&self, payload: &Value, _message: &str) {
        formatters::format_search_results(payload);
    }
}

// --- Pins ---

struct PinsFormatter;
impl PayloadFormatter for PinsFormatter {
    fn name(&self) -> &'static str {
        "pins"
    }
    fn matches(&self, payload: &Value) -> bool {
        // Only "pins" key, no "spotify"
        payload.get("pins").is_some() && payload.get("spotify").is_none()
    }
    fn format(&self, payload: &Value, _message: &str) {
        if let Some(pins) = payload.get("pins").and_then(|p| p.as_array()) {
            formatters::format_pins(pins);
        }
    }
}

// --- Categories ---

struct CategoryListFormatter;
impl PayloadFormatter for CategoryListFormatter {
    fn name(&self) -> &'static str {
        "category_list"
    }
    fn matches(&self, payload: &Value) -> bool {
        payload
            .get("categories")
            .and_then(|c| c.get("items"))
            .is_some()
    }
    fn format(&self, payload: &Value, _message: &str) {
        if let Some(items) = payload
            .get("categories")
            .and_then(|c| c.get("items"))
            .and_then(|i| i.as_array())
        {
            formatters::format_categories(items);
        }
    }
}

struct CategoryDetailFormatter;
impl PayloadFormatter for CategoryDetailFormatter {
    fn name(&self) -> &'static str {
        "category_detail"
    }
    fn matches(&self, payload: &Value) -> bool {
        // Single category: has icons and id, but not common keys like followers, owner
        payload.get("icons").is_some()
            && payload.get("id").is_some()
            && payload.get("followers").is_none()
            && payload.get("owner").is_none()
    }
    fn format(&self, payload: &Value, _message: &str) {
        formatters::format_category_detail(payload);
    }
}

// --- Single Resource Details ---

struct PlaylistDetailFormatter;
impl PayloadFormatter for PlaylistDetailFormatter {
    fn name(&self) -> &'static str {
        "playlist_detail"
    }
    fn matches(&self, payload: &Value) -> bool {
        payload.get("owner").is_some() && payload.get("tracks").is_some()
    }
    fn format(&self, payload: &Value, _message: &str) {
        formatters::format_playlist_detail(payload);
    }
}

struct TrackDetailFormatter;
impl PayloadFormatter for TrackDetailFormatter {
    fn name(&self) -> &'static str {
        "track_detail"
    }
    fn matches(&self, payload: &Value) -> bool {
        // Has album and artists, but no "item" wrapper
        payload.get("album").is_some()
            && payload.get("artists").is_some()
            && payload.get("duration_ms").is_some()
    }
    fn format(&self, payload: &Value, _message: &str) {
        formatters::format_track_detail(payload);
    }
}

struct AlbumDetailFormatter;
impl PayloadFormatter for AlbumDetailFormatter {
    fn name(&self) -> &'static str {
        "album_detail"
    }
    fn matches(&self, payload: &Value) -> bool {
        payload.get("album_type").is_some() && payload.get("tracks").is_some()
    }
    fn format(&self, payload: &Value, _message: &str) {
        formatters::format_album_detail(payload);
    }
}

struct ArtistDetailFormatter;
impl PayloadFormatter for ArtistDetailFormatter {
    fn name(&self) -> &'static str {
        "artist_detail"
    }
    fn matches(&self, payload: &Value) -> bool {
        // Has followers and genres, no album
        payload.get("followers").is_some()
            && payload.get("genres").is_some()
            && payload.get("album").is_none()
    }
    fn format(&self, payload: &Value, _message: &str) {
        formatters::format_artist_detail(payload);
    }
}

struct UserProfileFormatter;
impl PayloadFormatter for UserProfileFormatter {
    fn name(&self) -> &'static str {
        "user_profile"
    }
    fn matches(&self, payload: &Value) -> bool {
        // Has display_name and product, but not genres
        payload.get("display_name").is_some()
            && payload.get("product").is_some()
            && payload.get("genres").is_none()
    }
    fn format(&self, payload: &Value, _message: &str) {
        formatters::format_user_profile(payload);
    }
}

struct ShowDetailFormatter;
impl PayloadFormatter for ShowDetailFormatter {
    fn name(&self) -> &'static str {
        "show_detail"
    }
    fn matches(&self, payload: &Value) -> bool {
        payload.get("publisher").is_some() && payload.get("total_episodes").is_some()
    }
    fn format(&self, payload: &Value, _message: &str) {
        formatters::format_show_detail(payload);
    }
}

struct EpisodeDetailFormatter;
impl PayloadFormatter for EpisodeDetailFormatter {
    fn name(&self) -> &'static str {
        "episode_detail"
    }
    fn matches(&self, payload: &Value) -> bool {
        payload.get("show").is_some()
            && payload.get("release_date").is_some()
            && payload.get("duration_ms").is_some()
    }
    fn format(&self, payload: &Value, _message: &str) {
        formatters::format_episode_detail(payload);
    }
}

struct AudiobookDetailFormatter;
impl PayloadFormatter for AudiobookDetailFormatter {
    fn name(&self) -> &'static str {
        "audiobook_detail"
    }
    fn matches(&self, payload: &Value) -> bool {
        payload.get("authors").is_some() && payload.get("total_chapters").is_some()
    }
    fn format(&self, payload: &Value, _message: &str) {
        formatters::format_audiobook_detail(payload);
    }
}

struct ChapterDetailFormatter;
impl PayloadFormatter for ChapterDetailFormatter {
    fn name(&self) -> &'static str {
        "chapter_detail"
    }
    fn matches(&self, payload: &Value) -> bool {
        payload.get("audiobook").is_some() && payload.get("chapter_number").is_some()
    }
    fn format(&self, payload: &Value, _message: &str) {
        formatters::format_chapter_detail(payload);
    }
}

// --- List Formatters ---

struct PlaylistsFormatter;
impl PayloadFormatter for PlaylistsFormatter {
    fn name(&self) -> &'static str {
        "playlists"
    }
    fn matches(&self, payload: &Value) -> bool {
        if let Some(items) = payload.get("items").and_then(|i| i.as_array()) {
            !items.is_empty()
                && (items[0].get("tracks").is_some() || items[0].get("owner").is_some())
        } else {
            false
        }
    }
    fn format(&self, payload: &Value, _message: &str) {
        if let Some(items) = payload.get("items").and_then(|i| i.as_array()) {
            formatters::format_playlists(items);
        }
    }
}

struct SavedTracksFormatter;
impl PayloadFormatter for SavedTracksFormatter {
    fn name(&self) -> &'static str {
        "saved_tracks"
    }
    fn matches(&self, payload: &Value) -> bool {
        if let Some(items) = payload.get("items").and_then(|i| i.as_array()) {
            !items.is_empty()
                && items[0].get("track").is_some()
                && items[0].get("added_at").is_some()
        } else {
            false
        }
    }
    fn format(&self, payload: &Value, _message: &str) {
        if let Some(items) = payload.get("items").and_then(|i| i.as_array()) {
            formatters::format_saved_tracks(items);
        }
    }
}

struct PlayHistoryFormatter;
impl PayloadFormatter for PlayHistoryFormatter {
    fn name(&self) -> &'static str {
        "play_history"
    }
    fn matches(&self, payload: &Value) -> bool {
        if let Some(items) = payload.get("items").and_then(|i| i.as_array()) {
            !items.is_empty()
                && items[0].get("track").is_some()
                && items[0].get("played_at").is_some()
        } else {
            false
        }
    }
    fn format(&self, payload: &Value, _message: &str) {
        if let Some(items) = payload.get("items").and_then(|i| i.as_array()) {
            formatters::format_play_history(items);
        }
    }
}

struct SavedShowsFormatter;
impl PayloadFormatter for SavedShowsFormatter {
    fn name(&self) -> &'static str {
        "saved_shows"
    }
    fn matches(&self, payload: &Value) -> bool {
        if let Some(items) = payload.get("items").and_then(|i| i.as_array()) {
            !items.is_empty()
                && (items[0].get("show").is_some()
                    || (items[0].get("publisher").is_some()
                        && items[0].get("total_episodes").is_some()))
        } else {
            false
        }
    }
    fn format(&self, payload: &Value, message: &str) {
        if let Some(items) = payload.get("items").and_then(|i| i.as_array()) {
            formatters::format_shows(items, message);
        }
    }
}

struct ShowEpisodesFormatter;
impl PayloadFormatter for ShowEpisodesFormatter {
    fn name(&self) -> &'static str {
        "show_episodes"
    }
    fn matches(&self, payload: &Value) -> bool {
        if let Some(items) = payload.get("items").and_then(|i| i.as_array()) {
            !items.is_empty()
                && items[0].get("release_date").is_some()
                && items[0].get("duration_ms").is_some()
                && items[0].get("album").is_none()
                && items[0].get("artists").is_none()
        } else {
            false
        }
    }
    fn format(&self, payload: &Value, message: &str) {
        if let Some(items) = payload.get("items").and_then(|i| i.as_array()) {
            formatters::format_show_episodes(items, message);
        }
    }
}

struct SavedEpisodesFormatter;
impl PayloadFormatter for SavedEpisodesFormatter {
    fn name(&self) -> &'static str {
        "saved_episodes"
    }
    fn matches(&self, payload: &Value) -> bool {
        if let Some(items) = payload.get("items").and_then(|i| i.as_array()) {
            !items.is_empty() && items[0].get("episode").is_some()
        } else {
            false
        }
    }
    fn format(&self, payload: &Value, message: &str) {
        if let Some(items) = payload.get("items").and_then(|i| i.as_array()) {
            formatters::format_episodes(items, message);
        }
    }
}

struct SavedAudiobooksFormatter;
impl PayloadFormatter for SavedAudiobooksFormatter {
    fn name(&self) -> &'static str {
        "saved_audiobooks"
    }
    fn matches(&self, payload: &Value) -> bool {
        if let Some(items) = payload.get("items").and_then(|i| i.as_array()) {
            !items.is_empty()
                && (items[0].get("audiobook").is_some()
                    || (items[0].get("authors").is_some()
                        && items[0].get("total_chapters").is_some()))
        } else {
            false
        }
    }
    fn format(&self, payload: &Value, message: &str) {
        if let Some(items) = payload.get("items").and_then(|i| i.as_array()) {
            formatters::format_audiobooks(items, message);
        }
    }
}

struct AudiobookChaptersFormatter;
impl PayloadFormatter for AudiobookChaptersFormatter {
    fn name(&self) -> &'static str {
        "audiobook_chapters"
    }
    fn matches(&self, payload: &Value) -> bool {
        if let Some(items) = payload.get("items").and_then(|i| i.as_array()) {
            !items.is_empty()
                && (items[0].get("chapter_number").is_some()
                    || (items[0].get("audiobook").is_some()
                        && items[0].get("duration_ms").is_some()))
        } else {
            false
        }
    }
    fn format(&self, payload: &Value, message: &str) {
        if let Some(items) = payload.get("items").and_then(|i| i.as_array()) {
            formatters::format_audiobook_chapters(items, message);
        }
    }
}

struct TopTracksFormatter;
impl PayloadFormatter for TopTracksFormatter {
    fn name(&self) -> &'static str {
        "top_tracks"
    }
    fn matches(&self, payload: &Value) -> bool {
        if let Some(items) = payload.get("items").and_then(|i| i.as_array()) {
            !items.is_empty() && items[0].get("album").is_some()
        } else {
            false
        }
    }
    fn format(&self, payload: &Value, message: &str) {
        if let Some(items) = payload.get("items").and_then(|i| i.as_array()) {
            formatters::format_top_tracks(items, message);
        }
    }
}

struct TopArtistsFormatter;
impl PayloadFormatter for TopArtistsFormatter {
    fn name(&self) -> &'static str {
        "top_artists"
    }
    fn matches(&self, payload: &Value) -> bool {
        if let Some(items) = payload.get("items").and_then(|i| i.as_array()) {
            !items.is_empty() && items[0].get("genres").is_some()
        } else {
            false
        }
    }
    fn format(&self, payload: &Value, message: &str) {
        if let Some(items) = payload.get("items").and_then(|i| i.as_array()) {
            formatters::format_top_artists(items, message);
        }
    }
}

// --- Special Formats ---

struct ArtistTopTracksFormatter;
impl PayloadFormatter for ArtistTopTracksFormatter {
    fn name(&self) -> &'static str {
        "artist_top_tracks"
    }
    fn matches(&self, payload: &Value) -> bool {
        // Has tracks array at root (not nested)
        payload.get("tracks").map(|t| t.is_array()).unwrap_or(false)
            && payload.get("items").is_none()
    }
    fn format(&self, payload: &Value, _message: &str) {
        if let Some(tracks) = payload.get("tracks").and_then(|t| t.as_array()) {
            formatters::format_artist_top_tracks(tracks);
        }
    }
}

struct LibraryCheckFormatter;
impl PayloadFormatter for LibraryCheckFormatter {
    fn name(&self) -> &'static str {
        "library_check"
    }
    fn matches(&self, payload: &Value) -> bool {
        if let Some(arr) = payload.as_array() {
            !arr.is_empty() && arr[0].is_boolean()
        } else {
            false
        }
    }
    fn format(&self, payload: &Value, _message: &str) {
        if let Some(arr) = payload.as_array() {
            formatters::format_library_check(arr);
        }
    }
}
