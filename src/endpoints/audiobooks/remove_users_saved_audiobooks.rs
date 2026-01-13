use crate::http::api::SpotifyApi;
use crate::http::client::HttpError;
use crate::http::endpoints::Endpoint;
use serde_json::Value;

/// Remove audiobooks from user's library
pub async fn remove_audiobooks(
    client: &SpotifyApi,
    audiobook_ids: &[String],
) -> Result<Option<Value>, HttpError> {
    let ids = audiobook_ids.join(",");
    client.delete(&Endpoint::SavedAudiobooksIds { ids: &ids }.path()).await
}
