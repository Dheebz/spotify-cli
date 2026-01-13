//! Audiobook command handlers

use crate::endpoints::audiobooks::{
    check_users_saved_audiobooks, get_audiobook, get_audiobook_chapters,
    get_users_saved_audiobooks, remove_users_saved_audiobooks, save_audiobooks_for_current_user,
};

resource_get!(audiobook_get, get_audiobook::get_audiobook, "Audiobook");
resource_list!(audiobook_list, get_users_saved_audiobooks::get_users_saved_audiobooks, "Saved audiobooks");
resource_list_with_id!(audiobook_chapters, get_audiobook_chapters::get_audiobook_chapters, "Audiobook chapters", "No chapters");
resource_save!(audiobook_save, save_audiobooks_for_current_user::save_audiobooks, "audiobook");
resource_remove!(audiobook_remove, remove_users_saved_audiobooks::remove_audiobooks, "audiobook");
resource_check!(audiobook_check, check_users_saved_audiobooks::check_saved_audiobooks);
