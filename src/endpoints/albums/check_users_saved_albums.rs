use crate::http::api::SpotifyApi;
use crate::http::client::HttpError;
use crate::http::endpoints::Endpoint;
use serde_json::Value;

/// Check if albums are saved in user's library
pub async fn check_saved_albums(
    client: &SpotifyApi,
    album_ids: &[String],
) -> Result<Option<Value>, HttpError> {
    let ids = album_ids.join(",");
    client.get(&Endpoint::SavedAlbumsContains { ids: &ids }.path()).await
}
