use crate::http::api::SpotifyApi;
use crate::http::client::HttpError;
use crate::http::endpoints::Endpoint;
use serde_json::Value;

/// Save albums to user's library
pub async fn save_albums(
    client: &SpotifyApi,
    album_ids: &[String],
) -> Result<Option<Value>, HttpError> {
    let ids = album_ids.join(",");
    client.put(&Endpoint::SavedAlbumsIds { ids: &ids }.path()).await
}
