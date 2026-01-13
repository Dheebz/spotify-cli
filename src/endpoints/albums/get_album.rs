use crate::http::api::SpotifyApi;
use crate::http::client::HttpError;
use crate::http::endpoints::Endpoint;
use serde_json::Value;

/// Get album details by ID
pub async fn get_album(
    client: &SpotifyApi,
    album_id: &str,
) -> Result<Option<Value>, HttpError> {
    client.get(&Endpoint::Album { id: album_id }.path()).await
}
