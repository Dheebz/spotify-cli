use crate::http::api::SpotifyApi;
use crate::http::client::HttpError;
use crate::http::endpoints::Endpoint;
use serde_json::Value;

/// Get multiple tracks by IDs
pub async fn get_several_tracks(
    client: &SpotifyApi,
    track_ids: &[String],
) -> Result<Option<Value>, HttpError> {
    let ids = track_ids.join(",");
    client.get(&Endpoint::Tracks { ids: &ids }.path()).await
}
