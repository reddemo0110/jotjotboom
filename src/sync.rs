// SPDX-License-Identifier: GPL-3.0-only

//! Cloud sync. What holds the other copy is a [`backend::Backend`] — today a
//! self-hosted PocketBase server (`server/`, [`pocketbase`]).
//!
//! Files on disk stay the source of truth; the server is a dumb blob store.
//! Each note travels as an [`Envelope`] — the whole file text plus its trash
//! state — under the note's own id. The only things the server sees in the
//! clear are that id, a revision counter it owns, a timestamp and the device
//! that wrote it, which is exactly the boundary end-to-end encryption needs
//! later: encrypting the envelope changes nothing else.
//!
//! One [`run`] is one cycle: refresh the token, pull everything changed
//! since the cursor, push what changed locally. It is blocking (ureq) and
//! meant for `spawn_blocking`; applying the result to the store happens on
//! the app thread, see `Store::apply_remote`. Pictures, attached files and
//! `.folders` follow in the same cycle: see [`files`].

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub mod backend;
pub mod files;
#[cfg(test)]
pub mod memory;
pub mod pocketbase;

use backend::{Backend, NotePush, NoteUpload, RawNote};
use pocketbase::PocketBase;
pub use pocketbase::{refresh, sign_in, sign_up};

/// Keyring key for the bearer token.
pub const TOKEN_KEY: &str = "sync-token";

/// The opaque payload: everything the other device needs to recreate the
/// note. Plain JSON today; the ciphertext of this JSON later.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Envelope {
    /// Format version.
    pub v: u32,
    /// The note lives in `.trash/`.
    #[serde(default)]
    pub trashed: bool,
    /// A tombstone: the note was deleted for good.
    #[serde(default)]
    pub deleted: bool,
    /// The file text, frontmatter included. Empty for a tombstone.
    #[serde(default)]
    pub text: String,
}

impl Envelope {
    pub fn alive(text: String, trashed: bool) -> Self {
        Self {
            v: 1,
            trashed,
            deleted: false,
            text,
        }
    }

    pub fn tombstone() -> Self {
        Self {
            v: 1,
            trashed: false,
            deleted: true,
            text: String::new(),
        }
    }

    pub fn encode(&self) -> String {
        serde_json::to_string(self).unwrap_or_default()
    }

    pub fn decode(blob: &str) -> Result<Self> {
        serde_json::from_str(blob).context("decoding note payload")
    }
}

/// A signed-in account. The token is the only secret and lives in the
/// keyring; url and email are ordinary config.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Session {
    pub url: String,
    pub token: String,
    pub user_id: String,
    pub email: String,
}

impl Session {
    /// `<url>|<user id>` — what the local sync state belongs to. A different
    /// account or server means starting the cursor and state over.
    pub fn account(&self) -> String {
        format!("{}|{}", self.url, self.user_id)
    }
}

/// A record as it came off the server.
#[derive(Debug, Clone)]
pub struct Remote {
    pub record_id: String,
    pub note_id: String,
    pub revision: i64,
    pub device: String,
    /// The writer's modified time, RFC 3339.
    pub modified: String,
    pub envelope: Envelope,
}

/// A local change waiting to go up.
#[derive(Debug, Clone)]
pub struct Pending {
    pub note_id: String,
    /// The server record, if this note has been up before.
    pub record_id: Option<String>,
    /// The revision we last saw for it; the server refuses a stale base.
    pub base_revision: i64,
    pub envelope: Envelope,
    /// Content hash of the text (empty for a tombstone), remembered as the
    /// synced hash once the push lands.
    pub hash: String,
    pub modified: String,
}

/// What one cycle needs from the store.
#[derive(Debug, Clone)]
pub struct Job {
    pub session: Session,
    pub device_id: String,
    /// Last `updated` seen; empty pulls everything.
    pub cursor: String,
    pub pending: Vec<Pending>,
    /// note id → (record id, revision) we already hold, so echoes of our
    /// own pushes and records applied earlier are skipped.
    pub known: HashMap<String, (String, i64)>,
    /// The other half of the folder: `assets/` and `.folders`.
    pub files: files::FilesJob,
}

/// A push that landed.
#[derive(Debug, Clone)]
pub struct Pushed {
    pub note_id: String,
    pub record_id: String,
    pub revision: i64,
    pub hash: String,
    pub trashed: bool,
    pub deleted: bool,
}

#[derive(Debug, Clone, Default)]
pub struct Outcome {
    /// Fresh token (PocketBase rotates them on refresh).
    pub session: Option<Session>,
    pub incoming: Vec<Remote>,
    pub pushed: Vec<Pushed>,
    /// Notes changed on both sides; they come back through `incoming` and
    /// the local text becomes a conflict copy.
    pub conflicts: Vec<String>,
    pub cursor: String,
    pub errors: Vec<String>,
    /// The token was refused: sign in again.
    pub unauthorized: bool,
    pub files: files::FilesOutcome,
}

