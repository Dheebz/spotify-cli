//! Helpers for getting information about the currently playing track

use crate::endpoints::player::get_playback_state;
use crate::http::api::SpotifyApi;
use crate::io::output::{ErrorKind, Response};
use crate::types::PlaybackState;

/// Get the current playback state as a typed struct
pub async fn get_state(client: &SpotifyApi) -> Result<PlaybackState, Response> {
    match get_playback_state::get_playback_state(client).await {
        Ok(Some(value)) => {
            serde_json::from_value(value).map_err(|e| {
                Response::err_with_details(
                    500,
                    "Failed to parse playback state",
                    ErrorKind::Api,
                    e.to_string(),
                )
            })
        }
        Ok(None) => Err(Response::err(404, "Nothing currently playing", ErrorKind::Player)),
        Err(e) => Err(Response::from_http_error(&e, "Failed to get playback state")),
    }
}

/// Get the currently playing track's ID
pub async fn get_track_id(client: &SpotifyApi) -> Result<String, Response> {
    let state = get_state(client).await?;
    state
        .item
        .map(|track| track.id)
        .ok_or_else(|| Response::err(404, "No track in playback state", ErrorKind::Player))
}

/// Get the currently playing track's URI (spotify:track:xxx)
pub async fn get_track_uri(client: &SpotifyApi) -> Result<String, Response> {
    let state = get_state(client).await?;
    state
        .item
        .map(|track| track.uri)
        .ok_or_else(|| Response::err(404, "No track in playback state", ErrorKind::Player))
}

/// Get the album ID of the currently playing track
pub async fn get_album_id(client: &SpotifyApi) -> Result<String, Response> {
    let state = get_state(client).await?;
    state
        .item
        .and_then(|track| track.album)
        .map(|album| album.id)
        .ok_or_else(|| Response::err(404, "No album in playback state", ErrorKind::Player))
}

/// Get the primary artist ID of the currently playing track
pub async fn get_artist_id(client: &SpotifyApi) -> Result<String, Response> {
    let state = get_state(client).await?;
    state
        .item
        .and_then(|track| track.artists)
        .and_then(|artists| artists.into_iter().next())
        .map(|artist| artist.id)
        .ok_or_else(|| Response::err(404, "No artist in playback state", ErrorKind::Player))
}

/// Check if playback is currently active (playing vs paused)
pub async fn is_playing(client: &SpotifyApi) -> Result<bool, Response> {
    let state = get_state(client).await?;
    Ok(state.is_playing)
}
