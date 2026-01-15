# Spotify CLI - RPC Daemon

The spotify-cli daemon exposes a JSON-RPC 2.0 interface over Unix sockets, enabling control from external applications like Neovim, scripts, and other tools.

## Quick Start

```bash
# Start the daemon
spotify-cli daemon start

# Check status
spotify-cli daemon status

# Send a command via socket
echo '{"jsonrpc":"2.0","method":"ping","id":1}' | nc -U ~/.config/spotify-cli/daemon.sock

# Stop the daemon
spotify-cli daemon stop
```

## Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                      spotify-cli daemon                      │
├─────────────────────────────────────────────────────────────┤
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────────────┐  │
│  │ Unix Socket │  │   Event     │  │   Command           │  │
│  │   Server    │──│   Poller    │  │   Dispatcher        │  │
│  │ (JSON-RPC)  │  │ (Spotify)   │  │ (68 methods)        │  │
│  └─────────────┘  └─────────────┘  └─────────────────────┘  │
│         │                │                    │              │
│         └────────────────┴────────────────────┘              │
│                          │                                   │
│  ┌───────────────────────┴───────────────────────────────┐  │
│  │              Shared SpotifyApi Client                  │  │
│  │           (connection pooling, auto-refresh)           │  │
│  └────────────────────────────────────────────────────────┘  │
└─────────────────────────────────────────────────────────────┘
           │                              │
           ▼                              ▼
    ~/.config/spotify-cli/         ~/.config/spotify-cli/
         daemon.sock                    daemon.pid
