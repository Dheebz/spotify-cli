//! List formatters (playlists, saved tracks, top tracks/artists, etc.)

use serde_json::Value;

use super::PayloadFormatter;
use crate::io::formatters;
use crate::io::output::PayloadKind;

pub struct PlaylistsFormatter;

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

pub struct SavedTracksFormatter;

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

pub struct TopTracksFormatter;

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

pub struct TopArtistsFormatter;

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

pub struct ArtistTopTracksFormatter;

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

pub struct LibraryCheckFormatter;

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

pub struct SavedAlbumsFormatter;

impl PayloadFormatter for SavedAlbumsFormatter {
    fn name(&self) -> &'static str {
        "saved_albums"
    }

    fn supported_kinds(&self) -> &'static [PayloadKind] {
        &[PayloadKind::SavedAlbums]
    }

    fn matches(&self, payload: &Value) -> bool {
        // Saved albums have items with "album" and "added_at" fields
        if let Some(items) = payload.get("items").and_then(|i| i.as_array()) {
            !items.is_empty()
                && items[0].get("album").is_some()
                && items[0].get("added_at").is_some()
        } else {
            false
        }
    }

    fn format(&self, payload: &Value, _message: &str) {
        if let Some(items) = payload.get("items").and_then(|i| i.as_array()) {
            formatters::format_saved_albums(items);
        }
    }
}

pub struct MarketsFormatter;

impl PayloadFormatter for MarketsFormatter {
    fn name(&self) -> &'static str {
        "markets"
    }

    fn supported_kinds(&self) -> &'static [PayloadKind] {
        &[PayloadKind::Markets]
    }

    fn matches(&self, payload: &Value) -> bool {
        payload.get("markets").map(|m| m.is_array()).unwrap_or(false)
    }

    fn format(&self, payload: &Value, _message: &str) {
        if let Some(markets) = payload.get("markets").and_then(|m| m.as_array()) {
            formatters::format_markets(markets);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn playlist_formatter_supports_multiple_kinds() {
        let formatter = PlaylistsFormatter;
        let kinds = formatter.supported_kinds();
        assert!(kinds.contains(&PayloadKind::PlaylistList));
        assert!(kinds.contains(&PayloadKind::FeaturedPlaylists));
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
    fn top_tracks_formatter_matches() {
        let formatter = TopTracksFormatter;
        let payload = json!({ "items": [{ "album": {} }] });
        assert!(formatter.matches(&payload));
        let empty = json!({ "items": [] });
        assert!(!formatter.matches(&empty));
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
    fn formatter_names() {
        assert_eq!(PlaylistsFormatter.name(), "playlists");
        assert_eq!(SavedTracksFormatter.name(), "saved_tracks");
        assert_eq!(TopTracksFormatter.name(), "top_tracks");
        assert_eq!(TopArtistsFormatter.name(), "top_artists");
        assert_eq!(ArtistTopTracksFormatter.name(), "artist_top_tracks");
        assert_eq!(LibraryCheckFormatter.name(), "library_check");
    }

    #[test]
    fn playlists_format_runs() {
        let formatter = PlaylistsFormatter;
        let payload = json!({
            "items": [{
                "name": "My Playlist",
                "tracks": {"total": 10},
                "owner": {"display_name": "User"}
            }]
        });
        formatter.format(&payload, "Playlists");
    }

    #[test]
    fn saved_tracks_format_runs() {
        let formatter = SavedTracksFormatter;
        let payload = json!({
            "items": [{
                "track": {"name": "Track", "artists": [{"name": "Artist"}]},
                "added_at": "2024-01-01"
            }]
        });
        formatter.format(&payload, "Saved Tracks");
    }

    #[test]
    fn top_tracks_format_runs() {
        let formatter = TopTracksFormatter;
        let payload = json!({
            "items": [{
                "name": "Track",
                "album": {"name": "Album"},
                "artists": [{"name": "Artist"}]
            }]
        });
        formatter.format(&payload, "Top Tracks");
    }

    #[test]
    fn top_artists_format_runs() {
        let formatter = TopArtistsFormatter;
        let payload = json!({
            "items": [{
                "name": "Artist",
                "genres": ["rock"],
                "popularity": 80
            }]
        });
        formatter.format(&payload, "Top Artists");
    }

    #[test]
    fn artist_top_tracks_format_runs() {
        let formatter = ArtistTopTracksFormatter;
        let payload = json!({
            "tracks": [{
                "name": "Track",
                "album": {"name": "Album"}
            }]
        });
        formatter.format(&payload, "Artist Top Tracks");
    }

    #[test]
    fn library_check_format_runs() {
        let formatter = LibraryCheckFormatter;
        let payload = json!([true, false, true]);
        formatter.format(&payload, "Library Check");
    }

    #[test]
    fn playlists_matches_with_owner() {
        let formatter = PlaylistsFormatter;
        let payload = json!({ "items": [{ "owner": {} }] });
        assert!(formatter.matches(&payload));
    }

    #[test]
    fn saved_tracks_does_not_match_without_added_at() {
        let formatter = SavedTracksFormatter;
        let payload = json!({ "items": [{ "track": {} }] });
        assert!(!formatter.matches(&payload));
    }

    #[test]
    fn library_check_does_not_match_empty_array() {
        let formatter = LibraryCheckFormatter;
        let payload = json!([]);
        assert!(!formatter.matches(&payload));
    }

    #[test]
    fn library_check_does_not_match_non_array() {
        let formatter = LibraryCheckFormatter;
        let payload = json!({ "data": true });
        assert!(!formatter.matches(&payload));
    }
}
