//! Show (podcast) command handlers

use crate::endpoints::shows::{
    check_users_saved_shows, get_show, get_show_episodes, get_users_saved_shows,
    remove_users_saved_shows, save_shows_for_current_user,
};

// Generate standard resource commands using macros
resource_get!(show_get, get_show::get_show, "Show");
resource_list!(show_list, get_users_saved_shows::get_users_saved_shows, "Saved shows");
resource_list_with_id!(show_episodes, get_show_episodes::get_show_episodes, "Show episodes", "No episodes");
resource_save!(show_save, save_shows_for_current_user::save_shows, "show");
resource_remove!(show_remove, remove_users_saved_shows::remove_shows, "show");
resource_check!(show_check, check_users_saved_shows::check_saved_shows);
