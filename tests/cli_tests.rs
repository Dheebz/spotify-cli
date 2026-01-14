//! CLI integration tests
//!
//! Tests for command-line interface behavior, argument parsing,
//! help output, and error handling.

use predicates::prelude::*;

fn spotify_cli() -> assert_cmd::Command {
    #[allow(deprecated)]
    assert_cmd::Command::cargo_bin("spotify-cli").unwrap()
}

// ============================================================================
// Help and version tests
// ============================================================================

#[test]
fn help_displays_usage() {
    spotify_cli()
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("Usage:"))
        .stdout(predicate::str::contains("spotify-cli"));
}

#[test]
fn help_shows_commands() {
    spotify_cli()
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("auth"))
        .stdout(predicate::str::contains("player"))
        .stdout(predicate::str::contains("search"))
        .stdout(predicate::str::contains("playlist"));
}

#[test]
fn subcommand_help_works() {
    let subcommands = ["auth", "player", "playlist", "library", "search", "pin", "info"];

    for cmd in subcommands {
        spotify_cli()
            .arg(cmd)
            .arg("--help")
            .assert()
            .success()
            .stdout(predicate::str::contains("Usage:"));
    }
}

// Note: --version is not enabled in the CLI, so we skip this test

// ============================================================================
// Global flag tests
// ============================================================================

#[test]
fn json_flag_is_global() {
    spotify_cli()
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("--json"));
}

#[test]
fn verbose_flag_is_global() {
    spotify_cli()
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("--verbose"));
}

// ============================================================================
// Invalid argument tests
// ============================================================================

#[test]
fn invalid_subcommand_fails() {
    spotify_cli()
        .arg("invalid-command")
        .assert()
        .failure()
        .stderr(predicate::str::contains("invalid"));
}

#[test]
fn search_empty_query_shows_error() {
    // search with empty query shows helpful error message
    spotify_cli()
        .arg("search")
        .assert()
        .stderr(predicate::str::contains("Search query is empty"));
}

// ============================================================================
// Player subcommand tests
// ============================================================================

