//! Playlist selection and write authorization logic.

use anyhow::bail;

use crate::AppContext;
use crate::domain::playlist::Playlist;
use crate::domain::search::SearchType;
use crate::error::Result;

#[derive(Debug, Clone)]
pub struct PlaylistSelection {
    pub id: String,
    pub name: String,
}

pub fn resolve_for_write(
    ctx: &AppContext,
    query: Option<&str>,
    last: bool,
    user: bool,
    pick: Option<usize>,
) -> Result<PlaylistSelection> {
    let user_name = ctx.auth.ensure_user_name()?;
    if last {
        let item = resolve_search(ctx, None, true, user, pick)?;
        return Ok(PlaylistSelection {
            id: item.id,
            name: item.name,
        });
    }

    let Some(query) = query else {
        bail!("missing playlist query; use --last to reuse cached search results");
    };

    if let Some(playlist) = resolve_from_cache(ctx, query, pick, user_name.as_deref())? {
        if !is_writable(&playlist, user_name.as_deref()) {
            bail!("playlist is read-only; choose an owned or collaborative playlist");
        }
        return Ok(PlaylistSelection {
            id: playlist.id,
            name: playlist.name,
        });
    }

    let item = resolve_search(ctx, Some(query), false, user, pick)?;
    if let Some(user_name) = user_name.as_deref() {
        let detail = ctx.spotify()?.playlists().get(&item.id)?;
        let writable = detail
            .owner
            .as_ref()
            .map(|owner| owner.eq_ignore_ascii_case(user_name))
            .unwrap_or(false)
            || detail.collaborative;
        if !writable {
            bail!("playlist is read-only; choose an owned or collaborative playlist");
        }
    }

    Ok(PlaylistSelection {
        id: item.id,
        name: item.name,
    })
}

pub fn resolve_from_cache(
    ctx: &AppContext,
    query: &str,
    pick: Option<usize>,
    user_name: Option<&str>,
) -> Result<Option<Playlist>> {
    let snapshot = ctx.cache.playlist_cache().load()?;
    let Some(snapshot) = snapshot else {
        return Ok(None);
    };
    match_from_items(snapshot.items, query, pick, user_name)
}

pub fn match_from_items(
    items: Vec<Playlist>,
    query: &str,
    pick: Option<usize>,
    user_name: Option<&str>,
) -> Result<Option<Playlist>> {
    let query_lower = query.to_lowercase();
    let mut matches: Vec<(f32, Playlist)> = Vec::new();
    for playlist in items {
        let name_lower = playlist.name.to_lowercase();
        let score = fuzzy_score(&query_lower, &name_lower);
        if score > 0.0 {
            matches.push((score, playlist));
        }
    }

    if matches.is_empty() {
        return Ok(None);
    }

    matches.sort_by(|a, b| {
        let a_writable = is_writable(&a.1, user_name);
        let b_writable = is_writable(&b.1, user_name);
        b_writable
            .cmp(&a_writable)
            .then_with(|| b.0.partial_cmp(&a.0).unwrap_or(std::cmp::Ordering::Equal))
            .then_with(|| a.1.name.to_lowercase().cmp(&b.1.name.to_lowercase()))
    });

    if let Some(pick) = pick {
        validate_pick(pick, matches.len())?;
        return Ok(Some(matches[pick - 1].1.clone()));
    }

    Ok(Some(matches[0].1.clone()))
}

fn resolve_search(
    ctx: &AppContext,
    query: Option<&str>,
    last: bool,
    user: bool,
    pick: Option<usize>,
) -> Result<crate::domain::search::SearchItem> {
    let limit = pick.map(|_| 10).unwrap_or(1);
    let results = resolve_results(ctx, query, last, user, limit)?;
    if let Some(pick) = pick {
        validate_pick(pick, results.items.len())?;
    }
    let Some(item) = pick_result(&results.items, pick).cloned() else {
        bail!("no playlist results");
    };
    Ok(item)
}

