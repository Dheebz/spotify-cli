//! Pin output formatting.
use serde::Serialize;

use crate::domain::pin::PinnedPlaylist;
use crate::error::Result;
use crate::output::human::truncate_cell;
use crate::output::{DEFAULT_MAX_WIDTH, TableConfig};

pub fn pin_list_human(pins: Vec<PinnedPlaylist>, table: TableConfig) -> Result<()> {
    if pins.is_empty() {
        return Ok(());
    }

    let max_width = table.max_width.unwrap_or(DEFAULT_MAX_WIDTH);
    let mut names: Vec<String> = pins
        .iter()
        .map(|pin| {
            if table.truncate {
                truncate_cell(&pin.name, max_width)
            } else {
                pin.name.clone()
            }
        })
        .collect();
    let mut urls: Vec<String> = pins
        .iter()
        .map(|pin| {
            if table.truncate {
                truncate_cell(&pin.url, max_width)
            } else {
                pin.url.clone()
            }
        })
        .collect();

    let name_width = names
        .iter()
        .map(|name| name.len())
        .max()
        .unwrap_or(0)
        .max("NAME".len());

    println!("{:<width$}  URL", "NAME", width = name_width);

    for (name, url) in names.drain(..).zip(urls.drain(..)) {
        println!("{:<width$}  {}", name, url, width = name_width);
    }
    Ok(())
}

#[derive(Serialize)]
struct PinPayload {
    name: String,
    url: String,
}

pub fn pin_list_json(pins: Vec<PinnedPlaylist>) -> Result<()> {
    let payload = pin_list_payload(pins);
    println!("{}", serde_json::to_string(&payload)?);
    Ok(())
}

fn pin_list_payload(pins: Vec<PinnedPlaylist>) -> Vec<PinPayload> {
    pins.into_iter()
        .map(|pin| PinPayload {
            name: pin.name,
            url: pin.url,
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::pin_list_payload;
    use crate::domain::pin::PinnedPlaylist;

    #[test]
    fn pin_list_payload_shape() {
        let payload = pin_list_payload(vec![PinnedPlaylist {
            name: "Release Radar".to_string(),
            url: "url".to_string(),
        }]);
        assert_eq!(payload.len(), 1);
        assert_eq!(payload[0].name, "Release Radar");
    }
}
