# Spotify CLI

A command-line interface for Spotify, built in Rust. Control playback, manage playlists, search for music, and more — all from your terminal.

## Features

- **Playback Control**: Play, pause, skip, seek, volume, shuffle, repeat
- **Library Management**: Save and organize tracks, albums, podcasts, audiobooks
- **Playlist Management**: Create, edit, reorder, follow/unfollow playlists
- **Advanced Search**: Filter by artist, album, year, genre, ISRC, UPC
- **Device Control**: List devices and transfer playback
- **Pin System**: Create shortcuts to frequently used resources
- **JSON Output**: Machine-readable output for scripting
- **RPC Daemon**: JSON-RPC 2.0 over Unix sockets for external control (Neovim, scripts)

## Installation

### From Source

```bash
git clone https://github.com/dheebz/spotify-cli.git
cd spotify-cli
cargo build --release
```

### From Crates.io
```bash
cargo install spotify-cli
```

The binary will be at `target/release/spotify-cli`.

### Prerequisites

- Rust 2024 edition (rustc 1.85+)
- A Spotify account (Premium required for playback control)
- A Spotify Developer App (for OAuth credentials

## Quick Start

### 1. Create a Spotify App

1. Go to [Spotify Developer Dashboard](https://developer.spotify.com/dashboard)
2. Create a new app
3. Add `http://127.0.0.1:8888/callback` as a Redirect URI
4. Copy your Client ID

### 2. Configure the CLI

Create `~/.config/spotify-cli/config.toml`:

```toml
[spotify-cli]
client_id = "your_client_id_here"
```

### 3. Authenticate

```bash
spotify-cli auth login
```

This opens your browser for OAuth authentication. After approval, the CLI stores tokens locally.

### 4. Start Using

```bash
# Check what's playing
spotify-cli player status

# Play/pause
spotify-cli player toggle

# Search for music
spotify-cli search "Daft Punk"

# Add current track to a playlist
spotify-cli playlist add <playlist_id> --now-playing
```

## Commands

### Command Aliases

For faster typing, common commands have short aliases:

| Full Command | Alias | Example |
|--------------|-------|---------|
| `player` | `p` | `spotify-cli p status` |
| `search` | `s` | `spotify-cli s "query"` |
| `playlist` | `pl` | `spotify-cli pl list` |
| `library` | `lib` | `spotify-cli lib list` |
| `info` | `i` | `spotify-cli i track` |

Player subcommand aliases:

| Full Command | Alias | Example |
|--------------|-------|---------|
| `next` | `n` | `spotify-cli p n` |
| `previous` | `prev` | `spotify-cli p prev` |
| `toggle` | `t` | `spotify-cli p t` |
| `status` | `st` | `spotify-cli p st` |
| `volume` | `vol` | `spotify-cli p vol 50` |
| `shuffle` | `sh` | `spotify-cli p sh on` |
| `repeat` | `rep` | `spotify-cli p rep track` |
| `recent` | `rec` | `spotify-cli p rec` |
| `queue` | `q` | `spotify-cli p q list` |
| `devices` | `dev` | `spotify-cli p dev list` |

List subcommand alias: `list` → `ls` (works for playlist, library, queue)

### Authentication

```bash
spotify-cli auth login          # Login via browser OAuth
spotify-cli auth login --force  # Force re-authentication
spotify-cli auth logout         # Clear stored tokens
spotify-cli auth refresh        # Refresh access token
spotify-cli auth status         # Check auth status
```

### Player

```bash
spotify-cli player status       # Current playback info
spotify-cli player toggle       # Play/pause toggle
spotify-cli player play         # Resume playback
spotify-cli player pause        # Pause playback
spotify-cli player next         # Skip to next track
spotify-cli player previous     # Go to previous track
spotify-cli player seek 1:30    # Seek to 1 min 30 sec
spotify-cli player volume 75    # Set volume to 75%
spotify-cli player shuffle on   # Enable shuffle
spotify-cli player repeat track # Repeat current track
spotify-cli player recent       # Recently played tracks

# Play specific content
spotify-cli player play --uri spotify:album:xxx
spotify-cli player play --pin "my-playlist"
```

### Devices

```bash
spotify-cli player devices list              # List available devices
spotify-cli player devices transfer "iPhone" # Transfer playback
```

### Queue

```bash
spotify-cli player queue list                    # Show current queue
spotify-cli player queue add spotify:track:xxx   # Add to queue
spotify-cli player queue add --now-playing       # Add current track
```

### Search

```bash
spotify-cli search "query"                    # Search all types
spotify-cli search "query" --type track       # Search tracks only
spotify-cli search --artist "Beatles"         # Filter by artist
spotify-cli search --year 1990-2000           # Filter by year range
spotify-cli search --genre "rock"             # Filter by genre
spotify-cli search "query" --play             # Play first result
spotify-cli search "query" --pins-only        # Search pinned only
spotify-cli search "query" --exact            # Exact name match
spotify-cli search --new                      # New releases only
```

### Playlists

```bash
spotify-cli playlist list                         # Your playlists
spotify-cli playlist get <id>                     # Playlist details
spotify-cli playlist create "Name"                # Create playlist
spotify-cli playlist create "Name" -d "Desc"      # With description
spotify-cli playlist add <id> spotify:track:xxx   # Add tracks
spotify-cli playlist add <id> --now-playing       # Add current track
spotify-cli playlist remove <id> spotify:track:x  # Remove tracks
spotify-cli playlist edit <id> --name "New Name"  # Rename
spotify-cli playlist reorder <id> -f 0 -t 5       # Move track
spotify-cli playlist follow <id>                  # Follow playlist
spotify-cli playlist unfollow <id>                # Unfollow
spotify-cli playlist duplicate <id>               # Copy playlist
```

### Library (Liked Songs)

```bash
spotify-cli library list                   # Liked songs
spotify-cli library save <id>              # Like a track
spotify-cli library save --now-playing     # Like current track
spotify-cli library remove <id>            # Unlike track
spotify-cli library check <id>             # Check if liked
```

### Info

```bash
spotify-cli info track              # Current track details
spotify-cli info track <id>         # Specific track
spotify-cli info album              # Current album
spotify-cli info artist             # Current artist
spotify-cli info artist --top-tracks # Artist's top tracks
```

### User

```bash
spotify-cli user profile            # Your profile
spotify-cli user top tracks         # Top tracks (6 months)
spotify-cli user top artists -r short # Top artists (4 weeks)
```

### Shows (Podcasts)

```bash
spotify-cli show get <id>           # Show details
spotify-cli show episodes <id>      # Show episodes
spotify-cli show list               # Saved shows
spotify-cli show save <id>          # Save show
spotify-cli show remove <id>        # Remove show
```

### Episodes

```bash
spotify-cli episode get <id>        # Episode details
spotify-cli episode list            # Saved episodes
spotify-cli episode save <id>       # Save episode
spotify-cli episode remove <id>     # Remove episode
```

### Audiobooks

```bash
spotify-cli audiobook get <id>      # Audiobook details
spotify-cli audiobook chapters <id> # Audiobook chapters
spotify-cli audiobook list          # Saved audiobooks
spotify-cli audiobook save <id>     # Save audiobook
spotify-cli audiobook remove <id>   # Remove audiobook
```

### Categories

```bash
spotify-cli category list           # Browse categories
spotify-cli category get pop        # Category details
```

### Pins

Create shortcuts to frequently used resources:

```bash
spotify-cli pin add playlist <url> "work-music" --tags "focus,coding"
spotify-cli pin add album <id> "chill-album"
spotify-cli pin list                     # List all pins
spotify-cli pin list --type playlist     # Filter by type
spotify-cli pin remove "work-music"      # Remove pin

# Use pins in other commands
spotify-cli player play --pin "work-music"
spotify-cli search "work" --pins-only
```

### Daemon (RPC)

Run a background daemon for external control via JSON-RPC 2.0:

```bash
spotify-cli daemon start   # Start daemon in background
spotify-cli daemon stop    # Stop daemon
spotify-cli daemon status  # Check if running
spotify-cli daemon run     # Run in foreground (debugging)

# Send commands via Unix socket
echo '{"jsonrpc":"2.0","method":"player.next","id":1}' | nc -U ~/.config/spotify-cli/daemon.sock
```

The daemon exposes all 68 CLI commands via RPC, plus real-time playback events. Perfect for integrating with Neovim, scripts, or custom applications.

See [docs/RPC.md](docs/RPC.md) for the full API reference.

## JSON Output

Add `--json` or `-j` to any command for machine-readable output:

```bash
spotify-cli player status --json
spotify-cli search "query" --json | jq '.tracks.items[0]'
```

## Configuration

Configuration is stored in `~/.config/spotify-cli/config.toml`:

```toml
[spotify-cli]
client_id = "your_spotify_client_id"

# Optional: customize search behavior
[search]
show_scores = true      # Show fuzzy match scores in search results
sort_by_score = false   # Sort by score (false = use Spotify's relevance)

# Optional: tune fuzzy matching (advanced)
[search.fuzzy]
exact_match = 100.0          # Score for exact name match
starts_with = 50.0           # Score for name starts with query
contains = 30.0              # Score for name contains query
word_match = 10.0            # Score for each word match
similarity_threshold = 0.6  # Min Levenshtein similarity (0.0-1.0)
similarity_weight = 20.0    # Weight for similarity bonus
```

### File Locations

| Path | Purpose |
|------|---------|
| `~/.config/spotify-cli/config.toml` | Configuration |
| `~/.config/spotify-cli/token.json` | OAuth tokens |
| `~/.config/spotify-cli/pins.json` | Pinned resources |
| `~/.config/spotify-cli/daemon.sock` | RPC Unix socket |
| `~/.config/spotify-cli/daemon.pid` | Daemon process ID |

On Windows, files are stored in `%APPDATA%\spotify-cli\`.

### Verbose Logging

Use `-v` for debug output when troubleshooting:

```bash
spotify-cli -v player status      # Info level
spotify-cli -vv player status     # Debug level (shows API calls)
spotify-cli -vvv player status    # Trace level (full details)
```

## Scopes

The CLI requests these Spotify scopes:

- `user-read-private` - Read user profile
- `user-read-email` - Read user email
- `user-top-read` - Read top artists/tracks
- `user-read-playback-state` - Read playback state
- `user-modify-playback-state` - Control playback
- `user-read-currently-playing` - Read current track
- `user-read-recently-played` - Read play history
- `user-library-read` - Read saved content
- `user-library-modify` - Modify saved content
- `playlist-read-private` - Read private playlists
- `playlist-read-collaborative` - Read collaborative playlists
- `playlist-modify-public` - Modify public playlists
- `playlist-modify-private` - Modify private playlists
- `streaming` - Streaming (for future features)

## Troubleshooting

### "Config file not found"

Create the configuration file at `~/.config/spotify-cli/config.toml`:

```toml
[spotify-cli]
client_id = "your_client_id_here"
```

Get your client ID from the [Spotify Developer Dashboard](https://developer.spotify.com/dashboard).

### "No active device"

Spotify requires an active device for playback control. Open Spotify on any device (phone, desktop, web) to activate it.

### Token expired

Run `spotify-cli auth refresh` or `spotify-cli auth login --force`.

### "Premium required"

Playback control features require Spotify Premium.

### Debug mode

Use verbose flags to see what's happening:

```bash
spotify-cli -vv player status   # Shows API requests/responses
```

## License

MIT
