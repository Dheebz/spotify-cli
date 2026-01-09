//! Info command handlers.
use anyhow::bail;
use clap::{Args, ValueEnum};

use crate::AppContext;
use crate::cli::now_playing;
use crate::cli::playlist;
use crate::cli::search::{apply_fuzzy_scores, fuzzy_query, pick_best_match};
use crate::domain::search::{SearchItem, SearchResults, SearchType};
use crate::error::Result;

#[derive(Args, Debug)]
pub struct InfoCommand {
    #[arg(value_enum, help = "Info type")]
    kind: Option<InfoTypeArg>,
    #[arg(value_name = "QUERY")]
    query: Option<String>,
    #[arg(long, help = "Use market from token")]
    user: bool,
    #[arg(long, help = "Pick a specific result (1-based)")]
    pick: Option<usize>,
    #[arg(long, help = "Use the last cached search results")]
    last: bool,
    #[arg(long, help = "Play the best match result")]
    play: bool,
}

#[derive(ValueEnum, Debug, Clone, Copy)]
enum InfoTypeArg {
    Album,
    Artist,
    Track,
    Playlist,
}

pub fn handle(command: InfoCommand, ctx: &AppContext) -> Result<()> {
    let Some(kind) = command.kind else {
        let Some(query) = command.query else {
            bail!("missing info target; pass a type or query");
        };
        return info_any(ctx, &query, command.user, command.pick, command.play);
    };

    match kind {
        InfoTypeArg::Album => info_album(ctx, &command, command.play),
        InfoTypeArg::Artist => info_artist(ctx, &command, command.play),
        InfoTypeArg::Track => info_track(ctx, &command, command.play),
        InfoTypeArg::Playlist => info_playlist(ctx, &command, command.play),
    }
}

fn info_any(
    ctx: &AppContext,
    query: &str,
    user: bool,
    pick: Option<usize>,
    play: bool,
) -> Result<()> {
    let limit = pick.map(|_| 10).unwrap_or(10);
    let search_query = fuzzy_query(query);
    let mut results =
        ctx.spotify()?
            .search()
            .search(&search_query, SearchType::All, limit, user)?;

    apply_fuzzy_scores(query, &mut results);

    let item = if let Some(pick) = pick {
        pick_item(&results.items, pick)?
    } else {
        let owner_name = ctx.auth.user_name().ok().flatten();
        pick_best_match(&results, query, owner_name.as_deref())
    };

    let Some(item) = item else {
        bail!("no results found");
    };

    if play {
        play_item(ctx, &item)?;
        now_playing::show_with_delay(ctx, 100)?;
    }
    dispatch_item(ctx, item)
}

fn info_album(ctx: &AppContext, command: &InfoCommand, play: bool) -> Result<()> {
    if command.query.is_none() && !command.last {
        let status = ctx.spotify()?.playback().status()?;
        let Some(track) = status.track else {
            bail!("no track is currently playing");
        };
        let Some(album_id) = track.album_id else {
            bail!("current track has no album id; pass an album query");
        };
        let album = ctx.spotify()?.albums().get(&album_id)?;
        if play {
            ctx.spotify()?.playback().play_context(&album.uri)?;
            now_playing::show_with_delay(ctx, 100)?;
        }
        return ctx.output.album_info(album);
    }

    let item = resolve_item(
        ctx,
        SearchType::Album,
        command.query.as_deref(),
        command.last,
        command.user,
        command.pick,
    )?;
    let album = ctx.spotify()?.albums().get(&item.id)?;
    if play {
        ctx.spotify()?.playback().play_context(&item.uri)?;
        now_playing::show_with_delay(ctx, 100)?;
    }
    ctx.output.album_info(album)
}

fn info_artist(ctx: &AppContext, command: &InfoCommand, play: bool) -> Result<()> {
    if command.query.is_none() && !command.last {
        let status = ctx.spotify()?.playback().status()?;
        let Some(track) = status.track else {
            bail!("no track is currently playing");
        };
        let Some(artist_id) = track.artist_ids.first() else {
            bail!("current track has no artist id; pass an artist query");
        };
        let artist = ctx.spotify()?.artists().get(artist_id)?;
        if play {
            ctx.spotify()?.playback().play_context(&artist.uri)?;
            now_playing::show_with_delay(ctx, 100)?;
        }
        return ctx.output.artist_info(artist);
    }

    let item = resolve_item(
        ctx,
        SearchType::Artist,
        command.query.as_deref(),
        command.last,
        command.user,
        command.pick,
    )?;
    let artist = ctx.spotify()?.artists().get(&item.id)?;
    if play {
        ctx.spotify()?.playback().play_context(&item.uri)?;
        now_playing::show_with_delay(ctx, 100)?;
    }
    ctx.output.artist_info(artist)
}

