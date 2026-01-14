//! Pin formatting functions

use serde_json::Value;

pub fn format_pins(pins: &[Value]) {
    if pins.is_empty() {
        println!("No pins saved.");
        return;
    }
    println!("Pinned Resources:");
    for pin in pins {
        let alias = pin.get("alias").and_then(|v| v.as_str()).unwrap_or("Unknown");
        let rtype = pin.get("type").and_then(|v| v.as_str()).unwrap_or("unknown");
        let tags = pin
            .get("tags")
            .and_then(|v| v.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|t| t.as_str())
                    .collect::<Vec<_>>()
                    .join(", ")
            })
            .unwrap_or_default();

        if tags.is_empty() {
            println!("  [{}] {}", rtype, alias);
        } else {
            println!("  [{}] {} ({})", rtype, alias, tags);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn format_pins_empty() {
        let pins: Vec<Value> = vec![];
        format_pins(&pins);
    }

    #[test]
    fn format_pins_single_without_tags() {
        let pins = vec![json!({
            "alias": "favorite",
            "type": "track"
        })];
        format_pins(&pins);
    }

    #[test]
    fn format_pins_single_with_tags() {
        let pins = vec![json!({
            "alias": "workout mix",
            "type": "playlist",
            "tags": ["gym", "energy", "rock"]
        })];
        format_pins(&pins);
    }

    #[test]
    fn format_pins_multiple() {
        let pins = vec![
            json!({ "alias": "chill", "type": "playlist", "tags": ["relax"] }),
            json!({ "alias": "best song", "type": "track" }),
            json!({ "alias": "fav artist", "type": "artist", "tags": ["rock", "metal"] }),
        ];
        format_pins(&pins);
    }

    #[test]
    fn format_pins_minimal_data() {
        let pins = vec![json!({})];
        format_pins(&pins);
    }

    #[test]
    fn format_pins_empty_tags_array() {
        let pins = vec![json!({
            "alias": "test",
            "type": "album",
            "tags": []
        })];
        format_pins(&pins);
    }
}
