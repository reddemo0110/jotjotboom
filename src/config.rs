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
