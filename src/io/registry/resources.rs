//! Individual resource detail formatters (track, album, artist, playlist, user, category)

use serde_json::Value;

use super::PayloadFormatter;
use crate::io::formatters;
use crate::io::output::PayloadKind;

pub struct TrackDetailFormatter;

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

pub struct AlbumDetailFormatter;

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

pub struct ArtistDetailFormatter;

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

pub struct PlaylistDetailFormatter;

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

pub struct UserProfileFormatter;

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

pub struct CategoryListFormatter;

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

pub struct CategoryDetailFormatter;

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

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn track_formatter_supports_kind() {
        let formatter = TrackDetailFormatter;
        assert!(formatter.supported_kinds().contains(&PayloadKind::Track));
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
    fn playlist_detail_formatter_matches() {
        let formatter = PlaylistDetailFormatter;
        let payload = json!({ "owner": {}, "tracks": {} });
        assert!(formatter.matches(&payload));
        assert!(!formatter.matches(&json!({})));
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
    fn formatter_names() {
        assert_eq!(TrackDetailFormatter.name(), "track_detail");
        assert_eq!(AlbumDetailFormatter.name(), "album_detail");
        assert_eq!(ArtistDetailFormatter.name(), "artist_detail");
        assert_eq!(PlaylistDetailFormatter.name(), "playlist_detail");
        assert_eq!(UserProfileFormatter.name(), "user_profile");
        assert_eq!(CategoryListFormatter.name(), "category_list");
        assert_eq!(CategoryDetailFormatter.name(), "category_detail");
    }

    #[test]
    fn track_detail_format_runs() {
        let formatter = TrackDetailFormatter;
        let payload = json!({
            "name": "Test Track",
            "album": {"name": "Album"},
            "artists": [{"name": "Artist"}],
            "duration_ms": 180000,
            "popularity": 80
        });
        formatter.format(&payload, "Track");
    }

    #[test]
    fn album_detail_format_runs() {
        let formatter = AlbumDetailFormatter;
        let payload = json!({
            "name": "Test Album",
            "album_type": "album",
            "tracks": {"items": []},
            "artists": [{"name": "Artist"}],
            "release_date": "2024"
        });
        formatter.format(&payload, "Album");
    }

    #[test]
    fn artist_detail_format_runs() {
        let formatter = ArtistDetailFormatter;
        let payload = json!({
            "name": "Test Artist",
            "followers": {"total": 1000},
            "genres": ["rock"],
            "popularity": 75
        });
        formatter.format(&payload, "Artist");
    }

    #[test]
    fn playlist_detail_format_runs() {
        let formatter = PlaylistDetailFormatter;
        let payload = json!({
            "name": "Test Playlist",
            "owner": {"display_name": "User"},
            "tracks": {"total": 10, "items": []},
            "public": true
        });
        formatter.format(&payload, "Playlist");
    }

    #[test]
    fn user_profile_format_runs() {
        let formatter = UserProfileFormatter;
        let payload = json!({
            "display_name": "Test User",
            "product": "premium",
            "followers": {"total": 100},
            "id": "user123"
        });
        formatter.format(&payload, "User");
    }

    #[test]
    fn category_list_format_runs() {
        let formatter = CategoryListFormatter;
        let payload = json!({
            "categories": {
                "items": [{"id": "pop", "name": "Pop"}]
            }
        });
        formatter.format(&payload, "Categories");
    }

    #[test]
    fn category_detail_format_runs() {
        let formatter = CategoryDetailFormatter;
        let payload = json!({
            "id": "pop",
            "name": "Pop",
            "icons": [{"url": "http://example.com/icon.png"}]
        });
        formatter.format(&payload, "Category");
    }

    #[test]
    fn artist_detail_supports_related_artists() {
        let formatter = ArtistDetailFormatter;
        let kinds = formatter.supported_kinds();
        assert!(kinds.contains(&PayloadKind::Artist));
        assert!(kinds.contains(&PayloadKind::RelatedArtists));
    }

    #[test]
    fn track_detail_does_not_match_incomplete() {
        let formatter = TrackDetailFormatter;
        let payload = json!({ "album": {} });
        assert!(!formatter.matches(&payload));
    }

    #[test]
    fn category_detail_does_not_match_with_owner() {
        let formatter = CategoryDetailFormatter;
        let payload = json!({ "icons": [], "id": "test", "owner": {} });
        assert!(!formatter.matches(&payload));
    }
}
