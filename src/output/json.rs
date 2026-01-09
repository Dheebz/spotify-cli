//! JSON output formatting for machine-readable responses.
use serde::Serialize;

use crate::domain::album::Album;
use crate::domain::artist::Artist;
use crate::domain::auth::{AuthScopes, AuthStatus};
use crate::domain::device::Device;
use crate::domain::pin::PinnedPlaylist;
use crate::domain::player::PlayerStatus;
use crate::domain::playlist::{Playlist, PlaylistDetail};
use crate::domain::search::{SearchItem, SearchResults, SearchType};
use crate::error::Result;

#[derive(Serialize)]
struct AuthStatusPayload {
    logged_in: bool,
    expires_at: Option<u64>,
}

pub fn auth_status(status: AuthStatus) -> Result<()> {
    let payload = auth_status_payload(status);
    println!("{}", serde_json::to_string(&payload)?);
    Ok(())
}

fn auth_status_payload(status: AuthStatus) -> AuthStatusPayload {
    AuthStatusPayload {
        logged_in: status.logged_in,
        expires_at: status.expires_at,
    }
}

#[derive(Serialize)]
struct AuthScopesPayload {
    required: Vec<String>,
    granted: Option<Vec<String>>,
    missing: Vec<String>,
}

pub fn auth_scopes(scopes: AuthScopes) -> Result<()> {
    let payload = auth_scopes_payload(scopes);
    println!("{}", serde_json::to_string(&payload)?);
    Ok(())
}

fn auth_scopes_payload(scopes: AuthScopes) -> AuthScopesPayload {
    AuthScopesPayload {
        required: scopes.required,
        granted: scopes.granted,
        missing: scopes.missing,
    }
}

#[derive(Serialize)]
struct PlayerStatusPayload {
    is_playing: bool,
    track: Option<TrackPayload>,
    device: Option<DevicePayload>,
    context: Option<PlaybackContextPayload>,
    progress_ms: Option<u32>,
    repeat_state: Option<String>,
    shuffle_state: Option<bool>,
}

#[derive(Serialize)]
struct TrackPayload {
    id: String,
    name: String,
    artists: Vec<String>,
    album: Option<String>,
    album_id: Option<String>,
    duration_ms: Option<u32>,
}

#[derive(Serialize)]
struct PlaybackContextPayload {
    kind: String,
    uri: String,
}

pub fn player_status(status: PlayerStatus) -> Result<()> {
    let payload = player_status_payload(status);
    println!("{}", serde_json::to_string(&payload)?);
    Ok(())
}

fn player_status_payload(status: PlayerStatus) -> PlayerStatusPayload {
    let track = status.track.map(track_payload);
    let device = status.device.map(device_payload);
    let context = status.context.map(|context| PlaybackContextPayload {
        kind: context.kind,
        uri: context.uri,
    });

    PlayerStatusPayload {
        is_playing: status.is_playing,
        track,
        device,
        context,
        progress_ms: status.progress_ms,
        repeat_state: status.repeat_state,
        shuffle_state: status.shuffle_state,
    }
}

#[derive(Serialize)]
struct NowPlayingPayload {
    event: &'static str,
    status: PlayerStatusPayload,
}

pub fn now_playing(status: PlayerStatus) -> Result<()> {
    let payload = now_playing_payload(status);
    println!("{}", serde_json::to_string(&payload)?);
    Ok(())
}

fn now_playing_payload(status: PlayerStatus) -> NowPlayingPayload {
    let track = status.track.map(track_payload);
    let device = status.device.map(device_payload);
    let context = status.context.map(|context| PlaybackContextPayload {
        kind: context.kind,
        uri: context.uri,
    });

    let status_payload = PlayerStatusPayload {
        is_playing: status.is_playing,
        track,
        device,
        context,
        progress_ms: status.progress_ms,
        repeat_state: status.repeat_state,
        shuffle_state: status.shuffle_state,
    };

    NowPlayingPayload {
        event: "now_playing",
        status: status_payload,
    }
}

#[derive(Serialize)]
struct DevicePayload {
    id: String,
    name: String,
    volume_percent: Option<u32>,
}

#[derive(Serialize)]
struct ActionPayload<'a> {
    event: &'a str,
    message: &'a str,
}

pub fn action(event: &str, message: &str) -> Result<()> {
    let payload = action_payload(event, message);
    println!("{}", serde_json::to_string(&payload)?);
    Ok(())
}

fn action_payload<'a>(event: &'a str, message: &'a str) -> ActionPayload<'a> {
    ActionPayload { event, message }
}

