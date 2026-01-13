//! Playback helpers for search results

use crate::endpoints::player::start_resume_playback;
use crate::http::api::SpotifyApi;
use crate::io::output::Response;

/// Play a Spotify URI (track, album, playlist, or artist)
pub async fn play_uri(client: &SpotifyApi, uri: &str) -> Response {
    let is_context =
        uri.contains(":album:") || uri.contains(":playlist:") || uri.contains(":artist:");

    let result = if is_context {
        start_resume_playback::start_resume_playback(client, Some(uri), None).await
    } else {
        let uris = vec![uri.to_string()];
        start_resume_playback::start_resume_playback(client, None, Some(&uris)).await
    };

    match result {
        Ok(_) => Response::success(200, format!("Playing {}", uri)),
        Err(e) => Response::from_http_error(&e, "Failed to play"),
    }
}
