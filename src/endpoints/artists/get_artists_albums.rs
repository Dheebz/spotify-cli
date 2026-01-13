use crate::http::api::SpotifyApi;
use crate::http::client::HttpError;
use crate::http::endpoints::Endpoint;
use serde_json::Value;

/// Get artist's albums
pub async fn get_artists_albums(
    client: &SpotifyApi,
    artist_id: &str,
    limit: Option<u8>,
    offset: Option<u32>,
) -> Result<Option<Value>, HttpError> {
    let limit = limit.unwrap_or(20).min(50);
    let offset = offset.unwrap_or(0);
    client.get(&Endpoint::ArtistAlbums { id: artist_id, limit, offset }.path()).await
}
