use crate::endpoints::albums::get_album;
use crate::endpoints::artists::{get_artist, get_artist_top_tracks, get_artists_albums, get_artists_related_artists};
use crate::endpoints::tracks::get_track;
use crate::io::output::{ErrorKind, Response};
use crate::types::{Album, Artist, ArtistTopTracksResponse, RelatedArtistsResponse, Track};

use super::{extract_id, now_playing, with_client};

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
#[allow(clippy::too_many_arguments)]
pub async fn info_artist(
    id: Option<&str>,
    id_only: bool,
    top_tracks: bool,
    albums: bool,
    related: bool,
    market: &str,
    limit: u8,
    offset: u32,
) -> Response {
    let market = market.to_string();

    with_client(|client| async move {
        let artist_id = match id {
            Some(id) => extract_id(id),
            None => match now_playing::get_artist_id(&client).await {
                Ok(id) => id,
                Err(e) => return e,
            },
        };

        if id_only {
            return Response::success(200, &artist_id);
        }

        if top_tracks {
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
        } else if albums {
            match get_artists_albums::get_artists_albums(&client, &artist_id, Some(limit), Some(offset)).await {
                Ok(Some(payload)) => Response::success_with_payload(200, "Artist albums", payload),
                Ok(None) => Response::success_with_payload(200, "No albums", serde_json::json!({ "items": [] })),
                Err(e) => Response::from_http_error(&e, "Failed to get artist albums"),
            }
        } else if related {
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
        } else {
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
    }).await
}
