use crate::http::api::SpotifyApi;
use crate::http::client::HttpError;
use crate::http::endpoints::Endpoint;
use serde_json::Value;

/// Get multiple albums by IDs
pub async fn get_several_albums(
    client: &SpotifyApi,
    album_ids: &[String],
) -> Result<Option<Value>, HttpError> {
    let ids = album_ids.join(",");
    client.get(&Endpoint::Albums { ids: &ids }.path()).await
}
