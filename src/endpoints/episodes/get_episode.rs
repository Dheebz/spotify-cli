use crate::http::api::SpotifyApi;
use crate::http::client::HttpError;
use crate::http::endpoints::Endpoint;
use serde_json::Value;

/// Get episode details by ID
pub async fn get_episode(
    client: &SpotifyApi,
    episode_id: &str,
) -> Result<Option<Value>, HttpError> {
    client.get(&Endpoint::Episode { id: episode_id }.path()).await
}
