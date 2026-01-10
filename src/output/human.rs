//! Human-readable output formatting.
use crate::domain::album::Album;
use crate::domain::artist::Artist;
use crate::domain::auth::{AuthScopes, AuthStatus};
use crate::domain::device::Device;
use crate::domain::pin::PinnedPlaylist;
use crate::domain::player::PlayerStatus;
use crate::domain::playlist::{Playlist, PlaylistDetail};
use crate::domain::search::{SearchItem, SearchResults};
use crate::domain::track::Track;
use crate::error::Result;
use crate::output::{DEFAULT_MAX_WIDTH, TableConfig};

pub fn auth_status(status: AuthStatus) -> Result<()> {
    if status.logged_in {
        println!("logged_in");
        return Ok(());
    }

    println!("logged_out");
    Ok(())
}

pub fn auth_scopes(scopes: AuthScopes) -> Result<()> {
    println!("Scopes:");
    for scope in scopes.required {
        let status = if let Some(granted) = scopes.granted.as_ref() {
            if granted.iter().any(|item| item == &scope) {
                "ok"
            } else {
                "missing"
            }
        } else {
            "unknown"
        };
        println!("{:<32} {}", scope, status);
    }
    Ok(())
}

pub fn player_status(status: PlayerStatus) -> Result<()> {
    let state = if status.is_playing {
        "playing"
    } else {
        "paused"
    };
    let context = playback_context_line(&status);

    if let Some(track) = status.track {
        let artists = if track.artists.is_empty() {
            String::new()
        } else {
            format!(" - {}", track.artists.join(", "))
        };
        let album = track
            .album
            .as_ref()
            .map(|album| format!(" ({})", album))
            .unwrap_or_default();
        let progress = format_progress(status.progress_ms, track.duration_ms);
        println!("{}: {}{}{}{}", state, track.name, album, artists, progress);
        if let Some(line) = context {
            println!("{}", line);
        }
        return Ok(());
    }

    println!("{}", state);
    if let Some(line) = context {
        println!("{}", line);
    }
    Ok(())
}

pub fn now_playing(status: PlayerStatus) -> Result<()> {
    if let Some(track) = status.track {
        let artists = if track.artists.is_empty() {
            String::new()
        } else {
            format!(" - {}", track.artists.join(", "))
        };
        let album = track
            .album
            .as_ref()
            .map(|album| format!(" ({})", album))
            .unwrap_or_default();
        let progress = format_progress(status.progress_ms, track.duration_ms);
        println!(
            "Now Playing: {}{}{}{}",
            track.name, album, artists, progress
        );
        return Ok(());
    }

    println!("Now Playing: (no active track)");
    Ok(())
}

fn playback_context_line(status: &PlayerStatus) -> Option<String> {
    let repeat = status.repeat_state.as_deref();
    let shuffle = status.shuffle_state;
    let volume = status.device.as_ref().and_then(|d| d.volume_percent);

    if repeat.is_none() && shuffle.is_none() && volume.is_none() {
        return None;
    }

    let repeat_text = repeat.unwrap_or("unknown");
    let shuffle_text = match shuffle {
        Some(true) => "on",
        Some(false) => "off",
        None => "unknown",
    };
    let volume_text = volume
        .map(|v| format!("{}%", v))
        .unwrap_or_else(|| "unknown".to_string());

    Some(format!(
        "repeat: {}, shuffle: {}, volume: {}",
        repeat_text, shuffle_text, volume_text
    ))
}

pub fn action(message: &str) -> Result<()> {
    println!("{}", message);
    Ok(())
}

pub fn album_info(album: Album, table: TableConfig) -> Result<()> {
    let artists = if album.artists.is_empty() {
        String::new()
    } else {
        format!(" - {}", album.artists.join(", "))
    };
    let details = format_optional_details(&[
        album.release_date,
        album.total_tracks.map(|t| t.to_string()),
        album.duration_ms.map(format_duration),
    ]);
    if details.is_empty() {
        println!("{}{}", album.name, artists);
    } else {
        println!("{}{} ({})", album.name, artists, details);
    }
    let mut rows = Vec::new();
    for track in album.tracks {
        rows.push(vec![
            format!("{:02}.", track.track_number),
            track.name,
            format_duration(track.duration_ms as u64),
        ]);
    }
    print_table_with_header(&rows, &["NO", "TRACK", "DURATION"], table);
    Ok(())
}