fn remote(r: RawNote) -> Result<Remote> {
    Ok(Remote {
        envelope: Envelope::decode(&r.blob).with_context(|| format!("note {}", r.note_id))?,
        record_id: r.record_id,
        note_id: r.note_id,
        revision: r.revision,
        device: r.device,
        modified: r.modified,
    })
}

/// One sync cycle against the account's backend. Never panics on a network
/// failure: the outcome carries what worked and what did not, so a flaky
/// link only delays things.
pub fn run(job: Job) -> Outcome {
    let mut backend = PocketBase::new(job.session.clone());
    run_with(&mut backend, job)
}

/// The cycle itself, whatever is on the other end.
pub fn run_with(backend: &mut dyn Backend, job: Job) -> Outcome {
    let mut out = Outcome {
        cursor: job.cursor.clone(),
        ..Default::default()
    };

    // The sign-in first: nothing else means anything without it.
    match backend.authorize() {
        Ok(Some(session)) => out.session = Some(session),
        Ok(None) => {
            out.unauthorized = true;
            return out;
        }
        Err(err) => {
            out.errors.push(format!("{err:#}"));
            return out;
        }
    }

    // Pull.
    let mut incoming_ids = std::collections::HashSet::new();
    match backend.pull_notes(&job.cursor) {
        Ok(page) => {
            out.cursor = page.cursor;
            for raw in page.items {
                let r = match remote(raw) {
                    Ok(r) => r,
                    Err(err) => {
                        tracing::warn!(%err, "skipping unreadable remote note");
                        continue;
                    }
                };
                if job.known.get(&r.note_id).is_some_and(|(_, rev)| *rev == r.revision) {
                    continue;
                }
                incoming_ids.insert(r.note_id.clone());
                out.incoming.push(r);
            }
        }
        Err(err) => {
            out.errors.push(format!("{err:#}"));
            return out;
        }
    }

    // Push whatever did not just change up there too.
    for p in &job.pending {
        if incoming_ids.contains(&p.note_id) {
            out.conflicts.push(p.note_id.clone());
            continue;
        }
        let blob = p.envelope.encode();
        let up = NoteUpload {
            note_id: &p.note_id,
            record_id: p.record_id.as_deref(),
            base_revision: p.base_revision,
            device: &job.device_id,
            modified: &p.modified,
            blob: &blob,
        };
        match backend.push_note(&up) {
            Ok(NotePush::Landed {
                record_id,
                revision,
                cursor,
            }) => {
                if let Some(c) = cursor
                    && c > out.cursor
                {
                    out.cursor = c;
                }
                out.pushed.push(Pushed {
                    note_id: p.note_id.clone(),
                    record_id,
                    revision,
                    hash: p.hash.clone(),
                    trashed: p.envelope.trashed,
                    deleted: p.envelope.deleted,
                });
            }
            Ok(NotePush::Conflict(theirs)) => {
                out.conflicts.push(p.note_id.clone());
                match theirs.map(remote) {
                    Some(Ok(r)) => out.incoming.push(r),
                    Some(Err(err)) => out.errors.push(format!("{}: {err:#}", p.note_id)),
                    None => {}
                }
            }
            Err(err) => out.errors.push(format!("{}: {err:#}", p.note_id)),
        }
    }

    // Notes first, then what they show.
    out.files = files::run(backend, &job.device_id, job.files);
    out
}

/// This machine's name, for conflict copies.
pub fn device_name() -> String {
    std::fs::read_to_string("/etc/hostname")
        .ok()
        .map(|s| s.trim().to_owned())
        .filter(|s| !s.is_empty())
        .or_else(|| std::env::var("HOSTNAME").ok())
        .unwrap_or_else(|| "another device".to_owned())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn envelope_round_trips_and_tolerates_missing_fields() {
        let e = Envelope::alive("---\nid: x\n---\n# Hi\n".into(), true);
        let back = Envelope::decode(&e.encode()).unwrap();
        assert_eq!(e, back);
        let t = Envelope::decode(r#"{"v":1}"#).unwrap();
        assert!(!t.deleted && !t.trashed && t.text.is_empty());
        assert!(Envelope::decode("nope").is_err());
        assert!(Envelope::tombstone().deleted);
    }

    #[test]
    fn the_account_is_server_and_user() {
        let s = Session {
            url: "u".into(),
            token: String::new(),
            user_id: "id".into(),
            email: String::new(),
        };
        assert_eq!(s.account(), "u|id");
    }
}
