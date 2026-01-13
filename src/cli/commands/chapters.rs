//! Chapter command handlers

use crate::endpoints::chapters::get_chapter;

// Generate standard resource commands using macros
resource_get!(chapter_get, get_chapter::get_chapter, "Chapter");
