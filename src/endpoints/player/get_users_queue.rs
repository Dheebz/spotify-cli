/// Get the list of objects that make up the user's queue.
///
/// See: https://developer.spotify.com/documentation/web-api/reference/get-queue
use crate::http::api::SpotifyApi;
use crate::http::client::HttpError;
use crate::http::endpoints::Endpoint;
use serde_json::Value;

pub async fn get_users_queue(client: &SpotifyApi) -> Result<Option<Value>, HttpError> {
    client.get(&Endpoint::PlayerQueue.path()).await
}