fn info_playlist(ctx: &AppContext, command: &InfoCommand, play: bool) -> Result<()> {
    if command.query.is_none() && !command.last {
        let status = ctx.spotify()?.playback().status()?;
        let Some(context) = status.context else {
            bail!("no playlist context is active; pass a playlist query");
        };
        if context.kind != "playlist" {
            bail!("no playlist context is active; pass a playlist query");
        }
        let Some(id) = playlist::parse_playlist_id(&context.uri) else {
            bail!("unable to parse playlist context uri");
        };
        let playlist_detail = ctx.spotify()?.playlists().get(&id)?;
        if play {
            ctx.spotify()?.playback().play_context(&context.uri)?;
            now_playing::show_with_delay(ctx, 100)?;
        }
        return ctx.output.playlist_info(playlist_detail);
    }

    let item = resolve_item(
        ctx,
        SearchType::Playlist,
        command.query.as_deref(),
        command.last,
        command.user,
        command.pick,
    )?;
    let playlist_detail = ctx.spotify()?.playlists().get(&item.id)?;
    if play {
        ctx.spotify()?.playback().play_context(&item.uri)?;
        now_playing::show_with_delay(ctx, 100)?;
    }
    ctx.output.playlist_info(playlist_detail)
}

fn info_track(ctx: &AppContext, command: &InfoCommand, play: bool) -> Result<()> {
    if command.query.is_none() && !command.last {
        let status = ctx.spotify()?.playback().status()?;
        if play && let Some(track) = status.track.as_ref() {
            let uri = format!("spotify:track:{}", track.id);
            ctx.spotify()?.playback().play_track(&uri)?;
            now_playing::show_with_delay(ctx, 100)?;
        }
        return ctx.output.player_status(status);
    }

    let item = resolve_item(
        ctx,
        SearchType::Track,
        command.query.as_deref(),
        command.last,
        command.user,
        command.pick,
    )?;
    if play {
        ctx.spotify()?.playback().play_track(&item.uri)?;
        now_playing::show_with_delay(ctx, 100)?;
    }
    ctx.output.search_results(SearchResults {
        kind: SearchType::Track,
        items: vec![item],
    })
}

fn resolve_item(
    ctx: &AppContext,
    kind: SearchType,
    query: Option<&str>,
    last: bool,
    user: bool,
    pick: Option<usize>,
) -> Result<SearchItem> {
    let (query_text, results) = if last {
        let cached = ctx.cache.search_store().load()?;
        let Some(cached) = cached else {
            bail!("no cached search; run `spotify-cli search <query>`");
        };
        if cached.results.kind != kind {
            bail!(
                "cached search is {}; run `spotify-cli search {} <query>`",
                search_type_label(cached.results.kind),
                search_type_label(kind)
            );
        }
        (cached.query, cached.results)
    } else {
        let Some(query) = query else {
            bail!("missing query; use --last to reuse cached search results");
        };
        let limit = pick.map(|_| 10).unwrap_or(10);
        let search_query = fuzzy_query(query);
        let results = ctx
            .spotify()?
            .search()
            .search(&search_query, kind, limit, user)?;
        (query.to_string(), results)
    };

    let mut results = results;
    apply_fuzzy_scores(&query_text, &mut results);

    let item = if let Some(pick) = pick {
        pick_item(&results.items, pick)?
    } else {
        let owner_name = ctx.auth.user_name().ok().flatten();
        pick_best_match(&results, &query_text, owner_name.as_deref())
    };
    let Some(item) = item else {
        bail!("no results found");
    };

    Ok(item)
}

fn play_item(ctx: &AppContext, item: &SearchItem) -> Result<()> {
    let playback = ctx.spotify()?.playback();
    match item.kind {
        SearchType::Track => playback.play_track(&item.uri)?,
        SearchType::Album | SearchType::Artist | SearchType::Playlist => {
            playback.play_context(&item.uri)?
        }
        SearchType::All => {}
    }
    Ok(())
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

fn dispatch_item(ctx: &AppContext, item: SearchItem) -> Result<()> {
    match item.kind {
        SearchType::Album => {
            let album = ctx.spotify()?.albums().get(&item.id)?;
            ctx.output.album_info(album)
        }
        SearchType::Artist => {
            let artist = ctx.spotify()?.artists().get(&item.id)?;
            ctx.output.artist_info(artist)
        }
        SearchType::Playlist => {
            let playlist = ctx.spotify()?.playlists().get(&item.id)?;
            ctx.output.playlist_info(playlist)
        }
        SearchType::Track => ctx.output.search_results(SearchResults {
            kind: SearchType::Track,
            items: vec![item],
        }),
        SearchType::All => Ok(()),
    }
}

fn pick_item(items: &[SearchItem], pick: usize) -> Result<Option<SearchItem>> {
    if pick == 0 {
        bail!("pick must be 1 or greater");
    }
    let index = pick - 1;
    Ok(items.get(index).cloned())
}
