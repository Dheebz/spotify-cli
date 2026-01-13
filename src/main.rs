pub mod cli;
pub mod endpoints;
pub mod http;
pub mod io;
pub mod oauth;
pub mod storage;

use clap::Parser;
use cli::{
    AudiobookCommand, AuthCommand, CategoryCommand, ChapterCommand, Cli, Command, DevicesCommand,
    EpisodeCommand, InfoCommand, LibraryCommand, PinCommand, PlaylistCommand, PlayerCommand,
    QueueCommand, ShowCommand, UserCommand,
};
use io::output::{print_human, print_json};

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    let response = match cli.command {
        Command::Auth { command } => match command {
            AuthCommand::Login { force } => cli::commands::auth_login(force).await,
            AuthCommand::Logout => cli::commands::auth_logout().await,
            AuthCommand::Refresh => cli::commands::auth_refresh().await,
            AuthCommand::Status => cli::commands::auth_status().await,
        },
        Command::Player { command } => match command {
            PlayerCommand::Next => cli::commands::player_next().await,
            PlayerCommand::Previous => cli::commands::player_previous().await,
            PlayerCommand::Toggle => cli::commands::player_toggle().await,
            PlayerCommand::Play { uri, pin } => {
                cli::commands::player_play(uri.as_deref(), pin.as_deref()).await
            }
            PlayerCommand::Pause => cli::commands::player_pause().await,
            PlayerCommand::Status { id_only } => {
                cli::commands::player_status(id_only.as_deref()).await
            }
            PlayerCommand::Devices { command } => match command {
                DevicesCommand::List => cli::commands::player_devices_list().await,
                DevicesCommand::Transfer { device } => {
                    cli::commands::player_devices_transfer(&device).await
                }
            },
            PlayerCommand::Queue { command } => match command {
                QueueCommand::List => cli::commands::player_queue_list().await,
                QueueCommand::Add { uri, now_playing } => {
                    cli::commands::player_queue_add(uri.as_deref(), now_playing).await
                }
            },
            PlayerCommand::Seek { position } => {
                cli::commands::player_seek(&position).await
            }
            PlayerCommand::Repeat { mode } => cli::commands::player_repeat(&mode).await,
            PlayerCommand::Volume { percent } => cli::commands::player_volume(percent).await,
            PlayerCommand::Shuffle { state } => cli::commands::player_shuffle(&state).await,
            PlayerCommand::Recent => cli::commands::player_recent().await,
        },
        Command::Pin { command } => match command {
            PinCommand::Add {
                resource_type,
                url_or_id,
                alias,
                tags,
            } => cli::commands::pin_add(&resource_type, &url_or_id, &alias, tags.as_deref()).await,
            PinCommand::Remove { alias_or_id } => {
                cli::commands::pin_remove(&alias_or_id).await
            }
            PinCommand::List { resource_type } => {
                cli::commands::pin_list(resource_type.as_deref()).await
            }
        },
        Command::Search {
            query,
            types,
            limit,
            pins_only,
            exact,
            artist,
            album,
            track,
            year,
            genre,
            isrc,
            upc,
            new,
            hipster,
            play,
        } => {
            let filters = cli::commands::SearchFilters {
                artist,
                album,
                track,
                year,
                genre,
                isrc,
                upc,
                new,
                hipster,
            };
            cli::commands::search_command(&query, &types, limit, pins_only, exact, filters, play).await
        }
        Command::Playlist { command } => match command {
            PlaylistCommand::List { limit, offset } => {
                cli::commands::playlist_list(limit, offset).await
            }
            PlaylistCommand::Get { playlist } => cli::commands::playlist_get(&playlist).await,
            PlaylistCommand::Create {
                name,
                description,
                public,
            } => cli::commands::playlist_create(&name, description.as_deref(), public).await,
            PlaylistCommand::Add {
                playlist,
                uris,
                now_playing,
                position,
            } => cli::commands::playlist_add(&playlist, &uris, now_playing, position).await,
            PlaylistCommand::Remove { playlist, uris } => {
                cli::commands::playlist_remove(&playlist, &uris).await
            }
            PlaylistCommand::Edit {
                playlist,
                name,
                description,
                public,
                private,
            } => {
                let visibility = if public {
                    Some(true)
                } else if private {
                    Some(false)
                } else {
                    None
                };
                cli::commands::playlist_edit(
                    &playlist,
                    name.as_deref(),
                    description.as_deref(),
                    visibility,
                )
                .await
            }
            PlaylistCommand::Reorder {
                playlist,
                from,
                to,
                count,
            } => cli::commands::playlist_reorder(&playlist, from, to, count).await,
            PlaylistCommand::Follow { playlist, public } => {
                cli::commands::playlist_follow(&playlist, public).await
            }
            PlaylistCommand::Unfollow { playlist } => {
                cli::commands::playlist_unfollow(&playlist).await
            }
            PlaylistCommand::Duplicate { playlist, name } => {
                cli::commands::playlist_duplicate(&playlist, name.as_deref()).await
            }
        },
        Command::Library { command } => match command {
            LibraryCommand::List { limit, offset } => {
                cli::commands::library_list(limit, offset).await
            }
            LibraryCommand::Save { ids, now_playing } => {
                cli::commands::library_save(&ids, now_playing).await
            }
            LibraryCommand::Remove { ids } => cli::commands::library_remove(&ids).await,
            LibraryCommand::Check { ids } => cli::commands::library_check(&ids).await,
        },
        Command::Info { command } => match command {
            InfoCommand::Track { id, id_only } => {
                cli::commands::info_track(id.as_deref(), id_only).await
            }
            InfoCommand::Album { id, id_only } => {
                cli::commands::info_album(id.as_deref(), id_only).await
            }
            InfoCommand::Artist {
                id,
                id_only,
                top_tracks,
                market,
            } => cli::commands::info_artist(id.as_deref(), id_only, top_tracks, &market).await,
        },
        Command::User { command } => match command {
            UserCommand::Profile => cli::commands::user_profile().await,
            UserCommand::Top {
                item_type,
                range,
                limit,
            } => cli::commands::user_top(&item_type, &range, limit).await,
        },
        Command::Show { command } => match command {
            ShowCommand::Get { id } => cli::commands::show_get(&id).await,
            ShowCommand::Episodes { id, limit, offset } => {
                cli::commands::show_episodes(&id, limit, offset).await
            }
            ShowCommand::List { limit, offset } => cli::commands::show_list(limit, offset).await,
            ShowCommand::Save { ids } => cli::commands::show_save(&ids).await,
            ShowCommand::Remove { ids } => cli::commands::show_remove(&ids).await,
            ShowCommand::Check { ids } => cli::commands::show_check(&ids).await,
        },
        Command::Episode { command } => match command {
            EpisodeCommand::Get { id } => cli::commands::episode_get(&id).await,
            EpisodeCommand::List { limit, offset } => {
                cli::commands::episode_list(limit, offset).await
            }
            EpisodeCommand::Save { ids } => cli::commands::episode_save(&ids).await,
            EpisodeCommand::Remove { ids } => cli::commands::episode_remove(&ids).await,
            EpisodeCommand::Check { ids } => cli::commands::episode_check(&ids).await,
        },
        Command::Audiobook { command } => match command {
            AudiobookCommand::Get { id } => cli::commands::audiobook_get(&id).await,
            AudiobookCommand::Chapters { id, limit, offset } => {
                cli::commands::audiobook_chapters(&id, limit, offset).await
            }
            AudiobookCommand::List { limit, offset } => {
                cli::commands::audiobook_list(limit, offset).await
            }
            AudiobookCommand::Save { ids } => cli::commands::audiobook_save(&ids).await,
            AudiobookCommand::Remove { ids } => cli::commands::audiobook_remove(&ids).await,
            AudiobookCommand::Check { ids } => cli::commands::audiobook_check(&ids).await,
        },
        Command::Chapter { command } => match command {
            ChapterCommand::Get { id } => cli::commands::chapter_get(&id).await,
        },
        Command::Category { command } => match command {
            CategoryCommand::List { limit, offset } => {
                cli::commands::category_list(limit, offset).await
            }
            CategoryCommand::Get { id } => cli::commands::category_get(&id).await,
        },
    };

    if cli.json {
        print_json(&response);
    } else {
        print_human(&response);
    }
}
