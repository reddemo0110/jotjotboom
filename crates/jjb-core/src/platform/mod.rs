// SPDX-License-Identifier: GPL-3.0-only

//! What differs between operating systems, behind one trait: where notes
//! and the index live, the keyring, opening a URL. Everything else in the
//! core is the same bytes on every platform.
//!
//! Single-instance is deliberately not here: it belongs to the shell
//! (libcosmic's D-Bus activation on COSMIC and GNOME, Tauri's single-instance
//! plugin on Windows), because the shell is what receives the forwarded
//! arguments and raises the window.

use anyhow::Result;
use std::path::PathBuf;

#[cfg(target_os = "linux")]
mod linux;
#[cfg(not(target_os = "linux"))]
mod windows;

/// The folder name under the user's Documents.
pub const NOTES_FOLDER: &str = "JotJotBoom";

pub trait Platform: Send + Sync {
    /// A short name for logs and the About box ("linux", "windows").
    fn name(&self) -> &'static str;

    /// Where notes go when the user has not chosen a folder.
    fn default_notes_dir(&self) -> PathBuf;

    /// The per-user data directory for `app_id`, where the derived
    /// `index.db` lives. Safe to delete; rebuilt from the notes dir.
    fn data_dir(&self, app_id: &str) -> PathBuf;

    /// Store a secret under `key`, replacing any existing value. `label` is
    /// what the user sees in their keyring manager.
    fn store_secret(&self, key: &str, label: &str, secret: &[u8]) -> Result<()>;

    /// The secret stored under `key`, if any.
    fn get_secret(&self, key: &str) -> Result<Option<Vec<u8>>>;

    /// Remove the secret stored under `key`. No-op if absent.
    fn delete_secret(&self, key: &str) -> Result<()>;

    /// Open a URL or a file in whatever the desktop uses for it, without
    /// waiting for it to close.
    fn open(&self, target: &str) -> Result<()>;
}

/// The platform this binary was built for.
pub fn current() -> &'static dyn Platform {
    #[cfg(target_os = "linux")]
    {
        &linux::Linux
    }
    #[cfg(not(target_os = "linux"))]
    {
        &windows::Windows
    }
}

/// Resolve the notes directory from the user's setting (`""` = default).
/// `JJB_NOTES_DIR` wins over both, so the screenshot harness and tests can
/// point a run at a scratch folder without touching real notes.
pub fn notes_dir(configured: &str) -> PathBuf {
    if let Some(dir) = std::env::var_os("JJB_NOTES_DIR").filter(|d| !d.is_empty()) {
        return PathBuf::from(dir);
    }
    let configured = configured.trim();
    if !configured.is_empty() {
        return expand_home(configured);
    }
    current().default_notes_dir()
}

/// The derived SQLite index for `app_id`.
pub fn index_path(app_id: &str) -> PathBuf {
    current().data_dir(app_id).join("index.db")
}

/// `~/x` → the user's home + `x`. Other paths pass through.
pub fn expand_home(path: &str) -> PathBuf {
    if let Some(rest) = path.strip_prefix("~/").or_else(|| path.strip_prefix("~\\"))
        && let Some(home) = dirs::home_dir()
    {
        return home.join(rest);
    }
    PathBuf::from(path)
}

/// The Documents folder, or home, or the working directory.
fn documents_or_home() -> PathBuf {
    dirs::document_dir()
        .or_else(dirs::home_dir)
        .unwrap_or_else(|| PathBuf::from("."))
}

/// The per-user *local* data root, or the working directory. Local, not
/// roaming: on Windows `dirs::data_dir` is `%APPDATA%` (Roaming) and the
/// derived index must never follow a profile between machines. On Linux
/// both are `$XDG_DATA_HOME`.
fn data_root() -> PathBuf {
    dirs::data_local_dir().unwrap_or_else(|| PathBuf::from("."))
}

/// Open `target` with the desktop's handler, detached.
fn open_detached(target: &str) -> Result<()> {
    open::that_detached(target)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn env_override_wins() {
        // Set-and-restore so other tests are not disturbed.
        let before = std::env::var_os("JJB_NOTES_DIR");
        // SAFETY: tests in this module are the only writers of this variable.
        unsafe { std::env::set_var("JJB_NOTES_DIR", "/tmp/jjb-scratch") };
        assert_eq!(notes_dir("~/Notes"), PathBuf::from("/tmp/jjb-scratch"));
        unsafe {
            match before {
                Some(v) => std::env::set_var("JJB_NOTES_DIR", v),
                None => std::env::remove_var("JJB_NOTES_DIR"),
            }
        }
    }

    #[test]
    fn home_expands() {
        let home = dirs::home_dir().unwrap();
        assert_eq!(expand_home("~/Notes"), home.join("Notes"));
        assert_eq!(expand_home("/abs/Notes"), PathBuf::from("/abs/Notes"));
    }

    #[test]
    fn default_ends_in_the_app_folder() {
        assert!(current().default_notes_dir().ends_with(NOTES_FOLDER));
        assert!(index_path("io.test").ends_with("io.test/index.db"));
    }
}