#[test]
fn player_help_shows_subcommands() {
    spotify_cli()
        .args(["player", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("play"))
        .stdout(predicate::str::contains("pause"))
        .stdout(predicate::str::contains("next"))
        .stdout(predicate::str::contains("previous"))
        .stdout(predicate::str::contains("status"))
        .stdout(predicate::str::contains("queue"))
        .stdout(predicate::str::contains("devices"));
}

#[test]
fn player_volume_help_shows_range() {
    spotify_cli()
        .args(["player", "volume", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("PERCENT"));
}

#[test]
fn player_repeat_help_shows_modes() {
    spotify_cli()
        .args(["player", "repeat", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("MODE"));
}

// ============================================================================
// Search subcommand tests
// ============================================================================

#[test]
fn search_help_shows_options() {
    spotify_cli()
        .args(["search", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("QUERY"))
        .stdout(predicate::str::contains("--type"))
        .stdout(predicate::str::contains("--limit"));
}

#[test]
fn search_help_shows_filters() {
    spotify_cli()
        .args(["search", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("--artist"))
        .stdout(predicate::str::contains("--album"))
        .stdout(predicate::str::contains("--year"))
        .stdout(predicate::str::contains("--genre"));
}

// ============================================================================
// Playlist subcommand tests
// ============================================================================

#[test]
fn playlist_help_shows_subcommands() {
    spotify_cli()
        .args(["playlist", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("list"))
        .stdout(predicate::str::contains("get"))
        .stdout(predicate::str::contains("create"))
        .stdout(predicate::str::contains("add"))
        .stdout(predicate::str::contains("remove"));
}

#[test]
fn playlist_list_help_shows_pagination() {
    spotify_cli()
        .args(["playlist", "list", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("--limit"))
        .stdout(predicate::str::contains("--offset"));
}

// ============================================================================
// Pin subcommand tests
// ============================================================================

#[test]
fn pin_help_shows_subcommands() {
    spotify_cli()
        .args(["pin", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("add"))
        .stdout(predicate::str::contains("remove"))
        .stdout(predicate::str::contains("list"));
}

#[test]
fn pin_add_requires_arguments() {
    spotify_cli()
        .args(["pin", "add"])
        .assert()
        .failure();
}

// ============================================================================
// Auth subcommand tests
// ============================================================================

#[test]
fn auth_help_shows_subcommands() {
    spotify_cli()
        .args(["auth", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("login"))
        .stdout(predicate::str::contains("logout"))
        .stdout(predicate::str::contains("refresh"))
        .stdout(predicate::str::contains("status"));
}

// ============================================================================
// Completions tests
// ============================================================================

#[test]
fn completions_bash_works() {
    spotify_cli()
        .args(["completions", "bash"])
        .assert()
        .success()
        .stdout(predicate::str::contains("_spotify-cli"));
}

#[test]
fn completions_zsh_works() {
    spotify_cli()
        .args(["completions", "zsh"])
        .assert()
        .success()
        .stdout(predicate::str::contains("#compdef"));
}

#[test]
fn completions_fish_works() {
    spotify_cli()
        .args(["completions", "fish"])
        .assert()
        .success()
        .stdout(predicate::str::contains("complete"));
}

// ============================================================================
// Info subcommand tests
// ============================================================================

#[test]
fn info_help_shows_subcommands() {
    spotify_cli()
        .args(["info", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("track"))
        .stdout(predicate::str::contains("album"))
        .stdout(predicate::str::contains("artist"));
}

#[test]
fn info_artist_help_shows_views() {
    spotify_cli()
        .args(["info", "artist", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("--top-tracks"))
        .stdout(predicate::str::contains("--albums"))
        .stdout(predicate::str::contains("--related"));
}

// ============================================================================
// Library subcommand tests
// ============================================================================

#[test]
fn library_help_shows_subcommands() {
    spotify_cli()
        .args(["library", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("list"))
        .stdout(predicate::str::contains("save"))
        .stdout(predicate::str::contains("remove"))
        .stdout(predicate::str::contains("check"));
}

// ============================================================================
// Media subcommands tests (show, episode, audiobook)
// ============================================================================

#[test]
fn show_help_shows_subcommands() {
    spotify_cli()
        .args(["show", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("get"))
        .stdout(predicate::str::contains("episodes"))
        .stdout(predicate::str::contains("list"));
}

#[test]
fn episode_help_shows_subcommands() {
    spotify_cli()
        .args(["episode", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("get"))
        .stdout(predicate::str::contains("list"));
}

#[test]
fn audiobook_help_shows_subcommands() {
    spotify_cli()
        .args(["audiobook", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("get"))
        .stdout(predicate::str::contains("chapters"))
        .stdout(predicate::str::contains("list"));
}

// ============================================================================
// User subcommand tests
// ============================================================================

#[test]
fn user_help_shows_subcommands() {
    spotify_cli()
        .args(["user", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("profile"))
        .stdout(predicate::str::contains("top"));
}

#[test]
fn user_top_help_shows_options() {
    spotify_cli()
        .args(["user", "top", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("TYPE"))
        .stdout(predicate::str::contains("--range"));
}

// ============================================================================
// Follow subcommand tests
// ============================================================================

#[test]
fn follow_help_shows_subcommands() {
    spotify_cli()
        .args(["follow", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("artist"))
        .stdout(predicate::str::contains("user"))
        .stdout(predicate::str::contains("list"));
}

// ============================================================================
// Category and browse tests
// ============================================================================

#[test]
fn category_help_shows_subcommands() {
    spotify_cli()
        .args(["category", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("list"))
        .stdout(predicate::str::contains("get"))
        .stdout(predicate::str::contains("playlists"));
}

#[test]
fn album_help_shows_subcommands() {
    spotify_cli()
        .args(["album", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("list"))
        .stdout(predicate::str::contains("tracks"))
        .stdout(predicate::str::contains("new-releases"));
}

// ============================================================================
// Markets command test
// ============================================================================

#[test]
fn markets_command_exists() {
    spotify_cli()
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("markets"));
}
