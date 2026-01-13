use crate::http::api::SpotifyApi;
use crate::http::client::HttpError;
use crate::http::endpoints::Endpoint;
use serde_json::Value;

/// Get artists related to an artist
pub async fn get_artists_related_artists(
    client: &SpotifyApi,
    artist_id: &str,
) -> Result<Option<Value>, HttpError> {
    client.get(&Endpoint::ArtistRelatedArtists { id: artist_id }.path()).await
}
