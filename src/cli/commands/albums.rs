//! Album command handlers

use crate::endpoints::albums::{
    check_users_saved_albums, get_album_tracks, get_new_releases, get_users_saved_albums,
    remove_users_saved_albums, save_albums_for_current_user,
};

resource_list!(album_list, get_users_saved_albums::get_users_saved_albums, "Saved albums");
resource_list_with_id!(album_tracks, get_album_tracks::get_album_tracks, "Album tracks", "No tracks");
resource_save!(album_save, save_albums_for_current_user::save_albums, "album");
resource_remove!(album_remove, remove_users_saved_albums::remove_albums, "album");
resource_check!(album_check, check_users_saved_albums::check_saved_albums);
resource_list!(album_new_releases, get_new_releases::get_new_releases, "New releases");
