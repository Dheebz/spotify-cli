use crate::http::api::SpotifyApi;
use crate::http::client::HttpError;
use crate::http::endpoints::Endpoint;
use serde_json::Value;

/// Check if user follows artists or users
pub async fn check_if_user_follows_artist_or_users(
    client: &SpotifyApi,
    entity_type: &str, // "artist" or "user"
    ids: &[String],
) -> Result<Option<Value>, HttpError> {
    let ids_str = ids.join(",");
    client.get(&Endpoint::FollowingContains { entity_type, ids: &ids_str }.path()).await
}
