use crate::http::api::SpotifyApi;
use crate::http::client::HttpError;
use crate::http::endpoints::Endpoint;
use serde_json::Value;

/// Get artist's top tracks
pub async fn get_artist_top_tracks(
    client: &SpotifyApi,
    artist_id: &str,
    market: Option<&str>,
) -> Result<Option<Value>, HttpError> {
    let market = market.unwrap_or("US");
    client.get(&Endpoint::ArtistTopTracks { id: artist_id, market }.path()).await
}
