use crate::http::api::SpotifyApi;
use crate::http::client::HttpError;
use crate::http::endpoints::Endpoint;
use serde_json::Value;

/// Unfollow (remove) a playlist
pub async fn unfollow_playlist(
    client: &SpotifyApi,
    playlist_id: &str,
) -> Result<Option<Value>, HttpError> {
    client.delete(&Endpoint::PlaylistFollowers { id: playlist_id }.path()).await
}
