use crate::http::api::SpotifyApi;
use crate::http::client::HttpError;
use crate::http::endpoints::Endpoint;
use serde_json::Value;

/// Remove tracks from user's library (unlike songs)
pub async fn remove_tracks(
    client: &SpotifyApi,
    track_ids: &[String],
) -> Result<Option<Value>, HttpError> {
    let ids = track_ids.join(",");
    client.delete(&Endpoint::SavedTracksIds { ids: &ids }.path()).await
}
