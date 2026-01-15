# Changelog

All notable changes to this project will be documented in this file.

## [0.5.0] - 2025-01-15

### Release Candidate

A complete command-line interface for Spotify. Control your music, manage your library, and search the catalog—all from your terminal.

### Playback Control

Full control over Spotify playback from any device:

- **Transport**: Play, pause, toggle, next, previous, seek
- **Volume & Modes**: Volume control (0-100%), shuffle, repeat (off/track/context)
- **Queue**: View queue, add tracks, add from playlists
- **Devices**: List devices, transfer playback between them
- **Seek**: Flexible formats—seconds, `MM:SS`, or explicit units (`90s`, `5000ms`)

### Search

Powerful search across the entire Spotify catalog:

- **Multi-type**: Search tracks, artists, albums, playlists, shows, episodes, audiobooks
- **Field Filters**: `--artist`, `--album`, `--track`, `--year`, `--genre`, `--isrc`, `--upc`
- **Special Filters**: `--new` (recent releases), `--hipster` (hidden gems)
- **Fuzzy Matching**: Intelligent scoring with configurable thresholds
- **Instant Play**: `--play` flag to immediately play the top result

### Library Management

Complete control over your saved content:

- **Liked Songs**: List, save, remove, check status
- **Albums**: Browse saved albums, save/remove, check status
- **Shows & Episodes**: Full podcast library management
- **Audiobooks**: Browse and manage your audiobook collection
- **Bulk Operations**: Save/remove multiple items with `--dry-run` preview

### Playlist Management

Everything you need for playlist curation:

- **Browse**: List your playlists with pagination
- **Create & Edit**: Create playlists, rename, update descriptions, set visibility
- **Track Operations**: Add, remove, reorder tracks
- **Collaboration**: Follow/unfollow playlists, view other users' playlists
- **Duplication**: Copy playlists with custom names

### Pin System

Quick access to your favorites:

- **Aliases**: Create shortcuts like `work-music` or `chill-vibes` for any resource
- **Tags**: Organize pins with comma-separated tags
- **Universal**: Works with tracks, albums, artists, playlists, shows, episodes, audiobooks
- **Integration**: Use pins anywhere you'd use a Spotify ID or URL

### Information & Discovery

Explore the catalog and your stats:

- **Now Playing**: Current track details with album art, duration, popularity
- **Artist Deep Dive**: Top tracks, albums, related artists, bio
- **Your Stats**: Top tracks/artists across time ranges (4 weeks, 6 months, all-time)
- **Browse**: Categories, new releases, available markets
- **Recently Played**: Your listening history

### Authentication

Secure OAuth 2.0 with PKCE:

- **Browser Flow**: One-click authentication via your default browser
- **Auto Refresh**: Expired tokens refresh automatically—no manual intervention
- **Secure Storage**: Credentials stored in system keychain or encrypted file

### Output Formats

Flexible output for humans and scripts:

- **Pretty Print**: Human-readable formatted output (default)
- **JSON**: Complete structured output with `--json` for piping and automation
- **ID-only**: Extract just IDs for scripting with `--id-only`

### Command Shortcuts

Efficient aliases for power users:

- `p` → player, `s` → search, `pl` → playlist, `lib` → library, `i` → info
- `n` → next, `prev` → previous, `t` → toggle, `vol` → volume
- `ls` → list (works everywhere)

### Shell Completions

Auto-generated completions for bash, zsh, fish, and PowerShell.

### Build

- Cross-platform binaries: macOS (Intel + Apple Silicon), Windows, Linux (x86_64 + ARM64)
- Optimized release builds with LTO and symbol stripping
- Available via `cargo install` or direct binary download
