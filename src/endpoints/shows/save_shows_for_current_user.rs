use crate::http::api::SpotifyApi;
use crate::http::client::HttpError;
use crate::http::endpoints::Endpoint;
use serde_json::Value;

/// Save shows to user's library
pub async fn save_shows(
    client: &SpotifyApi,
    show_ids: &[String],
) -> Result<Option<Value>, HttpError> {
    let ids = show_ids.join(",");
    client.put(&Endpoint::SavedShowsIds { ids: &ids }.path()).await
}
