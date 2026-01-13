/// Transfer playback to a new device and optionally begin playback.
///
/// See: https://developer.spotify.com/documentation/web-api/reference/transfer-a-users-playback
use crate::http::api::SpotifyApi;
use crate::http::client::HttpError;
use crate::http::endpoints::Endpoint;
use serde_json::Value;

pub async fn transfer_playback(
    client: &SpotifyApi,
    device_id: &str,
) -> Result<Option<Value>, HttpError> {
    let body = serde_json::json!({
        "device_ids": [device_id]
    });
    client.put_json(&Endpoint::PlayerState.path(), &body).await
}
