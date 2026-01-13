//! Player settings commands: volume, seek, shuffle, repeat

use crate::endpoints::player::{
    seek_to_position, set_playback_volume, set_repeat_mode, toggle_playback_shuffle,
};
use crate::io::output::{ErrorKind, Response};

use crate::cli::commands::with_client;

fn parse_position_to_ms(position: &str) -> Result<u64, String> {
    let position = position.trim();

    if position.contains(':') {
        let parts: Vec<&str> = position.split(':').collect();
        match parts.len() {
            2 => {
                let mins: u64 = parts[0]
                    .parse()
                    .map_err(|_| "Invalid minutes".to_string())?;
                let secs: u64 = parts[1]
                    .parse()
                    .map_err(|_| "Invalid seconds".to_string())?;
                Ok((mins * 60 + secs) * 1000)
            }
            3 => {
                let hours: u64 = parts[0].parse().map_err(|_| "Invalid hours".to_string())?;
                let mins: u64 = parts[1]
                    .parse()
                    .map_err(|_| "Invalid minutes".to_string())?;
                let secs: u64 = parts[2]
                    .parse()
                    .map_err(|_| "Invalid seconds".to_string())?;
                Ok((hours * 3600 + mins * 60 + secs) * 1000)
            }
            _ => Err("Invalid time format. Use mm:ss or hh:mm:ss".to_string()),
        }
    } else if let Some(ms_str) = position.strip_suffix("ms") {
        ms_str
            .parse()
            .map_err(|_| "Invalid milliseconds".to_string())
    } else if let Some(s_str) = position.strip_suffix('s') {
        let secs: u64 = s_str
            .parse()
            .map_err(|_| "Invalid seconds".to_string())?;
        Ok(secs * 1000)
    } else {
        let secs: u64 = position
            .parse()
            .map_err(|_| "Invalid position. Use: 90, 1:30, 90s, or 5000ms".to_string())?;
        Ok(secs * 1000)
    }
}

pub async fn player_seek(position: &str) -> Response {
    let position_ms = match parse_position_to_ms(position) {
        Ok(ms) => ms,
        Err(e) => return Response::err(400, &e, ErrorKind::Validation),
    };

    with_client(|client| async move {
        match seek_to_position::seek_to_position(&client, position_ms).await {
            Ok(_) => Response::success(204, "Seeked to position"),
            Err(e) => Response::from_http_error(&e, "Failed to seek"),
        }
    })
    .await
}

pub async fn player_repeat(mode: &str) -> Response {
    let mode = mode.to_string();
    with_client(|client| async move {
        match set_repeat_mode::set_repeat_mode(&client, &mode).await {
            Ok(_) => Response::success(204, format!("Repeat mode set to {}", mode)),
            Err(e) => Response::from_http_error(&e, "Failed to set repeat mode"),
        }
    })
    .await
}

pub async fn player_volume(percent: u8) -> Response {
    with_client(|client| async move {
        match set_playback_volume::set_playback_volume(&client, percent).await {
            Ok(_) => Response::success(204, format!("Volume set to {}%", percent)),
            Err(e) => Response::from_http_error(&e, "Failed to set volume"),
        }
    })
    .await
}

pub async fn player_shuffle(state: &str) -> Response {
    let enabled = state == "on";
    with_client(|client| async move {
        match toggle_playback_shuffle::toggle_playback_shuffle(&client, enabled).await {
            Ok(_) => Response::success(
                204,
                if enabled {
                    "Shuffle enabled"
                } else {
                    "Shuffle disabled"
                },
            ),
            Err(e) => Response::from_http_error(&e, "Failed to set shuffle"),
        }
    })
    .await
}
