use crate::http::api::SpotifyApi;
use crate::http::client::HttpError;
use crate::http::endpoints::Endpoint;
use serde_json::Value;

/// Get audiobook details by ID
pub async fn get_audiobook(
    client: &SpotifyApi,
    audiobook_id: &str,
) -> Result<Option<Value>, HttpError> {
    client.get(&Endpoint::Audiobook { id: audiobook_id }.path()).await
}
