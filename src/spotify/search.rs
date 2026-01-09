use anyhow::bail;
use reqwest::blocking::Client as HttpClient;
use serde::Deserialize;

use crate::domain::search::{SearchItem, SearchResults, SearchType};
use crate::error::Result;
use crate::spotify::auth::AuthService;
use crate::spotify::base::api_base;
use crate::spotify::error::format_api_error;

/// Spotify search API client.
#[derive(Debug, Clone)]
pub struct SearchClient {
    http: HttpClient,
    auth: AuthService,
}

impl SearchClient {
    pub fn new(http: HttpClient, auth: AuthService) -> Self {
        Self { http, auth }
    }

    pub fn search(
        &self,
        query: &str,
        kind: SearchType,
        limit: u32,
        market_from_token: bool,
    ) -> Result<SearchResults> {
        if kind == SearchType::All {
            let mut items = Vec::new();
            let kinds = [
                SearchType::Track,
                SearchType::Album,
                SearchType::Artist,
                SearchType::Playlist,
            ];
            for kind in kinds {
                let results = self.search(query, kind, limit, market_from_token)?;
                items.extend(results.items);
            }
            return Ok(SearchResults {
                kind: SearchType::All,
                items,
            });
        }

        let token = self.auth.token()?;
        let kind_param = search_type_param(kind);
        let mut url = format!(
            "{}/search?q={}&type={}&limit={}",
            api_base(),
            urlencoding::encode(query),
            kind_param,
            limit
        );

        if market_from_token {
            url.push_str("&market=from_token");
        }

        let response = self.http.get(url).bearer_auth(token.access_token).send()?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().unwrap_or_else(|_| "<no body>".to_string());
            bail!(format_api_error("spotify search failed", status, &body));
        }

        let payload: SearchResponse = response.json()?;
        let items = match kind {
            SearchType::Track => payload
                .tracks
                .map(|list| {
                    list.items
                        .into_iter()
                        .flatten()
                        .map(|item| SearchItem {
                            id: item.id,
                            name: item.name,
                            uri: item.uri,
                            kind: SearchType::Track,
                            artists: item.artists.into_iter().map(|artist| artist.name).collect(),
                            album: item.album.map(|album| album.name),
                            duration_ms: item.duration_ms,
                            owner: None,
                            score: None,
                        })
                        .collect::<Vec<_>>()
                })
                .unwrap_or_default(),
            SearchType::Album => payload
                .albums
                .map(|list| {
                    list.items
                        .into_iter()
                        .flatten()
                        .map(|item| SearchItem {
                            id: item.id,
                            name: item.name,
                            uri: item.uri,
                            kind: SearchType::Album,
                            artists: item.artists.into_iter().map(|artist| artist.name).collect(),
                            album: None,
                            duration_ms: None,
                            owner: None,
                            score: None,
                        })
                        .collect::<Vec<_>>()
                })
                .unwrap_or_default(),
            SearchType::Artist => payload
                .artists
                .map(|list| {
                    list.items
                        .into_iter()
                        .flatten()
                        .map(|item| SearchItem {
                            id: item.id,
                            name: item.name,
                            uri: item.uri,
                            kind: SearchType::Artist,
                            artists: Vec::new(),
                            album: None,
                            duration_ms: None,
                            owner: None,
                            score: None,
                        })
                        .collect::<Vec<_>>()
                })
                .unwrap_or_default(),
            SearchType::Playlist => payload
                .playlists
                .map(|list| {
                    list.items
                        .into_iter()
                        .flatten()
                        .map(|item| SearchItem {
                            id: item.id,
                            name: item.name,
                            uri: item.uri,
                            kind: SearchType::Playlist,
                            artists: Vec::new(),
                            album: None,
                            duration_ms: None,
                            owner: item.owner.and_then(|owner| owner.display_name),
                            score: None,
                        })
                        .collect::<Vec<_>>()
                })
                .unwrap_or_default(),
            SearchType::All => Vec::new(),
        };

        Ok(SearchResults { kind, items })
    }

    pub fn recently_played(&self, limit: u32) -> Result<Vec<SearchItem>> {
        let token = self.auth.token()?;
        let url = format!("{}/me/player/recently-played?limit={}", api_base(), limit);

        let response = self.http.get(url).bearer_auth(token.access_token).send()?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().unwrap_or_else(|_| "<no body>".to_string());
            bail!(format_api_error(
                "spotify recently played failed",
                status,
                &body
            ));
        }

        let payload: RecentlyPlayedResponse = response.json()?;
        Ok(payload
            .items
            .into_iter()
            .filter_map(|item| item.track.map(map_track))
            .collect())
    }
}

fn search_type_param(kind: SearchType) -> &'static str {
    match kind {
        SearchType::All => "track,album,artist,playlist",
        SearchType::Track => "track",
        SearchType::Album => "album",
        SearchType::Artist => "artist",
        SearchType::Playlist => "playlist",
    }
}

#[derive(Debug, Deserialize)]
struct SearchResponse {
    tracks: Option<ItemList<SpotifyTrack>>,
    albums: Option<ItemList<SpotifyAlbum>>,
    artists: Option<ItemList<SpotifyArtist>>,
    playlists: Option<ItemList<SpotifyPlaylist>>,
}

#[derive(Debug, Deserialize)]
struct ItemList<T> {
    items: Vec<Option<T>>,
}

#[derive(Debug, Deserialize)]
struct SpotifyTrack {
    id: String,
    name: String,
    uri: String,
    artists: Vec<SpotifyArtistRef>,
    album: Option<SpotifyAlbumRef>,
    duration_ms: Option<u32>,
}

#[derive(Debug, Deserialize)]
struct SpotifyAlbum {
    id: String,
    name: String,
    uri: String,
    artists: Vec<SpotifyArtistRef>,
}

#[derive(Debug, Deserialize)]
struct RecentlyPlayedResponse {
    items: Vec<RecentlyPlayedItem>,
}

#[derive(Debug, Deserialize)]
pub struct RecentlyPlayedItem {
    track: Option<SpotifyTrack>,
}

fn map_track(item: SpotifyTrack) -> SearchItem {
    SearchItem {
        id: item.id,
        name: item.name,
        uri: item.uri,
        kind: SearchType::Track,
        artists: item.artists.into_iter().map(|artist| artist.name).collect(),
        album: item.album.map(|album| album.name),
        duration_ms: item.duration_ms,
        owner: None,
        score: None,
    }
}

#[derive(Debug, Deserialize)]
struct SpotifyAlbumRef {
    name: String,
}

#[derive(Debug, Deserialize)]
struct SpotifyArtist {
    id: String,
    name: String,
    uri: String,
}

#[derive(Debug, Deserialize)]
struct SpotifyPlaylist {
    id: String,
    name: String,
    uri: String,
    owner: Option<SpotifyOwner>,
}

#[derive(Debug, Deserialize)]
struct SpotifyArtistRef {
    name: String,
}

#[derive(Debug, Deserialize)]
struct SpotifyOwner {
    display_name: Option<String>,
}
