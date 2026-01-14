//! Media formatters (podcasts/shows, episodes, audiobooks, chapters)

use serde_json::Value;

use super::PayloadFormatter;
use crate::io::formatters;
use crate::io::output::PayloadKind;

// ============================================================================
// Show/Podcast Formatters
// ============================================================================

pub struct ShowDetailFormatter;

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

pub struct SavedShowsFormatter;

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

// ============================================================================
// Episode Formatters
// ============================================================================

pub struct EpisodeDetailFormatter;

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

pub struct ShowEpisodesFormatter;

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

pub struct SavedEpisodesFormatter;

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

// ============================================================================
// Audiobook Formatters
// ============================================================================

pub struct AudiobookDetailFormatter;

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

pub struct SavedAudiobooksFormatter;

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

// ============================================================================
// Chapter Formatters
// ============================================================================

pub struct ChapterDetailFormatter;

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

pub struct AudiobookChaptersFormatter;

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

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    // Show tests
    #[test]
    fn show_detail_formatter_matches() {
        let formatter = ShowDetailFormatter;
        let payload = json!({ "publisher": "Test", "total_episodes": 10 });
        assert!(formatter.matches(&payload));
        assert!(!formatter.matches(&json!({})));
    }

    #[test]
    fn saved_shows_formatter_matches() {
        let formatter = SavedShowsFormatter;
        let payload = json!({ "items": [{ "show": {} }] });
        assert!(formatter.matches(&payload));
        let direct = json!({ "items": [{ "publisher": "Test", "total_episodes": 5 }] });
        assert!(formatter.matches(&direct));
    }

    // Episode tests
    #[test]
    fn episode_detail_formatter_matches() {
        let formatter = EpisodeDetailFormatter;
        let payload = json!({ "show": {}, "release_date": "2024", "duration_ms": 1000 });
        assert!(formatter.matches(&payload));
        assert!(!formatter.matches(&json!({})));
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

    // Audiobook tests
    #[test]
    fn audiobook_detail_formatter_matches() {
        let formatter = AudiobookDetailFormatter;
        let payload = json!({ "authors": [], "total_chapters": 10 });
        assert!(formatter.matches(&payload));
        assert!(!formatter.matches(&json!({})));
    }

    #[test]
    fn saved_audiobooks_formatter_matches() {
        let formatter = SavedAudiobooksFormatter;
        let payload = json!({ "items": [{ "audiobook": {} }] });
        assert!(formatter.matches(&payload));
        let direct = json!({ "items": [{ "authors": [], "total_chapters": 5 }] });
        assert!(formatter.matches(&direct));
    }

    // Chapter tests
    #[test]
    fn chapter_detail_formatter_matches() {
        let formatter = ChapterDetailFormatter;
        let payload = json!({ "audiobook": {}, "chapter_number": 1 });
        assert!(formatter.matches(&payload));
        assert!(!formatter.matches(&json!({})));
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
    fn formatter_names() {
        assert_eq!(ShowDetailFormatter.name(), "show_detail");
        assert_eq!(SavedShowsFormatter.name(), "saved_shows");
        assert_eq!(EpisodeDetailFormatter.name(), "episode_detail");
        assert_eq!(ShowEpisodesFormatter.name(), "show_episodes");
        assert_eq!(SavedEpisodesFormatter.name(), "saved_episodes");
        assert_eq!(AudiobookDetailFormatter.name(), "audiobook_detail");
        assert_eq!(SavedAudiobooksFormatter.name(), "saved_audiobooks");
        assert_eq!(ChapterDetailFormatter.name(), "chapter_detail");
        assert_eq!(AudiobookChaptersFormatter.name(), "audiobook_chapters");
    }

    // Format method tests
    #[test]
    fn show_detail_format_runs() {
        let formatter = ShowDetailFormatter;
        let payload = json!({
            "name": "Test Show",
            "publisher": "Publisher",
            "total_episodes": 50,
            "description": "A test show"
        });
        formatter.format(&payload, "Show");
    }

    #[test]
    fn saved_shows_format_runs() {
        let formatter = SavedShowsFormatter;
        let payload = json!({
            "items": [{
                "show": {"name": "Show", "publisher": "Publisher"}
            }]
        });
        formatter.format(&payload, "Shows");
    }

    #[test]
    fn episode_detail_format_runs() {
        let formatter = EpisodeDetailFormatter;
        let payload = json!({
            "name": "Episode 1",
            "show": {"name": "Show"},
            "release_date": "2024-01-01",
            "duration_ms": 3600000,
            "description": "First episode"
        });
        formatter.format(&payload, "Episode");
    }

    #[test]
    fn show_episodes_format_runs() {
        let formatter = ShowEpisodesFormatter;
        let payload = json!({
            "items": [{
                "name": "Episode",
                "release_date": "2024-01-01",
                "duration_ms": 1800000
            }]
        });
        formatter.format(&payload, "Episodes");
    }

    #[test]
    fn saved_episodes_format_runs() {
        let formatter = SavedEpisodesFormatter;
        let payload = json!({
            "items": [{
                "episode": {"name": "Saved Episode", "duration_ms": 1800000}
            }]
        });
        formatter.format(&payload, "Saved Episodes");
    }

    #[test]
    fn audiobook_detail_format_runs() {
        let formatter = AudiobookDetailFormatter;
        let payload = json!({
            "name": "Test Audiobook",
            "authors": [{"name": "Author"}],
            "total_chapters": 20,
            "description": "An audiobook"
        });
        formatter.format(&payload, "Audiobook");
    }

    #[test]
    fn saved_audiobooks_format_runs() {
        let formatter = SavedAudiobooksFormatter;
        let payload = json!({
            "items": [{
                "audiobook": {"name": "Audiobook", "authors": []}
            }]
        });
        formatter.format(&payload, "Audiobooks");
    }

    #[test]
    fn chapter_detail_format_runs() {
        let formatter = ChapterDetailFormatter;
        let payload = json!({
            "name": "Chapter 1",
            "audiobook": {"name": "Book"},
            "chapter_number": 1,
            "duration_ms": 900000
        });
        formatter.format(&payload, "Chapter");
    }

    #[test]
    fn audiobook_chapters_format_runs() {
        let formatter = AudiobookChaptersFormatter;
        let payload = json!({
            "items": [{
                "name": "Chapter 1",
                "chapter_number": 1,
                "duration_ms": 900000
            }]
        });
        formatter.format(&payload, "Chapters");
    }

    // Edge case tests
    #[test]
    fn saved_shows_does_not_match_empty() {
        let formatter = SavedShowsFormatter;
        let payload = json!({ "items": [] });
        assert!(!formatter.matches(&payload));
    }

    #[test]
    fn saved_audiobooks_does_not_match_empty() {
        let formatter = SavedAudiobooksFormatter;
        let payload = json!({ "items": [] });
        assert!(!formatter.matches(&payload));
    }

    #[test]
    fn audiobook_chapters_does_not_match_empty() {
        let formatter = AudiobookChaptersFormatter;
        let payload = json!({ "items": [] });
        assert!(!formatter.matches(&payload));
    }

    #[test]
    fn show_episodes_does_not_match_with_artists() {
        let formatter = ShowEpisodesFormatter;
        let payload = json!({ "items": [{ "release_date": "2024", "duration_ms": 1000, "artists": [] }] });
        assert!(!formatter.matches(&payload));
    }
}
