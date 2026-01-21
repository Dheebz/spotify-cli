use crate::http::api::SpotifyApi;
use crate::http::client::HttpError;
use crate::http::endpoints::Endpoint;
use serde_json::Value;

/// Get multiple chapters by IDs
pub async fn get_several_chapters(
    client: &SpotifyApi,
    chapter_ids: &[String],
) -> Result<Option<Value>, HttpError> {
    let ids = chapter_ids.join(",");
    client.get(&Endpoint::Chapters { ids: &ids }.path()).await
}
