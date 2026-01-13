use crate::http::api::SpotifyApi;
use crate::http::client::HttpError;
use crate::http::endpoints::Endpoint;
use serde_json::Value;

/// Check if tracks are saved in user's library
pub async fn check_saved_tracks(
    client: &SpotifyApi,
    track_ids: &[String],
) -> Result<Option<Value>, HttpError> {
    let ids = track_ids.join(",");
    client.get(&Endpoint::SavedTracksContains { ids: &ids }.path()).await
}
