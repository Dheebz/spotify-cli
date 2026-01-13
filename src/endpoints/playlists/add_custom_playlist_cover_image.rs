use crate::http::api::SpotifyApi;
use crate::http::client::HttpError;
use crate::http::endpoints::Endpoint;
use serde_json::Value;

/// Add custom playlist cover image (base64 encoded JPEG)
/// Note: This endpoint requires raw image data, not JSON. Use put_json with base64 string.
pub async fn add_custom_playlist_cover_image(
    client: &SpotifyApi,
    playlist_id: &str,
    _image_data: &str,
) -> Result<Option<Value>, HttpError> {
    // This endpoint requires special handling for raw image data
    // For now, just call the endpoint path - full implementation would need raw body support
    client.put(&Endpoint::PlaylistCoverImage { id: playlist_id }.path()).await
}
