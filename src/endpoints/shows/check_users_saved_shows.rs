use crate::http::api::SpotifyApi;
use crate::http::client::HttpError;
use crate::http::endpoints::Endpoint;
use serde_json::Value;

/// Check if shows are saved in user's library
pub async fn check_saved_shows(
    client: &SpotifyApi,
    show_ids: &[String],
) -> Result<Option<Value>, HttpError> {
    let ids = show_ids.join(",");
    client.get(&Endpoint::SavedShowsContains { ids: &ids }.path()).await
}
