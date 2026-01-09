//! Playlist command handlers.
use anyhow::bail;
use clap::Subcommand;

use crate::AppContext;
use crate::action::playlist::resolve_for_write;
use crate::domain::playlist::Playlist;
use crate::domain::search::SearchItem;
use crate::domain::search::SearchType;
use crate::error::Result;

#[derive(Subcommand, Debug)]
pub enum PlaylistCommand {
    List {
        #[arg(long, help = "Only show collaborative playlists")]
        collaborative: bool,
        #[arg(long, help = "Only show playlists you own")]
        owned: bool,
        #[arg(long, help = "Only show public playlists")]
        public: bool,
        #[arg(long, help = "Only show private playlists")]
        private: bool,
        #[arg(long, value_enum, default_value = "name", help = "Sort playlists")]
        sort: PlaylistSort,
    },
    #[command(name = "addto")]
    AddTo {
        #[arg(value_name = "QUERY")]
        query: Option<String>,
        #[arg(long, help = "Use market from token")]
        user: bool,
        #[arg(long, help = "Pick a specific result (1-based)")]
        pick: Option<usize>,
        #[arg(long, help = "Use the last cached search results")]
        last: bool,
    },
    Create {
        name: String,
        #[arg(long, conflicts_with = "private", help = "Create as a public playlist")]
        public: bool,
        #[arg(long, conflicts_with = "public", help = "Create as a private playlist")]
        private: bool,
    },
    Rename {
        #[arg(value_name = "QUERY")]
        query: Option<String>,
        new_name: String,
        #[arg(long, help = "Use market from token")]
        user: bool,
        #[arg(long, help = "Pick a specific result (1-based)")]
        pick: Option<usize>,
        #[arg(long, help = "Use the last cached search results")]
        last: bool,
    },
    Delete {
        #[arg(value_name = "QUERY")]
        query: Option<String>,
        #[arg(long, help = "Use market from token")]
        user: bool,
        #[arg(long, help = "Pick a specific result (1-based)")]
        pick: Option<usize>,
        #[arg(long, help = "Use the last cached search results")]
        last: bool,
    },
}

pub fn handle(command: PlaylistCommand, ctx: &AppContext) -> Result<()> {
    match command {
        PlaylistCommand::List {
            collaborative,
            owned,
            public,
            private,
            sort,
        } => list(ctx, collaborative, owned, public, private, sort),
        PlaylistCommand::AddTo {
            query,
            user,
            pick,
            last,
        } => add_to(ctx, query.as_deref(), user, pick, last),
        PlaylistCommand::Create {
            name,
            public,
            private,
        } => create(ctx, &name, public, private),
        PlaylistCommand::Rename {
            query,
            new_name,
            user,
            pick,
            last,
        } => rename(ctx, query.as_deref(), &new_name, user, pick, last),
        PlaylistCommand::Delete {
            query,
            user,
            pick,
            last,
        } => delete(ctx, query.as_deref(), user, pick, last),
    }
}

fn list(
    ctx: &AppContext,
    collaborative: bool,
    owned: bool,
    public: bool,
    private: bool,
    sort: PlaylistSort,
) -> Result<()> {
    let snapshot = ctx.cache.playlist_cache().load()?;
    let Some(snapshot) = snapshot else {
        bail!("playlist cache empty; run `spotify sync`");
    };
    let mut playlists = snapshot.items;
    if collaborative {
        playlists.retain(|playlist| playlist.collaborative);
    }
    if public {
        playlists.retain(|playlist| playlist.public == Some(true));
    }
    if private {
        playlists.retain(|playlist| playlist.public == Some(false));
    }
    if owned {
        let Some(owner_name) = ctx.auth.user_name()? else {
            bail!("missing user name; run `spotify sync` or `spotify cache user <name>`");
        };
        let owner_name = owner_name.to_lowercase();
        playlists.retain(|playlist| {
            playlist
                .owner
                .as_ref()
                .is_some_and(|owner| owner.to_lowercase() == owner_name)
        });
    }
    sort_playlists(&mut playlists, sort);
    let pins = ctx.cache.pin_store().load()?.items;
    ctx.output.playlist_list_with_pins(playlists, pins)
}

