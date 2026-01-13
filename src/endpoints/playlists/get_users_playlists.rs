use crate::http::api::SpotifyApi;
use crate::http::client::HttpError;
use crate::http::endpoints::Endpoint;
use serde_json::Value;

/// Get a user's playlists
pub async fn get_users_playlists(
    client: &SpotifyApi,
    user_id: &str,
) -> Result<Option<Value>, HttpError> {
    client.get(&Endpoint::UserPlaylists { user_id }.path()).await
}
