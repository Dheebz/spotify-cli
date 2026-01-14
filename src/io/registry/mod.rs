//! Formatter registry for type-safe payload formatting
//!
//! This module contains the registry pattern for payload formatters.
//! Each formatter declares its supported `PayloadKind`s for fast matching,
//! with fallback payload inspection for legacy compatibility.

mod player;
mod search;
mod resources;
mod lists;
mod media;

use serde_json::Value;
use std::sync::LazyLock;

use super::output::PayloadKind;

// Re-export formatter structs for registration
pub use player::*;
pub use search::*;
pub use resources::*;
pub use lists::*;
pub use media::*;

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

        // Player formatters
        registry.register(Box::new(PlayerStatusFormatter));
        registry.register(Box::new(QueueFormatter));
        registry.register(Box::new(DevicesFormatter));
        registry.register(Box::new(PlayHistoryFormatter));

        // Search formatters
        registry.register(Box::new(CombinedSearchFormatter));
        registry.register(Box::new(SpotifySearchFormatter));
        registry.register(Box::new(PinsFormatter));

        // Resource detail formatters
        registry.register(Box::new(TrackDetailFormatter));
        registry.register(Box::new(AlbumDetailFormatter));
        registry.register(Box::new(ArtistDetailFormatter));
        registry.register(Box::new(PlaylistDetailFormatter));
        registry.register(Box::new(UserProfileFormatter));

        // Category formatters
        registry.register(Box::new(CategoryListFormatter));
        registry.register(Box::new(CategoryDetailFormatter));

        // Media formatters (podcasts, audiobooks)
        registry.register(Box::new(ShowDetailFormatter));
        registry.register(Box::new(EpisodeDetailFormatter));
        registry.register(Box::new(AudiobookDetailFormatter));
        registry.register(Box::new(ChapterDetailFormatter));

        // List formatters
        registry.register(Box::new(PlaylistsFormatter));
        registry.register(Box::new(SavedTracksFormatter));
        registry.register(Box::new(SavedAlbumsFormatter)); // Must be before TopTracksFormatter
        registry.register(Box::new(SavedShowsFormatter));
        registry.register(Box::new(ShowEpisodesFormatter));
        registry.register(Box::new(SavedEpisodesFormatter));
        registry.register(Box::new(SavedAudiobooksFormatter));
        registry.register(Box::new(AudiobookChaptersFormatter));
        registry.register(Box::new(TopTracksFormatter));
        registry.register(Box::new(TopArtistsFormatter));
        registry.register(Box::new(ArtistTopTracksFormatter));
        registry.register(Box::new(LibraryCheckFormatter));
        registry.register(Box::new(MarketsFormatter));

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

    /// Get the number of registered formatters (for testing)
    #[cfg(test)]
    pub fn len(&self) -> usize {
        self.formatters.len()
    }

    /// Check if registry is empty (for testing)
    #[cfg(test)]
    pub fn is_empty(&self) -> bool {
        self.formatters.is_empty()
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

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn formatter_registry_has_formatters() {
        let registry = FormatterRegistry::new();
        assert!(!registry.is_empty());
    }

    #[test]
    fn registry_format_with_kind_uses_kind_matching() {
        let registry = FormatterRegistry::new();
        let payload = json!({ "item": { "name": "Test" }, "is_playing": true });
        registry.format_with_kind(&payload, "Test", Some(PayloadKind::PlayerStatus));
    }

    #[test]
    fn registry_format_with_kind_falls_back_to_payload_matching() {
        let registry = FormatterRegistry::new();
        let payload = json!({ "item": { "name": "Test" }, "is_playing": true });
        registry.format_with_kind(&payload, "Test", None);
    }

    #[test]
    fn registry_format_with_unknown_prints_message() {
        let registry = FormatterRegistry::new();
        let payload = json!({ "unknown_field": "value" });
        registry.format(&payload, "No match found");
    }

    #[test]
    fn global_registry_accessible() {
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
        assert_eq!(default_registry.len(), new_registry.len());
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
}
