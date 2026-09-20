// SPDX-License-Identifier: GPL-3.0-only

//! A backend that lives in memory, for tests: the same contract as the real
//! ones (server-owned revisions, stale writes refused, one record per note
//! id or file key), shared between "devices" through a handle.

use super::Session;
use super::backend::{Backend, FileBody, FilePush, FileUpload, NotePush, NoteUpload, Page, RawFile, RawNote};
use anyhow::Result;
use std::path::Path;
use std::sync::{Arc, Mutex};

#[derive(Default)]
struct Server {
    /// Bumped on every write; a record's `seq` is the pull cursor.
    seq: u64,
    notes: Vec<(u64, RawNote)>,
    files: Vec<(u64, RawFile, Vec<u8>)>,
}

#[derive(Clone, Default)]
pub struct Memory {
    server: Arc<Mutex<Server>>,
    /// Flip to make `authorize` refuse.
    pub signed_out: bool,
}

fn cursor(seq: u64) -> String {
    format!("{seq:020}")
}

fn since(c: &str) -> u64 {
    c.parse().unwrap_or(0)
}

impl Backend for Memory {
    fn authorize(&mut self) -> Result<Option<Session>> {
        Ok((!self.signed_out).then(|| Session {
            url: "memory".into(),
            token: "token".into(),
            user_id: "user".into(),
            email: "user@example.com".into(),
        }))
    }

    fn pull_notes(&mut self, c: &str) -> Result<Page<RawNote>> {
        let server = self.server.lock().unwrap();
        let mut items: Vec<&(u64, RawNote)> = server.notes.iter().filter(|(seq, _)| *seq >= since(c)).collect();
        items.sort_by_key(|(seq, _)| *seq);
        Ok(Page {
            cursor: items.last().map_or_else(|| c.to_owned(), |(seq, _)| cursor(*seq)),
            items: items.into_iter().map(|(_, n)| n.clone()).collect(),
        })
    }

    fn push_note(&mut self, up: &NoteUpload) -> Result<NotePush> {
        let mut server = self.server.lock().unwrap();
        server.seq += 1;
        let seq = server.seq;
        let existing = server.notes.iter().position(|(_, n)| n.note_id == up.note_id);
        let revision = match (up.record_id, existing) {
            (None, Some(i)) => return Ok(NotePush::Conflict(Some(server.notes[i].1.clone()))),
            (Some(_), None) => return Ok(NotePush::Conflict(None)),
            (Some(_), Some(i)) if server.notes[i].1.revision != up.base_revision => {
                return Ok(NotePush::Conflict(Some(server.notes[i].1.clone())));
            }
            (Some(_), Some(i)) => {
                let revision = server.notes[i].1.revision + 1;
                server.notes.remove(i);
                revision
            }
            (None, None) => 1,
        };
        let record_id = format!("n-{}", up.note_id);
        server.notes.push((
            seq,
            RawNote {
                record_id: record_id.clone(),
                note_id: up.note_id.to_owned(),
                revision,
                device: up.device.to_owned(),
                modified: up.modified.to_owned(),
                blob: up.blob.to_owned(),
            },
        ));
        Ok(NotePush::Landed {
            record_id,
            revision,
            cursor: None,
        })
    }

    fn list_files(&mut self, c: &str) -> Result<Page<RawFile>> {
        let server = self.server.lock().unwrap();
        let mut items: Vec<&(u64, RawFile, Vec<u8>)> =
            server.files.iter().filter(|(seq, ..)| *seq >= since(c)).collect();
        items.sort_by_key(|(seq, ..)| *seq);
        Ok(Page {
            cursor: items.last().map_or_else(|| c.to_owned(), |(seq, ..)| cursor(*seq)),
            items: items.into_iter().map(|(_, f, _)| f.clone()).collect(),
        })
    }

    fn file_by_key(&mut self, key: &str) -> Result<Option<RawFile>> {
        let server = self.server.lock().unwrap();
        Ok(server.files.iter().find(|(_, f, _)| f.key == key).map(|(_, f, _)| f.clone()))
    }

    fn download(&mut self, file: &RawFile, dest: &Path) -> Result<()> {
        let server = self.server.lock().unwrap();
        let Some((_, _, bytes)) = server.files.iter().find(|(_, f, _)| f.record_id == file.record_id) else {
            anyhow::bail!("no such file");
        };
        std::fs::write(dest, bytes)?;
        Ok(())
    }

    fn upload(&mut self, up: &FileUpload) -> Result<FilePush> {
        let bytes = match up.body {
            FileBody::Bytes(b) => b.to_vec(),
            FileBody::Path(p) => std::fs::read(p)?,
        };
        let mut server = self.server.lock().unwrap();
        server.seq += 1;
        let seq = server.seq;
        let existing = server.files.iter().position(|(_, f, _)| f.key == up.key);
        let revision = match (up.record_id, existing) {
            (None, Some(_)) => return Ok(FilePush::Taken),
            (Some(_), None) => return Ok(FilePush::Gone),
            (Some(_), Some(i)) if server.files[i].1.revision != up.base_revision => return Ok(FilePush::Taken),
            (Some(_), Some(i)) => {
                let revision = server.files[i].1.revision + 1;
                server.files.remove(i);
                revision
            }
            (None, None) => 1,
        };
        let record_id = format!("f-{}", up.key);
        server.files.push((
            seq,
            RawFile {
                record_id: record_id.clone(),
                key: up.key.to_owned(),
                revision,
                meta: up.meta.to_owned(),
                locator: String::new(),
            },
            bytes,
        ));
        Ok(FilePush::Landed { record_id, revision })
    }
}
