//! Type-safe Spotify API endpoint paths
//!
//! This module provides a typed enum for all Spotify API endpoints,
//! eliminating string formatting errors and providing a single source
//! of truth for endpoint paths.
//!
//! # Design Pattern
//!
//! The `Endpoint` enum represents **URL paths**, not operations.
//! Multiple endpoint wrapper files in `src/endpoints/` can share
//! the same variant when they use different HTTP methods:
//!
//! - `Endpoint::PlaylistTracks` → GET (get_playlist_items), POST (add_items), DELETE (remove_items)
//! - `Endpoint::PlayerState` → GET (get_playback_state), PUT (transfer_playback)
//! - `Endpoint::SavedTracksIds` → PUT (save_tracks), DELETE (remove_tracks)
//!
//! This follows REST conventions: same URL, different HTTP verbs for different operations.

use urlencoding::encode;

/// Spotify API endpoint paths
pub enum Endpoint<'a> {
    // Player endpoints
    PlayerState,
    PlayerCurrentlyPlaying,
    PlayerPlay,
    PlayerPause,
    PlayerNext,
    PlayerPrevious,
    PlayerQueue,
    PlayerQueueAdd { uri: &'a str },
    PlayerDevices,
    PlayerRecentlyPlayed,
    PlayerSeek { position_ms: u64 },
    PlayerVolume { volume_percent: u8 },
    PlayerShuffle { state: bool },
    PlayerRepeat { state: &'a str },

    // Playlist endpoints
    Playlist { id: &'a str },
    PlaylistTracks { id: &'a str },
    PlaylistItems { id: &'a str, limit: u8, offset: u32 },
    PlaylistFollowers { id: &'a str },
    PlaylistCoverImage { id: &'a str },
    CurrentUserPlaylists { limit: u8, offset: u32 },
    UserPlaylists { user_id: &'a str },
    FeaturedPlaylists { limit: u8, offset: u32 },
    CategoryPlaylists { category_id: &'a str, limit: u8, offset: u32 },

    // Library endpoints (tracks)
    SavedTracks { limit: u8, offset: u32 },
    SavedTracksIds { ids: &'a str },
    SavedTracksContains { ids: &'a str },

    // Library endpoints (albums)
    SavedAlbums { limit: u8, offset: u32 },
    SavedAlbumsIds { ids: &'a str },
    SavedAlbumsContains { ids: &'a str },

    // Track endpoints
    Track { id: &'a str },
    Tracks { ids: &'a str },

    // Album endpoints
    Album { id: &'a str },
    Albums { ids: &'a str },
    AlbumTracks { id: &'a str, limit: u8, offset: u32 },
    NewReleases { limit: u8, offset: u32 },

    // Artist endpoints
    Artist { id: &'a str },
    Artists { ids: &'a str },
    ArtistTopTracks { id: &'a str, market: &'a str },
    ArtistAlbums { id: &'a str, limit: u8, offset: u32 },
    ArtistRelatedArtists { id: &'a str },

    // User endpoints
    CurrentUser,
    UserProfile { user_id: &'a str },
    UserTopItems { item_type: &'a str, time_range: &'a str, limit: u8, offset: u32 },
    FollowedArtists { limit: u8 },
    FollowArtistsOrUsers { entity_type: &'a str, ids: &'a str },
    FollowingContains { entity_type: &'a str, ids: &'a str },
    FollowPlaylist { playlist_id: &'a str },
    FollowPlaylistContains { playlist_id: &'a str, ids: &'a str },

    // Category endpoints
    Category { id: &'a str },
    Categories { limit: u8, offset: u32 },

    // Markets endpoint
    Markets,

    // Search endpoint
    Search { query: &'a str, types: &'a str, limit: u8 },

    // Show endpoints
    Show { id: &'a str },
    Shows { ids: &'a str },
    ShowEpisodes { id: &'a str, limit: u8, offset: u32 },
    SavedShows { limit: u8, offset: u32 },
    SavedShowsIds { ids: &'a str },
    SavedShowsContains { ids: &'a str },

    // Episode endpoints
    Episode { id: &'a str },
    Episodes { ids: &'a str },
    SavedEpisodes { limit: u8, offset: u32 },
    SavedEpisodesIds { ids: &'a str },
    SavedEpisodesContains { ids: &'a str },

    // Audiobook endpoints
    Audiobook { id: &'a str },
    Audiobooks { ids: &'a str },
    AudiobookChapters { id: &'a str, limit: u8, offset: u32 },
    SavedAudiobooks { limit: u8, offset: u32 },
    SavedAudiobooksIds { ids: &'a str },
    SavedAudiobooksContains { ids: &'a str },

    // Chapter endpoints
    Chapter { id: &'a str },
    Chapters { ids: &'a str },
}

impl<'a> Endpoint<'a> {
    /// Get the path string for this endpoint
    pub fn path(&self) -> String {
        match self {
            // Player
            Endpoint::PlayerState => "/me/player".to_string(),
            Endpoint::PlayerCurrentlyPlaying => "/me/player/currently-playing".to_string(),
            Endpoint::PlayerPlay => "/me/player/play".to_string(),
            Endpoint::PlayerPause => "/me/player/pause".to_string(),
            Endpoint::PlayerNext => "/me/player/next".to_string(),
            Endpoint::PlayerPrevious => "/me/player/previous".to_string(),
            Endpoint::PlayerQueue => "/me/player/queue".to_string(),
            Endpoint::PlayerQueueAdd { uri } => format!("/me/player/queue?uri={}", encode(uri)),
            Endpoint::PlayerDevices => "/me/player/devices".to_string(),
            Endpoint::PlayerRecentlyPlayed => "/me/player/recently-played".to_string(),
            Endpoint::PlayerSeek { position_ms } => {
                format!("/me/player/seek?position_ms={}", position_ms)
            }
            Endpoint::PlayerVolume { volume_percent } => {
                format!("/me/player/volume?volume_percent={}", volume_percent)
            }
            Endpoint::PlayerShuffle { state } => format!("/me/player/shuffle?state={}", state),
            Endpoint::PlayerRepeat { state } => format!("/me/player/repeat?state={}", state),

            // Playlists
            Endpoint::Playlist { id } => format!("/playlists/{}", id),
            Endpoint::PlaylistTracks { id } => format!("/playlists/{}/tracks", id),
            Endpoint::PlaylistItems { id, limit, offset } => {
                format!("/playlists/{}/tracks?limit={}&offset={}", id, limit, offset)
            }
            Endpoint::PlaylistFollowers { id } => format!("/playlists/{}/followers", id),
            Endpoint::PlaylistCoverImage { id } => format!("/playlists/{}/images", id),
            Endpoint::CurrentUserPlaylists { limit, offset } => {
                format!("/me/playlists?limit={}&offset={}", limit, offset)
            }
            Endpoint::UserPlaylists { user_id } => format!("/users/{}/playlists", user_id),
            Endpoint::FeaturedPlaylists { limit, offset } => {
                format!("/browse/featured-playlists?limit={}&offset={}", limit, offset)
            }
            Endpoint::CategoryPlaylists { category_id, limit, offset } => {
                format!("/browse/categories/{}/playlists?limit={}&offset={}", category_id, limit, offset)
            }

            // Library (tracks)
            Endpoint::SavedTracks { limit, offset } => {
                format!("/me/tracks?limit={}&offset={}", limit, offset)
            }
            Endpoint::SavedTracksIds { ids } => format!("/me/tracks?ids={}", ids),
            Endpoint::SavedTracksContains { ids } => format!("/me/tracks/contains?ids={}", ids),

            // Library (albums)
            Endpoint::SavedAlbums { limit, offset } => {
                format!("/me/albums?limit={}&offset={}", limit, offset)
            }
            Endpoint::SavedAlbumsIds { ids } => format!("/me/albums?ids={}", ids),
            Endpoint::SavedAlbumsContains { ids } => format!("/me/albums/contains?ids={}", ids),

            // Tracks
            Endpoint::Track { id } => format!("/tracks/{}", id),
            Endpoint::Tracks { ids } => format!("/tracks?ids={}", ids),

            // Albums
            Endpoint::Album { id } => format!("/albums/{}", id),
            Endpoint::Albums { ids } => format!("/albums?ids={}", ids),
            Endpoint::AlbumTracks { id, limit, offset } => {
                format!("/albums/{}/tracks?limit={}&offset={}", id, limit, offset)
            }
            Endpoint::NewReleases { limit, offset } => {
                format!("/browse/new-releases?limit={}&offset={}", limit, offset)
            }

            // Artists
            Endpoint::Artist { id } => format!("/artists/{}", id),
            Endpoint::Artists { ids } => format!("/artists?ids={}", ids),
            Endpoint::ArtistTopTracks { id, market } => {
                format!("/artists/{}/top-tracks?market={}", id, market)
            }
            Endpoint::ArtistAlbums { id, limit, offset } => {
                format!("/artists/{}/albums?limit={}&offset={}", id, limit, offset)
            }
            Endpoint::ArtistRelatedArtists { id } => format!("/artists/{}/related-artists", id),

            // User
            Endpoint::CurrentUser => "/me".to_string(),
            Endpoint::UserProfile { user_id } => format!("/users/{}", user_id),
            Endpoint::UserTopItems {
                item_type,
                time_range,
                limit,
                offset,
            } => {
                format!(
                    "/me/top/{}?time_range={}&limit={}&offset={}",
                    item_type, time_range, limit, offset
                )
            }
            Endpoint::FollowedArtists { limit } => {
                format!("/me/following?type=artist&limit={}", limit)
            }
            Endpoint::FollowArtistsOrUsers { entity_type, ids } => {
                format!("/me/following?type={}&ids={}", entity_type, ids)
            }
            Endpoint::FollowingContains { entity_type, ids } => {
                format!("/me/following/contains?type={}&ids={}", entity_type, ids)
            }
            Endpoint::FollowPlaylist { playlist_id } => {
                format!("/playlists/{}/followers", playlist_id)
            }
            Endpoint::FollowPlaylistContains { playlist_id, ids } => {
                format!("/playlists/{}/followers/contains?ids={}", playlist_id, ids)
            }

            // Categories
            Endpoint::Category { id } => format!("/browse/categories/{}", id),
            Endpoint::Categories { limit, offset } => {
                format!("/browse/categories?limit={}&offset={}", limit, offset)
            }

            // Markets
            Endpoint::Markets => "/markets".to_string(),

            // Search
            Endpoint::Search {
                query,
                types,
                limit,
            } => format!("/search?q={}&type={}&limit={}", encode(query), types, limit),

            // Shows
            Endpoint::Show { id } => format!("/shows/{}", id),
            Endpoint::Shows { ids } => format!("/shows?ids={}", ids),
            Endpoint::ShowEpisodes { id, limit, offset } => {
                format!("/shows/{}/episodes?limit={}&offset={}", id, limit, offset)
            }
            Endpoint::SavedShows { limit, offset } => {
                format!("/me/shows?limit={}&offset={}", limit, offset)
            }
            Endpoint::SavedShowsIds { ids } => format!("/me/shows?ids={}", ids),
            Endpoint::SavedShowsContains { ids } => format!("/me/shows/contains?ids={}", ids),

            // Episodes
            Endpoint::Episode { id } => format!("/episodes/{}", id),
            Endpoint::Episodes { ids } => format!("/episodes?ids={}", ids),
            Endpoint::SavedEpisodes { limit, offset } => {
                format!("/me/episodes?limit={}&offset={}", limit, offset)
            }
            Endpoint::SavedEpisodesIds { ids } => format!("/me/episodes?ids={}", ids),
            Endpoint::SavedEpisodesContains { ids } => format!("/me/episodes/contains?ids={}", ids),

            // Audiobooks
            Endpoint::Audiobook { id } => format!("/audiobooks/{}", id),
            Endpoint::Audiobooks { ids } => format!("/audiobooks?ids={}", ids),
            Endpoint::AudiobookChapters { id, limit, offset } => {
                format!("/audiobooks/{}/chapters?limit={}&offset={}", id, limit, offset)
            }
            Endpoint::SavedAudiobooks { limit, offset } => {
                format!("/me/audiobooks?limit={}&offset={}", limit, offset)
            }
            Endpoint::SavedAudiobooksIds { ids } => format!("/me/audiobooks?ids={}", ids),
            Endpoint::SavedAudiobooksContains { ids } => {
                format!("/me/audiobooks/contains?ids={}", ids)
            }

            // Chapters
            Endpoint::Chapter { id } => format!("/chapters/{}", id),
            Endpoint::Chapters { ids } => format!("/chapters?ids={}", ids),
        }
    }
}
