use crate::http::api::SpotifyApi;
use crate::http::client::HttpError;
use crate::http::endpoints::Endpoint;
use serde_json::Value;

/// Get user's followed artists
pub async fn get_followed_artists(
    client: &SpotifyApi,
    limit: Option<u8>,
) -> Result<Option<Value>, HttpError> {
    let limit = limit.unwrap_or(20).min(50);
    client.get(&Endpoint::FollowedArtists { limit }.path()).await
}
