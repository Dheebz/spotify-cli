/// Toggle shuffle on or off for user's playback.
///
/// See: https://developer.spotify.com/documentation/web-api/reference/toggle-shuffle-for-users-playback
use crate::http::api::SpotifyApi;
use crate::http::client::HttpError;
use crate::http::endpoints::Endpoint;
use serde_json::Value;

pub async fn toggle_playback_shuffle(
    client: &SpotifyApi,
    state: bool,
) -> Result<Option<Value>, HttpError> {
    client.put(&Endpoint::PlayerShuffle { state }.path()).await
}
