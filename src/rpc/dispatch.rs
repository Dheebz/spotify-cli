//! RPC method dispatcher
//!
//! Maps JSON-RPC methods to CLI command handlers.
//! Full CLI-RPC parity: everything available in CLI is available via RPC.

use tracing::debug;

use crate::cli::commands::{self, ArtistQuery, ArtistView, SearchFilters};
use crate::io::output::{ErrorKind, Response};

use super::protocol::RpcRequest;

/// Command dispatcher
pub struct Dispatcher;

impl Dispatcher {
    pub fn new() -> Self {
        Self
    }

    /// Dispatch an RPC request to the appropriate handler
    pub async fn dispatch(&self, request: &RpcRequest) -> Response {
        debug!(method = %request.method, "Dispatching RPC request");

        let params = request.params.as_ref();

        match request.method.as_str() {
            // ============================================================
            // Daemon
            // ============================================================
            "ping" => Response::success(200, "pong"),
            "version" => Response::success_with_payload(
                200,
                "Version info",
                serde_json::json!({
                    "version": env!("CARGO_PKG_VERSION"),
                    "name": env!("CARGO_PKG_NAME"),
                }),
            ),

            // ============================================================
            // Auth
            // ============================================================
            "auth.login" => {
                let force = params
                    .and_then(|p| p.get("force"))
                    .and_then(|v| v.as_bool())
                    .unwrap_or(false);
                commands::auth_login(force).await
            }
            "auth.logout" => commands::auth_logout().await,
            "auth.refresh" => commands::auth_refresh().await,
            "auth.status" => commands::auth_status().await,

            // ============================================================
            // Player
            // ============================================================
            "player.status" => {
                let id_only = params
                    .and_then(|p| p.get("id_only"))
                    .and_then(|v| v.as_str());
                commands::player_status(id_only).await
            }
            "player.play" => {
                let uri = params.and_then(|p| p.get("uri")).and_then(|v| v.as_str());
                let pin = params.and_then(|p| p.get("pin")).and_then(|v| v.as_str());
                commands::player_play(uri, pin).await
            }
            "player.pause" => commands::player_pause().await,
            "player.toggle" => commands::player_toggle().await,
            "player.next" => commands::player_next().await,
            "player.previous" => commands::player_previous().await,
            "player.seek" => {
                let position = params
                    .and_then(|p| p.get("position"))
                    .and_then(|v| v.as_str())
                    .unwrap_or("0");
                commands::player_seek(position).await
            }
            "player.volume" => {
                let percent = params
                    .and_then(|p| p.get("percent"))
                    .and_then(|v| v.as_u64())
                    .unwrap_or(50) as u8;
                commands::player_volume(percent).await
            }
            "player.shuffle" => {
                let state = params
                    .and_then(|p| p.get("state"))
                    .and_then(|v| v.as_str())
                    .unwrap_or("on");
                commands::player_shuffle(state).await
            }
            "player.repeat" => {
                let mode = params
                    .and_then(|p| p.get("mode"))
                    .and_then(|v| v.as_str())
                    .unwrap_or("off");
                commands::player_repeat(mode).await
            }
            "player.devices" => commands::player_devices_list().await,
            "player.transfer" => {
                let device = params
                    .and_then(|p| p.get("device"))
                    .and_then(|v| v.as_str());
                if let Some(dev) = device {
                    commands::player_devices_transfer(dev).await
                } else {
                    Response::err(400, "Missing 'device' parameter", ErrorKind::Validation)
                }
            }
            "player.recent" => commands::player_recent().await,

            // ============================================================
            // Queue
            // ============================================================
            "queue.list" => commands::player_queue_list().await,
            "queue.add" => {
                let uri = params.and_then(|p| p.get("uri")).and_then(|v| v.as_str());
                let now_playing = params
                    .and_then(|p| p.get("now_playing"))
                    .and_then(|v| v.as_bool())
                    .unwrap_or(false);
                commands::player_queue_add(uri, now_playing).await
            }

            // ============================================================
            // Search
            // ============================================================
            "search" => {
                let query = params
                    .and_then(|p| p.get("query"))
                    .and_then(|v| v.as_str())
                    .unwrap_or("");
                let types: Vec<String> = params
                    .and_then(|p| p.get("types"))
                    .and_then(|v| v.as_array())
                    .map(|arr| {
                        arr.iter()
                            .filter_map(|v| v.as_str().map(String::from))
                            .collect()
                    })
                    .unwrap_or_default();
                let limit = params
                    .and_then(|p| p.get("limit"))
                    .and_then(|v| v.as_u64())
                    .unwrap_or(20) as u8;
                let pins_only = params
                    .and_then(|p| p.get("pins_only"))
                    .and_then(|v| v.as_bool())
                    .unwrap_or(false);
                let exact = params
                    .and_then(|p| p.get("exact"))
                    .and_then(|v| v.as_bool())
                    .unwrap_or(false);
                let play = params
                    .and_then(|p| p.get("play"))
                    .and_then(|v| v.as_bool())
                    .unwrap_or(false);

                let filters = SearchFilters {
                    artist: params
                        .and_then(|p| p.get("artist"))
                        .and_then(|v| v.as_str())
                        .map(String::from),
                    album: params
                        .and_then(|p| p.get("album"))
                        .and_then(|v| v.as_str())
                        .map(String::from),
                    track: params
                        .and_then(|p| p.get("track"))
                        .and_then(|v| v.as_str())
                        .map(String::from),
                    year: params
                        .and_then(|p| p.get("year"))
                        .and_then(|v| v.as_str())
                        .map(String::from),
                    genre: params
                        .and_then(|p| p.get("genre"))
                        .and_then(|v| v.as_str())
                        .map(String::from),
                    isrc: params
                        .and_then(|p| p.get("isrc"))
                        .and_then(|v| v.as_str())
                        .map(String::from),
                    upc: params
                        .and_then(|p| p.get("upc"))
                        .and_then(|v| v.as_str())
                        .map(String::from),
                    new: params
                        .and_then(|p| p.get("new"))
                        .and_then(|v| v.as_bool())
                        .unwrap_or(false),
                    hipster: params
                        .and_then(|p| p.get("hipster"))
                        .and_then(|v| v.as_bool())
                        .unwrap_or(false),
                };

                commands::search_command(query, &types, limit, pins_only, exact, filters, play)
                    .await
            }

            // ============================================================
            // Pin
            // ============================================================
            "pin.add" => {
                let resource_type = params
                    .and_then(|p| p.get("type"))
                    .and_then(|v| v.as_str())
                    .unwrap_or("track");
                let url_or_id = params
                    .and_then(|p| p.get("id"))
                    .and_then(|v| v.as_str())
                    .unwrap_or("");
                let alias = params
                    .and_then(|p| p.get("alias"))
                    .and_then(|v| v.as_str())
                    .unwrap_or("");
                let tags = params
                    .and_then(|p| p.get("tags"))
                    .and_then(|v| v.as_str());
                commands::pin_add(resource_type, url_or_id, alias, tags).await
            }
            "pin.remove" => {
                let alias_or_id = params
                    .and_then(|p| p.get("id"))
                    .and_then(|v| v.as_str())
                    .unwrap_or("");
                commands::pin_remove(alias_or_id).await
            }
            "pin.list" => {
                let resource_type = params
                    .and_then(|p| p.get("type"))
                    .and_then(|v| v.as_str());
                commands::pin_list(resource_type).await
            }

            // ============================================================
            // Playlist
            // ============================================================
            "playlist.list" => {
                let limit = params
                    .and_then(|p| p.get("limit"))
                    .and_then(|v| v.as_u64())
                    .unwrap_or(20) as u8;
                let offset = params
                    .and_then(|p| p.get("offset"))
                    .and_then(|v| v.as_u64())
                    .unwrap_or(0) as u32;
                commands::playlist_list(limit, offset).await
            }
            "playlist.get" => {
                let id = params
                    .and_then(|p| p.get("id"))
                    .and_then(|v| v.as_str())
                    .unwrap_or("");
                commands::playlist_get(id).await
            }
            "playlist.create" => {
                let name = params
                    .and_then(|p| p.get("name"))
                    .and_then(|v| v.as_str())
                    .unwrap_or("New Playlist");
                let description = params
                    .and_then(|p| p.get("description"))
                    .and_then(|v| v.as_str());
                let public = params
                    .and_then(|p| p.get("public"))
                    .and_then(|v| v.as_bool())
                    .unwrap_or(false);
                commands::playlist_create(name, description, public).await
            }
            "playlist.add" => {
                let id = params
                    .and_then(|p| p.get("id"))
                    .and_then(|v| v.as_str())
                    .unwrap_or("");
                let uris: Vec<String> = params
                    .and_then(|p| p.get("uris"))
                    .and_then(|v| v.as_array())
                    .map(|arr| {
                        arr.iter()
                            .filter_map(|v| v.as_str().map(String::from))
                            .collect()
                    })
                    .unwrap_or_default();
                let now_playing = params
                    .and_then(|p| p.get("now_playing"))
                    .and_then(|v| v.as_bool())
                    .unwrap_or(false);
                let position = params
                    .and_then(|p| p.get("position"))
                    .and_then(|v| v.as_u64())
                    .map(|p| p as u32);
                let dry_run = params
                    .and_then(|p| p.get("dry_run"))
                    .and_then(|v| v.as_bool())
                    .unwrap_or(false);
                commands::playlist_add(id, &uris, now_playing, position, dry_run).await
            }
            "playlist.remove" => {
                let id = params
                    .and_then(|p| p.get("id"))
                    .and_then(|v| v.as_str())
                    .unwrap_or("");
                let uris: Vec<String> = params
                    .and_then(|p| p.get("uris"))
                    .and_then(|v| v.as_array())
                    .map(|arr| {
                        arr.iter()
                            .filter_map(|v| v.as_str().map(String::from))
                            .collect()
                    })
                    .unwrap_or_default();
                let dry_run = params
                    .and_then(|p| p.get("dry_run"))
                    .and_then(|v| v.as_bool())
                    .unwrap_or(false);
                commands::playlist_remove(id, &uris, dry_run).await
            }
            "playlist.edit" => {
                let id = params
                    .and_then(|p| p.get("id"))
                    .and_then(|v| v.as_str())
                    .unwrap_or("");
                let name = params
                    .and_then(|p| p.get("name"))
                    .and_then(|v| v.as_str());
                let description = params
                    .and_then(|p| p.get("description"))
                    .and_then(|v| v.as_str());
                let public = params
                    .and_then(|p| p.get("public"))
                    .and_then(|v| v.as_bool());
                commands::playlist_edit(id, name, description, public).await
            }
            "playlist.reorder" => {
                let id = params
                    .and_then(|p| p.get("id"))
                    .and_then(|v| v.as_str())
                    .unwrap_or("");
                let from = params
                    .and_then(|p| p.get("from"))
                    .and_then(|v| v.as_u64())
                    .unwrap_or(0) as u32;
                let to = params
                    .and_then(|p| p.get("to"))
                    .and_then(|v| v.as_u64())
                    .unwrap_or(0) as u32;
                let count = params
                    .and_then(|p| p.get("count"))
                    .and_then(|v| v.as_u64())
                    .unwrap_or(1) as u32;
                commands::playlist_reorder(id, from, to, count).await
            }
            "playlist.follow" => {
                let id = params
                    .and_then(|p| p.get("id"))
                    .and_then(|v| v.as_str())
                    .unwrap_or("");
                let public = params
                    .and_then(|p| p.get("public"))
                    .and_then(|v| v.as_bool())
                    .unwrap_or(true);
                commands::playlist_follow(id, public).await
            }
            "playlist.unfollow" => {
                let id = params
                    .and_then(|p| p.get("id"))
                    .and_then(|v| v.as_str())
                    .unwrap_or("");
                commands::playlist_unfollow(id).await
            }
            "playlist.duplicate" => {
                let id = params
                    .and_then(|p| p.get("id"))
                    .and_then(|v| v.as_str())
                    .unwrap_or("");
                let name = params
                    .and_then(|p| p.get("name"))
                    .and_then(|v| v.as_str());
                commands::playlist_duplicate(id, name).await
            }
            "playlist.cover" => {
                let id = params
                    .and_then(|p| p.get("id"))
                    .and_then(|v| v.as_str())
                    .unwrap_or("");
                commands::playlist_cover(id).await
            }
            "playlist.user" => {
                let user_id = params
                    .and_then(|p| p.get("user_id"))
                    .and_then(|v| v.as_str())
                    .unwrap_or("");
                commands::playlist_user(user_id).await
            }

            // ============================================================
            // Library
            // ============================================================
            "library.list" => {
                let limit = params
                    .and_then(|p| p.get("limit"))
                    .and_then(|v| v.as_u64())
                    .unwrap_or(20) as u8;
                let offset = params
                    .and_then(|p| p.get("offset"))
                    .and_then(|v| v.as_u64())
                    .unwrap_or(0) as u32;
                commands::library_list(limit, offset).await
            }
            "library.save" => {
                let ids: Vec<String> = params
                    .and_then(|p| p.get("ids"))
                    .and_then(|v| v.as_array())
                    .map(|arr| {
                        arr.iter()
                            .filter_map(|v| v.as_str().map(String::from))
                            .collect()
                    })
                    .unwrap_or_default();
                let now_playing = params
                    .and_then(|p| p.get("now_playing"))
                    .and_then(|v| v.as_bool())
                    .unwrap_or(false);
                let dry_run = params
                    .and_then(|p| p.get("dry_run"))
                    .and_then(|v| v.as_bool())
                    .unwrap_or(false);
                commands::library_save(&ids, now_playing, dry_run).await
            }
            "library.remove" => {
                let ids: Vec<String> = params
                    .and_then(|p| p.get("ids"))
                    .and_then(|v| v.as_array())
                    .map(|arr| {
                        arr.iter()
                            .filter_map(|v| v.as_str().map(String::from))
                            .collect()
                    })
                    .unwrap_or_default();
                let dry_run = params
                    .and_then(|p| p.get("dry_run"))
                    .and_then(|v| v.as_bool())
                    .unwrap_or(false);
                commands::library_remove(&ids, dry_run).await
            }
            "library.check" => {
                let ids: Vec<String> = params
                    .and_then(|p| p.get("ids"))
                    .and_then(|v| v.as_array())
                    .map(|arr| {
                        arr.iter()
                            .filter_map(|v| v.as_str().map(String::from))
                            .collect()
                    })
                    .unwrap_or_default();
                commands::library_check(&ids).await
            }

            // ============================================================
            // Info
            // ============================================================
            "info.track" => {
                let id = params.and_then(|p| p.get("id")).and_then(|v| v.as_str());
                let id_only = params
                    .and_then(|p| p.get("id_only"))
                    .and_then(|v| v.as_bool())
                    .unwrap_or(false);
                commands::info_track(id, id_only).await
            }
            "info.album" => {
                let id = params.and_then(|p| p.get("id")).and_then(|v| v.as_str());
                let id_only = params
                    .and_then(|p| p.get("id_only"))
                    .and_then(|v| v.as_bool())
                    .unwrap_or(false);
                commands::info_album(id, id_only).await
            }
            "info.artist" => {
                let id = params.and_then(|p| p.get("id")).and_then(|v| v.as_str());
                let id_only = params
                    .and_then(|p| p.get("id_only"))
                    .and_then(|v| v.as_bool())
                    .unwrap_or(false);
                let view = params
                    .and_then(|p| p.get("view"))
                    .and_then(|v| v.as_str())
                    .unwrap_or("details");
                let market = params
                    .and_then(|p| p.get("market"))
                    .and_then(|v| v.as_str())
                    .map(String::from)
                    .unwrap_or_default();
                let limit = params
                    .and_then(|p| p.get("limit"))
                    .and_then(|v| v.as_u64())
                    .unwrap_or(20) as u8;
                let offset = params
                    .and_then(|p| p.get("offset"))
                    .and_then(|v| v.as_u64())
                    .unwrap_or(0) as u32;

                let artist_view = match view {
                    "top_tracks" => ArtistView::TopTracks,
                    "albums" => ArtistView::Albums,
                    "related" => ArtistView::Related,
                    _ => ArtistView::Details,
                };

                let query = ArtistQuery::new()
                    .with_id(id.map(String::from))
                    .id_only(id_only)
                    .view(artist_view)
                    .market(market)
                    .paginate(limit, offset);
                commands::info_artist(query).await
            }

            // ============================================================
            // User
            // ============================================================
            "user.profile" => commands::user_profile().await,
            "user.top" => {
                let item_type = params
                    .and_then(|p| p.get("type"))
                    .and_then(|v| v.as_str())
                    .unwrap_or("tracks");
                let range = params
                    .and_then(|p| p.get("range"))
                    .and_then(|v| v.as_str())
                    .unwrap_or("medium");
                let limit = params
                    .and_then(|p| p.get("limit"))
                    .and_then(|v| v.as_u64())
                    .unwrap_or(20) as u8;
                commands::user_top(item_type, range, limit).await
            }
            "user.get" => {
                let user_id = params
                    .and_then(|p| p.get("id"))
                    .and_then(|v| v.as_str())
                    .unwrap_or("");
                commands::user_get(user_id).await
            }

            // ============================================================
            // Show (Podcasts)
            // ============================================================
            "show.get" => {
                let id = params
                    .and_then(|p| p.get("id"))
                    .and_then(|v| v.as_str())
                    .unwrap_or("");
                commands::show_get(id).await
            }
            "show.episodes" => {
                let id = params
                    .and_then(|p| p.get("id"))
                    .and_then(|v| v.as_str())
                    .unwrap_or("");
                let limit = params
                    .and_then(|p| p.get("limit"))
                    .and_then(|v| v.as_u64())
                    .unwrap_or(20) as u8;
                let offset = params
                    .and_then(|p| p.get("offset"))
                    .and_then(|v| v.as_u64())
                    .unwrap_or(0) as u32;
                commands::show_episodes(id, limit, offset).await
            }
            "show.list" => {
                let limit = params
                    .and_then(|p| p.get("limit"))
                    .and_then(|v| v.as_u64())
                    .unwrap_or(20) as u8;
                let offset = params
                    .and_then(|p| p.get("offset"))
                    .and_then(|v| v.as_u64())
                    .unwrap_or(0) as u32;
                commands::show_list(limit, offset).await
            }
            "show.save" => {
                let ids: Vec<String> = params
                    .and_then(|p| p.get("ids"))
                    .and_then(|v| v.as_array())
                    .map(|arr| {
                        arr.iter()
                            .filter_map(|v| v.as_str().map(String::from))
                            .collect()
                    })
                    .unwrap_or_default();
                commands::show_save(&ids).await
            }
            "show.remove" => {
                let ids: Vec<String> = params
                    .and_then(|p| p.get("ids"))
                    .and_then(|v| v.as_array())
                    .map(|arr| {
                        arr.iter()
                            .filter_map(|v| v.as_str().map(String::from))
                            .collect()
                    })
                    .unwrap_or_default();
                commands::show_remove(&ids).await
            }
            "show.check" => {
                let ids: Vec<String> = params
                    .and_then(|p| p.get("ids"))
                    .and_then(|v| v.as_array())
                    .map(|arr| {
                        arr.iter()
                            .filter_map(|v| v.as_str().map(String::from))
                            .collect()
                    })
                    .unwrap_or_default();
                commands::show_check(&ids).await
            }

            // ============================================================
            // Episode
            // ============================================================
            "episode.get" => {
                let id = params
                    .and_then(|p| p.get("id"))
                    .and_then(|v| v.as_str())
                    .unwrap_or("");
                commands::episode_get(id).await
            }
            "episode.list" => {
                let limit = params
                    .and_then(|p| p.get("limit"))
                    .and_then(|v| v.as_u64())
                    .unwrap_or(20) as u8;
                let offset = params
                    .and_then(|p| p.get("offset"))
                    .and_then(|v| v.as_u64())
                    .unwrap_or(0) as u32;
                commands::episode_list(limit, offset).await
            }
            "episode.save" => {
                let ids: Vec<String> = params
                    .and_then(|p| p.get("ids"))
                    .and_then(|v| v.as_array())
                    .map(|arr| {
                        arr.iter()
                            .filter_map(|v| v.as_str().map(String::from))
                            .collect()
                    })
                    .unwrap_or_default();
                commands::episode_save(&ids).await
            }
            "episode.remove" => {
                let ids: Vec<String> = params
                    .and_then(|p| p.get("ids"))
                    .and_then(|v| v.as_array())
                    .map(|arr| {
                        arr.iter()
                            .filter_map(|v| v.as_str().map(String::from))
                            .collect()
                    })
                    .unwrap_or_default();
                commands::episode_remove(&ids).await
            }
            "episode.check" => {
                let ids: Vec<String> = params
                    .and_then(|p| p.get("ids"))
                    .and_then(|v| v.as_array())
                    .map(|arr| {
                        arr.iter()
                            .filter_map(|v| v.as_str().map(String::from))
                            .collect()
                    })
                    .unwrap_or_default();
                commands::episode_check(&ids).await
            }

            // ============================================================
            // Audiobook
            // ============================================================
            "audiobook.get" => {
                let id = params
                    .and_then(|p| p.get("id"))
                    .and_then(|v| v.as_str())
                    .unwrap_or("");
                commands::audiobook_get(id).await
            }
            "audiobook.chapters" => {
                let id = params
                    .and_then(|p| p.get("id"))
                    .and_then(|v| v.as_str())
                    .unwrap_or("");
                let limit = params
                    .and_then(|p| p.get("limit"))
                    .and_then(|v| v.as_u64())
                    .unwrap_or(20) as u8;
                let offset = params
                    .and_then(|p| p.get("offset"))
                    .and_then(|v| v.as_u64())
                    .unwrap_or(0) as u32;
                commands::audiobook_chapters(id, limit, offset).await
            }
            "audiobook.list" => {
                let limit = params
                    .and_then(|p| p.get("limit"))
                    .and_then(|v| v.as_u64())
                    .unwrap_or(20) as u8;
                let offset = params
                    .and_then(|p| p.get("offset"))
                    .and_then(|v| v.as_u64())
                    .unwrap_or(0) as u32;
                commands::audiobook_list(limit, offset).await
            }
            "audiobook.save" => {
                let ids: Vec<String> = params
                    .and_then(|p| p.get("ids"))
                    .and_then(|v| v.as_array())
                    .map(|arr| {
                        arr.iter()
                            .filter_map(|v| v.as_str().map(String::from))
                            .collect()
                    })
                    .unwrap_or_default();
                commands::audiobook_save(&ids).await
            }
            "audiobook.remove" => {
                let ids: Vec<String> = params
                    .and_then(|p| p.get("ids"))
                    .and_then(|v| v.as_array())
                    .map(|arr| {
                        arr.iter()
                            .filter_map(|v| v.as_str().map(String::from))
                            .collect()
                    })
                    .unwrap_or_default();
                commands::audiobook_remove(&ids).await
            }
            "audiobook.check" => {
                let ids: Vec<String> = params
                    .and_then(|p| p.get("ids"))
                    .and_then(|v| v.as_array())
                    .map(|arr| {
                        arr.iter()
                            .filter_map(|v| v.as_str().map(String::from))
                            .collect()
                    })
                    .unwrap_or_default();
                commands::audiobook_check(&ids).await
            }

            // ============================================================
            // Album
            // ============================================================
            "album.list" => {
                let limit = params
                    .and_then(|p| p.get("limit"))
                    .and_then(|v| v.as_u64())
                    .unwrap_or(20) as u8;
                let offset = params
                    .and_then(|p| p.get("offset"))
                    .and_then(|v| v.as_u64())
                    .unwrap_or(0) as u32;
                commands::album_list(limit, offset).await
            }
            "album.tracks" => {
                let id = params
                    .and_then(|p| p.get("id"))
                    .and_then(|v| v.as_str())
                    .unwrap_or("");
                let limit = params
                    .and_then(|p| p.get("limit"))
                    .and_then(|v| v.as_u64())
                    .unwrap_or(20) as u8;
                let offset = params
                    .and_then(|p| p.get("offset"))
                    .and_then(|v| v.as_u64())
                    .unwrap_or(0) as u32;
                commands::album_tracks(id, limit, offset).await
            }
            "album.save" => {
                let ids: Vec<String> = params
                    .and_then(|p| p.get("ids"))
                    .and_then(|v| v.as_array())
                    .map(|arr| {
                        arr.iter()
                            .filter_map(|v| v.as_str().map(String::from))
                            .collect()
                    })
                    .unwrap_or_default();
                commands::album_save(&ids).await
            }
            "album.remove" => {
                let ids: Vec<String> = params
                    .and_then(|p| p.get("ids"))
                    .and_then(|v| v.as_array())
                    .map(|arr| {
                        arr.iter()
                            .filter_map(|v| v.as_str().map(String::from))
                            .collect()
                    })
                    .unwrap_or_default();
                commands::album_remove(&ids).await
            }
            "album.check" => {
                let ids: Vec<String> = params
                    .and_then(|p| p.get("ids"))
                    .and_then(|v| v.as_array())
                    .map(|arr| {
                        arr.iter()
                            .filter_map(|v| v.as_str().map(String::from))
                            .collect()
                    })
                    .unwrap_or_default();
                commands::album_check(&ids).await
            }
            "album.newReleases" => {
                let limit = params
                    .and_then(|p| p.get("limit"))
                    .and_then(|v| v.as_u64())
                    .unwrap_or(20) as u8;
                let offset = params
                    .and_then(|p| p.get("offset"))
                    .and_then(|v| v.as_u64())
                    .unwrap_or(0) as u32;
                commands::album_new_releases(limit, offset).await
            }

            // ============================================================
            // Chapter
            // ============================================================
            "chapter.get" => {
                let id = params
                    .and_then(|p| p.get("id"))
                    .and_then(|v| v.as_str())
                    .unwrap_or("");
                commands::chapter_get(id).await
            }

            // ============================================================
            // Category
            // ============================================================
            "category.list" => {
                let limit = params
                    .and_then(|p| p.get("limit"))
                    .and_then(|v| v.as_u64())
                    .unwrap_or(20) as u8;
                let offset = params
                    .and_then(|p| p.get("offset"))
                    .and_then(|v| v.as_u64())
                    .unwrap_or(0) as u32;
                commands::category_list(limit, offset).await
            }
            "category.get" => {
                let id = params
                    .and_then(|p| p.get("id"))
                    .and_then(|v| v.as_str())
                    .unwrap_or("");
                commands::category_get(id).await
            }
            "category.playlists" => {
                let id = params
                    .and_then(|p| p.get("id"))
                    .and_then(|v| v.as_str())
                    .unwrap_or("");
                let limit = params
                    .and_then(|p| p.get("limit"))
                    .and_then(|v| v.as_u64())
                    .unwrap_or(20) as u8;
                let offset = params
                    .and_then(|p| p.get("offset"))
                    .and_then(|v| v.as_u64())
                    .unwrap_or(0) as u32;
                commands::category_playlists(id, limit, offset).await
            }

            // ============================================================
            // Follow
            // ============================================================
            "follow.artist" => {
                let ids: Vec<String> = params
                    .and_then(|p| p.get("ids"))
                    .and_then(|v| v.as_array())
                    .map(|arr| {
                        arr.iter()
                            .filter_map(|v| v.as_str().map(String::from))
                            .collect()
                    })
                    .unwrap_or_default();
                let dry_run = params
                    .and_then(|p| p.get("dry_run"))
                    .and_then(|v| v.as_bool())
                    .unwrap_or(false);
                commands::follow_artist(&ids, dry_run).await
            }
            "follow.user" => {
                let ids: Vec<String> = params
                    .and_then(|p| p.get("ids"))
                    .and_then(|v| v.as_array())
                    .map(|arr| {
                        arr.iter()
                            .filter_map(|v| v.as_str().map(String::from))
                            .collect()
                    })
                    .unwrap_or_default();
                let dry_run = params
                    .and_then(|p| p.get("dry_run"))
                    .and_then(|v| v.as_bool())
                    .unwrap_or(false);
                commands::follow_user(&ids, dry_run).await
            }
            "follow.unfollowArtist" => {
                let ids: Vec<String> = params
                    .and_then(|p| p.get("ids"))
                    .and_then(|v| v.as_array())
                    .map(|arr| {
                        arr.iter()
                            .filter_map(|v| v.as_str().map(String::from))
                            .collect()
                    })
                    .unwrap_or_default();
                let dry_run = params
                    .and_then(|p| p.get("dry_run"))
                    .and_then(|v| v.as_bool())
                    .unwrap_or(false);
                commands::unfollow_artist(&ids, dry_run).await
            }
            "follow.unfollowUser" => {
                let ids: Vec<String> = params
                    .and_then(|p| p.get("ids"))
                    .and_then(|v| v.as_array())
                    .map(|arr| {
                        arr.iter()
                            .filter_map(|v| v.as_str().map(String::from))
                            .collect()
                    })
                    .unwrap_or_default();
                let dry_run = params
                    .and_then(|p| p.get("dry_run"))
                    .and_then(|v| v.as_bool())
                    .unwrap_or(false);
                commands::unfollow_user(&ids, dry_run).await
            }
            "follow.list" => {
                let limit = params
                    .and_then(|p| p.get("limit"))
                    .and_then(|v| v.as_u64())
                    .unwrap_or(20) as u8;
                commands::follow_list(limit).await
            }
            "follow.checkArtist" => {
                let ids: Vec<String> = params
                    .and_then(|p| p.get("ids"))
                    .and_then(|v| v.as_array())
                    .map(|arr| {
                        arr.iter()
                            .filter_map(|v| v.as_str().map(String::from))
                            .collect()
                    })
                    .unwrap_or_default();
                commands::follow_check_artist(&ids).await
            }
            "follow.checkUser" => {
                let ids: Vec<String> = params
                    .and_then(|p| p.get("ids"))
                    .and_then(|v| v.as_array())
                    .map(|arr| {
                        arr.iter()
                            .filter_map(|v| v.as_str().map(String::from))
                            .collect()
                    })
                    .unwrap_or_default();
                commands::follow_check_user(&ids).await
            }

            // ============================================================
            // Markets
            // ============================================================
            "markets.list" => commands::markets_list().await,

            // ============================================================
            // Unknown method
            // ============================================================
            _ => Response::err(
                404, // Use HTTP-style code; JSON-RPC error code added in from_response
                &format!("Method not found: {}", request.method),
                ErrorKind::Validation,
            ),
        }
    }
}

