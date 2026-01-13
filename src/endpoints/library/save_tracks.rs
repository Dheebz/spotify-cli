use crate::http::api::SpotifyApi;
use crate::http::client::HttpError;
use crate::http::endpoints::Endpoint;
use serde_json::Value;

/// Save tracks to user's library (like songs)
pub async fn save_tracks(
    client: &SpotifyApi,
    track_ids: &[String],
) -> Result<Option<Value>, HttpError> {
    let ids = track_ids.join(",");
    client.put(&Endpoint::SavedTracksIds { ids: &ids }.path()).await
}
