use crate::http::api::SpotifyApi;
use crate::http::client::HttpError;
use crate::http::endpoints::Endpoint;
use serde_json::Value;

/// Get multiple shows by IDs
pub async fn get_several_shows(
    client: &SpotifyApi,
    show_ids: &[String],
) -> Result<Option<Value>, HttpError> {
    let ids = show_ids.join(",");
    client.get(&Endpoint::Shows { ids: &ids }.path()).await
}
