//! Player command modules

mod playback;
mod queue;
mod settings;
mod state;

pub use playback::{player_next, player_pause, player_play, player_previous, player_toggle};
pub use queue::{player_queue_add, player_queue_list, player_recent};
pub use settings::{player_repeat, player_seek, player_shuffle, player_volume};
pub use state::{player_devices_list, player_devices_transfer, player_status};
