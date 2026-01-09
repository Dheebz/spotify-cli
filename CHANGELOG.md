# Changelog

## 0.2.3
- Comprehensive CI/CD pipeline with GitHub Actions
  - Tests workflow: unit, integration, and doc tests with clippy and fmt checks
  - Crates.io package verification on Ubuntu, macOS, and Windows
  - Code coverage with cargo-tarpaulin and Codecov integration
  - Security audit with cargo-audit and cargo-deny
  - MSRV check for Rust 1.85+ compatibility
- Dependabot configuration for automated Cargo and GitHub Actions updates
- Revamped README with badges, table of contents, and streamlined documentation
- Code formatting improvements

## 0.2.2
- Release automation workflow with cross-platform builds (macOS, Linux, Windows)
- Switch to rustls for cross-platform TLS compatibility
- Cross compilation support for Linux ARM64 with OpenSSL fixes

## 0.2.1
- Fixed play via URL command

## 0.2.0
- Simplified command surface: auth, info, search, nowplaying, player, playlist, pin, device, sync, queue, recentlyplayed
- Unified info/search behavior with type-optional queries, cached search reuse, and `--play` actions
- Queue and recently played commands with default limit 10 and hard cap 100
- Now-playing delay control via `nowplaying --delay-ms`, plus auto refresh after state changes
- Playlist create/rename/delete and add-to flow aligned to current CLI
- JSON output expanded with now-playing markers for queue/recently played
- Removed legacy flags and commands (verbose, width/no-trunc, old search/album/artist/track namespaces, system/completions)

## 0.1.0
- PKCE OAuth login with local redirect listener, token caching, and scope inspection
- Client ID handling via `--client-id` or `SPOTIFY_CLIENT_ID`, plus optional redirect override
- Player controls: play/pause/next/prev/status, shuffle, repeat modes
- Track commands: search, like/unlike current track, now-playing info
- Search across tracks, albums, artists, playlists with fuzzy mode, pick, last, and play
- Album and artist search/info/play with fuzzy matching and pick selection
- Playlist search/list/info/play/add/follow/unfollow with fuzzy/pick/last support
- Pinning playlists locally (add/remove/list/play) for fast access
- Device listing and device selection (cached or live)
- Sync command to cache devices/playlists for faster lookups and completions
- Cache commands for status, country, and user settings
- Output controls: `--json`, `--width`, `--no-trunc`, `-v/--verbose`
- Shell completions (bash/zsh/fish) with cached suggestions
- Security: cryptographically random OAuth state, loopback-only redirect enforcement, and restrictive token cache permissions on Unix
