//! Player-related formatters (status, queue, devices, history)

use serde_json::Value;

use super::PayloadFormatter;
use crate::io::formatters;
use crate::io::output::PayloadKind;

pub struct PlayerStatusFormatter;

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

pub struct QueueFormatter;

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

pub struct DevicesFormatter;

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

pub struct PlayHistoryFormatter;

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

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn player_status_formatter_supports_kind() {
        let formatter = PlayerStatusFormatter;
        assert!(formatter.supported_kinds().contains(&PayloadKind::PlayerStatus));
    }

    #[test]
    fn player_status_matches_payload_with_item() {
        let formatter = PlayerStatusFormatter;
        let payload = json!({ "item": {"name": "Track"}, "is_playing": true });
        assert!(formatter.matches(&payload));
    }

    #[test]
    fn player_status_does_not_match_without_item() {
        let formatter = PlayerStatusFormatter;
        let payload = json!({"is_playing": true});
        assert!(!formatter.matches(&payload));
    }

    #[test]
    fn queue_formatter_supports_kind() {
        let formatter = QueueFormatter;
        assert!(formatter.supported_kinds().contains(&PayloadKind::Queue));
    }

    #[test]
    fn queue_matches_payload_with_currently_playing_and_queue() {
        let formatter = QueueFormatter;
        let payload = json!({ "currently_playing": {"name": "Track"}, "queue": [] });
        assert!(formatter.matches(&payload));
    }

    #[test]
    fn devices_formatter_matches() {
        let formatter = DevicesFormatter;
        let payload = json!({ "devices": [] });
        assert!(formatter.matches(&payload));
        assert!(!formatter.matches(&json!({})));
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
    fn formatter_names() {
        assert_eq!(PlayerStatusFormatter.name(), "player_status");
        assert_eq!(QueueFormatter.name(), "queue");
        assert_eq!(DevicesFormatter.name(), "devices");
        assert_eq!(PlayHistoryFormatter.name(), "play_history");
    }

    #[test]
    fn player_status_format_runs() {
        let formatter = PlayerStatusFormatter;
        let payload = json!({
            "item": {
                "name": "Test Track",
                "artists": [{"name": "Artist"}],
                "album": {"name": "Album"},
                "duration_ms": 180000
            },
            "is_playing": true,
            "progress_ms": 60000
        });
        formatter.format(&payload, "Playing");
    }

    #[test]
    fn queue_format_runs() {
        let formatter = QueueFormatter;
        let payload = json!({
            "currently_playing": {
                "name": "Current",
                "artists": [{"name": "Artist"}],
                "duration_ms": 180000
            },
            "queue": []
        });
        formatter.format(&payload, "Queue");
    }

    #[test]
    fn devices_format_runs() {
        let formatter = DevicesFormatter;
        let payload = json!({ "devices": [] });
        formatter.format(&payload, "Devices");
    }

    #[test]
    fn play_history_format_runs() {
        let formatter = PlayHistoryFormatter;
        let payload = json!({
            "items": [{
                "track": {"name": "Track", "artists": [{"name": "Artist"}]},
                "played_at": "2024-01-01T00:00:00Z"
            }]
        });
        formatter.format(&payload, "History");
    }

    #[test]
    fn player_status_format_with_null_item() {
        let formatter = PlayerStatusFormatter;
        let payload = json!({ "item": null, "is_playing": false });
        formatter.format(&payload, "No playback");
    }

    #[test]
    fn queue_does_not_match_without_queue() {
        let formatter = QueueFormatter;
        let payload = json!({ "currently_playing": {} });
        assert!(!formatter.matches(&payload));
    }

    #[test]
    fn play_history_does_not_match_without_played_at() {
        let formatter = PlayHistoryFormatter;
        let payload = json!({ "items": [{ "track": {} }] });
        assert!(!formatter.matches(&payload));
    }
}