#[derive(Serialize)]
struct AlbumPayload {
    id: String,
    name: String,
    uri: String,
    artists: Vec<String>,
    release_date: Option<String>,
    total_tracks: Option<u32>,
    duration_ms: Option<u64>,
    tracks: Vec<AlbumTrackPayload>,
}

pub fn album_info(album: Album) -> Result<()> {
    let payload = album_info_payload(album);
    println!("{}", serde_json::to_string(&payload)?);
    Ok(())
}

fn album_info_payload(album: Album) -> AlbumPayload {
    AlbumPayload {
        id: album.id,
        name: album.name,
        uri: album.uri,
        artists: album.artists,
        release_date: album.release_date,
        total_tracks: album.total_tracks,
        duration_ms: album.duration_ms,
        tracks: album
            .tracks
            .into_iter()
            .map(|track| AlbumTrackPayload {
                name: track.name,
                duration_ms: track.duration_ms,
                track_number: track.track_number,
            })
            .collect(),
    }
}

#[derive(Serialize)]
struct AlbumTrackPayload {
    name: String,
    duration_ms: u32,
    track_number: u32,
}

#[derive(Serialize)]
struct ArtistPayload {
    id: String,
    name: String,
    uri: String,
    genres: Vec<String>,
    followers: Option<u64>,
}

pub fn artist_info(artist: Artist) -> Result<()> {
    let payload = artist_info_payload(artist);
    println!("{}", serde_json::to_string(&payload)?);
    Ok(())
}

fn artist_info_payload(artist: Artist) -> ArtistPayload {
    ArtistPayload {
        id: artist.id,
        name: artist.name,
        uri: artist.uri,
        genres: artist.genres,
        followers: artist.followers,
    }
}

#[derive(Serialize)]
struct PlaylistPayload {
    id: String,
    name: String,
    owner: Option<String>,
    collaborative: bool,
    public: Option<bool>,
}

pub fn playlist_list(playlists: Vec<Playlist>) -> Result<()> {
    let payload = playlist_list_payload(playlists);
    println!("{}", serde_json::to_string(&payload)?);
    Ok(())
}

fn playlist_list_payload(playlists: Vec<Playlist>) -> Vec<PlaylistPayload> {
    playlists.into_iter().map(playlist_payload).collect()
}

#[derive(Serialize)]
struct PlaylistListPayload {
    playlists: Vec<PlaylistPayload>,
    pinned: Vec<PinPayload>,
}

#[derive(Serialize)]
struct PinPayload {
    name: String,
    url: String,
}

