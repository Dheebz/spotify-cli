use crate::http::api::SpotifyApi;
use crate::http::client::HttpError;
use crate::http::endpoints::Endpoint;
use serde_json::Value;

/// Get chapters for an audiobook
pub async fn get_audiobook_chapters(
    client: &SpotifyApi,
    audiobook_id: &str,
    limit: Option<u8>,
    offset: Option<u32>,
) -> Result<Option<Value>, HttpError> {
    let limit = limit.unwrap_or(20).min(50);
    let offset = offset.unwrap_or(0);
    client.get(&Endpoint::AudiobookChapters { id: audiobook_id, limit, offset }.path()).await
}