pub fn artist_info(artist: Artist) -> Result<()> {
    let mut parts = Vec::new();
    if !artist.genres.is_empty() {
        parts.push(artist.genres.join(", "));
    }
    if let Some(followers) = artist.followers {
        parts.push(format!("followers {}", followers));
    }
    if parts.is_empty() {
        println!("{}", artist.name);
    } else {
        println!("{} ({})", artist.name, parts.join(" | "));
    }
    Ok(())
}

pub fn playlist_list(
    playlists: Vec<Playlist>,
    user_name: Option<&str>,
    table: TableConfig,
) -> Result<()> {
    let mut rows = Vec::new();
    for playlist in playlists {
        let mut tags = Vec::new();
        if playlist.collaborative {
            tags.push("collaborative");
        }
        if let Some(public) = playlist.public {
            tags.push(if public { "public" } else { "private" });
        }

        let tag_text = tags.join(", ");
        if let Some(owner) = playlist.owner.as_ref() {
            rows.push(vec![
                playlist.name,
                display_owner(owner, user_name),
                tag_text,
            ]);
        } else {
            rows.push(vec![playlist.name, String::new(), tag_text]);
        }
    }
    print_table_with_header(&rows, &["NAME", "OWNER", "TAGS"], table);
    Ok(())
}

pub fn playlist_list_with_pins(
    playlists: Vec<Playlist>,
    pins: Vec<PinnedPlaylist>,
    user_name: Option<&str>,
    table: TableConfig,
) -> Result<()> {
    let mut rows = Vec::new();
    for playlist in playlists {
        let mut tags = Vec::new();
        if playlist.collaborative {
            tags.push("collaborative");
        }
        if let Some(public) = playlist.public {
            tags.push(if public { "public" } else { "private" });
        }
        let tag_text = tags.join(", ");
        if let Some(owner) = playlist.owner.as_ref() {
            rows.push(vec![
                playlist.name,
                display_owner(owner, user_name),
                tag_text,
            ]);
        } else {
            rows.push(vec![playlist.name, String::new(), tag_text]);
        }
    }
    for pin in pins {
        rows.push(vec![pin.name, "pinned".to_string(), String::new()]);
    }
    print_table_with_header(&rows, &["NAME", "OWNER", "TAGS"], table);
    Ok(())
}

pub fn help() -> Result<()> {
    println!("spotify-cli <object> <verb> [target] [flags]");
    println!(
        "objects: auth, device, info, search, nowplaying, player, playlist, pin, sync, queue, recentlyplayed"
    );
    println!("flags: --json");
    println!("examples:");
    println!("  spotify-cli auth status");
    println!("  spotify-cli search track \"boards of canada\" --play");
    println!("  spotify-cli search \"boards of canada\"");
    println!("  spotify-cli info album \"geogaddi\"");
    println!("  spotify-cli nowplaying");
    println!("  spotify-cli nowplaying like");
    println!("  spotify-cli nowplaying addto \"MyRadar\"");
    println!("  spotify-cli playlist list");
    println!("  spotify-cli pin add \"Release Radar\" \"<url>\"");
    Ok(())
}

pub fn playlist_info(playlist: PlaylistDetail, user_name: Option<&str>) -> Result<()> {
    let owner = playlist
        .owner
        .as_ref()
        .map(|owner| display_owner(owner, user_name))
        .unwrap_or_else(|| "unknown".to_string());
    let mut tags = Vec::new();
    if playlist.collaborative {
        tags.push("collaborative");
    }
    if let Some(public) = playlist.public {
        tags.push(if public { "public" } else { "private" });
    }
    let suffix = if tags.is_empty() {
        String::new()
    } else {
        format!(" [{}]", tags.join(", "))
    };
    if let Some(total) = playlist.tracks_total {
        println!("{} ({}) - {} tracks{}", playlist.name, owner, total, suffix);
    } else {
        println!("{} ({}){}", playlist.name, owner, suffix);
    }
    Ok(())
}

pub fn device_list(devices: Vec<Device>, table: TableConfig) -> Result<()> {
    let mut rows = Vec::new();
    for device in devices {
        let volume = device
            .volume_percent
            .map(|v| v.to_string())
            .unwrap_or_default();
        rows.push(vec![device.name, volume]);
    }
    print_table_with_header(&rows, &["NAME", "VOLUME"], table);
    Ok(())
}

fn format_optional_details(parts: &[Option<String>]) -> String {
    let filtered: Vec<String> = parts.iter().filter_map(|part| part.clone()).collect();
    filtered.join(" | ")
}

#[allow(clippy::collapsible_if)]
fn display_owner(owner: &str, user_name: Option<&str>) -> String {
    if let Some(user_name) = user_name {
        if user_name.eq_ignore_ascii_case(owner) {
            return "You".to_string();
        }
    }
    owner.to_string()
}