pub fn playlist_list_with_pins(playlists: Vec<Playlist>, pins: Vec<PinnedPlaylist>) -> Result<()> {
    let payload = playlist_list_with_pins_payload(playlists, pins);
    println!("{}", serde_json::to_string(&payload)?);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::album::AlbumTrack;
    use crate::domain::artist::Artist;
    use crate::domain::auth::{AuthScopes, AuthStatus};
    use crate::domain::device::Device;
    use crate::domain::player::PlayerStatus;
    use crate::domain::playlist::{Playlist, PlaylistDetail};
    use crate::domain::search::{SearchItem, SearchResults, SearchType};

    #[test]
    fn auth_status_payload_shape() {
        let payload = auth_status_payload(AuthStatus {
            logged_in: true,
            expires_at: Some(1),
        });
        assert!(payload.logged_in);
        assert_eq!(payload.expires_at, Some(1));
    }

    #[test]
    fn auth_scopes_payload_shape() {
        let payload = auth_scopes_payload(AuthScopes {
            required: vec!["a".into()],
            granted: Some(vec!["a".into()]),
            missing: vec![],
        });
        assert_eq!(payload.required.len(), 1);
    }

    #[test]
    fn player_status_payload_shape() {
        let payload = player_status_payload(PlayerStatus {
            is_playing: true,
            track: None,
            device: None,
            context: None,
            progress_ms: None,
            repeat_state: Some("off".into()),
            shuffle_state: Some(false),
        });
        assert!(payload.is_playing);
    }

    #[test]
    fn now_playing_payload_shape() {
        let payload = now_playing_payload(PlayerStatus {
            is_playing: false,
            track: None,
            device: None,
            context: None,
            progress_ms: None,
            repeat_state: Some("context".into()),
            shuffle_state: Some(true),
        });
        assert_eq!(payload.event, "now_playing");
    }

    #[test]
    fn action_payload_shape() {
        let payload = action_payload("event", "message");
        assert_eq!(payload.event, "event");
        assert_eq!(payload.message, "message");
    }

    #[test]
    fn album_info_payload_shape() {
        let payload = album_info_payload(Album {
            id: "1".into(),
            name: "Album".into(),
            uri: "uri".into(),
            artists: vec!["Artist".into()],
            release_date: None,
            total_tracks: Some(1),
            tracks: vec![AlbumTrack {
                name: "Track".into(),
                duration_ms: 1000,
                track_number: 1,
            }],
            duration_ms: Some(1000),
        });
        assert_eq!(payload.tracks.len(), 1);
    }

    #[test]
    fn artist_info_payload_shape() {
        let payload = artist_info_payload(Artist {
            id: "1".into(),
            name: "Artist".into(),
            uri: "uri".into(),
            genres: vec![],
            followers: Some(10),
        });
        assert_eq!(payload.followers, Some(10));
    }

    #[test]
    fn playlist_list_payload_shape() {
        let payload = playlist_list_payload(vec![Playlist {
            id: "1".into(),
            name: "List".into(),
            owner: None,
            collaborative: false,
            public: Some(true),
        }]);
        assert_eq!(payload.len(), 1);
    }

    #[test]
    fn playlist_list_with_pins_payload_shape() {
        let payload = playlist_list_with_pins_payload(
            vec![Playlist {
                id: "1".into(),
                name: "List".into(),
                owner: None,
                collaborative: false,
                public: Some(true),
            }],
            vec![PinnedPlaylist {
                name: "Pin".into(),
                url: "url".into(),
            }],
        );
        assert_eq!(payload.pinned.len(), 1);
    }

    #[test]
    fn playlist_info_payload_shape() {
        let payload = playlist_info_payload(PlaylistDetail {
            id: "1".into(),
            name: "List".into(),
            uri: "uri".into(),
            owner: None,
            tracks_total: Some(2),
            collaborative: false,
            public: Some(true),
        });
        assert_eq!(payload.tracks_total, Some(2));
    }

    #[test]
    fn device_list_payload_shape() {
        let payload = device_list_payload(vec![Device {
            id: "1".into(),
            name: "Device".into(),
            volume_percent: Some(10),
        }]);
        assert_eq!(payload.len(), 1);
    }

    #[test]
    fn search_results_payload_shape() {
        let payload = search_results_payload(SearchResults {
            kind: SearchType::All,
            items: vec![SearchItem {
                id: "1".into(),
                name: "Track".into(),
                uri: "uri".into(),
                kind: SearchType::Track,
                artists: vec!["Artist".into()],
                album: Some("Album".into()),
                duration_ms: Some(1000),
                owner: None,
                score: None,
            }],
        });
        assert_eq!(payload.kind, "all");
        assert_eq!(payload.items[0].kind, "track");
        assert_eq!(payload.items.len(), 1);
    }

    #[test]
    fn help_payload_shape() {
        let payload = help_payload();
        assert!(payload.objects.contains(&"auth"));
    }
}
fn playlist_list_with_pins_payload(
    playlists: Vec<Playlist>,
    pins: Vec<PinnedPlaylist>,
) -> PlaylistListPayload {
    let playlists = playlists.into_iter().map(playlist_payload).collect();

    let pinned = pins.into_iter().map(pin_payload).collect();

    PlaylistListPayload { playlists, pinned }
}

#[derive(Serialize)]
struct HelpPayload {
    usage: &'static str,
    objects: Vec<&'static str>,
    examples: Vec<&'static str>,
}

pub fn help() -> Result<()> {
    let payload = help_payload();
    println!("{}", serde_json::to_string(&payload)?);
    Ok(())
}

fn help_payload() -> HelpPayload {
    HelpPayload {
        usage: "spotify-cli <object> <verb> [target] [flags]",
        objects: vec![
            "auth",
            "device",
            "info",
            "search",
            "nowplaying",
            "player",
            "playlist",
            "pin",
            "sync",
            "queue",
            "recentlyplayed",
        ],
        examples: vec![
            "spotify-cli auth status",
            "spotify-cli search track \"boards of canada\" --play",
            "spotify-cli search \"boards of canada\"",
            "spotify-cli info album \"geogaddi\"",
            "spotify-cli nowplaying",
            "spotify-cli nowplaying like",
            "spotify-cli playlist list",
            "spotify-cli nowplaying addto \"MyRadar\"",
            "spotify-cli pin add \"Release Radar\" \"<url>\"",
        ],
    }
}

#[derive(Serialize)]
struct PlaylistDetailPayload {
    id: String,
    name: String,
    uri: String,
    owner: Option<String>,
    tracks_total: Option<u32>,
    collaborative: bool,
    public: Option<bool>,
}

pub fn playlist_info(playlist: PlaylistDetail) -> Result<()> {
    let payload = playlist_info_payload(playlist);
    println!("{}", serde_json::to_string(&payload)?);
    Ok(())
}

