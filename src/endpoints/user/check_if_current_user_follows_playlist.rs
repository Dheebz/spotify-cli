use crate::http::api::SpotifyApi;
use crate::http::client::HttpError;
use crate::http::endpoints::Endpoint;
use serde_json::Value;

/// Check if current user follows a playlist
pub async fn check_if_current_user_follows_playlist(
    client: &SpotifyApi,
    playlist_id: &str,
    user_ids: &[String],
) -> Result<Option<Value>, HttpError> {
    let ids = user_ids.join(",");
    client.get(&Endpoint::FollowPlaylistContains { playlist_id, ids: &ids }.path()).await
}
