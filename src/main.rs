use clap::Parser;
use spotify_cli::cli::{
    print_completions, AlbumCommand, AudiobookCommand, AuthCommand, CategoryCommand,
    ChapterCommand, Cli, Command, DevicesCommand, EpisodeCommand, FollowCommand, InfoCommand,
    LibraryCommand, PinCommand, PlaylistCommand, PlayerCommand, QueueCommand, ShowCommand,
    UserCommand,
};
use spotify_cli::io::output::{print_human, print_json};
use spotify_cli::cli::commands;
use tracing_subscriber::EnvFilter;

fn init_logging(verbose: u8) {
    let filter = match verbose {
        0 => EnvFilter::new("warn"),
        1 => EnvFilter::new("info"),
        2 => EnvFilter::new("debug"),
        _ => EnvFilter::new("trace"),
    };

    tracing_subscriber::fmt()
        .with_env_filter(filter)
        .with_target(false)
        .without_time()
        .init();
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    init_logging(cli.verbose);

    let response = match cli.command {
        Command::Auth { command } => match command {
            AuthCommand::Login { force } => commands::auth_login(force).await,
            AuthCommand::Logout => commands::auth_logout().await,
            AuthCommand::Refresh => commands::auth_refresh().await,
            AuthCommand::Status => commands::auth_status().await,
        },
        Command::Player { command } => match command {
            PlayerCommand::Next => commands::player_next().await,
            PlayerCommand::Previous => commands::player_previous().await,
            PlayerCommand::Toggle => commands::player_toggle().await,
            PlayerCommand::Play { uri, pin } => {
                commands::player_play(uri.as_deref(), pin.as_deref()).await
            }
            PlayerCommand::Pause => commands::player_pause().await,
            PlayerCommand::Status { id_only } => {
                commands::player_status(id_only.as_deref()).await
            }
            PlayerCommand::Devices { command } => match command {
                DevicesCommand::List => commands::player_devices_list().await,
                DevicesCommand::Transfer { device } => {
                    commands::player_devices_transfer(&device).await
                }
            },
            PlayerCommand::Queue { command } => match command {
                QueueCommand::List => commands::player_queue_list().await,
                QueueCommand::Add { uri, now_playing } => {
                    commands::player_queue_add(uri.as_deref(), now_playing).await
                }
            },
            PlayerCommand::Seek { position } => {
                commands::player_seek(&position).await
            }
            PlayerCommand::Repeat { mode } => commands::player_repeat(&mode).await,
            PlayerCommand::Volume { percent } => commands::player_volume(percent).await,
            PlayerCommand::Shuffle { state } => commands::player_shuffle(&state).await,
            PlayerCommand::Recent => commands::player_recent().await,
        },
        Command::Pin { command } => match command {
            PinCommand::Add {
                resource_type,
                url_or_id,
                alias,
                tags,
            } => commands::pin_add(&resource_type, &url_or_id, &alias, tags.as_deref()).await,
            PinCommand::Remove { alias_or_id } => {
                commands::pin_remove(&alias_or_id).await
            }
            PinCommand::List { resource_type } => {
                commands::pin_list(resource_type.as_deref()).await
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
            let filters = commands::SearchFilters {
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
            commands::search_command(&query, &types, limit, pins_only, exact, filters, play).await
        }
        Command::Playlist { command } => match command {
            PlaylistCommand::List { limit, offset } => {
                commands::playlist_list(limit, offset).await
            }
            PlaylistCommand::Get { playlist } => commands::playlist_get(&playlist).await,
            PlaylistCommand::Create {
                name,
                description,
                public,
            } => commands::playlist_create(&name, description.as_deref(), public).await,
            PlaylistCommand::Add {
                playlist,
                uris,
                now_playing,
                position,
                dry_run,
            } => commands::playlist_add(&playlist, &uris, now_playing, position, dry_run).await,
            PlaylistCommand::Remove { playlist, uris, dry_run } => {
                commands::playlist_remove(&playlist, &uris, dry_run).await
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
                commands::playlist_edit(
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
            } => commands::playlist_reorder(&playlist, from, to, count).await,
            PlaylistCommand::Follow { playlist, public } => {
                commands::playlist_follow(&playlist, public).await
            }
            PlaylistCommand::Unfollow { playlist } => {
                commands::playlist_unfollow(&playlist).await
            }
            PlaylistCommand::Duplicate { playlist, name } => {
                commands::playlist_duplicate(&playlist, name.as_deref()).await
            }
            PlaylistCommand::Featured { limit, offset } => {
                commands::playlist_featured(limit, offset).await
            }
            PlaylistCommand::Cover { playlist } => {
                commands::playlist_cover(&playlist).await
            }
            PlaylistCommand::User { user_id } => {
                commands::playlist_user(&user_id).await
            }
        },
        Command::Library { command } => match command {
            LibraryCommand::List { limit, offset } => {
                commands::library_list(limit, offset).await
            }
            LibraryCommand::Save { ids, now_playing, dry_run } => {
                commands::library_save(&ids, now_playing, dry_run).await
            }
            LibraryCommand::Remove { ids, dry_run } => commands::library_remove(&ids, dry_run).await,
            LibraryCommand::Check { ids } => commands::library_check(&ids).await,
        },
        Command::Info { command } => match command {
            InfoCommand::Track { id, id_only } => {
                commands::info_track(id.as_deref(), id_only).await
            }
            InfoCommand::Album { id, id_only } => {
                commands::info_album(id.as_deref(), id_only).await
            }
            InfoCommand::Artist {
                id,
                id_only,
                top_tracks,
                albums,
                related,
                market,
                limit,
                offset,
            } => commands::info_artist(id.as_deref(), id_only, top_tracks, albums, related, &market, limit, offset).await,
        },
        Command::User { command } => match command {
            UserCommand::Profile => commands::user_profile().await,
            UserCommand::Top {
                item_type,
                range,
                limit,
            } => commands::user_top(&item_type, &range, limit).await,
            UserCommand::Get { user_id } => commands::user_get(&user_id).await,
        },
        Command::Show { command } => match command {
            ShowCommand::Get { id } => commands::show_get(&id).await,
            ShowCommand::Episodes { id, limit, offset } => {
                commands::show_episodes(&id, limit, offset).await
            }
            ShowCommand::List { limit, offset } => commands::show_list(limit, offset).await,
            ShowCommand::Save { ids } => commands::show_save(&ids).await,
            ShowCommand::Remove { ids } => commands::show_remove(&ids).await,
            ShowCommand::Check { ids } => commands::show_check(&ids).await,
        },
        Command::Episode { command } => match command {
            EpisodeCommand::Get { id } => commands::episode_get(&id).await,
            EpisodeCommand::List { limit, offset } => {
                commands::episode_list(limit, offset).await
            }
            EpisodeCommand::Save { ids } => commands::episode_save(&ids).await,
            EpisodeCommand::Remove { ids } => commands::episode_remove(&ids).await,
            EpisodeCommand::Check { ids } => commands::episode_check(&ids).await,
        },
        Command::Audiobook { command } => match command {
            AudiobookCommand::Get { id } => commands::audiobook_get(&id).await,
            AudiobookCommand::Chapters { id, limit, offset } => {
                commands::audiobook_chapters(&id, limit, offset).await
            }
            AudiobookCommand::List { limit, offset } => {
                commands::audiobook_list(limit, offset).await
            }
            AudiobookCommand::Save { ids } => commands::audiobook_save(&ids).await,
            AudiobookCommand::Remove { ids } => commands::audiobook_remove(&ids).await,
            AudiobookCommand::Check { ids } => commands::audiobook_check(&ids).await,
        },
        Command::Album { command } => match command {
            AlbumCommand::List { limit, offset } => {
                commands::album_list(limit, offset).await
            }
            AlbumCommand::Tracks { id, limit, offset } => {
                commands::album_tracks(&id, limit, offset).await
            }
            AlbumCommand::Save { ids } => commands::album_save(&ids).await,
            AlbumCommand::Remove { ids } => commands::album_remove(&ids).await,
            AlbumCommand::Check { ids } => commands::album_check(&ids).await,
            AlbumCommand::NewReleases { limit, offset } => {
                commands::album_new_releases(limit, offset).await
            }
        },
        Command::Chapter { command } => match command {
            ChapterCommand::Get { id } => commands::chapter_get(&id).await,
        },
        Command::Category { command } => match command {
            CategoryCommand::List { limit, offset } => {
                commands::category_list(limit, offset).await
            }
            CategoryCommand::Get { id } => commands::category_get(&id).await,
            CategoryCommand::Playlists { id, limit, offset } => {
                commands::category_playlists(&id, limit, offset).await
            }
        },
        Command::Follow { command } => match command {
            FollowCommand::Artist { ids, dry_run } => commands::follow_artist(&ids, dry_run).await,
            FollowCommand::User { ids, dry_run } => commands::follow_user(&ids, dry_run).await,
            FollowCommand::UnfollowArtist { ids, dry_run } => commands::unfollow_artist(&ids, dry_run).await,
            FollowCommand::UnfollowUser { ids, dry_run } => commands::unfollow_user(&ids, dry_run).await,
            FollowCommand::List { limit } => commands::follow_list(limit).await,
            FollowCommand::CheckArtist { ids } => commands::follow_check_artist(&ids).await,
            FollowCommand::CheckUser { ids } => commands::follow_check_user(&ids).await,
        },
        Command::Markets => commands::markets_list().await,
        Command::Completions { shell } => {
            print_completions(shell);
            return;
        }
    };

    if cli.json {
        print_json(&response);
    } else {
        print_human(&response);
    }
}
