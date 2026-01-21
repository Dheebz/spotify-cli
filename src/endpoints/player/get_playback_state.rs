/// Get information about the user's current playback state.
///
/// See: https://developer.spotify.com/documentation/web-api/reference/get-information-about-the-users-current-playback
use crate::http::api::SpotifyApi;
use crate::http::client::HttpError;
use crate::http::endpoints::Endpoint;
use serde_json::Value;

pub async fn get_playback_state(client: &SpotifyApi) -> Result<Option<Value>, HttpError> {
    client.get(&Endpoint::PlayerState.path()).await
}