fn resolve_results(
    ctx: &AppContext,
    query: Option<&str>,
    last: bool,
    user: bool,
    limit: u32,
) -> Result<crate::domain::search::SearchResults> {
    if last {
        return load_cached(ctx, SearchType::Playlist);
    }

    let Some(query) = query else {
        bail!("missing playlist query; use --last to reuse cached search results");
    };

    let query = build_query(query);
    ctx.spotify()?
        .search()
        .search(&query, SearchType::Playlist, limit, user)
}

fn load_cached(
    ctx: &AppContext,
    expected: SearchType,
) -> Result<crate::domain::search::SearchResults> {
    let cached = ctx.cache.search_store().load()?;
    let Some(cached) = cached else {
        bail!("no cached search; run `spotify-cli search <query>`");
    };
    if cached.results.kind != expected {
        bail!(
            "cached search is {}; run `spotify-cli search playlist <query>`",
            search_type_label(cached.results.kind)
        );
    }
    Ok(cached.results)
}

fn search_type_label(kind: SearchType) -> &'static str {
    match kind {
        SearchType::Track => "track",
        SearchType::Album => "album",
        SearchType::Artist => "artist",
        SearchType::Playlist => "playlist",
        SearchType::All => "all",
    }
}

pub fn build_query(query: &str) -> String {
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

fn pick_result(
    items: &[crate::domain::search::SearchItem],
    pick: Option<usize>,
) -> Option<&crate::domain::search::SearchItem> {
    if let Some(pick) = pick {
        if pick == 0 {
            return None;
        }
        return items.get(pick - 1);
    }
    items.first()
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

fn fuzzy_score(query: &str, candidate: &str) -> f32 {
    let tokens: Vec<&str> = query
        .split_whitespace()
        .map(|token| token.trim_matches('*'))
        .filter(|token| !token.is_empty())
        .collect();
    if tokens.is_empty() {
        return 0.0;
    }

    if candidate == query {
        return 1.0;
    }

    let candidate_len = candidate.len().max(1) as f32;
    let query_len = query.len().max(1) as f32;
    let length_penalty = ((candidate_len - query_len).max(0.0) / candidate_len) * 0.3;

    if tokens.len() == 1 {
        let token = tokens[0];
        let score = if candidate.starts_with(token) {
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
        return (score - length_penalty).max(0.0);
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
        return 0.0;
    }

    let mut score = matched as f32 / tokens.len() as f32;
    if candidate.starts_with(tokens[0]) {
        score += 0.1;
    }
    if candidate.contains(query) {
        score += 0.1;
    }
    (score - length_penalty).clamp(0.0, 1.0)
}

fn is_writable(playlist: &Playlist, user_name: Option<&str>) -> bool {
    if playlist.collaborative {
        return true;
    }
    let Some(user_name) = user_name else {
        return false;
    };
    playlist
        .owner
        .as_ref()
        .map(|owner| owner.eq_ignore_ascii_case(user_name))
        .unwrap_or(false)
}

#[cfg(test)]
mod tests {
    use super::{match_from_items, resolve_for_write};
    use crate::AppContext;
    use crate::cache::Cache;
    use crate::cache::metadata::MetadataStore;
    use crate::domain::playlist::Playlist;
    use crate::output::Output;
    use crate::spotify::auth::AuthService;

    #[test]
    fn match_from_items_prefers_writable() {
        let items = vec![
            Playlist {
                id: "1".to_string(),
                name: "Radar".to_string(),
                owner: Some("Other".to_string()),
                collaborative: false,
                public: Some(true),
            },
            Playlist {
                id: "2".to_string(),
                name: "Radar".to_string(),
                owner: Some("Me".to_string()),
                collaborative: false,
                public: Some(false),
            },
        ];
        let found = match_from_items(items, "Radar", None, Some("Me"))
            .unwrap()
            .unwrap();
        assert_eq!(found.id, "2");
    }

    #[test]
    fn resolve_for_write_requires_query_without_last() {
        let cache = Cache::new().unwrap();
        let auth = AuthService::new(MetadataStore::new(cache.root().join("metadata.json")));
        let ctx = AppContext {
            cache,
            auth,
            output: Output::new(false, None, None, false),
            verbose: false,
            spotify: std::sync::OnceLock::new(),
        };
        let result = resolve_for_write(&ctx, None, false, false, None);
        assert!(result.is_err());
    }
}