pub(crate) fn add_to(
    ctx: &AppContext,
    query: Option<&str>,
    user: bool,
    pick: Option<usize>,
    last: bool,
) -> Result<()> {
    let status = ctx.spotify()?.playback().status()?;
    let Some(track) = status.track else {
        bail!("no track is currently playing");
    };
    let selection = resolve_for_write(ctx, query, last, user, pick)?;
    let uri = format!("spotify:track:{}", track.id);
    ctx.spotify()?
        .playlists()
        .add_tracks(&selection.id, &[uri])?;
    let message = format!("Added: {} -> {}", format_track(&track), selection.name);
    ctx.output.action("playlist_add", &message)?;
    Ok(())
}

fn create(ctx: &AppContext, name: &str, public: bool, private: bool) -> Result<()> {
    let public = if public {
        Some(true)
    } else if private {
        Some(false)
    } else {
        None
    };
    let playlist = ctx.spotify()?.playlists().create(name, public)?;
    let message = format!("Created: {} ({})", playlist.name, playlist.id);
    ctx.output.action("playlist_create", &message)
}

fn rename(
    ctx: &AppContext,
    query: Option<&str>,
    new_name: &str,
    user: bool,
    pick: Option<usize>,
    last: bool,
) -> Result<()> {
    let selection = resolve_for_write(ctx, query, last, user, pick)?;
    ctx.spotify()?.playlists().rename(&selection.id, new_name)?;
    let message = format!("Renamed: {} -> {}", selection.name, new_name);
    ctx.output.action("playlist_rename", &message)
}

fn delete(
    ctx: &AppContext,
    query: Option<&str>,
    user: bool,
    pick: Option<usize>,
    last: bool,
) -> Result<()> {
    let item = resolve_playlist(ctx, query, last, user, pick)?;
    ctx.spotify()?.playlists().delete(&item.id)?;
    let message = format!("Deleted (unfollowed): {}", item.name);
    ctx.output.action("playlist_delete", &message)
}

fn resolve_playlist(
    ctx: &AppContext,
    query: Option<&str>,
    last: bool,
    user: bool,
    pick: Option<usize>,
) -> Result<crate::domain::search::SearchItem> {
    if user
        && !last
        && let Some(query) = query
        && let Some(item) = resolve_cached_playlist_match(ctx, query, pick)?
    {
        return Ok(item);
    }
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

    let query = crate::action::playlist::build_query(query);
    ctx.spotify()?
        .search()
        .search(&query, SearchType::Playlist, limit, user)
}

struct CachedPlaylistMatch {
    item: SearchItem,
    score: f32,
    name_lower: String,
}