impl Default for Dispatcher {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rpc::protocol::RpcRequest;

    fn make_request(method: &str, params: Option<serde_json::Value>) -> RpcRequest {
        RpcRequest {
            jsonrpc: "2.0".to_string(),
            method: method.to_string(),
            params,
            id: Some(serde_json::json!(1)),
        }
    }

    #[tokio::test]
    async fn test_ping() {
        let dispatcher = Dispatcher::new();
        let req = make_request("ping", None);
        let resp = dispatcher.dispatch(&req).await;
        assert_eq!(resp.message, "pong");
    }

    #[tokio::test]
    async fn test_version() {
        let dispatcher = Dispatcher::new();
        let req = make_request("version", None);
        let resp = dispatcher.dispatch(&req).await;
        assert!(resp.payload.is_some());
        let payload = resp.payload.unwrap();
        assert!(payload.get("version").is_some());
        assert!(payload.get("name").is_some());
    }

    #[tokio::test]
    async fn test_unknown_method() {
        let dispatcher = Dispatcher::new();
        let req = make_request("unknown.method", None);
        let resp = dispatcher.dispatch(&req).await;
        assert_eq!(resp.code, 404);
        assert!(resp.message.contains("Method not found"));
    }

    #[tokio::test]
    async fn test_player_transfer_missing_device() {
        let dispatcher = Dispatcher::new();
        let req = make_request("player.transfer", None);
        let resp = dispatcher.dispatch(&req).await;
        assert_eq!(resp.code, 400);
        assert!(resp.message.contains("device"));
    }

    #[test]
    fn test_dispatcher_default() {
        let _dispatcher = Dispatcher::default();
    }

}
