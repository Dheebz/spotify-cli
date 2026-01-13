use crate::http::api::SpotifyApi;
use crate::http::client::HttpError;
use crate::http::endpoints::Endpoint;
use serde_json::Value;

/// Get playlist cover image
pub async fn get_playlist_cover_image(
    client: &SpotifyApi,
    playlist_id: &str,
) -> Result<Option<Value>, HttpError> {
    client.get(&Endpoint::PlaylistCoverImage { id: playlist_id }.path()).await
}
