use crate::http::api::SpotifyApi;
use crate::http::client::HttpError;
use crate::http::endpoints::Endpoint;
use serde_json::Value;

/// Save audiobooks to user's library
pub async fn save_audiobooks(
    client: &SpotifyApi,
    audiobook_ids: &[String],
) -> Result<Option<Value>, HttpError> {
    let ids = audiobook_ids.join(",");
    client.put(&Endpoint::SavedAudiobooksIds { ids: &ids }.path()).await
}
