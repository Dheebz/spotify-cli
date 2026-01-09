//! Search command handlers.
use anyhow::bail;
use clap::{Args, ValueEnum};

use crate::AppContext;
use crate::cli::now_playing;
use crate::cli::playlist::parse_playlist_id;
use crate::domain::search::{SearchItem, SearchResults, SearchType};
use crate::error::Result;

#[derive(Args, Debug)]
pub struct SearchCommand {
    #[arg(value_enum, help = "Search type")]
    search_type: Option<SearchTypeArg>,
    #[arg(value_name = "QUERY")]
    pub query: Option<String>,
    #[arg(long, help = "Use market from token")]
    user: bool,
    #[arg(long, default_value_t = 10, help = "Limit results")]
    limit: u32,
    #[arg(long, help = "Pick a specific result (1-based)")]
    pick: Option<usize>,
    #[arg(long, help = "Use the last cached search results")]
    last: bool,
    #[arg(long, help = "Play the best match result")]
    play: bool,
}

#[derive(Args, Debug, Clone)]
pub struct SearchArgs {
    pub query: Option<String>,
    #[arg(long, help = "Use market from token")]
    user: bool,
    #[arg(long, default_value_t = 10, help = "Limit results")]
    limit: u32,
    #[arg(long, help = "Pick a specific result (1-based)")]
    pick: Option<usize>,
    #[arg(long, help = "Use the last cached search results")]
    last: bool,
    #[arg(long, help = "Play the best match result")]
    play: bool,
}

#[derive(ValueEnum, Debug, Clone, Copy)]
enum SearchTypeArg {
    All,
    Track,
    Album,
    Artist,
    Playlist,
}

pub fn handle(command: SearchCommand, ctx: &AppContext) -> Result<()> {
    let kind = match command.search_type.unwrap_or(SearchTypeArg::All) {
        SearchTypeArg::All => SearchType::All,
        SearchTypeArg::Track => SearchType::Track,
        SearchTypeArg::Album => SearchType::Album,
        SearchTypeArg::Artist => SearchType::Artist,
        SearchTypeArg::Playlist => SearchType::Playlist,
    };
    let args = SearchArgs {
        query: command.query,
        user: command.user,
        limit: command.limit,
        pick: command.pick,
        last: command.last,
        play: command.play,
    };
    handle_inner(kind, args, ctx, false)
}

fn handle_inner(
    kind: SearchType,
    command: SearchArgs,
    ctx: &AppContext,
    enforce_kind: bool,
) -> Result<()> {
    let (raw_query, mut results) = if command.last || command.query.is_none() {
        let cached = ctx.cache.search_store().load()?;
        let Some(cached) = cached else {
            bail!("no cached search; run `spotify-cli search <query>`");
        };
        if enforce_kind && cached.results.kind != kind {
            bail!(
                "cached search is {}; run `spotify-cli search {} <query>`",
                search_type_label(cached.results.kind),
                search_type_label(kind)
            );
        }
        (cached.query, cached.results)
    } else {
        let raw_query = command.query.clone().unwrap_or_default();
        if kind == SearchType::Playlist && command.user {
            if let Some(results) = local_playlist_results(ctx, &raw_query, command.limit)? {
                let cached = crate::cache::search::CachedSearch {
                    query: raw_query.clone(),
                    results: results.clone(),
                };
                ctx.cache.search_store().save(&cached)?;
                (raw_query, results)
            } else {
                let query = fuzzy_query(&raw_query);
                let results =
                    ctx.spotify()?
                        .search()
                        .search(&query, kind, command.limit, command.user)?;
                let cached = crate::cache::search::CachedSearch {
                    query: raw_query.clone(),
                    results: results.clone(),
                };
                ctx.cache.search_store().save(&cached)?;
                (raw_query, results)
            }
        } else {
            let query = fuzzy_query(&raw_query);
            let results =
                ctx.spotify()?
                    .search()
                    .search(&query, kind, command.limit, command.user)?;
            let cached = crate::cache::search::CachedSearch {
                query: raw_query.clone(),
                results: results.clone(),
            };
            ctx.cache.search_store().save(&cached)?;
            (raw_query, results)
        }
    };

    if !raw_query.is_empty() {
        apply_fuzzy_scores(&raw_query, &mut results);
    }

    let picked = if let Some(pick) = command.pick {
        validate_pick(pick, results.items.len())?;
        pick_item(&results.items, pick)?
    } else if command.play {
        let owner_name = ctx.auth.user_name().ok().flatten();
        pick_best_match(&results, &raw_query, owner_name.as_deref())
    } else {
        None
    };

    if let Some(item) = picked.clone() {
        if command.play {
            let playback = ctx.spotify()?.playback();
            let kind = if results.kind == SearchType::All {
                item.kind
            } else {
                results.kind
            };
            match kind {
                SearchType::Track => playback.play_track(&item.uri)?,
                SearchType::Album | SearchType::Artist | SearchType::Playlist => {
                    playback.play_context(&item.uri)?
                }
                SearchType::All => {}
            }
            let label = search_item_label(&item);
            let message = format!("Playing: {}", label);
            ctx.output.action("search_play", &message)?;
            now_playing::show_with_delay(ctx, 100)?;
        }
    } else if command.play {
        bail!("no results to play");
    }

    if let Some(pick) = command.pick
        && let Some(item) = pick_item(&results.items, pick)?
    {
        results.items = vec![item];
    }

    ctx.output.search_results(results)
}

