//! Playback control commands: play, pause, toggle, next, previous

use crate::endpoints::player::{
    get_playback_state, pause_playback, skip_to_next, skip_to_previous, start_resume_playback,
};
use crate::io::output::{ErrorKind, Response};

use crate::cli::commands::{init_pin_store, with_client};

/// Convert a Spotify URL to a Spotify URI, or return input if already a URI
fn url_to_uri(input: &str) -> String {
    if input.contains("open.spotify.com") {
        let parts: Vec<&str> = input.split('/').collect();
        if parts.len() >= 2 {
            let id = parts
                .last()
                .unwrap_or(&"")
                .split('?')
                .next()
                .unwrap_or("");
            let resource_type = parts.get(parts.len() - 2).unwrap_or(&"");
            return format!("spotify:{}:{}", resource_type, id);
        }
    }
    input.to_string()
}

pub async fn player_next() -> Response {
    with_client(|client| async move {
        match skip_to_next::skip_to_next(&client).await {
            Ok(_) => Response::success(204, "Skipped to next track"),
            Err(e) => Response::from_http_error(&e, "Failed to skip to next track"),
        }
    })
    .await
}

pub async fn player_previous() -> Response {
    with_client(|client| async move {
        match skip_to_previous::skip_to_previous(&client).await {
            Ok(_) => Response::success(204, "Skipped to previous track"),
            Err(e) => Response::from_http_error(&e, "Failed to skip to previous track"),
        }
    })
    .await
}

pub async fn player_toggle() -> Response {
    with_client(|client| async move {
        match get_playback_state::get_playback_state(&client).await {
            Ok(Some(state)) => {
                let is_playing = state
                    .get("is_playing")
                    .and_then(|v| v.as_bool())
                    .unwrap_or(false);

                if is_playing {
                    match pause_playback::pause_playback(&client).await {
                        Ok(_) => Response::success(204, "Playback paused"),
                        Err(e) => Response::from_http_error(&e, "Failed to pause playback"),
                    }
                } else {
                    match start_resume_playback::start_resume_playback(&client, None, None).await {
                        Ok(_) => Response::success(204, "Playback started"),
                        Err(e) => Response::from_http_error(&e, "Failed to start playback"),
                    }
                }
            }
            Ok(None) => Response::err(404, "No active playback device", ErrorKind::Player),
            Err(e) => Response::from_http_error(&e, "Failed to get playback state"),
        }
    })
    .await
}

pub async fn player_play(uri: Option<&str>, pin: Option<&str>) -> Response {
    // Resolve pin to URI if provided (before auth)
    let context_uri: Option<String> = if let Some(pin_alias) = pin {
        let store = match init_pin_store() {
            Ok(s) => s,
            Err(e) => return e,
        };

        match store.find_by_alias(pin_alias) {
            Some(p) => Some(p.uri()),
            None => return Response::err(404, "Pin not found", ErrorKind::NotFound),
        }
    } else {
        uri.map(url_to_uri)
    };

    let has_uri = context_uri.is_some();
    with_client(|client| async move {
        match start_resume_playback::start_resume_playback(&client, context_uri.as_deref(), None)
            .await
        {
            Ok(_) => {
                if has_uri {
                    Response::success(204, "Playing requested content")
                } else {
                    Response::success(204, "Playback started")
                }
            }
            Err(e) => Response::from_http_error(&e, "Failed to start playback"),
        }
    })
    .await
}

pub async fn player_pause() -> Response {
    with_client(|client| async move {
        match pause_playback::pause_playback(&client).await {
            Ok(_) => Response::success(204, "Playback paused"),
            Err(e) => Response::from_http_error(&e, "Failed to pause playback"),
        }
    })
    .await
}