fn playlist_info_payload(playlist: PlaylistDetail) -> PlaylistDetailPayload {
    PlaylistDetailPayload {
        id: playlist.id,
        name: playlist.name,
        uri: playlist.uri,
        owner: playlist.owner,
        tracks_total: playlist.tracks_total,
        collaborative: playlist.collaborative,
        public: playlist.public,
    }
}

pub fn device_list(devices: Vec<Device>) -> Result<()> {
    let payload = device_list_payload(devices);
    println!("{}", serde_json::to_string(&payload)?);
    Ok(())
}

fn device_list_payload(devices: Vec<Device>) -> Vec<DevicePayload> {
    devices.into_iter().map(device_payload).collect()
}

#[derive(Serialize)]
struct SearchResultsPayload {
    kind: &'static str,
    items: Vec<SearchItemPayload>,
}

#[derive(Serialize)]
struct SearchItemPayload {
    id: String,
    name: String,
    uri: String,
    kind: &'static str,
    artists: Vec<String>,
    album: Option<String>,
    duration_ms: Option<u32>,
    owner: Option<String>,
    score: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    now_playing: Option<bool>,
}

pub fn search_results(results: SearchResults) -> Result<()> {
    let payload = search_results_payload(results);
    println!("{}", serde_json::to_string(&payload)?);
    Ok(())
}

fn search_results_payload(results: SearchResults) -> SearchResultsPayload {
    let items = results.items.into_iter().map(search_item_payload).collect();

    SearchResultsPayload {
        kind: search_type_label(results.kind),
        items,
    }
}

pub fn queue(now_playing_id: Option<&str>, items: Vec<SearchItem>) -> Result<()> {
    let payload = search_results_payload_with_now(
        SearchResults {
            kind: SearchType::Track,
            items,
        },
        now_playing_id,
    );
    println!("{}", serde_json::to_string(&payload)?);
    Ok(())
}

pub fn recently_played(now_playing_id: Option<&str>, items: Vec<SearchItem>) -> Result<()> {
    let payload = search_results_payload_with_now(
        SearchResults {
            kind: SearchType::Track,
            items,
        },
        now_playing_id,
    );
    println!("{}", serde_json::to_string(&payload)?);
    Ok(())
}

fn search_results_payload_with_now(
    results: SearchResults,
    now_playing_id: Option<&str>,
) -> SearchResultsPayload {
    let items = results
        .items
        .into_iter()
        .map(|item| search_item_payload_with_now(item, now_playing_id))
        .collect();

    SearchResultsPayload {
        kind: search_type_label(results.kind),
        items,
    }
}

fn track_payload(track: crate::domain::track::Track) -> TrackPayload {
    TrackPayload {
        id: track.id,
        name: track.name,
        artists: track.artists,
        album: track.album,
        album_id: track.album_id,
        duration_ms: track.duration_ms,
    }
}

fn device_payload(device: Device) -> DevicePayload {
    DevicePayload {
        id: device.id,
        name: device.name,
        volume_percent: device.volume_percent,
    }
}

fn playlist_payload(playlist: Playlist) -> PlaylistPayload {
    PlaylistPayload {
        id: playlist.id,
        name: playlist.name,
        owner: playlist.owner,
        collaborative: playlist.collaborative,
        public: playlist.public,
    }
}

fn pin_payload(pin: PinnedPlaylist) -> PinPayload {
    PinPayload {
        name: pin.name,
        url: pin.url,
    }
}

fn search_item_payload(item: crate::domain::search::SearchItem) -> SearchItemPayload {
    SearchItemPayload {
        id: item.id,
        name: item.name,
        uri: item.uri,
        kind: search_type_label(item.kind),
        artists: item.artists,
        album: item.album,
        duration_ms: item.duration_ms,
        owner: item.owner,
        score: item.score,
        now_playing: None,
    }
}

fn search_item_payload_with_now(
    item: crate::domain::search::SearchItem,
    now_playing_id: Option<&str>,
) -> SearchItemPayload {
    let is_now_playing = now_playing_id.is_some_and(|id| id == item.id);
    SearchItemPayload {
        id: item.id,
        name: item.name,
        uri: item.uri,
        kind: search_type_label(item.kind),
        artists: item.artists,
        album: item.album,
        duration_ms: item.duration_ms,
        owner: item.owner,
        score: item.score,
        now_playing: if is_now_playing { Some(true) } else { None },
    }
}

fn search_type_label(kind: SearchType) -> &'static str {
    match kind {
        SearchType::All => "all",
        SearchType::Track => "track",
        SearchType::Album => "album",
        SearchType::Artist => "artist",
        SearchType::Playlist => "playlist",
    }
}
