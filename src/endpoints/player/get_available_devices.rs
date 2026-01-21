/// Get information about a user's available devices.
///
/// See: https://developer.spotify.com/documentation/web-api/reference/get-a-users-available-devices
use crate::http::api::SpotifyApi;
use crate::http::client::HttpError;
use crate::http::endpoints::Endpoint;
use serde_json::Value;

pub async fn get_available_devices(client: &SpotifyApi) -> Result<Option<Value>, HttpError> {
    client.get(&Endpoint::PlayerDevices.path()).await
}
