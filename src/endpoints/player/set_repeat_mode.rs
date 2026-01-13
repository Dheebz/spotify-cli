/// Set the repeat mode for the user's playback.
///
/// See: https://developer.spotify.com/documentation/web-api/reference/set-repeat-mode-on-users-playback
use crate::http::api::SpotifyApi;
use crate::http::client::HttpError;
use crate::http::endpoints::Endpoint;
use serde_json::Value;

pub async fn set_repeat_mode(
    client: &SpotifyApi,
    state: &str,
) -> Result<Option<Value>, HttpError> {
    client.put(&Endpoint::PlayerRepeat { state }.path()).await
}
