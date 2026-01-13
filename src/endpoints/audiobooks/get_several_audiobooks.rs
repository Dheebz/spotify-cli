use crate::http::api::SpotifyApi;
use crate::http::client::HttpError;
use crate::http::endpoints::Endpoint;
use serde_json::Value;

/// Get multiple audiobooks by IDs
pub async fn get_several_audiobooks(
    client: &SpotifyApi,
    audiobook_ids: &[String],
) -> Result<Option<Value>, HttpError> {
    let ids = audiobook_ids.join(",");
    client.get(&Endpoint::Audiobooks { ids: &ids }.path()).await
}
