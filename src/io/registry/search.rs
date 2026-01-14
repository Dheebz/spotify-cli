//! Search-related formatters (combined search, Spotify search, pins)

use serde_json::Value;

use super::PayloadFormatter;
use crate::io::formatters;
use crate::io::output::PayloadKind;

pub struct CombinedSearchFormatter;

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

pub struct SpotifySearchFormatter;

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

pub struct PinsFormatter;

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

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

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
        assert_eq!(CombinedSearchFormatter.name(), "combined_search");
        assert_eq!(SpotifySearchFormatter.name(), "spotify_search");
        assert_eq!(PinsFormatter.name(), "pins");
    }

    #[test]
    fn combined_search_format_runs() {
        let formatter = CombinedSearchFormatter;
        let payload = json!({
            "spotify": {
                "tracks": { "items": [] },
                "artists": { "items": [] }
            },
            "pins": []
        });
        formatter.format(&payload, "Search Results");
    }

    #[test]
    fn spotify_search_format_runs() {
        let formatter = SpotifySearchFormatter;
        let payload = json!({
            "tracks": {
                "items": [{"name": "Track", "artists": [{"name": "Artist"}]}]
            }
        });
        formatter.format(&payload, "Tracks");
    }

    #[test]
    fn pins_format_runs() {
        let formatter = PinsFormatter;
        let payload = json!({
            "pins": [{
                "alias": "my-song",
                "resource_type": "track",
                "id": "abc123"
            }]
        });
        formatter.format(&payload, "Pins");
    }

    #[test]
    fn combined_search_supports_kind() {
        let formatter = CombinedSearchFormatter;
        assert!(formatter.supported_kinds().contains(&PayloadKind::CombinedSearch));
    }

    #[test]
    fn spotify_search_supports_kind() {
        let formatter = SpotifySearchFormatter;
        assert!(formatter.supported_kinds().contains(&PayloadKind::SearchResults));
    }

    #[test]
    fn pins_supports_kind() {
        let formatter = PinsFormatter;
        assert!(formatter.supported_kinds().contains(&PayloadKind::Pins));
    }

    #[test]
    fn pins_format_handles_empty_array() {
        let formatter = PinsFormatter;
        let payload = json!({ "pins": [] });
        formatter.format(&payload, "No pins");
    }
}