pub(crate) fn fuzzy_query(query: &str) -> String {
    let tokens: Vec<String> = query
        .split_whitespace()
        .map(|token| format!("*{}*", token))
        .collect();
    if tokens.is_empty() {
        query.to_string()
    } else {
        tokens.join(" ")
    }
}

fn search_item_label(item: &crate::domain::search::SearchItem) -> String {
    if !item.artists.is_empty() {
        return format!("{} - {}", item.name, item.artists.join(", "));
    }

    if let Some(owner) = item.owner.as_ref() {
        return format!("{} ({})", item.name, owner);
    }

    item.name.clone()
}

fn pick_item(
    items: &[crate::domain::search::SearchItem],
    pick: usize,
) -> Result<Option<crate::domain::search::SearchItem>> {
    if pick == 0 {
        bail!("pick must be 1 or greater");
    }
    let index = pick - 1;
    Ok(items.get(index).cloned())
}

fn validate_pick(pick: usize, len: usize) -> Result<()> {
    if pick == 0 {
        bail!("pick must be 1 or greater");
    }
    if pick > len {
        bail!("pick out of range; got {pick}, max {len}");
    }
    Ok(())
}

pub(crate) fn apply_fuzzy_scores(query: &str, results: &mut crate::domain::search::SearchResults) {
    for item in &mut results.items {
        item.score = fuzzy_score(query, &item.name);
    }

    results.items.sort_by(|a, b| {
        let a_score = a.score.unwrap_or(0.0);
        let b_score = b.score.unwrap_or(0.0);
        b_score
            .partial_cmp(&a_score)
            .unwrap_or(std::cmp::Ordering::Equal)
            .then_with(|| a.name.to_lowercase().cmp(&b.name.to_lowercase()))
    });
}

pub(crate) fn pick_best_match(
    results: &crate::domain::search::SearchResults,
    query: &str,
    owner_name: Option<&str>,
) -> Option<crate::domain::search::SearchItem> {
    let mut best: Option<(f32, usize, crate::domain::search::SearchItem)> = None;
    for (index, item) in results.items.iter().enumerate() {
        let mut score = fuzzy_score(query, &item.name).unwrap_or(0.0);
        if item.kind == SearchType::Playlist
            && let Some(owner_name) = owner_name
            && item
                .owner
                .as_ref()
                .is_some_and(|owner| owner.eq_ignore_ascii_case(owner_name))
        {
            score += 1.0;
        }

        match &best {
            None => best = Some((score, index, item.clone())),
            Some((best_score, best_index, _)) => {
                if score > *best_score || (score == *best_score && index < *best_index) {
                    best = Some((score, index, item.clone()));
                }
            }
        }
    }
    best.map(|(_, _, item)| item)
}

struct LocalPlaylistMatch {
    item: SearchItem,
    score: f32,
    name_lower: String,
}