```

## Daemon Commands

```bash
spotify-cli daemon start   # Start daemon in background
spotify-cli daemon stop    # Stop running daemon
spotify-cli daemon status  # Check if daemon is running
spotify-cli daemon run     # Run in foreground (for debugging)
```

## File Locations

| File | Purpose |
|------|---------|
| `~/.config/spotify-cli/daemon.sock` | Unix socket for RPC |
| `~/.config/spotify-cli/daemon.pid` | Process ID file |

## JSON-RPC 2.0 Protocol

### Request Format

```json
{
  "jsonrpc": "2.0",
  "method": "player.next",
  "params": { "optional": "parameters" },
  "id": 1
}
```

### Response Format

**Success:**
```json
{
  "jsonrpc": "2.0",
  "result": {
    "message": "Skipped to next track",
    "payload": { ... }
  },
  "id": 1
}
```

**Error:**
```json
{
  "jsonrpc": "2.0",
  "error": {
    "code": 401,
    "message": "Token expired",
    "data": { "kind": "Auth" }
  },
  "id": 1
}
```

### Notifications (Server → Client)

```json
{
  "jsonrpc": "2.0",
  "method": "event.trackChanged",
  "params": { "track": { ... } }
}
```

## RPC Methods

Full CLI-RPC parity: **68 methods** covering all CLI functionality.

### Daemon

| Method | Description | Params |
|--------|-------------|--------|
| `ping` | Health check | - |
| `version` | Get daemon version | - |

### Auth

| Method | Description | Params |
|--------|-------------|--------|
| `auth.login` | Start OAuth login | `force` |
| `auth.logout` | Clear credentials | - |
| `auth.refresh` | Refresh access token | - |
| `auth.status` | Check auth status | - |

### Player

| Method | Description | Params |
|--------|-------------|--------|
| `player.status` | Get playback status | `id_only` |
| `player.play` | Start playback | `uri`, `pin` |
| `player.pause` | Pause playback | - |
| `player.toggle` | Toggle play/pause | - |
| `player.next` | Skip to next track | - |
| `player.previous` | Go to previous track | - |
| `player.seek` | Seek to position | `position` |
| `player.volume` | Set volume | `percent` |
| `player.shuffle` | Set shuffle | `state` |
| `player.repeat` | Set repeat mode | `mode` |
| `player.devices` | List devices | - |
| `player.transfer` | Transfer playback | `device` |
| `player.recent` | Recently played | - |

### Queue

| Method | Description | Params |
|--------|-------------|--------|
| `queue.list` | Get queue | - |
| `queue.add` | Add to queue | `uri`, `now_playing` |

### Search

| Method | Description | Params |
|--------|-------------|--------|
| `search` | Search Spotify | `query`, `types[]`, `limit`, `pins_only`, `exact`, `play`, `artist`, `album`, `track`, `year`, `genre`, `isrc`, `upc`, `new`, `hipster` |

### Pin

| Method | Description | Params |
|--------|-------------|--------|
| `pin.add` | Add pin | `type`, `id`, `alias`, `tags` |
| `pin.remove` | Remove pin | `id` |
| `pin.list` | List pins | `type` |

### Playlist

| Method | Description | Params |
|--------|-------------|--------|
| `playlist.list` | List playlists | `limit`, `offset` |
| `playlist.get` | Get playlist | `id` |
| `playlist.create` | Create playlist | `name`, `description`, `public` |
| `playlist.add` | Add tracks | `id`, `uris[]`, `now_playing`, `position`, `dry_run` |
| `playlist.remove` | Remove tracks | `id`, `uris[]`, `dry_run` |
| `playlist.edit` | Edit playlist | `id`, `name`, `description`, `public` |
| `playlist.reorder` | Reorder tracks | `id`, `from`, `to`, `count` |
| `playlist.follow` | Follow playlist | `id`, `public` |
| `playlist.unfollow` | Unfollow playlist | `id` |
| `playlist.duplicate` | Duplicate playlist | `id`, `name` |
| `playlist.cover` | Get cover image | `id` |
| `playlist.user` | Get user's playlists | `user_id` |

### Library

| Method | Description | Params |
|--------|-------------|--------|
| `library.list` | List saved tracks | `limit`, `offset` |
| `library.save` | Save tracks | `ids[]`, `now_playing`, `dry_run` |
| `library.remove` | Remove tracks | `ids[]`, `dry_run` |
| `library.check` | Check if saved | `ids[]` |

### Info

| Method | Description | Params |
|--------|-------------|--------|
| `info.track` | Get track info | `id`, `id_only` |
| `info.album` | Get album info | `id`, `id_only` |
| `info.artist` | Get artist info | `id`, `id_only`, `view`, `market`, `limit`, `offset` |

Artist `view` options: `details`, `top_tracks`, `albums`, `related`

### User

| Method | Description | Params |
|--------|-------------|--------|
| `user.profile` | Get current user profile | - |
| `user.top` | Get top items | `type`, `range`, `limit` |
| `user.get` | Get user profile | `id` |

### Show (Podcasts)

| Method | Description | Params |
|--------|-------------|--------|
| `show.get` | Get show | `id` |
| `show.episodes` | Get episodes | `id`, `limit`, `offset` |
| `show.list` | List saved shows | `limit`, `offset` |
| `show.save` | Save shows | `ids[]` |
| `show.remove` | Remove shows | `ids[]` |
| `show.check` | Check if saved | `ids[]` |

### Episode

| Method | Description | Params |
|--------|-------------|--------|
| `episode.get` | Get episode | `id` |
| `episode.list` | List saved episodes | `limit`, `offset` |
| `episode.save` | Save episodes | `ids[]` |
| `episode.remove` | Remove episodes | `ids[]` |
| `episode.check` | Check if saved | `ids[]` |

### Audiobook

| Method | Description | Params |
|--------|-------------|--------|
| `audiobook.get` | Get audiobook | `id` |
| `audiobook.chapters` | Get chapters | `id`, `limit`, `offset` |
| `audiobook.list` | List saved audiobooks | `limit`, `offset` |
| `audiobook.save` | Save audiobooks | `ids[]` |
| `audiobook.remove` | Remove audiobooks | `ids[]` |
| `audiobook.check` | Check if saved | `ids[]` |

### Album

| Method | Description | Params |
|--------|-------------|--------|
| `album.list` | List saved albums | `limit`, `offset` |
| `album.tracks` | Get album tracks | `id`, `limit`, `offset` |
| `album.save` | Save albums | `ids[]` |
| `album.remove` | Remove albums | `ids[]` |
| `album.check` | Check if saved | `ids[]` |
| `album.newReleases` | Get new releases | `limit`, `offset` |

### Chapter

| Method | Description | Params |
|--------|-------------|--------|
| `chapter.get` | Get chapter | `id` |

### Category

| Method | Description | Params |
|--------|-------------|--------|
| `category.list` | List categories | `limit`, `offset` |
| `category.get` | Get category | `id` |
| `category.playlists` | Get category playlists | `id`, `limit`, `offset` |

### Follow

| Method | Description | Params |
|--------|-------------|--------|
| `follow.artist` | Follow artists | `ids[]`, `dry_run` |
| `follow.user` | Follow users | `ids[]`, `dry_run` |
| `follow.unfollowArtist` | Unfollow artists | `ids[]`, `dry_run` |
| `follow.unfollowUser` | Unfollow users | `ids[]`, `dry_run` |
| `follow.list` | List followed artists | `limit` |
| `follow.checkArtist` | Check if following artists | `ids[]` |
| `follow.checkUser` | Check if following users | `ids[]` |

### Markets

| Method | Description | Params |
|--------|-------------|--------|
| `markets.list` | List available markets | - |

## Events

The daemon broadcasts real-time events to connected clients:

| Event | Description |
|-------|-------------|
| `event.trackChanged` | Track changed |
| `event.playbackStateChanged` | Play/pause state changed |
| `event.volumeChanged` | Volume changed |
| `event.shuffleChanged` | Shuffle state changed |
| `event.repeatChanged` | Repeat mode changed |
| `event.deviceChanged` | Active device changed |

Events are polled every 2 seconds from the Spotify API.

## Examples

### Basic Commands

```bash
# Ping
echo '{"jsonrpc":"2.0","method":"ping","id":1}' | nc -U ~/.config/spotify-cli/daemon.sock

