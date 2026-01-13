use crate::http::api::SpotifyApi;
use crate::http::client::HttpError;
use crate::http::endpoints::Endpoint;
use serde_json::Value;

/// Get artist details by ID
pub async fn get_artist(
    client: &SpotifyApi,
    artist_id: &str,
) -> Result<Option<Value>, HttpError> {
    client.get(&Endpoint::Artist { id: artist_id }.path()).await
}
