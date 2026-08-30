// SPDX-License-Identifier: GPL-3.0-only

use cosmic::cosmic_config::{self, CosmicConfigEntry, cosmic_config_derive::CosmicConfigEntry};
use std::path::PathBuf;

#[derive(Debug, Default, Clone, CosmicConfigEntry, Eq, PartialEq)]
#[version = 1]
pub struct Config {
    /// Where the `.md` files live. Empty string = default (`~/Documents/JotJotBoom`).
    pub notes_dir: String,
    /// Stable per-installation id, stamped into the oplog for sync. Generated on first run.
    pub device_id: String,
    /// Retro theme key (see `retro::Theme::key`). Empty = default.
    pub theme: String,
    /// Paint markdown syntax markers visibly (dim) instead of ghosted.
    pub show_markers: bool,
    /// Collapse the views/tags column. (Inverted so the default is shown.)
    pub hide_nav: bool,
    /// Collapse the notes list.
    pub hide_list: bool,
    /// Tags whose sub-tags are folded away in the sidebar (full paths).
    pub collapsed_tags: Vec<String>,
    /// Editor font key (see `retro::EditorFont::key`). Empty = system monospace.
    pub editor_font: String,
    /// Sidebar + notes-list font key. Empty = system monospace.
    pub ui_font: String,
    /// Pane-title font key. Empty = VT323.
    pub title_font: String,
    /// Editor text size in px. 0 = default.
    pub editor_font_size: u16,
    /// Sidebar (views/tags) and notes-list text sizes in px. 0 = default.
    pub sidebar_font_size: u16,
    pub list_font_size: u16,
    /// Dock size key: small, medium, large, wow. Empty = medium.
    pub dock_size: String,
    /// What goes inside the brackets of a finished task. Empty = "x".
    pub task_marker: String,
    /// Widest the note text gets before it centres: narrow, medium, wide, full.
    pub text_width: String,
    /// Launcher icon: a theme key, or empty to follow the colour theme.
    pub icon_theme: String,
    /// The hidden "Long Black" theme has been found (search for "coffee").
    pub coffee_unlocked: bool,
}

impl Config {
    /// Resolve the notes directory, falling back to `~/Documents/JotJotBoom`.
    pub fn notes_dir(&self) -> PathBuf {
        if !self.notes_dir.trim().is_empty() {
            return PathBuf::from(shellexpand_home(self.notes_dir.trim()));
        }
        dirs::document_dir()
            .or_else(dirs::home_dir)
            .unwrap_or_else(|| PathBuf::from("."))
            .join("JotJotBoom")
    }

    /// The derived SQLite index. Safe to delete — rebuilt from the notes dir on start.
    pub fn index_path(app_id: &str) -> PathBuf {
        dirs::data_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join(app_id)
            .join("index.db")
    }
}

fn shellexpand_home(path: &str) -> String {
    if let Some(rest) = path.strip_prefix("~/")
        && let Some(home) = dirs::home_dir()
    {
        return home.join(rest).to_string_lossy().into_owned();
    }
    path.to_owned()
}