fn format_progress(progress_ms: Option<u32>, duration_ms: Option<u32>) -> String {
    let Some(progress_ms) = progress_ms else {
        return String::new();
    };
    let duration_ms = duration_ms.unwrap_or(0);
    if duration_ms == 0 {
        return format!(" [{}]", format_time(progress_ms));
    }
    format!(
        " [{} / {}]",
        format_time(progress_ms),
        format_time(duration_ms)
    )
}

fn format_time(ms: u32) -> String {
    let total_seconds = ms / 1000;
    let minutes = total_seconds / 60;
    let seconds = total_seconds % 60;
    format!("{minutes}:{seconds:02}")
}

fn format_duration(ms: u64) -> String {
    let total_seconds = ms / 1000;
    let minutes = total_seconds / 60;
    let seconds = total_seconds % 60;
    format!("{minutes}:{seconds:02}")
}

pub fn search_results(results: SearchResults, table: TableConfig) -> Result<()> {
    let mut rows = Vec::new();
    let show_kind = results.kind == crate::domain::search::SearchType::All;
    for (index, item) in results.items.into_iter().enumerate() {
        if show_kind {
            let name = item.name;
            let by = if !item.artists.is_empty() {
                item.artists.join(", ")
            } else {
                item.owner.unwrap_or_default()
            };
            let score = item
                .score
                .map(|score| format!("{:.2}", score))
                .unwrap_or_default();
            rows.push(vec![
                (index + 1).to_string(),
                format_search_kind(item.kind),
                name,
                by,
                score,
            ]);
            continue;
        }

        match results.kind {
            crate::domain::search::SearchType::Track => {
                let artists = item.artists.join(", ");
                let album = item.album.unwrap_or_default();
                let duration = item
                    .duration_ms
                    .map(|ms| format_duration(ms as u64))
                    .unwrap_or_default();
                let score = item
                    .score
                    .map(|score| format!("{:.2}", score))
                    .unwrap_or_default();
                rows.push(vec![
                    (index + 1).to_string(),
                    item.name,
                    artists,
                    album,
                    duration,
                    score,
                ]);
            }
            crate::domain::search::SearchType::Album => {
                let artists = item.artists.join(", ");
                let score = item
                    .score
                    .map(|score| format!("{:.2}", score))
                    .unwrap_or_default();
                rows.push(vec![(index + 1).to_string(), item.name, artists, score]);
            }
            crate::domain::search::SearchType::Artist => {
                let score = item
                    .score
                    .map(|score| format!("{:.2}", score))
                    .unwrap_or_default();
                rows.push(vec![(index + 1).to_string(), item.name, score]);
            }
            crate::domain::search::SearchType::Playlist => {
                let owner = item.owner.unwrap_or_default();
                let score = item
                    .score
                    .map(|score| format!("{:.2}", score))
                    .unwrap_or_default();
                rows.push(vec![(index + 1).to_string(), item.name, owner, score]);
            }
            crate::domain::search::SearchType::All => {}
        }
    }
    if show_kind {
        print_table_with_header(&rows, &["#", "TYPE", "NAME", "BY", "SCORE"], table);
    } else {
        match results.kind {
            crate::domain::search::SearchType::Track => {
                print_table_with_header(
                    &rows,
                    &["#", "TRACK", "ARTIST", "ALBUM", "DURATION", "SCORE"],
                    table,
                );
            }
            crate::domain::search::SearchType::Album => {
                print_table_with_header(&rows, &["#", "ALBUM", "ARTIST", "SCORE"], table);
            }
            crate::domain::search::SearchType::Artist => {
                print_table_with_header(&rows, &["#", "ARTIST", "SCORE"], table);
            }
            crate::domain::search::SearchType::Playlist => {
                print_table_with_header(&rows, &["#", "PLAYLIST", "OWNER", "SCORE"], table);
            }
            crate::domain::search::SearchType::All => {}
        }
    }
    Ok(())
}

pub fn queue(items: Vec<Track>, now_playing_id: Option<&str>, table: TableConfig) -> Result<()> {
    let mut rows = Vec::new();
    for (index, track) in items.into_iter().enumerate() {
        let Track {
            id,
            name,
            artists,
            album,
            duration_ms,
            ..
        } = track;
        let mut name = name;
        if now_playing_id.is_some_and(|needle| needle == id) {
            name = format!("* {}", name);
        }
        let artists = artists.join(", ");
        let album = album.unwrap_or_default();
        let duration = duration_ms
            .map(|ms| format_duration(ms as u64))
            .unwrap_or_default();
        rows.push(vec![
            (index + 1).to_string(),
            name,
            artists,
            album,
            duration,
        ]);
    }
    print_table_with_header(&rows, &["#", "TRACK", "ARTIST", "ALBUM", "DURATION"], table);
    Ok(())
}

