//! Formatter registry for type-safe payload formatting
//!
//! This module replaces the brittle if-else chain in dispatcher.rs with a
//! registry pattern where each formatter declares its own matching rules.
//!
//! ## Matching Strategy
//!
//! 1. If a `PayloadKind` is provided, use direct type-based matching (fast, reliable)
//! 2. Otherwise, fall back to payload inspection (legacy, for backward compatibility)

use serde_json::Value;
use std::sync::LazyLock;

use super::formatters;
use super::output::PayloadKind;

/// Trait for payload formatters
pub trait PayloadFormatter: Send + Sync {
    /// Unique identifier for this formatter (for debugging)
    fn name(&self) -> &'static str;

    /// PayloadKind(s) this formatter handles (preferred matching method)
    fn supported_kinds(&self) -> &'static [PayloadKind] {
        &[] // Default: no type-based matching, use payload inspection
    }

    /// Check if this formatter can handle the payload (fallback matching)
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

        registry.register(Box::new(PlayerStatusFormatter));
        registry.register(Box::new(QueueFormatter));
        registry.register(Box::new(DevicesFormatter));
        registry.register(Box::new(CombinedSearchFormatter));
        registry.register(Box::new(SpotifySearchFormatter));
        registry.register(Box::new(PinsFormatter));
        registry.register(Box::new(CategoryListFormatter));
        registry.register(Box::new(CategoryDetailFormatter));
        registry.register(Box::new(PlaylistDetailFormatter));
        registry.register(Box::new(TrackDetailFormatter));
        registry.register(Box::new(AlbumDetailFormatter));
        registry.register(Box::new(ArtistDetailFormatter));
        registry.register(Box::new(UserProfileFormatter));
        registry.register(Box::new(ShowDetailFormatter));
        registry.register(Box::new(EpisodeDetailFormatter));
        registry.register(Box::new(AudiobookDetailFormatter));
        registry.register(Box::new(ChapterDetailFormatter));
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
        registry.register(Box::new(ArtistTopTracksFormatter));
        registry.register(Box::new(LibraryCheckFormatter));

        registry
    }

    fn register(&mut self, formatter: Box<dyn PayloadFormatter>) {
        self.formatters.push(formatter);
    }

    /// Format the payload using the first matching formatter (legacy, uses payload inspection)
    pub fn format(&self, payload: &Value, message: &str) {
        self.format_with_kind(payload, message, None);
    }

    /// Format the payload, optionally using a type hint for reliable matching.
    pub fn format_with_kind(&self, payload: &Value, message: &str, kind: Option<PayloadKind>) {
        // If a kind is provided, try type-based matching first (fast path)
        if let Some(kind) = kind {
            for formatter in &self.formatters {
                if formatter.supported_kinds().contains(&kind) {
                    formatter.format(payload, message);
                    return;
                }
            }
        }

        // Fall back to payload inspection (slow path, for backward compatibility)
        for formatter in &self.formatters {
            if formatter.matches(payload) {
                formatter.format(payload, message);
                return;
            }
        }
        println!("{}", message);
    }
}

impl Default for FormatterRegistry {
    fn default() -> Self {
        Self::new()
    }
}

pub static REGISTRY: LazyLock<FormatterRegistry> = LazyLock::new(FormatterRegistry::new);

/// Format a payload using the global registry (legacy, uses payload inspection)
pub fn format_payload(payload: &Value, message: &str) {
    REGISTRY.format(payload, message);
}

/// Format a payload with optional type hint for reliable matching.
pub fn format_payload_with_kind(payload: &Value, message: &str, kind: Option<PayloadKind>) {
    REGISTRY.format_with_kind(payload, message, kind);
}

struct PlayerStatusFormatter;
impl PayloadFormatter for PlayerStatusFormatter {
    fn name(&self) -> &'static str {
        "player_status"
    }
    fn supported_kinds(&self) -> &'static [PayloadKind] {
        &[PayloadKind::PlayerStatus]
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
    fn supported_kinds(&self) -> &'static [PayloadKind] {
        &[PayloadKind::Queue]
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
    fn supported_kinds(&self) -> &'static [PayloadKind] {
        &[PayloadKind::Devices]
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

