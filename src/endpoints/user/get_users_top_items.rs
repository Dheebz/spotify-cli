use crate::http::api::SpotifyApi;
use crate::http::client::HttpError;
use crate::http::endpoints::Endpoint;
use serde_json::Value;

/// Get the current user's top artists or tracks
/// item_type: "artists" or "tracks"
/// time_range: "short_term" (4 weeks), "medium_term" (6 months), "long_term" (years)
pub async fn get_users_top_items(
    client: &SpotifyApi,
    item_type: &str,
    time_range: Option<&str>,
    limit: Option<u8>,
    offset: Option<u32>,
) -> Result<Option<Value>, HttpError> {
    let time_range = time_range.unwrap_or("medium_term");
    let limit = limit.unwrap_or(20);
    let offset = offset.unwrap_or(0);

    client.get(&Endpoint::UserTopItems { item_type, time_range, limit, offset }.path()).await
}