pub fn recently_played(
    items: Vec<SearchItem>,
    now_playing_id: Option<&str>,
    table: TableConfig,
) -> Result<()> {
    let mut rows = Vec::new();
    for (index, item) in items.into_iter().enumerate() {
        let mut name = item.name;
        if now_playing_id.is_some_and(|id| id == item.id) {
            name = format!("* {}", name);
        }
        let artists = item.artists.join(", ");
        let album = item.album.unwrap_or_default();
        let duration = item
            .duration_ms
            .map(|ms| format_duration(ms as u64))
            .unwrap_or_default();
        rows.push(vec![
            (index + 1).to_string(),
            name,
            artists,
            album,
            duration,
        ]);
    }
    print_table_with_header(&rows, &["#", "TRACK", "ARTIST", "ALBUM", "DURATION"], table);
    Ok(())
}

fn format_search_kind(kind: crate::domain::search::SearchType) -> String {
    match kind {
        crate::domain::search::SearchType::Track => "track",
        crate::domain::search::SearchType::Album => "album",
        crate::domain::search::SearchType::Artist => "artist",
        crate::domain::search::SearchType::Playlist => "playlist",
        crate::domain::search::SearchType::All => "all",
    }
    .to_string()
}

fn print_table_with_header(rows: &[Vec<String>], headers: &[&str], table: TableConfig) {
    let mut all_rows = Vec::new();
    if !headers.is_empty() {
        all_rows.push(headers.iter().map(|text| text.to_string()).collect());
    }
    all_rows.extend_from_slice(rows);
    print_table(&all_rows, table);
}

fn print_table(rows: &[Vec<String>], table: TableConfig) {
    if rows.is_empty() {
        return;
    }
    let columns = rows.iter().map(|row| row.len()).max().unwrap_or(0);
    let mut widths = vec![0usize; columns];
    let mut processed = Vec::with_capacity(rows.len());
    let max_width = table.max_width.unwrap_or(DEFAULT_MAX_WIDTH);

    for row in rows {
        let mut new_row = Vec::with_capacity(row.len());
        for (index, cell) in row.iter().enumerate() {
            let truncated = if table.truncate {
                truncate_cell(cell, max_width)
            } else {
                cell.to_string()
            };
            widths[index] = widths[index].max(truncated.len());
            new_row.push(truncated);
        }
        processed.push(new_row);
    }

    for row in processed {
        let mut line = String::new();
        for (index, cell) in row.iter().enumerate() {
            if index > 0 {
                line.push_str("  ");
            }
            let width = widths[index];
            line.push_str(&format!("{:<width$}", cell, width = width));
        }
        println!("{}", line.trim_end());
    }
}

pub(crate) fn truncate_cell(text: &str, max: usize) -> String {
    if text.chars().count() <= max {
        return text.to_string();
    }
    if max <= 3 {
        return "...".to_string();
    }
    let mut truncated: String = text.chars().take(max - 3).collect();
    truncated.push_str("...");
    truncated
}

#[cfg(test)]
mod tests {
    use super::{
        format_duration, format_optional_details, format_progress, format_time, truncate_cell,
    };

    #[test]
    fn truncate_cell_keeps_short_values() {
        assert_eq!(truncate_cell("short", 10), "short");
    }

    #[test]
    fn truncate_cell_adds_ellipsis() {
        assert_eq!(truncate_cell("0123456789", 8), "01234...");
    }

    #[test]
    fn format_progress_with_duration() {
        assert_eq!(format_progress(Some(61000), Some(120000)), " [1:01 / 2:00]");
    }

    #[test]
    fn format_progress_without_duration() {
        assert_eq!(format_progress(Some(61000), None), " [1:01]");
    }

    #[test]
    fn format_time_minutes_seconds() {
        assert_eq!(format_time(61000), "1:01");
    }

    #[test]
    fn format_duration_minutes_seconds() {
        assert_eq!(format_duration(125000), "2:05");
    }

    #[test]
    fn format_optional_details_joins() {
        let value =
            format_optional_details(&[Some("2024".to_string()), None, Some("10".to_string())]);
        assert_eq!(value, "2024 | 10");
    }
}
