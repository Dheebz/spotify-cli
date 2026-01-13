use crate::http::api::SpotifyApi;
use crate::http::client::HttpError;
use crate::http::endpoints::Endpoint;
use serde_json::Value;

/// Remove shows from user's library
pub async fn remove_shows(
    client: &SpotifyApi,
    show_ids: &[String],
) -> Result<Option<Value>, HttpError> {
    let ids = show_ids.join(",");
    client.delete(&Endpoint::SavedShowsIds { ids: &ids }.path()).await
}
