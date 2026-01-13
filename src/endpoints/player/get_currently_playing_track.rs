use crate::http::api::SpotifyApi;
use crate::http::client::HttpError;
use crate::http::endpoints::Endpoint;
use serde_json::Value;

/// Get the currently playing track (lighter endpoint than get_playback_state)
pub async fn get_currently_playing_track(
    client: &SpotifyApi,
) -> Result<Option<Value>, HttpError> {
    client.get(&Endpoint::PlayerCurrentlyPlaying.path()).await
}
