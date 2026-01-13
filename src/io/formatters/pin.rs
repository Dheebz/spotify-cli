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
