use crate::http::api::SpotifyApi;
use crate::http::client::HttpError;
use crate::http::endpoints::Endpoint;
use serde_json::Value;

/// Remove episodes from user's library
pub async fn remove_episodes(
    client: &SpotifyApi,
    episode_ids: &[String],
) -> Result<Option<Value>, HttpError> {
    let ids = episode_ids.join(",");
    client.delete(&Endpoint::SavedEpisodesIds { ids: &ids }.path()).await
}
