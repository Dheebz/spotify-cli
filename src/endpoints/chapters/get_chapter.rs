use crate::http::api::SpotifyApi;
use crate::http::client::HttpError;
use crate::http::endpoints::Endpoint;
use serde_json::Value;

/// Get chapter details by ID
pub async fn get_chapter(
    client: &SpotifyApi,
    chapter_id: &str,
) -> Result<Option<Value>, HttpError> {
    client.get(&Endpoint::Chapter { id: chapter_id }.path()).await
}
