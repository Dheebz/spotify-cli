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

    Playlist { id: &'a str },
    PlaylistTracks { id: &'a str },
    PlaylistItems { id: &'a str, limit: u8, offset: u32 },
    PlaylistFollowers { id: &'a str },
    PlaylistCoverImage { id: &'a str },
    CurrentUserPlaylists { limit: u8, offset: u32 },
    UserPlaylists { user_id: &'a str },
    CategoryPlaylists { category_id: &'a str, limit: u8, offset: u32 },

    SavedTracks { limit: u8, offset: u32 },
    SavedTracksIds { ids: &'a str },
    SavedTracksContains { ids: &'a str },

    SavedAlbums { limit: u8, offset: u32 },
    SavedAlbumsIds { ids: &'a str },
    SavedAlbumsContains { ids: &'a str },

    Track { id: &'a str },
    Tracks { ids: &'a str },

    Album { id: &'a str },
    Albums { ids: &'a str },
    AlbumTracks { id: &'a str, limit: u8, offset: u32 },
    NewReleases { limit: u8, offset: u32 },

    Artist { id: &'a str },
    Artists { ids: &'a str },
    ArtistTopTracks { id: &'a str, market: &'a str },
    ArtistAlbums { id: &'a str, limit: u8, offset: u32 },
    ArtistRelatedArtists { id: &'a str },

    CurrentUser,
    UserProfile { user_id: &'a str },
    UserTopItems { item_type: &'a str, time_range: &'a str, limit: u8, offset: u32 },
    FollowedArtists { limit: u8 },
    FollowArtistsOrUsers { entity_type: &'a str, ids: &'a str },
    FollowingContains { entity_type: &'a str, ids: &'a str },
    FollowPlaylist { playlist_id: &'a str },
    FollowPlaylistContains { playlist_id: &'a str, ids: &'a str },

    Category { id: &'a str },
    Categories { limit: u8, offset: u32 },

    Markets,

    Search { query: &'a str, types: &'a str, limit: u8 },

    Show { id: &'a str },
    Shows { ids: &'a str },
    ShowEpisodes { id: &'a str, limit: u8, offset: u32 },
    SavedShows { limit: u8, offset: u32 },
    SavedShowsIds { ids: &'a str },
    SavedShowsContains { ids: &'a str },

    Episode { id: &'a str },
    Episodes { ids: &'a str },
    SavedEpisodes { limit: u8, offset: u32 },
    SavedEpisodesIds { ids: &'a str },
    SavedEpisodesContains { ids: &'a str },

    Audiobook { id: &'a str },
    Audiobooks { ids: &'a str },
    AudiobookChapters { id: &'a str, limit: u8, offset: u32 },
    SavedAudiobooks { limit: u8, offset: u32 },
    SavedAudiobooksIds { ids: &'a str },
    SavedAudiobooksContains { ids: &'a str },

    Chapter { id: &'a str },
    Chapters { ids: &'a str },
}

impl<'a> Endpoint<'a> {
    /// Get the path string for this endpoint
    pub fn path(&self) -> String {
        match self {
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
            Endpoint::CategoryPlaylists { category_id, limit, offset } => {
                format!("/browse/categories/{}/playlists?limit={}&offset={}", category_id, limit, offset)
            }

            Endpoint::SavedTracks { limit, offset } => {
                format!("/me/tracks?limit={}&offset={}", limit, offset)
            }
            Endpoint::SavedTracksIds { ids } => format!("/me/tracks?ids={}", ids),
            Endpoint::SavedTracksContains { ids } => format!("/me/tracks/contains?ids={}", ids),

            Endpoint::SavedAlbums { limit, offset } => {
                format!("/me/albums?limit={}&offset={}", limit, offset)
            }
            Endpoint::SavedAlbumsIds { ids } => format!("/me/albums?ids={}", ids),
            Endpoint::SavedAlbumsContains { ids } => format!("/me/albums/contains?ids={}", ids),

            Endpoint::Track { id } => format!("/tracks/{}", id),
            Endpoint::Tracks { ids } => format!("/tracks?ids={}", ids),

            Endpoint::Album { id } => format!("/albums/{}", id),
            Endpoint::Albums { ids } => format!("/albums?ids={}", ids),
            Endpoint::AlbumTracks { id, limit, offset } => {
                format!("/albums/{}/tracks?limit={}&offset={}", id, limit, offset)
            }
            Endpoint::NewReleases { limit, offset } => {
                format!("/browse/new-releases?limit={}&offset={}", limit, offset)
            }

            Endpoint::Artist { id } => format!("/artists/{}", id),
            Endpoint::Artists { ids } => format!("/artists?ids={}", ids),
            Endpoint::ArtistTopTracks { id, market } => {
                format!("/artists/{}/top-tracks?market={}", id, market)
            }
            Endpoint::ArtistAlbums { id, limit, offset } => {
                format!("/artists/{}/albums?limit={}&offset={}", id, limit, offset)
            }
            Endpoint::ArtistRelatedArtists { id } => format!("/artists/{}/related-artists", id),

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

            Endpoint::Category { id } => format!("/browse/categories/{}", id),
            Endpoint::Categories { limit, offset } => {
                format!("/browse/categories?limit={}&offset={}", limit, offset)
            }

            Endpoint::Markets => "/markets".to_string(),

            Endpoint::Search {
                query,
                types,
                limit,
            } => format!("/search?q={}&type={}&limit={}", encode(query), types, limit),

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

            Endpoint::Episode { id } => format!("/episodes/{}", id),
            Endpoint::Episodes { ids } => format!("/episodes?ids={}", ids),
            Endpoint::SavedEpisodes { limit, offset } => {
                format!("/me/episodes?limit={}&offset={}", limit, offset)
            }
            Endpoint::SavedEpisodesIds { ids } => format!("/me/episodes?ids={}", ids),
            Endpoint::SavedEpisodesContains { ids } => format!("/me/episodes/contains?ids={}", ids),

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

            Endpoint::Chapter { id } => format!("/chapters/{}", id),
            Endpoint::Chapters { ids } => format!("/chapters?ids={}", ids),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn player_endpoints() {
        assert_eq!(Endpoint::PlayerState.path(), "/me/player");
        assert_eq!(Endpoint::PlayerCurrentlyPlaying.path(), "/me/player/currently-playing");
        assert_eq!(Endpoint::PlayerPlay.path(), "/me/player/play");
        assert_eq!(Endpoint::PlayerPause.path(), "/me/player/pause");
        assert_eq!(Endpoint::PlayerNext.path(), "/me/player/next");
        assert_eq!(Endpoint::PlayerPrevious.path(), "/me/player/previous");
        assert_eq!(Endpoint::PlayerQueue.path(), "/me/player/queue");
        assert_eq!(Endpoint::PlayerDevices.path(), "/me/player/devices");
        assert_eq!(Endpoint::PlayerRecentlyPlayed.path(), "/me/player/recently-played");
    }

    #[test]
    fn player_endpoints_with_params() {
        assert_eq!(
            Endpoint::PlayerSeek { position_ms: 30000 }.path(),
            "/me/player/seek?position_ms=30000"
        );
        assert_eq!(
            Endpoint::PlayerVolume { volume_percent: 50 }.path(),
            "/me/player/volume?volume_percent=50"
        );
        assert_eq!(
            Endpoint::PlayerShuffle { state: true }.path(),
            "/me/player/shuffle?state=true"
        );
        assert_eq!(
            Endpoint::PlayerRepeat { state: "track" }.path(),
            "/me/player/repeat?state=track"
        );
    }

    #[test]
    fn queue_add_encodes_uri() {
        let uri = "spotify:track:abc123";
        let path = Endpoint::PlayerQueueAdd { uri }.path();
        assert!(path.starts_with("/me/player/queue?uri="));
        assert!(path.contains("spotify"));
    }

    #[test]
    fn playlist_endpoints() {
        assert_eq!(
            Endpoint::Playlist { id: "abc123" }.path(),
            "/playlists/abc123"
        );
        assert_eq!(
            Endpoint::PlaylistTracks { id: "abc123" }.path(),
            "/playlists/abc123/tracks"
        );
        assert_eq!(
            Endpoint::PlaylistItems { id: "abc123", limit: 50, offset: 0 }.path(),
            "/playlists/abc123/tracks?limit=50&offset=0"
        );
        assert_eq!(
            Endpoint::PlaylistFollowers { id: "abc123" }.path(),
            "/playlists/abc123/followers"
        );
    }

    #[test]
    fn user_playlist_endpoints() {
        assert_eq!(
            Endpoint::CurrentUserPlaylists { limit: 20, offset: 40 }.path(),
            "/me/playlists?limit=20&offset=40"
        );
        assert_eq!(
            Endpoint::UserPlaylists { user_id: "user123" }.path(),
            "/users/user123/playlists"
        );
    }

    #[test]
    fn browse_endpoints() {
        assert_eq!(
            Endpoint::CategoryPlaylists { category_id: "pop", limit: 20, offset: 0 }.path(),
            "/browse/categories/pop/playlists?limit=20&offset=0"
        );
        assert_eq!(
            Endpoint::NewReleases { limit: 20, offset: 0 }.path(),
            "/browse/new-releases?limit=20&offset=0"
        );
        assert_eq!(
            Endpoint::Category { id: "rock" }.path(),
            "/browse/categories/rock"
        );
        assert_eq!(
            Endpoint::Categories { limit: 50, offset: 0 }.path(),
            "/browse/categories?limit=50&offset=0"
        );
    }

    #[test]
    fn saved_tracks_endpoints() {
        assert_eq!(
            Endpoint::SavedTracks { limit: 20, offset: 0 }.path(),
            "/me/tracks?limit=20&offset=0"
        );
        assert_eq!(
            Endpoint::SavedTracksIds { ids: "id1,id2" }.path(),
            "/me/tracks?ids=id1,id2"
        );
        assert_eq!(
            Endpoint::SavedTracksContains { ids: "id1" }.path(),
            "/me/tracks/contains?ids=id1"
        );
    }

    #[test]
    fn saved_albums_endpoints() {
        assert_eq!(
            Endpoint::SavedAlbums { limit: 20, offset: 0 }.path(),
            "/me/albums?limit=20&offset=0"
        );
        assert_eq!(
            Endpoint::SavedAlbumsIds { ids: "id1,id2" }.path(),
            "/me/albums?ids=id1,id2"
        );
        assert_eq!(
            Endpoint::SavedAlbumsContains { ids: "id1" }.path(),
            "/me/albums/contains?ids=id1"
        );
    }

    #[test]
    fn track_endpoints() {
        assert_eq!(Endpoint::Track { id: "track123" }.path(), "/tracks/track123");
        assert_eq!(Endpoint::Tracks { ids: "t1,t2,t3" }.path(), "/tracks?ids=t1,t2,t3");
    }

    #[test]
    fn album_endpoints() {
        assert_eq!(Endpoint::Album { id: "album123" }.path(), "/albums/album123");
        assert_eq!(Endpoint::Albums { ids: "a1,a2" }.path(), "/albums?ids=a1,a2");
        assert_eq!(
            Endpoint::AlbumTracks { id: "album123", limit: 50, offset: 0 }.path(),
            "/albums/album123/tracks?limit=50&offset=0"
        );
    }

    #[test]
    fn artist_endpoints() {
        assert_eq!(Endpoint::Artist { id: "artist123" }.path(), "/artists/artist123");
        assert_eq!(Endpoint::Artists { ids: "a1,a2" }.path(), "/artists?ids=a1,a2");
        assert_eq!(
            Endpoint::ArtistTopTracks { id: "artist123", market: "US" }.path(),
            "/artists/artist123/top-tracks?market=US"
        );
        assert_eq!(
            Endpoint::ArtistAlbums { id: "artist123", limit: 20, offset: 0 }.path(),
            "/artists/artist123/albums?limit=20&offset=0"
        );
        assert_eq!(
            Endpoint::ArtistRelatedArtists { id: "artist123" }.path(),
            "/artists/artist123/related-artists"
        );
    }

    #[test]
    fn user_endpoints() {
        assert_eq!(Endpoint::CurrentUser.path(), "/me");
        assert_eq!(
            Endpoint::UserProfile { user_id: "user123" }.path(),
            "/users/user123"
        );
        assert_eq!(
            Endpoint::UserTopItems { item_type: "tracks", time_range: "medium_term", limit: 20, offset: 0 }.path(),
            "/me/top/tracks?time_range=medium_term&limit=20&offset=0"
        );
    }

    #[test]
    fn follow_endpoints() {
        assert_eq!(
            Endpoint::FollowedArtists { limit: 20 }.path(),
            "/me/following?type=artist&limit=20"
        );
        assert_eq!(
            Endpoint::FollowArtistsOrUsers { entity_type: "artist", ids: "id1,id2" }.path(),
            "/me/following?type=artist&ids=id1,id2"
        );
        assert_eq!(
            Endpoint::FollowingContains { entity_type: "artist", ids: "id1" }.path(),
            "/me/following/contains?type=artist&ids=id1"
        );
        assert_eq!(
            Endpoint::FollowPlaylist { playlist_id: "pl123" }.path(),
            "/playlists/pl123/followers"
        );
        assert_eq!(
            Endpoint::FollowPlaylistContains { playlist_id: "pl123", ids: "user1" }.path(),
            "/playlists/pl123/followers/contains?ids=user1"
        );
    }

    #[test]
    fn markets_endpoint() {
        assert_eq!(Endpoint::Markets.path(), "/markets");
    }

    #[test]
    fn search_encodes_query() {
        let path = Endpoint::Search { query: "hello world", types: "track", limit: 20 }.path();
        assert!(path.contains("hello%20world") || path.contains("hello+world"));
        assert!(path.contains("type=track"));
        assert!(path.contains("limit=20"));
    }

    #[test]
    fn show_endpoints() {
        assert_eq!(Endpoint::Show { id: "show123" }.path(), "/shows/show123");
        assert_eq!(Endpoint::Shows { ids: "s1,s2" }.path(), "/shows?ids=s1,s2");
        assert_eq!(
            Endpoint::ShowEpisodes { id: "show123", limit: 20, offset: 0 }.path(),
            "/shows/show123/episodes?limit=20&offset=0"
        );
        assert_eq!(
            Endpoint::SavedShows { limit: 20, offset: 0 }.path(),
            "/me/shows?limit=20&offset=0"
        );
    }

    #[test]
    fn episode_endpoints() {
        assert_eq!(Endpoint::Episode { id: "ep123" }.path(), "/episodes/ep123");
        assert_eq!(Endpoint::Episodes { ids: "e1,e2" }.path(), "/episodes?ids=e1,e2");
        assert_eq!(
            Endpoint::SavedEpisodes { limit: 20, offset: 0 }.path(),
            "/me/episodes?limit=20&offset=0"
        );
    }

    #[test]
    fn audiobook_endpoints() {
        assert_eq!(Endpoint::Audiobook { id: "ab123" }.path(), "/audiobooks/ab123");
        assert_eq!(Endpoint::Audiobooks { ids: "ab1,ab2" }.path(), "/audiobooks?ids=ab1,ab2");
        assert_eq!(
            Endpoint::AudiobookChapters { id: "ab123", limit: 50, offset: 0 }.path(),
            "/audiobooks/ab123/chapters?limit=50&offset=0"
        );
        assert_eq!(
            Endpoint::SavedAudiobooks { limit: 20, offset: 0 }.path(),
            "/me/audiobooks?limit=20&offset=0"
        );
    }

    #[test]
    fn chapter_endpoints() {
        assert_eq!(Endpoint::Chapter { id: "ch123" }.path(), "/chapters/ch123");
        assert_eq!(Endpoint::Chapters { ids: "c1,c2" }.path(), "/chapters?ids=c1,c2");
    }

    #[test]
    fn saved_shows_ids_endpoint() {
        assert_eq!(
            Endpoint::SavedShowsIds { ids: "show1,show2" }.path(),
            "/me/shows?ids=show1,show2"
        );
    }

    #[test]
    fn saved_shows_contains_endpoint() {
        assert_eq!(
            Endpoint::SavedShowsContains { ids: "show1" }.path(),
            "/me/shows/contains?ids=show1"
        );
    }

    #[test]
    fn saved_episodes_ids_endpoint() {
        assert_eq!(
            Endpoint::SavedEpisodesIds { ids: "ep1,ep2" }.path(),
            "/me/episodes?ids=ep1,ep2"
        );
    }

    #[test]
    fn saved_episodes_contains_endpoint() {
        assert_eq!(
            Endpoint::SavedEpisodesContains { ids: "ep1" }.path(),
            "/me/episodes/contains?ids=ep1"
        );
    }

    #[test]
    fn saved_audiobooks_ids_endpoint() {
        assert_eq!(
            Endpoint::SavedAudiobooksIds { ids: "ab1,ab2" }.path(),
            "/me/audiobooks?ids=ab1,ab2"
        );
    }

    #[test]
    fn saved_audiobooks_contains_endpoint() {
        assert_eq!(
            Endpoint::SavedAudiobooksContains { ids: "ab1" }.path(),
            "/me/audiobooks/contains?ids=ab1"
        );
    }

    #[test]
    fn playlist_cover_image_endpoint() {
        assert_eq!(
            Endpoint::PlaylistCoverImage { id: "pl123" }.path(),
            "/playlists/pl123/images"
        );
    }
}
