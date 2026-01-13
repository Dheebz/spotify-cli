use crate::http::api::SpotifyApi;
use crate::http::client::HttpError;
use crate::http::endpoints::Endpoint;
use serde_json::Value;

/// Check if audiobooks are saved in user's library
pub async fn check_saved_audiobooks(
    client: &SpotifyApi,
    audiobook_ids: &[String],
) -> Result<Option<Value>, HttpError> {
    let ids = audiobook_ids.join(",");
    client.get(&Endpoint::SavedAudiobooksContains { ids: &ids }.path()).await
}
