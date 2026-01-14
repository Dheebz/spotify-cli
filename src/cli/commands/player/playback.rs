//! Playback control commands: play, pause, toggle, next, previous

use crate::endpoints::player::{pause_playback, skip_to_next, skip_to_previous, start_resume_playback};
use crate::io::output::{ErrorKind, Response};

use crate::cli::commands::{init_pin_store, now_playing, with_client};

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
        let playing = match now_playing::is_playing(&client).await {
            Ok(p) => p,
            Err(e) => return e,
        };

        if playing {
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

    // Determine if this is a track URI (needs uris param) or context URI (album/playlist/artist)
    let is_track_uri = context_uri
        .as_ref()
        .map(|u| u.starts_with("spotify:track:"))
        .unwrap_or(false);

    with_client(|client| async move {
        // If no URI provided, check if already playing to avoid 403
        if !has_uri {
            match now_playing::is_playing(&client).await {
                Ok(true) => return Response::success(204, "Already playing"),
                Ok(false) => {}
                Err(e) => return e,
            }
        }

        // Track URIs must be passed via `uris` param, context URIs via `context_uri`
        let result = if is_track_uri {
            let track_uris = vec![context_uri.clone().unwrap()];
            start_resume_playback::start_resume_playback(&client, None, Some(&track_uris)).await
        } else {
            start_resume_playback::start_resume_playback(&client, context_uri.as_deref(), None).await
        };

        match result {
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
        // Check if already paused to avoid 403
        match now_playing::is_playing(&client).await {
            Ok(false) => return Response::success(204, "Already paused"),
            Ok(true) => {}
            Err(e) => return e,
        }

        match pause_playback::pause_playback(&client).await {
            Ok(_) => Response::success(204, "Playback paused"),
            Err(e) => Response::from_http_error(&e, "Failed to pause playback"),
        }
    })
    .await
}
