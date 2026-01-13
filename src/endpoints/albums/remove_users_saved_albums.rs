use crate::http::api::SpotifyApi;
use crate::http::client::HttpError;
use crate::http::endpoints::Endpoint;
use serde_json::Value;

/// Remove albums from user's library
pub async fn remove_albums(
    client: &SpotifyApi,
    album_ids: &[String],
) -> Result<Option<Value>, HttpError> {
    let ids = album_ids.join(",");
    client.delete(&Endpoint::SavedAlbumsIds { ids: &ids }.path()).await
}
