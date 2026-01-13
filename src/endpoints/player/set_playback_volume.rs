/// Set the volume for the user's current playback device.
///
/// See: https://developer.spotify.com/documentation/web-api/reference/set-volume-for-users-playback
use crate::http::api::SpotifyApi;
use crate::http::client::HttpError;
use crate::http::endpoints::Endpoint;
use serde_json::Value;

pub async fn set_playback_volume(
    client: &SpotifyApi,
    volume_percent: u8,
) -> Result<Option<Value>, HttpError> {
    client.put(&Endpoint::PlayerVolume { volume_percent }.path()).await
}
