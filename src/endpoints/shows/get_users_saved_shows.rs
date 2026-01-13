use crate::http::api::SpotifyApi;
use crate::http::client::HttpError;
use crate::http::endpoints::Endpoint;
use serde_json::Value;

/// Get user's saved shows (podcasts)
pub async fn get_users_saved_shows(
    client: &SpotifyApi,
    limit: Option<u8>,
    offset: Option<u32>,
) -> Result<Option<Value>, HttpError> {
    let limit = limit.unwrap_or(20).min(50);
    let offset = offset.unwrap_or(0);
    client.get(&Endpoint::SavedShows { limit, offset }.path()).await
}
