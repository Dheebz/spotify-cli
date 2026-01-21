use crate::http::api::SpotifyApi;
use crate::http::client::HttpError;
use crate::http::endpoints::Endpoint;
use serde_json::Value;

/// Get multiple episodes by IDs
pub async fn get_several_episodes(
    client: &SpotifyApi,
    episode_ids: &[String],
) -> Result<Option<Value>, HttpError> {
    let ids = episode_ids.join(",");
    client.get(&Endpoint::Episodes { ids: &ids }.path()).await
}