struct CombinedSearchFormatter;
impl PayloadFormatter for CombinedSearchFormatter {
    fn name(&self) -> &'static str {
        "combined_search"
    }
    fn supported_kinds(&self) -> &'static [PayloadKind] {
        &[PayloadKind::CombinedSearch]
    }
    fn matches(&self, payload: &Value) -> bool {
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
    fn supported_kinds(&self) -> &'static [PayloadKind] {
        &[PayloadKind::SearchResults]
    }
    fn matches(&self, payload: &Value) -> bool {
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

struct PinsFormatter;
impl PayloadFormatter for PinsFormatter {
    fn name(&self) -> &'static str {
        "pins"
    }
    fn supported_kinds(&self) -> &'static [PayloadKind] {
        &[PayloadKind::Pins]
    }
    fn matches(&self, payload: &Value) -> bool {
        payload.get("pins").is_some() && payload.get("spotify").is_none()
    }
    fn format(&self, payload: &Value, _message: &str) {
        if let Some(pins) = payload.get("pins").and_then(|p| p.as_array()) {
            formatters::format_pins(pins);
        }
    }
}

struct CategoryListFormatter;
impl PayloadFormatter for CategoryListFormatter {
    fn name(&self) -> &'static str {
        "category_list"
    }
    fn supported_kinds(&self) -> &'static [PayloadKind] {
        &[PayloadKind::CategoryList]
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
    fn supported_kinds(&self) -> &'static [PayloadKind] {
        &[PayloadKind::Category]
    }
    fn matches(&self, payload: &Value) -> bool {
        payload.get("icons").is_some()
            && payload.get("id").is_some()
            && payload.get("followers").is_none()
            && payload.get("owner").is_none()
    }
    fn format(&self, payload: &Value, _message: &str) {
        formatters::format_category_detail(payload);
    }
}

struct PlaylistDetailFormatter;
impl PayloadFormatter for PlaylistDetailFormatter {
    fn name(&self) -> &'static str {
        "playlist_detail"
    }
    fn supported_kinds(&self) -> &'static [PayloadKind] {
        &[PayloadKind::Playlist]
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
    fn supported_kinds(&self) -> &'static [PayloadKind] {
        &[PayloadKind::Track]
    }
    fn matches(&self, payload: &Value) -> bool {
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
    fn supported_kinds(&self) -> &'static [PayloadKind] {
        &[PayloadKind::Album]
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
    fn supported_kinds(&self) -> &'static [PayloadKind] {
        &[PayloadKind::Artist, PayloadKind::RelatedArtists]
    }
    fn matches(&self, payload: &Value) -> bool {
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
    fn supported_kinds(&self) -> &'static [PayloadKind] {
        &[PayloadKind::User]
    }
    fn matches(&self, payload: &Value) -> bool {
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
    fn supported_kinds(&self) -> &'static [PayloadKind] {
        &[PayloadKind::Show]
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
    fn supported_kinds(&self) -> &'static [PayloadKind] {
        &[PayloadKind::Episode]
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
    fn supported_kinds(&self) -> &'static [PayloadKind] {
        &[PayloadKind::Audiobook]
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
    fn supported_kinds(&self) -> &'static [PayloadKind] {
        &[PayloadKind::Chapter]
    }
    fn matches(&self, payload: &Value) -> bool {
        payload.get("audiobook").is_some() && payload.get("chapter_number").is_some()
    }
    fn format(&self, payload: &Value, _message: &str) {
        formatters::format_chapter_detail(payload);
    }
}

