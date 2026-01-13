use crate::http::api::SpotifyApi;
use crate::http::client::HttpError;
use crate::http::endpoints::Endpoint;
use serde_json::Value;

/// Get track details by ID
pub async fn get_track(
    client: &SpotifyApi,
    track_id: &str,
) -> Result<Option<Value>, HttpError> {
    client.get(&Endpoint::Track { id: track_id }.path()).await
}
