use crate::endpoints::albums::get_album;
use crate::endpoints::artists::{get_artist, get_artist_top_tracks, get_artists_albums, get_artists_related_artists};
use crate::endpoints::tracks::get_track;
use crate::io::output::{ErrorKind, Response};
use crate::types::{Album, Artist, ArtistTopTracksResponse, RelatedArtistsResponse, Track};

use super::{extract_id, now_playing, with_client};

/// What information to retrieve for an artist
#[derive(Debug, Clone, Copy, Default)]
pub enum ArtistView {
    /// Basic artist details (default)
    #[default]
    Details,
    /// Artist's top tracks
    TopTracks,
    /// Artist's albums
    Albums,
    /// Related artists
    Related,
}

/// Query parameters for artist info
#[derive(Debug, Clone)]
pub struct ArtistQuery {
    /// Artist ID or URL (None = now playing artist)
    pub id: Option<String>,
    /// Only output the ID
    pub id_only: bool,
    /// What view/information to retrieve
    pub view: ArtistView,
    /// Market for top tracks (ISO country code)
    pub market: String,
    /// Number of results for albums
    pub limit: u8,
    /// Offset for album pagination
    pub offset: u32,
}

impl ArtistQuery {
    /// Create a new query for artist details
    pub fn new() -> Self {
        Self {
            id: None,
            id_only: false,
            view: ArtistView::Details,
            market: "US".to_string(),
            limit: 20,
            offset: 0,
        }
    }

    /// Set the artist ID
    pub fn with_id(mut self, id: Option<String>) -> Self {
        self.id = id;
        self
    }

    /// Set id_only flag
    pub fn id_only(mut self, id_only: bool) -> Self {
        self.id_only = id_only;
        self
    }

    /// Set the view type
    pub fn view(mut self, view: ArtistView) -> Self {
        self.view = view;
        self
    }

    /// Set the market
    pub fn market(mut self, market: String) -> Self {
        self.market = market;
        self
    }

    /// Set pagination
    pub fn paginate(mut self, limit: u8, offset: u32) -> Self {
        self.limit = limit;
        self.offset = offset;
        self
    }
}

impl Default for ArtistQuery {
    fn default() -> Self {
        Self::new()
    }
}

/// Get track info - defaults to now playing if no ID provided
pub async fn info_track(id: Option<&str>, id_only: bool) -> Response {
    with_client(|client| async move {
        let track_id = match id {
            Some(id) => extract_id(id),
            None => match now_playing::get_track_id(&client).await {
                Ok(id) => id,
                Err(e) => return e,
            },
        };

        if id_only {
            return Response::success(200, &track_id);
        }

        match get_track::get_track(&client, &track_id).await {
            Ok(Some(payload)) => {
                // Validate response structure and extract info for message
                match serde_json::from_value::<Track>(payload.clone()) {
                    Ok(track) => {
                        let msg = format!("{} - {}", track.name, track.artist_names());
                        Response::success_with_payload(200, msg, payload)
                    }
                    Err(_) => Response::success_with_payload(200, "Track details", payload),
                }
            }
            Ok(None) => Response::err(404, "Track not found", ErrorKind::NotFound),
            Err(e) => Response::from_http_error(&e, "Failed to get track"),
        }
    }).await
}

/// Get album info - defaults to now playing album if no ID provided
pub async fn info_album(id: Option<&str>, id_only: bool) -> Response {
    with_client(|client| async move {
        let album_id = match id {
            Some(id) => extract_id(id),
            None => match now_playing::get_album_id(&client).await {
                Ok(id) => id,
                Err(e) => return e,
            },
        };

        if id_only {
            return Response::success(200, &album_id);
        }

        match get_album::get_album(&client, &album_id).await {
            Ok(Some(payload)) => {
                // Validate response structure and extract info for message
                match serde_json::from_value::<Album>(payload.clone()) {
                    Ok(album) => {
                        let artist = album.artist_name().unwrap_or("Unknown Artist");
                        let msg = format!("{} - {}", album.name, artist);
                        Response::success_with_payload(200, msg, payload)
                    }
                    Err(_) => Response::success_with_payload(200, "Album details", payload),
                }
            }
            Ok(None) => Response::err(404, "Album not found", ErrorKind::NotFound),
            Err(e) => Response::from_http_error(&e, "Failed to get album"),
        }
    }).await
}

/// Get artist info - defaults to now playing artist if no ID provided
pub async fn info_artist(query: ArtistQuery) -> Response {
    let ArtistQuery { id, id_only, view, market, limit, offset } = query;

    with_client(|client| async move {
        let artist_id = match id.as_deref() {
            Some(id) => extract_id(id),
            None => match now_playing::get_artist_id(&client).await {
                Ok(id) => id,
                Err(e) => return e,
            },
        };

        if id_only {
            return Response::success(200, &artist_id);
        }

        match view {
            ArtistView::TopTracks => {
                match get_artist_top_tracks::get_artist_top_tracks(&client, &artist_id, Some(&market)).await {
                    Ok(Some(payload)) => {
                        match serde_json::from_value::<ArtistTopTracksResponse>(payload.clone()) {
                            Ok(resp) => {
                                let count = resp.tracks.len();
                                Response::success_with_payload(200, format!("Top {} tracks", count), payload)
                            }
                            Err(_) => Response::success_with_payload(200, "Top tracks", payload),
                        }
                    }
                    Ok(None) => Response::success_with_payload(200, "No top tracks", serde_json::json!({ "tracks": [] })),
                    Err(e) => Response::from_http_error(&e, "Failed to get top tracks"),
                }
            }
            ArtistView::Albums => {
                match get_artists_albums::get_artists_albums(&client, &artist_id, Some(limit), Some(offset)).await {
                    Ok(Some(payload)) => Response::success_with_payload(200, "Artist albums", payload),
                    Ok(None) => Response::success_with_payload(200, "No albums", serde_json::json!({ "items": [] })),
                    Err(e) => Response::from_http_error(&e, "Failed to get artist albums"),
                }
            }
            ArtistView::Related => {
                match get_artists_related_artists::get_artists_related_artists(&client, &artist_id).await {
                    Ok(Some(payload)) => {
                        match serde_json::from_value::<RelatedArtistsResponse>(payload.clone()) {
                            Ok(resp) => {
                                let count = resp.artists.len();
                                Response::success_with_payload(200, format!("{} related artists", count), payload)
                            }
                            Err(_) => Response::success_with_payload(200, "Related artists", payload),
                        }
                    }
                    Ok(None) => Response::success_with_payload(200, "No related artists", serde_json::json!({ "artists": [] })),
                    Err(e) => Response::from_http_error(&e, "Failed to get related artists"),
                }
            }
            ArtistView::Details => {
                match get_artist::get_artist(&client, &artist_id).await {
                    Ok(Some(payload)) => {
                        match serde_json::from_value::<Artist>(payload.clone()) {
                            Ok(artist) => {
                                Response::success_with_payload(200, artist.name.clone(), payload)
                            }
                            Err(_) => Response::success_with_payload(200, "Artist details", payload),
                        }
                    }
                    Ok(None) => Response::err(404, "Artist not found", ErrorKind::NotFound),
                    Err(e) => Response::from_http_error(&e, "Failed to get artist"),
                }
            }
        }
    }).await
}
