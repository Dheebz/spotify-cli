/// Add an item to the end of the user's current playback queue.
///
/// See: https://developer.spotify.com/documentation/web-api/reference/add-to-queue
use crate::http::api::SpotifyApi;
use crate::http::client::HttpError;
use crate::http::endpoints::Endpoint;
use serde_json::Value;

pub async fn add_item_to_playback_queue(
    client: &SpotifyApi,
    uri: &str,
) -> Result<Option<Value>, HttpError> {
    client.post(&Endpoint::PlayerQueueAdd { uri }.path()).await
}