struct PlaylistsFormatter;
impl PayloadFormatter for PlaylistsFormatter {
    fn name(&self) -> &'static str {
        "playlists"
    }
    fn supported_kinds(&self) -> &'static [PayloadKind] {
        &[PayloadKind::PlaylistList, PayloadKind::FeaturedPlaylists]
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
    fn supported_kinds(&self) -> &'static [PayloadKind] {
        &[PayloadKind::SavedTracks]
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
    fn supported_kinds(&self) -> &'static [PayloadKind] {
        &[PayloadKind::PlayHistory]
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
    fn supported_kinds(&self) -> &'static [PayloadKind] {
        &[PayloadKind::SavedShows, PayloadKind::ShowList]
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
    fn supported_kinds(&self) -> &'static [PayloadKind] {
        &[PayloadKind::EpisodeList]
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
    fn supported_kinds(&self) -> &'static [PayloadKind] {
        &[PayloadKind::SavedEpisodes]
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
    fn supported_kinds(&self) -> &'static [PayloadKind] {
        &[PayloadKind::SavedAudiobooks, PayloadKind::AudiobookList]
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
    fn supported_kinds(&self) -> &'static [PayloadKind] {
        &[PayloadKind::ChapterList]
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
    fn supported_kinds(&self) -> &'static [PayloadKind] {
        &[PayloadKind::TopTracks, PayloadKind::TrackList]
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
    fn supported_kinds(&self) -> &'static [PayloadKind] {
        &[PayloadKind::TopArtists, PayloadKind::ArtistList, PayloadKind::FollowedArtists]
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