# Get current playback
echo '{"jsonrpc":"2.0","method":"player.status","id":1}' | nc -U ~/.config/spotify-cli/daemon.sock

# Skip track
echo '{"jsonrpc":"2.0","method":"player.next","id":1}' | nc -U ~/.config/spotify-cli/daemon.sock

# Set volume
echo '{"jsonrpc":"2.0","method":"player.volume","params":{"percent":50},"id":1}' | nc -U ~/.config/spotify-cli/daemon.sock
```

### Search

```bash
echo '{"jsonrpc":"2.0","method":"search","params":{"query":"daft punk","types":["track"],"limit":5},"id":1}' | nc -U ~/.config/spotify-cli/daemon.sock
```

### Play by URI

```bash
echo '{"jsonrpc":"2.0","method":"player.play","params":{"uri":"spotify:track:4uLU6hMCjMI75M1A2tKUQC"},"id":1}' | nc -U ~/.config/spotify-cli/daemon.sock
```

### Get User Profile

```bash
echo '{"jsonrpc":"2.0","method":"user.profile","id":1}' | nc -U ~/.config/spotify-cli/daemon.sock
```

## Integration Examples

### Shell Script

```bash
#!/bin/bash
SOCKET=~/.config/spotify-cli/daemon.sock

rpc() {
    echo "$1" | nc -U "$SOCKET"
}

# Get current track
rpc '{"jsonrpc":"2.0","method":"player.status","id":1}' | jq -r '.result.payload.item.name'

# Toggle playback
rpc '{"jsonrpc":"2.0","method":"player.toggle","id":1}'
```

### Python

```python
import socket
import json

def rpc_call(method, params=None, id=1):
    sock = socket.socket(socket.AF_UNIX, socket.SOCK_STREAM)
    sock.connect("/Users/you/.config/spotify-cli/daemon.sock")

    request = {"jsonrpc": "2.0", "method": method, "id": id}
    if params:
        request["params"] = params

    sock.send((json.dumps(request) + "\n").encode())
    response = sock.recv(65536).decode()
    sock.close()

    return json.loads(response)

# Get current track
status = rpc_call("player.status")
print(status["result"]["payload"]["item"]["name"])

# Skip track
rpc_call("player.next")
```

### Neovim (Lua)

```lua
local socket = require("socket.unix")

local function spotify_rpc(method, params)
    local sock = socket()
    sock:connect("/Users/you/.config/spotify-cli/daemon.sock")

    local request = vim.fn.json_encode({
        jsonrpc = "2.0",
        method = method,
        params = params,
        id = 1
    })

    sock:send(request .. "\n")
    local response = sock:receive("*l")
    sock:close()

    return vim.fn.json_decode(response)
end

-- Skip track
vim.keymap.set("n", "<leader>sn", function()
    spotify_rpc("player.next")
end)

-- Toggle playback
vim.keymap.set("n", "<leader>st", function()
    spotify_rpc("player.toggle")
end)
```

## Error Handling

| HTTP Code | Meaning |
|-----------|---------|
| 200 | Success |
| 400 | Bad request / validation error |
| 401 | Unauthorized / token expired |
| 403 | Forbidden |
| 404 | Not found / method not found |
| 429 | Rate limited |
| 500 | Internal error |

## Testing

The RPC module includes 32 automated tests:

```bash
# Run all RPC tests
cargo test rpc

# Run dispatcher tests
cargo test dispatch::

# Run integration tests
cargo test --test rpc_tests
```

## Source Files

```
src/rpc/
├── mod.rs           # Module exports
├── protocol.rs      # JSON-RPC 2.0 types
├── server.rs        # Unix socket server
├── dispatch.rs      # Method → command routing (68 methods)
└── events.rs        # Event polling & broadcasting
```
