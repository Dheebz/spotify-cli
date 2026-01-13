//! Episode command handlers

use crate::endpoints::episodes::{
    check_users_saved_episodes, get_episode, get_users_saved_episodes,
    remove_users_saved_episodes, save_episodes_for_current_user,
};

// Generate standard resource commands using macros
resource_get!(episode_get, get_episode::get_episode, "Episode");
resource_list!(episode_list, get_users_saved_episodes::get_users_saved_episodes, "Saved episodes");
resource_save!(episode_save, save_episodes_for_current_user::save_episodes, "episode");
resource_remove!(episode_remove, remove_users_saved_episodes::remove_episodes, "episode");
resource_check!(episode_check, check_users_saved_episodes::check_saved_episodes);