struct ArtistTopTracksFormatter;
impl PayloadFormatter for ArtistTopTracksFormatter {
    fn name(&self) -> &'static str {
        "artist_top_tracks"
    }
    fn supported_kinds(&self) -> &'static [PayloadKind] {
        &[PayloadKind::ArtistTopTracks]
    }
    fn matches(&self, payload: &Value) -> bool {
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
    fn supported_kinds(&self) -> &'static [PayloadKind] {
        &[PayloadKind::LibraryCheck]
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

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn formatter_registry_has_formatters() {
        let registry = FormatterRegistry::new();
        assert!(!registry.formatters.is_empty());
    }

    #[test]
    fn player_status_formatter_supports_kind() {
        let formatter = PlayerStatusFormatter;
        assert!(formatter.supported_kinds().contains(&PayloadKind::PlayerStatus));
    }

    #[test]
    fn queue_formatter_supports_kind() {
        let formatter = QueueFormatter;
        assert!(formatter.supported_kinds().contains(&PayloadKind::Queue));
    }

    #[test]
    fn track_formatter_supports_kind() {
        let formatter = TrackDetailFormatter;
        assert!(formatter.supported_kinds().contains(&PayloadKind::Track));
    }

    #[test]
    fn playlist_formatter_supports_multiple_kinds() {
        let formatter = PlaylistsFormatter;
        let kinds = formatter.supported_kinds();
        assert!(kinds.contains(&PayloadKind::PlaylistList));
        assert!(kinds.contains(&PayloadKind::FeaturedPlaylists));
    }

    #[test]
    fn top_artists_formatter_supports_multiple_kinds() {
        let formatter = TopArtistsFormatter;
        let kinds = formatter.supported_kinds();
        assert!(kinds.contains(&PayloadKind::TopArtists));
        assert!(kinds.contains(&PayloadKind::ArtistList));
        assert!(kinds.contains(&PayloadKind::FollowedArtists));
    }

    #[test]
    fn player_status_matches_payload_with_item() {
        let formatter = PlayerStatusFormatter;
        let payload = json!({
            "item": {"name": "Track"},
            "is_playing": true
        });
        assert!(formatter.matches(&payload));
    }

    #[test]
    fn player_status_does_not_match_without_item() {
        let formatter = PlayerStatusFormatter;
        let payload = json!({"is_playing": true});
        assert!(!formatter.matches(&payload));
    }

    #[test]
    fn queue_matches_payload_with_currently_playing_and_queue() {
        let formatter = QueueFormatter;
        let payload = json!({
            "currently_playing": {"name": "Track"},
            "queue": []
        });
        assert!(formatter.matches(&payload));
    }

    #[test]
    fn track_detail_matches_payload() {
        let formatter = TrackDetailFormatter;
        let payload = json!({
            "album": {"name": "Album"},
            "artists": [{"name": "Artist"}],
            "duration_ms": 300000
        });
        assert!(formatter.matches(&payload));
    }

    #[test]
    fn library_check_matches_boolean_array() {
        let formatter = LibraryCheckFormatter;
        let payload = json!([true, false, true]);
        assert!(formatter.matches(&payload));
    }

    #[test]
    fn library_check_does_not_match_non_boolean_array() {
        let formatter = LibraryCheckFormatter;
        let payload = json!(["string", "array"]);
        assert!(!formatter.matches(&payload));
    }

    #[test]
    fn default_supported_kinds_is_empty() {
        struct TestFormatter;
        impl PayloadFormatter for TestFormatter {
            fn name(&self) -> &'static str { "test" }
            fn matches(&self, _: &Value) -> bool { false }
            fn format(&self, _: &Value, _: &str) {}
        }
        let formatter = TestFormatter;
        assert!(formatter.supported_kinds().is_empty());
    }

    #[test]
    fn devices_formatter_matches() {
        let formatter = DevicesFormatter;
        let payload = json!({ "devices": [] });
        assert!(formatter.matches(&payload));
        assert!(!formatter.matches(&json!({})));
    }

    #[test]
    fn combined_search_formatter_matches() {
        let formatter = CombinedSearchFormatter;
        let payload = json!({ "spotify": { "tracks": {} } });
        assert!(formatter.matches(&payload));
        assert!(!formatter.matches(&json!({})));
    }

    #[test]
    fn pins_formatter_matches() {
        let formatter = PinsFormatter;
        let payload = json!({ "pins": [] });
        assert!(formatter.matches(&payload));
        let payload_with_spotify = json!({ "pins": [], "spotify": {} });
        assert!(!formatter.matches(&payload_with_spotify));
    }

    #[test]
    fn category_list_formatter_matches() {
        let formatter = CategoryListFormatter;
        let payload = json!({ "categories": { "items": [] } });
        assert!(formatter.matches(&payload));
        assert!(!formatter.matches(&json!({})));
    }

    #[test]
    fn category_detail_formatter_matches() {
        let formatter = CategoryDetailFormatter;
        let payload = json!({ "icons": [], "id": "test" });
        assert!(formatter.matches(&payload));
        let with_followers = json!({ "icons": [], "id": "test", "followers": {} });
        assert!(!formatter.matches(&with_followers));
    }

    #[test]
    fn playlist_detail_formatter_matches() {
        let formatter = PlaylistDetailFormatter;
        let payload = json!({ "owner": {}, "tracks": {} });
        assert!(formatter.matches(&payload));
        assert!(!formatter.matches(&json!({})));
    }

    #[test]
    fn album_detail_formatter_matches() {
        let formatter = AlbumDetailFormatter;
        let payload = json!({ "album_type": "album", "tracks": {} });
        assert!(formatter.matches(&payload));
        assert!(!formatter.matches(&json!({})));
    }

    #[test]
    fn artist_detail_formatter_matches() {
        let formatter = ArtistDetailFormatter;
        let payload = json!({ "followers": {}, "genres": [] });
        assert!(formatter.matches(&payload));
        let with_album = json!({ "followers": {}, "genres": [], "album": {} });
        assert!(!formatter.matches(&with_album));
    }

    #[test]
    fn user_profile_formatter_matches() {
        let formatter = UserProfileFormatter;
        let payload = json!({ "display_name": "User", "product": "premium" });
        assert!(formatter.matches(&payload));
        let with_genres = json!({ "display_name": "User", "product": "premium", "genres": [] });
        assert!(!formatter.matches(&with_genres));
    }

    #[test]
    fn show_detail_formatter_matches() {
        let formatter = ShowDetailFormatter;
        let payload = json!({ "publisher": "Test", "total_episodes": 10 });
        assert!(formatter.matches(&payload));
        assert!(!formatter.matches(&json!({})));
    }

    #[test]
    fn episode_detail_formatter_matches() {
        let formatter = EpisodeDetailFormatter;
        let payload = json!({ "show": {}, "release_date": "2024", "duration_ms": 1000 });
        assert!(formatter.matches(&payload));
        assert!(!formatter.matches(&json!({})));
    }

    #[test]
    fn audiobook_detail_formatter_matches() {
        let formatter = AudiobookDetailFormatter;
        let payload = json!({ "authors": [], "total_chapters": 10 });
        assert!(formatter.matches(&payload));
        assert!(!formatter.matches(&json!({})));
    }

    #[test]
    fn chapter_detail_formatter_matches() {
        let formatter = ChapterDetailFormatter;
        let payload = json!({ "audiobook": {}, "chapter_number": 1 });
        assert!(formatter.matches(&payload));
        assert!(!formatter.matches(&json!({})));
    }

    #[test]
    fn playlists_formatter_matches() {
        let formatter = PlaylistsFormatter;
        let payload = json!({ "items": [{ "tracks": {} }] });
        assert!(formatter.matches(&payload));
        let empty = json!({ "items": [] });
        assert!(!formatter.matches(&empty));
    }

    #[test]
    fn saved_tracks_formatter_matches() {
        let formatter = SavedTracksFormatter;
        let payload = json!({ "items": [{ "track": {}, "added_at": "2024" }] });
        assert!(formatter.matches(&payload));
        let empty = json!({ "items": [] });
        assert!(!formatter.matches(&empty));
    }

    #[test]
    fn play_history_formatter_matches() {
        let formatter = PlayHistoryFormatter;
        let payload = json!({ "items": [{ "track": {}, "played_at": "2024" }] });
        assert!(formatter.matches(&payload));
        let empty = json!({ "items": [] });
        assert!(!formatter.matches(&empty));
    }

    #[test]
    fn saved_shows_formatter_matches() {
        let formatter = SavedShowsFormatter;
        let payload = json!({ "items": [{ "show": {} }] });
        assert!(formatter.matches(&payload));
        let direct = json!({ "items": [{ "publisher": "Test", "total_episodes": 5 }] });
        assert!(formatter.matches(&direct));
    }

    #[test]
    fn show_episodes_formatter_matches() {
        let formatter = ShowEpisodesFormatter;
        let payload = json!({ "items": [{ "release_date": "2024", "duration_ms": 1000 }] });
        assert!(formatter.matches(&payload));
        let with_album = json!({ "items": [{ "release_date": "2024", "duration_ms": 1000, "album": {} }] });
        assert!(!formatter.matches(&with_album));
    }

    #[test]
    fn saved_episodes_formatter_matches() {
        let formatter = SavedEpisodesFormatter;
        let payload = json!({ "items": [{ "episode": {} }] });
        assert!(formatter.matches(&payload));
        let empty = json!({ "items": [] });
        assert!(!formatter.matches(&empty));
    }

    #[test]
    fn saved_audiobooks_formatter_matches() {
        let formatter = SavedAudiobooksFormatter;
        let payload = json!({ "items": [{ "audiobook": {} }] });
        assert!(formatter.matches(&payload));
        let direct = json!({ "items": [{ "authors": [], "total_chapters": 5 }] });
        assert!(formatter.matches(&direct));
    }

    #[test]
    fn audiobook_chapters_formatter_matches() {
        let formatter = AudiobookChaptersFormatter;
        let payload = json!({ "items": [{ "chapter_number": 1 }] });
        assert!(formatter.matches(&payload));
        let alt = json!({ "items": [{ "audiobook": {}, "duration_ms": 1000 }] });
        assert!(formatter.matches(&alt));
    }

    #[test]
    fn top_tracks_formatter_matches() {
        let formatter = TopTracksFormatter;
        let payload = json!({ "items": [{ "album": {} }] });
        assert!(formatter.matches(&payload));
        let empty = json!({ "items": [] });
        assert!(!formatter.matches(&empty));
    }

    #[test]
    fn top_artists_formatter_matches() {
        let formatter = TopArtistsFormatter;
        let payload = json!({ "items": [{ "genres": [] }] });
        assert!(formatter.matches(&payload));
        let empty = json!({ "items": [] });
        assert!(!formatter.matches(&empty));
    }

    #[test]
    fn artist_top_tracks_formatter_matches() {
        let formatter = ArtistTopTracksFormatter;
        let payload = json!({ "tracks": [] });
        assert!(formatter.matches(&payload));
        let with_items = json!({ "tracks": [], "items": [] });
        assert!(!formatter.matches(&with_items));
    }

    #[test]
    fn spotify_search_formatter_matches() {
        let formatter = SpotifySearchFormatter;
        let tracks = json!({ "tracks": { "items": [] } });
        assert!(formatter.matches(&tracks));
        let albums = json!({ "albums": { "items": [] } });
        assert!(formatter.matches(&albums));
        let artists = json!({ "artists": { "items": [] } });
        assert!(formatter.matches(&artists));
        let playlists = json!({ "playlists": { "items": [] } });
        assert!(formatter.matches(&playlists));
        // Should not match playlist or album detail
        let playlist = json!({ "owner": {}, "tracks": { "items": [] } });
        assert!(!formatter.matches(&playlist));
        let album = json!({ "album_type": "album", "tracks": { "items": [] } });
        assert!(!formatter.matches(&album));
    }

    #[test]
    fn formatter_names() {
        assert_eq!(PlayerStatusFormatter.name(), "player_status");
        assert_eq!(QueueFormatter.name(), "queue");
        assert_eq!(DevicesFormatter.name(), "devices");
        assert_eq!(TrackDetailFormatter.name(), "track_detail");
        assert_eq!(AlbumDetailFormatter.name(), "album_detail");
        assert_eq!(ArtistDetailFormatter.name(), "artist_detail");
        assert_eq!(PlaylistDetailFormatter.name(), "playlist_detail");
        assert_eq!(UserProfileFormatter.name(), "user_profile");
    }

    #[test]
    fn registry_format_with_kind_uses_kind_matching() {
        let registry = FormatterRegistry::new();
        let payload = json!({ "item": { "name": "Test" }, "is_playing": true });
        // This should use the player status formatter via kind matching
        registry.format_with_kind(&payload, "Test", Some(PayloadKind::PlayerStatus));
    }

    #[test]
    fn registry_format_with_kind_falls_back_to_payload_matching() {
        let registry = FormatterRegistry::new();
        let payload = json!({ "item": { "name": "Test" }, "is_playing": true });
        // With no kind, should fall back to payload matching
        registry.format_with_kind(&payload, "Test", None);
    }

    #[test]
    fn registry_format_with_unknown_prints_message() {
        let registry = FormatterRegistry::new();
        let payload = json!({ "unknown_field": "value" });
        // Should just print message when no formatter matches
        registry.format(&payload, "No match found");
    }

    #[test]
    fn global_registry_accessible() {
        // Just verify the global registry can be accessed
        let _ = &*REGISTRY;
    }

    #[test]
    fn format_payload_works() {
        let payload = json!({ "unknown": "data" });
        format_payload(&payload, "Test message");
    }

    #[test]
    fn format_payload_with_kind_works() {
        let payload = json!({ "unknown": "data" });
        format_payload_with_kind(&payload, "Test message", None);
    }

    #[test]
    fn registry_default_same_as_new() {
        let default_registry = FormatterRegistry::default();
        let new_registry = FormatterRegistry::new();
        assert_eq!(default_registry.formatters.len(), new_registry.formatters.len());
    }
}