fn local_playlist_results(
    ctx: &AppContext,
    query: &str,
    limit: u32,
) -> Result<Option<SearchResults>> {
    let mut matches = Vec::new();

    if let Some(snapshot) = ctx.cache.playlist_cache().load()? {
        for playlist in snapshot.items {
            if let Some(score) = playlist_match_score(query, &playlist.name) {
                let name = playlist.name;
                let uri = format!("spotify:playlist:{}", playlist.id);
                matches.push(LocalPlaylistMatch {
                    item: SearchItem {
                        id: playlist.id,
                        name: name.clone(),
                        uri,
                        kind: SearchType::Playlist,
                        artists: Vec::new(),
                        album: None,
                        duration_ms: None,
                        owner: playlist.owner,
                        score: None,
                    },
                    score,
                    name_lower: name.to_lowercase(),
                });
            }
        }
    }

    let pins = ctx.cache.pin_store().load()?.items;
    for pin in pins {
        if let Some(score) = playlist_match_score(query, &pin.name) {
            let name = pin.name;
            let Some(id) = parse_playlist_id(&pin.url) else {
                continue;
            };
            let uri = format!("spotify:playlist:{id}");
            matches.push(LocalPlaylistMatch {
                item: SearchItem {
                    id,
                    name: name.clone(),
                    uri,
                    kind: SearchType::Playlist,
                    artists: Vec::new(),
                    album: None,
                    duration_ms: None,
                    owner: Some("pinned".to_string()),
                    score: None,
                },
                score,
                name_lower: name.to_lowercase(),
            });
        }
    }

    if matches.is_empty() {
        return Ok(None);
    }

    matches.sort_by(|a, b| {
        b.score
            .partial_cmp(&a.score)
            .unwrap_or(std::cmp::Ordering::Equal)
            .then_with(|| a.name_lower.cmp(&b.name_lower))
    });

    let limit = limit as usize;
    if limit > 0 && matches.len() > limit {
        matches.truncate(limit);
    }

    let items = matches.into_iter().map(|item| item.item).collect();
    Ok(Some(SearchResults {
        kind: SearchType::Playlist,
        items,
    }))
}

fn playlist_match_score(query: &str, candidate: &str) -> Option<f32> {
    let query_lower = query.to_lowercase();
    let candidate_lower = candidate.to_lowercase();
    let score = fuzzy_score(&query_lower, &candidate_lower).unwrap_or(0.0);
    if score > 0.0 {
        return Some(score);
    }
    None
}

fn fuzzy_score(query: &str, candidate: &str) -> Option<f32> {
    let query = query.to_lowercase();
    let candidate = candidate.to_lowercase();
    let tokens: Vec<&str> = query
        .split_whitespace()
        .map(|token| token.trim_matches('*'))
        .filter(|token| !token.is_empty())
        .collect();
    if tokens.is_empty() {
        return None;
    }

    if candidate == query {
        return Some(1.0);
    }

    let candidate_len = candidate.len().max(1) as f32;
    let query_len = query.len().max(1) as f32;
    let length_penalty = ((candidate_len - query_len).max(0.0) / candidate_len) * 0.3;

    if tokens.len() == 1 {
        let token = tokens[0];
        let mut score = if candidate.starts_with(token) {
            0.9
        } else if candidate
            .split_whitespace()
            .any(|word| word.starts_with(token))
        {
            0.85
        } else if candidate.contains(token) {
            0.7
        } else {
            0.0
        };
        score = (score - length_penalty).max(0.0);
        return Some(score);
    }

    let mut matched = 0usize;
    for token in &tokens {
        if candidate
            .split_whitespace()
            .any(|word| word.starts_with(token))
        {
            matched += 1;
        }
    }
    if matched == 0 {
        return Some(0.0);
    }

    let mut score = matched as f32 / tokens.len() as f32;
    if candidate.starts_with(tokens[0]) {
        score += 0.1;
    }
    if candidate.contains(&query) {
        score += 0.1;
    }
    score = (score - length_penalty).clamp(0.0, 1.0);
    Some(score)
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

#[cfg(test)]
mod tests {
    use super::{fuzzy_query, fuzzy_score, validate_pick};

    #[test]
    fn fuzzy_query_wraps_tokens() {
        assert_eq!(fuzzy_query("boards of canada"), "*boards* *of* *canada*");
        assert_eq!(fuzzy_query("solo"), "*solo*");
    }

    #[test]
    fn fuzzy_score_matches_tokens() {
        assert_eq!(fuzzy_score("my radar", "My Radar"), Some(1.0));
        assert!(fuzzy_score("my radar", "Radar Only").unwrap_or(0.0) < 0.5);
    }

    #[test]
    fn validate_pick_rejects_zero() {
        let result = validate_pick(0, 10);
        assert!(result.is_err());
    }

    #[test]
    fn validate_pick_rejects_out_of_range() {
        let result = validate_pick(11, 10);
        assert!(result.is_err());
    }
}
