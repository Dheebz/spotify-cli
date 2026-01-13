/// Seek to the given position in the user's currently playing track.
///
/// See: https://developer.spotify.com/documentation/web-api/reference/seek-to-position-in-currently-playing-track
use crate::http::api::SpotifyApi;
use crate::http::client::HttpError;
use crate::http::endpoints::Endpoint;
use serde_json::Value;

pub async fn seek_to_position(
    client: &SpotifyApi,
    position_ms: u64,
) -> Result<Option<Value>, HttpError> {
    client.put(&Endpoint::PlayerSeek { position_ms }.path()).await
}
