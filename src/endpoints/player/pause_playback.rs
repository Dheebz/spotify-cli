/// Pause playback on the user's account.
///
/// See: https://developer.spotify.com/documentation/web-api/reference/pause-a-users-playback
use crate::http::api::SpotifyApi;
use crate::http::client::HttpError;
use crate::http::endpoints::Endpoint;
use serde_json::Value;

pub async fn pause_playback(client: &SpotifyApi) -> Result<Option<Value>, HttpError> {
    client.put(&Endpoint::PlayerPause.path()).await
}