fn resolve_cached_playlist_match(
    ctx: &AppContext,
    query: &str,
    pick: Option<usize>,
) -> Result<Option<SearchItem>> {
    let mut matches = Vec::new();

    if let Some(snapshot) = ctx.cache.playlist_cache().load()? {
        for playlist in snapshot.items {
            if let Some(score) = match_score(query, &playlist.name) {
                let name = playlist.name;
                let uri = format!("spotify:playlist:{}", playlist.id);
                matches.push(CachedPlaylistMatch {
                    item: SearchItem {
                        id: playlist.id,
                        name: name.clone(),
                        uri,
                        kind: SearchType::Playlist,
                        artists: Vec::new(),
                        album: None,
                        duration_ms: None,
                        owner: playlist.owner,
                        score: Some(score),
                    },
                    score,
                    name_lower: name.to_lowercase(),
                });
            }
        }
    }

    let pins = ctx.cache.pin_store().load()?.items;
    for pin in pins {
        if let Some(score) = match_score(query, &pin.name) {
            let name = pin.name;
            let Some(id) = parse_playlist_id(&pin.url) else {
                continue;
            };
            let uri = format!("spotify:playlist:{id}");
            matches.push(CachedPlaylistMatch {
                item: SearchItem {
                    id,
                    name: name.clone(),
                    uri,
                    kind: SearchType::Playlist,
                    artists: Vec::new(),
                    album: None,
                    duration_ms: None,
                    owner: None,
                    score: Some(score),
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

    if let Some(pick) = pick {
        validate_pick(pick, matches.len())?;
        return Ok(Some(matches[pick - 1].item.clone()));
    }

    Ok(Some(matches[0].item.clone()))
}

fn match_score(query: &str, candidate: &str) -> Option<f32> {
    let query_lower = query.to_lowercase();
    let candidate_lower = candidate.to_lowercase();
    let score = fuzzy_score(&query_lower, &candidate_lower);
    if score > 0.0 {
        return Some(score);
    }
    None
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

pub(crate) fn parse_playlist_id(input: &str) -> Option<String> {
    let cleaned: String = input.split_whitespace().collect();
    let cleaned = cleaned.trim();

    if cleaned.starts_with("spotify:") {
        if let Some(uri) = cleaned.strip_prefix("spotify:playlist:") {
            return Some(split_playlist_id(uri));
        }
        if let Some(index) = cleaned.find(":playlist:") {
            let uri = &cleaned[index + ":playlist:".len()..];
            return Some(split_playlist_id(uri));
        }
    }

    if cleaned.starts_with("http")
        && let Ok(url) = url::Url::parse(cleaned)
        && let Some(segments) = url.path_segments()
    {
        let segments: Vec<_> = segments.collect();
        if segments.len() >= 2 && segments[0] == "playlist" {
            return Some(segments[1].to_string());
        }
    }

    None
}

fn split_playlist_id(value: &str) -> String {
    value
        .split([':', '?', '#'])
        .next()
        .unwrap_or(value)
        .to_string()
}

fn format_track(track: &crate::domain::track::Track) -> String {
    if track.artists.is_empty() {
        track.name.clone()
    } else {
        format!("{} - {}", track.name, track.artists.join(", "))
    }
}

#[derive(clap::ValueEnum, Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum PlaylistSort {
    Name,
    Owner,
    Public,
    Collaborative,
}

fn sort_playlists(playlists: &mut [Playlist], sort: PlaylistSort) {
    match sort {
        PlaylistSort::Name => {
            playlists.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()))
        }
        PlaylistSort::Owner => playlists.sort_by(|a, b| {
            let a_owner = a.owner.as_deref().unwrap_or("").to_lowercase();
            let b_owner = b.owner.as_deref().unwrap_or("").to_lowercase();
            a_owner
                .cmp(&b_owner)
                .then_with(|| a.name.to_lowercase().cmp(&b.name.to_lowercase()))
        }),
        PlaylistSort::Public => playlists.sort_by(|a, b| {
            let a_public = a.public.unwrap_or(false);
            let b_public = b.public.unwrap_or(false);
            b_public
                .cmp(&a_public)
                .then_with(|| a.name.to_lowercase().cmp(&b.name.to_lowercase()))
        }),
        PlaylistSort::Collaborative => playlists.sort_by(|a, b| {
            b.collaborative
                .cmp(&a.collaborative)
                .then_with(|| a.name.to_lowercase().cmp(&b.name.to_lowercase()))
        }),
    }
}

#[cfg(test)]
mod tests {
    use super::{PlaylistSort, parse_playlist_id, sort_playlists};
    use crate::action::playlist::{build_query, match_from_items};
    use crate::domain::playlist::Playlist;
    use crate::error::Result;

    #[test]
    fn build_query_wraps_tokens() {
        assert_eq!(build_query("deep focus"), "*deep* *focus*");
    }

    #[test]
    fn sort_playlists_by_name() {
        let mut playlists = vec![
            Playlist {
                id: "2".to_string(),
                name: "Beta".to_string(),
                owner: None,
                collaborative: false,
                public: Some(true),
            },
            Playlist {
                id: "1".to_string(),
                name: "Alpha".to_string(),
                owner: None,
                collaborative: false,
                public: Some(true),
            },
        ];
        sort_playlists(&mut playlists, PlaylistSort::Name);
        assert_eq!(playlists[0].name, "Alpha");
    }

    #[test]
    fn sort_playlists_by_owner() {
        let mut playlists = vec![
            Playlist {
                id: "1".to_string(),
                name: "Alpha".to_string(),
                owner: Some("Zed".to_string()),
                collaborative: false,
                public: Some(true),
            },
            Playlist {
                id: "2".to_string(),
                name: "Beta".to_string(),
                owner: Some("Amy".to_string()),
                collaborative: false,
                public: Some(true),
            },
        ];
        sort_playlists(&mut playlists, PlaylistSort::Owner);
        assert_eq!(playlists[0].owner.as_deref(), Some("Amy"));
    }

    #[test]
    fn sort_playlists_by_public() {
        let mut playlists = vec![
            Playlist {
                id: "1".to_string(),
                name: "Public".to_string(),
                owner: None,
                collaborative: false,
                public: Some(true),
            },
            Playlist {
                id: "2".to_string(),
                name: "Private".to_string(),
                owner: None,
                collaborative: false,
                public: Some(false),
            },
        ];
        sort_playlists(&mut playlists, PlaylistSort::Public);
        assert_eq!(playlists[0].name, "Public");
    }

    #[test]
    fn sort_playlists_by_collaborative() {
        let mut playlists = vec![
            Playlist {
                id: "1".to_string(),
                name: "Solo".to_string(),
                owner: None,
                collaborative: false,
                public: Some(true),
            },
            Playlist {
                id: "2".to_string(),
                name: "Collab".to_string(),
                owner: None,
                collaborative: true,
                public: Some(true),
            },
        ];
        sort_playlists(&mut playlists, PlaylistSort::Collaborative);
        assert_eq!(playlists[0].name, "Collab");
    }

    #[test]
    fn resolve_playlist_from_cache_prefers_match() -> Result<()> {
        let items = vec![
            Playlist {
                id: "1".to_string(),
                name: "MyRadar".to_string(),
                owner: Some("Me".to_string()),
                collaborative: false,
                public: Some(false),
            },
            Playlist {
                id: "2".to_string(),
                name: "Radar Hits".to_string(),
                owner: Some("Other".to_string()),
                collaborative: false,
                public: Some(true),
            },
        ];

        let found = match_from_items(items, "MyRadar", None, None)?;
        assert!(found.is_some());
        assert_eq!(found.unwrap().id, "1");
        Ok(())
    }

    #[test]
    fn resolve_playlist_from_cache_fuzzy_match() -> Result<()> {
        let items = vec![
            Playlist {
                id: "1".to_string(),
                name: "Daily Mix 1".to_string(),
                owner: Some("Me".to_string()),
                collaborative: false,
                public: Some(false),
            },
            Playlist {
                id: "2".to_string(),
                name: "Daily Drive".to_string(),
                owner: Some("Other".to_string()),
                collaborative: false,
                public: Some(true),
            },
        ];

        let found = match_from_items(items, "daily mix", None, None)?;
        assert!(found.is_some());
        assert_eq!(found.unwrap().id, "1");
        Ok(())
    }

    #[test]
    fn resolve_playlist_from_cache_pick_index() -> Result<()> {
        let items = vec![
            Playlist {
                id: "1".to_string(),
                name: "MyRadar".to_string(),
                owner: Some("Me".to_string()),
                collaborative: false,
                public: Some(false),
            },
            Playlist {
                id: "2".to_string(),
                name: "MyRadar Hits".to_string(),
                owner: Some("Me".to_string()),
                collaborative: false,
                public: Some(false),
            },
        ];

        let found = match_from_items(items, "MyRadar", Some(2), None)?;
        assert!(found.is_some());
        assert_eq!(found.unwrap().id, "2");
        Ok(())
    }

    #[test]
    fn resolve_playlist_from_cache_prefers_writable() -> Result<()> {
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

        let found = match_from_items(items, "Radar", None, Some("Me"))?;
        assert!(found.is_some());
        assert_eq!(found.unwrap().id, "2");
        Ok(())
    }

    #[test]
    fn parse_playlist_id_from_uri() {
        let id = parse_playlist_id("spotify:playlist:abc123");
        assert_eq!(id.as_deref(), Some("abc123"));
    }

    #[test]
    fn parse_playlist_id_from_uri_with_suffix() {
        let id = parse_playlist_id("spotify:playlist:abc123:recommended");
        assert_eq!(id.as_deref(), Some("abc123"));
    }

    #[test]
    fn parse_playlist_id_from_user_uri() {
        let id = parse_playlist_id("spotify:user:alice:playlist:abc123");
        assert_eq!(id.as_deref(), Some("abc123"));
    }

    #[test]
    fn parse_playlist_id_from_url() {
        let id = parse_playlist_id("https://open.spotify.com/playlist/abc123?si=xyz");
        assert_eq!(id.as_deref(), Some("abc123"));
    }

    #[test]
    fn parse_playlist_id_rejects_invalid() {
        let id = parse_playlist_id("not a playlist");
        assert!(id.is_none());
    }
}
