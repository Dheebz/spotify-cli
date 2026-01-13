use crate::http::api::SpotifyApi;
use crate::http::client::HttpError;
use crate::http::endpoints::Endpoint;
use serde_json::Value;

/// Check if episodes are saved in user's library
pub async fn check_saved_episodes(
    client: &SpotifyApi,
    episode_ids: &[String],
) -> Result<Option<Value>, HttpError> {
    let ids = episode_ids.join(",");
    client.get(&Endpoint::SavedEpisodesContains { ids: &ids }.path()).await
}
