use reqwest::blocking::Client as HttpClient;

use crate::error::Result;
use crate::spotify::albums::AlbumsClient;
use crate::spotify::artists::ArtistsClient;
use crate::spotify::auth::AuthService;
use crate::spotify::devices::DevicesClient;
use crate::spotify::playback::PlaybackClient;
use crate::spotify::playlists::PlaylistsClient;
use crate::spotify::search::SearchClient;
use crate::spotify::track::TrackClient;

/// Top-level Spotify API client factory.
#[derive(Debug, Clone)]
pub struct SpotifyClient {
    http: HttpClient,
    auth: AuthService,
}

impl SpotifyClient {
    pub fn new(auth: AuthService) -> Result<Self> {
        let http = HttpClient::builder().build()?;
        Ok(Self { http, auth })
    }

    pub fn playback(&self) -> PlaybackClient {
        PlaybackClient::new(self.http.clone(), self.auth.clone())
    }

    pub fn albums(&self) -> AlbumsClient {
        AlbumsClient::new(self.http.clone(), self.auth.clone())
    }

    pub fn artists(&self) -> ArtistsClient {
        ArtistsClient::new(self.http.clone(), self.auth.clone())
    }

    pub fn devices(&self) -> DevicesClient {
        DevicesClient::new(self.http.clone(), self.auth.clone())
    }

    pub fn playlists(&self) -> PlaylistsClient {
        PlaylistsClient::new(self.http.clone(), self.auth.clone())
    }

    pub fn search(&self) -> SearchClient {
        SearchClient::new(self.http.clone(), self.auth.clone())
    }

    pub fn track(&self) -> TrackClient {
        TrackClient::new(self.http.clone(), self.auth.clone())
    }
}
