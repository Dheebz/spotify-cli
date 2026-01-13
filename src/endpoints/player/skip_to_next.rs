/// Skip to next track in the user's queue.
///
/// See: https://developer.spotify.com/documentation/web-api/reference/skip-users-playback-to-next-track
use crate::http::api::SpotifyApi;
use crate::http::client::HttpError;
use crate::http::endpoints::Endpoint;
use serde_json::Value;

pub async fn skip_to_next(client: &SpotifyApi) -> Result<Option<Value>, HttpError> {
    client.post(&Endpoint::PlayerNext.path()).await
}
