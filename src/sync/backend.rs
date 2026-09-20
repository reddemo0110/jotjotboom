// SPDX-License-Identifier: GPL-3.0-only

//! The seam between the sync cycle and whatever holds the other copy.
//!
//! A [`Backend`] is a dumb, revisioned blob store: notes are opaque strings
//! under a note id, files are opaque bytes plus an opaque `meta` string
//! under a key. Everything that gives those meaning — envelopes, conflict
//! rules, path safety, hashes — lives on this side of the trait, so every
//! backend gets the same behaviour and none of them ever needs to read a
//! note.

use super::Session;
use anyhow::Result;
use std::path::Path;

/// A note record as the backend holds it.
#[derive(Debug, Clone)]
pub struct RawNote {
    pub record_id: String,
    pub note_id: String,
    pub revision: i64,
    pub device: String,
    /// The writer's modified time, RFC 3339.
    pub modified: String,
    pub blob: String,
}

/// Everything changed since a cursor, oldest first, and the cursor to use
/// next time.
#[derive(Debug, Clone, Default)]
pub struct Page<T> {
    pub items: Vec<T>,
    pub cursor: String,
}

#[derive(Debug, Clone)]
pub struct NoteUpload<'a> {
    pub note_id: &'a str,
    /// The record, if this note has been up before.
    pub record_id: Option<&'a str>,
    /// The revision the change was based on; a backend that can refuses a
    /// stale one.
    pub base_revision: i64,
    pub device: &'a str,
    pub modified: &'a str,
    pub blob: &'a str,
}

#[derive(Debug, Clone)]
pub enum NotePush {
    Landed {
        record_id: String,
        revision: i64,
        /// A cursor that already covers this write, when the backend has one.
        cursor: Option<String>,
    },
    /// Someone else wrote first (here is their copy), or the record went
    /// away under us (`None`: start over as a create next time).
    Conflict(Option<RawNote>),
}

/// A file record as the backend holds it.
#[derive(Debug, Clone)]
pub struct RawFile {
    pub record_id: String,
    pub key: String,
    pub revision: i64,
    pub meta: String,
    /// Whatever the backend needs to find the bytes again.
    pub locator: String,
}

#[derive(Debug, Clone, Copy)]
pub enum FileBody<'a> {
    Path(&'a Path),
    Bytes(&'a [u8]),
}

#[derive(Debug, Clone)]
pub struct FileUpload<'a> {
    pub key: &'a str,
    pub record_id: Option<&'a str>,
    pub base_revision: i64,
    pub device: &'a str,
    pub modified: &'a str,
    pub meta: &'a str,
    pub body: FileBody<'a>,
}

#[derive(Debug, Clone)]
pub enum FilePush {
    Landed { record_id: String, revision: i64 },
    /// The key is held by a record we did not base this on.
    Taken,
    /// The record went away under us.
    Gone,
}

pub trait Backend {
    /// Check the sign-in before anything else. `Ok(None)`: refused, sign in
    /// again. `Ok(Some(_))`: carry on, with whatever the token is now.
    fn authorize(&mut self) -> Result<Option<Session>>;

    fn pull_notes(&mut self, cursor: &str) -> Result<Page<RawNote>>;
    fn push_note(&mut self, up: &NoteUpload) -> Result<NotePush>;

    fn list_files(&mut self, cursor: &str) -> Result<Page<RawFile>>;
    fn file_by_key(&mut self, key: &str) -> Result<Option<RawFile>>;
    /// Write the file's bytes to `dest` (the caller checks them).
    fn download(&mut self, file: &RawFile, dest: &Path) -> Result<()>;
    fn upload(&mut self, up: &FileUpload) -> Result<FilePush>;
}
