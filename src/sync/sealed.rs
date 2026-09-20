// SPDX-License-Identifier: GPL-3.0-only

//! Encryption as a backend around a backend: everything the cycle hands
//! down is sealed on the way out and opened on the way in, so the cycle
//! and the store never know, and any backend gets it for free.
//!
//! What the inner backend still sees: note ids (random UUIDs), revisions,
//! device ids, timestamps, sizes and how many of everything there are.
//! What it no longer sees: note text, titles, tags, trash state, file
//! names (keys become a keyed hash), file meta, file contents.

// Only `probe` is wired into the app so far; the rest waits for the
// passphrase field in Options → Sync.
#![allow(dead_code)]

use super::Session;
use super::backend::{Backend, FileBody, FilePush, FileUpload, NotePush, NoteUpload, Page, RawFile, RawNote};
use super::files::{Meta, key_for};
use super::seal::{Key, KeyFile};
use anyhow::{Context, Result};
use std::collections::HashMap;
use std::path::{Path, PathBuf};

/// Where an encrypted account keeps its [`KeyFile`]: a file record with
/// this key, the settings in `meta`, no bytes worth reading.
pub const KEYFILE_KEY: &str = "jjb-keyfile";

/// The account's key settings, if it is an encrypted one. Ask before the
/// first cycle: `Some` means "get the passphrase, wrap the backend".
pub fn probe(backend: &mut dyn Backend) -> Result<Option<KeyFile>> {
    backend
        .file_by_key(KEYFILE_KEY)?
        .map(|f| KeyFile::decode(&f.meta))
        .transpose()
}

/// Turn encryption on for an account that has nothing in it yet.
pub fn enroll(backend: &mut dyn Backend, device: &str, passphrase: &str) -> Result<Key> {
    if let Some(existing) = probe(backend)? {
        return existing.unlock(passphrase);
    }
    let (file, key) = KeyFile::create(passphrase)?;
    store_keyfile(backend, device, &file)?;
    Ok(key)
}

pub fn store_keyfile(backend: &mut dyn Backend, device: &str, file: &KeyFile) -> Result<()> {
    let pushed = backend.upload(&FileUpload {
        key: KEYFILE_KEY,
        record_id: None,
        base_revision: 0,
        device,
        modified: "",
        meta: &file.encode(),
        body: FileBody::Bytes(b"jotjotboom"),
    })?;
    match pushed {
        FilePush::Landed { .. } => Ok(()),
        // Another device got there first: theirs is the account's key.
        FilePush::Taken | FilePush::Gone => anyhow::bail!("this account already has a passphrase; enter that one"),
    }
}

pub struct Sealed<B: Backend> {
    inner: B,
    key: Key,
    /// Ciphertext on its way up or down rests here (never plaintext).
    scratch: PathBuf,
    /// plain key → sealed key, learnt from uploads and listings, so the
    /// cycle can keep asking by the key it knows.
    names: HashMap<String, String>,
}

impl<B: Backend> Sealed<B> {
    pub fn new(inner: B, key: Key, scratch: PathBuf) -> Self {
        Self {
            inner,
            key,
            scratch,
            names: HashMap::new(),
        }
    }

    fn scratch_file(&self, name: &str) -> Result<PathBuf> {
        std::fs::create_dir_all(&self.scratch).context("creating the scratch dir")?;
        Ok(self.scratch.join(format!("{name}.sealed")))
    }

    fn open_note(&self, mut n: RawNote) -> Result<RawNote> {
        n.blob = self.key.open_text(&n.blob).with_context(|| format!("note {}", n.note_id))?;
        Ok(n)
    }

    /// A listed record as the cycle expects it: meta opened, key turned
    /// back into the plain one. `None` for the key file and for anything
    /// that will not open (logged; one bad record must not stop the rest).
    fn open_file(&mut self, mut f: RawFile) -> Option<RawFile> {
        if f.key == KEYFILE_KEY {
            return None;
        }
        let meta = match self.key.open_text(&f.meta) {
            Ok(m) => m,
            Err(err) => {
                tracing::warn!(%err, key = f.key, "skipping a file record that does not open");
                return None;
            }
        };
        let path = serde_json::from_str::<Meta>(&meta).ok()?.path;
        // The record must sit under the name its own meta claims.
        if self.key.name(&path) != f.key {
            tracing::warn!(key = f.key, "skipping a file record filed under the wrong name");
            return None;
        }
        let plain_key = key_for(&path);
        self.names.insert(plain_key.clone(), f.key.clone());
        f.key = plain_key;
        f.meta = meta;
        Some(f)
    }
}

impl<B: Backend> Backend for Sealed<B> {
    fn authorize(&mut self) -> Result<Option<Session>> {
        self.inner.authorize()
    }

    fn pull_notes(&mut self, cursor: &str) -> Result<Page<RawNote>> {
        let page = self.inner.pull_notes(cursor)?;
        let items = page
            .items
            .into_iter()
            .filter_map(|n| match self.open_note(n) {
                Ok(n) => Some(n),
                Err(err) => {
                    tracing::warn!(%err, "skipping a note that does not open");
                    None
                }
            })
            .collect();
        Ok(Page {
            items,
            cursor: page.cursor,
        })
    }

    fn push_note(&mut self, up: &NoteUpload) -> Result<NotePush> {
        let blob = self.key.seal_text(up.blob)?;
        let pushed = self.inner.push_note(&NoteUpload { blob: &blob, ..up.clone() })?;
        Ok(match pushed {
            NotePush::Conflict(Some(theirs)) => NotePush::Conflict(Some(self.open_note(theirs)?)),
            other => other,
        })
    }

    fn list_files(&mut self, cursor: &str) -> Result<Page<RawFile>> {
        let page = self.inner.list_files(cursor)?;
        let items = page.items.into_iter().filter_map(|f| self.open_file(f)).collect();
        Ok(Page {
            items,
            cursor: page.cursor,
        })
    }

    fn file_by_key(&mut self, key: &str) -> Result<Option<RawFile>> {
        let Some(sealed_key) = self.names.get(key).cloned() else {
            return Ok(None);
        };
        Ok(self.inner.file_by_key(&sealed_key)?.and_then(|f| self.open_file(f)))
    }

    fn download(&mut self, file: &RawFile, dest: &Path) -> Result<()> {
        let tmp = self.scratch_file(&file.record_id)?;
        let result = self
            .inner
            .download(file, &tmp)
            .and_then(|()| self.key.open_file(&tmp, dest));
        std::fs::remove_file(&tmp).ok();
        result
    }

    fn upload(&mut self, up: &FileUpload) -> Result<FilePush> {
        let path = serde_json::from_str::<Meta>(up.meta)
            .context("reading file details")?
            .path;
        let sealed_key = self.key.name(&path);
        self.names.insert(up.key.to_owned(), sealed_key.clone());
        let meta = self.key.seal_text(up.meta)?;
        let tmp = self.scratch_file(&sealed_key)?;
        let sealed = match up.body {
            FileBody::Bytes(mut bytes) => self.key.seal_file(&mut bytes, &tmp),
            FileBody::Path(p) => std::fs::File::open(p)
                .map_err(Into::into)
                .and_then(|mut f| self.key.seal_file(&mut f, &tmp)),
        };
        let result = sealed.and_then(|()| {
            self.inner.upload(&FileUpload {
                key: &sealed_key,
                meta: &meta,
                body: FileBody::Path(&tmp),
                ..up.clone()
            })
        });
        std::fs::remove_file(&tmp).ok();
        result
    }
}
