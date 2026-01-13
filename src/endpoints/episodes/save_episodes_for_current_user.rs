use crate::http::api::SpotifyApi;
use crate::http::client::HttpError;
use crate::http::endpoints::Endpoint;
use serde_json::Value;

/// Save episodes to user's library
pub async fn save_episodes(
    client: &SpotifyApi,
    episode_ids: &[String],
) -> Result<Option<Value>, HttpError> {
    let ids = episode_ids.join(",");
    client.put(&Endpoint::SavedEpisodesIds { ids: &ids }.path()).await
}
