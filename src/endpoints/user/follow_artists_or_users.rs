use crate::http::api::SpotifyApi;
use crate::http::client::HttpError;
use crate::http::endpoints::Endpoint;
use serde_json::Value;

/// Follow artists or users
pub async fn follow_artists_or_users(
    client: &SpotifyApi,
    entity_type: &str, // "artist" or "user"
    ids: &[String],
) -> Result<Option<Value>, HttpError> {
    let ids_str = ids.join(",");
    client.put(&Endpoint::FollowArtistsOrUsers { entity_type, ids: &ids_str }.path()).await
}
