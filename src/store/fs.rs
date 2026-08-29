// SPDX-License-Identifier: GPL-3.0-only

//! The notes directory: plain `.md` files, with `.trash/` for trashed notes.

use crate::note::slug_filename;
use anyhow::{Context, Result};
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::time::SystemTime;

pub const TRASH_DIR: &str = ".trash";

#[derive(Debug, Clone)]
pub struct NotesDir {
    root: PathBuf,
}

#[derive(Debug, Clone)]
pub struct DiskEntry {
    pub path: PathBuf,
    pub trashed: bool,
    pub modified: SystemTime,
}

impl NotesDir {
    pub fn open(root: PathBuf) -> Result<Self> {
        fs::create_dir_all(&root)
            .with_context(|| format!("creating notes dir {}", root.display()))?;
        fs::create_dir_all(root.join(TRASH_DIR)).context("creating trash dir")?;
        Ok(Self { root })
    }

    pub fn root(&self) -> &Path {
        &self.root
    }

    pub fn trash_dir(&self) -> PathBuf {
        self.root.join(TRASH_DIR)
    }

    /// Every `.md` file in the root (not recursive — subfolders are left to the
    /// user) plus everything in `.trash/`.
    pub fn scan(&self) -> Result<Vec<DiskEntry>> {
        let mut out = Vec::new();
        for (dir, trashed) in [(self.root.clone(), false), (self.trash_dir(), true)] {
            let entries = match fs::read_dir(&dir) {
                Ok(e) => e,
                Err(err) if err.kind() == std::io::ErrorKind::NotFound => continue,
                Err(err) => return Err(err).with_context(|| format!("reading {}", dir.display())),
            };
            for entry in entries {
                let entry = entry?;
                let path = entry.path();
                if !path.is_file() || path.extension().and_then(|e| e.to_str()) != Some("md") {
                    continue;
                }
                if path
                    .file_name()
                    .and_then(|n| n.to_str())
                    .is_some_and(|n| n.starts_with('.'))
                {
                    continue;
                }
                let modified = entry
                    .metadata()?
                    .modified()
                    .unwrap_or(SystemTime::UNIX_EPOCH);
                out.push(DiskEntry {
                    path,
                    trashed,
                    modified,
                });
            }
        }
        Ok(out)
    }

    pub fn read(&self, path: &Path) -> Result<String> {
        fs::read_to_string(path).with_context(|| format!("reading {}", path.display()))
    }

    /// Write via a temp file + rename so a crash never leaves a half-written note.
    pub fn write_atomic(&self, path: &Path, text: &str) -> Result<()> {
        let dir = path.parent().context("note path has no parent")?;
        fs::create_dir_all(dir)?;
        let tmp = dir.join(format!(
            ".{}.tmp",
            path.file_name().and_then(|n| n.to_str()).unwrap_or("note")
        ));
        {
            let mut f =
                fs::File::create(&tmp).with_context(|| format!("creating {}", tmp.display()))?;
            f.write_all(text.as_bytes())?;
            f.sync_all()?;
        }
        fs::rename(&tmp, path).with_context(|| format!("renaming into {}", path.display()))?;
        Ok(())
    }

    /// Pick a filename for `title` inside `dir` that doesn't collide with any
    /// other note. `current` is the note's existing path, which is allowed.
    pub fn unique_path(&self, dir: &Path, title: &str, current: Option<&Path>) -> PathBuf {
        let stem = slug_filename(title);
        let candidate = dir.join(format!("{stem}.md"));
        if Some(candidate.as_path()) == current || !candidate.exists() {
            return candidate;
        }
        for n in 2..10_000 {
            let candidate = dir.join(format!("{stem} ({n}).md"));
            if Some(candidate.as_path()) == current || !candidate.exists() {
                return candidate;
            }
        }
        dir.join(format!("{stem} {}.md", crate::note::new_id()))
    }

    pub fn rename(&self, from: &Path, to: &Path) -> Result<()> {
        if from == to {
            return Ok(());
        }
        fs::rename(from, to)
            .with_context(|| format!("moving {} -> {}", from.display(), to.display()))
    }

    pub fn remove(&self, path: &Path) -> Result<()> {
        match fs::remove_file(path) {
            Ok(()) => Ok(()),
            Err(err) if err.kind() == std::io::ErrorKind::NotFound => Ok(()),
            Err(err) => Err(err).with_context(|| format!("deleting {}", path.display())),
        }
    }
}
