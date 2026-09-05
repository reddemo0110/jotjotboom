// SPDX-License-Identifier: GPL-3.0-only

//! The store's half of sync: what needs pushing, and how a record that came
//! down is folded into the files on disk. See `crate::sync` for the wire
//! side and DECISIONS.md (2026-09-05) for the rules.

use super::Store;
use super::db::SyncState;
use crate::note::{self, Note};
use crate::sync::{Envelope, Job, Outcome, Pending, Pushed, Remote, Session};
use anyhow::{Context, Result};
use chrono::Utc;
use std::collections::HashMap;

const META_CURSOR: &str = "cursor";
const META_ACCOUNT: &str = "account";

/// What applying a remote record did locally.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Applied {
    /// The server's text now lives in the file.
    Adopted,
    /// The note was deleted here to match.
    Deleted,
    /// Both sides had changed: the server's text took the note over and the
    /// local text was kept as a new note with this id.
    Conflict { copy_id: String },
    /// Nothing to do (already identical, or a tombstone for a note we no
    /// longer have — or one we changed since, which will go back up).
    Unchanged,
}

impl Store {
    /// The account the local sync state belongs to, if any.
    pub fn sync_account(&self) -> Option<String> {
        self.db.sync_meta(META_ACCOUNT).ok().flatten()
    }

    /// Bind the sync state to an account. A different account or server
    /// starts from nothing: every note is pushed, everything is pulled, and
    /// the hashes sort out what is really new.
    pub fn set_sync_account(&mut self, account: &str) -> Result<()> {
        if self.sync_account().as_deref() != Some(account) {
            self.db.clear_sync()?;
            self.db.set_sync_meta(META_ACCOUNT, account)?;
        }
        Ok(())
    }

    /// Drop every trace of the server (sign out and forget).
    pub fn clear_sync(&mut self) -> Result<()> {
        self.db.clear_sync()
    }

    pub fn sync_cursor(&self) -> String {
        self.db.sync_meta(META_CURSOR).ok().flatten().unwrap_or_default()
    }

    pub fn set_sync_cursor(&mut self, cursor: &str) -> Result<()> {
        self.db.set_sync_meta(META_CURSOR, cursor)
    }

    /// note id → (record id, revision) as last agreed with the server.
    pub fn sync_known(&self) -> HashMap<String, (String, i64)> {
        self.db
            .all_sync_state()
            .unwrap_or_default()
            .into_iter()
            .map(|s| (s.note_id, (s.record_id, s.revision)))
            .collect()
    }

    /// Everything that differs from what the server last agreed to: changed
    /// or new notes (by hash and trash state, so edits made outside the app
    /// count too) and tombstones for notes that were synced and are gone.
    /// Rows first, then the file text — the count alone never reads a file.
    fn sync_diff(&self) -> Result<(Vec<super::db::NoteRow>, Vec<SyncState>)> {
        let states: HashMap<String, SyncState> = self
            .db
            .all_sync_state()?
            .into_iter()
            .map(|s| (s.note_id.clone(), s))
            .collect();
        let mut changed = Vec::new();
        let mut present = std::collections::HashSet::new();
        for row in self.db.all_rows()? {
            present.insert(row.id.clone());
            let synced = states
                .get(&row.id)
                .is_some_and(|s| s.hash == row.hash && s.trashed == row.trashed);
            if !synced {
                changed.push(row);
            }
        }
        let gone = states
            .into_values()
            .filter(|s| !present.contains(&s.note_id) && !s.hash.is_empty())
            .collect();
        Ok((changed, gone))
    }

    pub fn sync_pending(&self) -> Result<Vec<Pending>> {
        let (changed, gone) = self.sync_diff()?;
        let mut out = Vec::with_capacity(changed.len() + gone.len());
        for row in changed {
            let state = self.db.sync_state(&row.id)?;
            let text = match self.dir.read(&row.path) {
                Ok(t) => t,
                Err(err) => {
                    tracing::warn!(%err, id = row.id, "skipping unreadable note for sync");
                    continue;
                }
            };
            out.push(Pending {
                note_id: row.id.clone(),
                record_id: state.as_ref().map(|s| s.record_id.clone()),
                base_revision: state.as_ref().map_or(0, |s| s.revision),
                envelope: Envelope::alive(text, row.trashed),
                hash: row.hash.clone(),
                modified: super::db::fmt_ts(row.modified),
            });
        }
        for s in gone {
            out.push(Pending {
                note_id: s.note_id.clone(),
                record_id: Some(s.record_id.clone()),
                base_revision: s.revision,
                envelope: Envelope::tombstone(),
                hash: String::new(),
                modified: super::db::fmt_ts(Utc::now()),
            });
        }
        Ok(out)
    }

    /// How many notes are waiting to go up.
    pub fn sync_pending_count(&self) -> usize {
        self.sync_diff().map_or(0, |(c, g)| c.len() + g.len())
    }

    /// A push landed: remember what the server now holds.
    pub fn mark_pushed(&mut self, p: &Pushed) -> Result<()> {
        if p.deleted {
            return self.db.remove_sync_state(&p.note_id);
        }
        self.db.set_sync_state(&SyncState {
            note_id: p.note_id.clone(),
            record_id: p.record_id.clone(),
            revision: p.revision,
            hash: p.hash.clone(),
            trashed: p.trashed,
        })
    }

    /// Everything one cycle needs, gathered under the store's lock.
    pub fn sync_job(&self, session: Session, device_id: String) -> Result<Job> {
        Ok(Job {
            session,
            device_id,
            cursor: self.sync_cursor(),
            pending: self.sync_pending()?,
            known: self.sync_known(),
        })
    }

    /// Fold a finished cycle in: incoming records first, then the pushes
    /// that landed, then the cursor. Returns what each incoming record did,
    /// so the app can refresh whatever is open.
    pub fn apply_outcome(&mut self, out: &Outcome, device_name: &str) -> Vec<(String, Applied)> {
        let mut applied = Vec::new();
        for r in &out.incoming {
            tracing::debug!(
                id = r.note_id,
                from = r.device,
                modified = r.modified,
                revision = r.revision,
                "applying remote note"
            );
            match self.apply_remote(r, device_name) {
                Ok(a) => applied.push((r.note_id.clone(), a)),
                Err(err) => tracing::error!(%err, id = r.note_id, "applying remote note"),
            }
        }
        for p in &out.pushed {
            if let Err(err) = self.mark_pushed(p) {
                tracing::error!(%err, id = p.note_id, "recording push");
            }
        }
        if !out.cursor.is_empty()
            && let Err(err) = self.set_sync_cursor(&out.cursor)
        {
            tracing::error!(%err, "recording sync cursor");
        }
        applied
    }

    /// Fold a record from the server into the notes dir.
    pub fn apply_remote(&mut self, r: &Remote, device_name: &str) -> Result<Applied> {
        let row = self.db.get(&r.note_id)?;
        let state = self.db.sync_state(&r.note_id)?;
        let local_changed = match (&row, &state) {
            (Some(row), Some(st)) => row.hash != st.hash || row.trashed != st.trashed,
            (Some(_), None) => true,
            // Synced alive, deleted here since.
            (None, Some(st)) => !st.hash.is_empty(),
            (None, None) => false,
        };
        let remote_hash = if r.envelope.deleted {
            String::new()
        } else {
            note::content_hash(&r.envelope.text)
        };
        let new_state = SyncState {
            note_id: r.note_id.clone(),
            record_id: r.record_id.clone(),
            revision: r.revision,
            hash: remote_hash.clone(),
            trashed: r.envelope.trashed,
        };

        if r.envelope.deleted {
            return match row {
                // Their delete against our edit: the edit wins and goes back
                // up on the next push (the tombstone's revision is the base).
                Some(_) if local_changed => {
                    self.db.set_sync_state(&new_state)?;
                    Ok(Applied::Unchanged)
                }
                Some(row) => {
                    self.delete_forever(&row.id)?;
                    self.db.remove_sync_state(&r.note_id)?;
                    Ok(Applied::Deleted)
                }
                None => {
                    self.db.remove_sync_state(&r.note_id)?;
                    Ok(Applied::Unchanged)
                }
            };
        }

        // Same text already here (a reinstall, or both sides made the same
        // change): just agree.
        if row
            .as_ref()
            .is_some_and(|row| row.hash == remote_hash && row.trashed == r.envelope.trashed)
        {
            self.db.set_sync_state(&new_state)?;
            return Ok(Applied::Unchanged);
        }

        let mut copy = None;
        if local_changed && let Some(row) = &row {
            let local_text = self.dir.read(&row.path)?;
            let id = self
                .write_conflict_copy(&local_text, row.trashed, device_name)
                .context("keeping the local version as a conflict copy")?;
            copy = Some(id);
        }
        self.write_remote_text(&r.note_id, &r.envelope.text, r.envelope.trashed)
            .context("writing the server's version")?;
        self.db.set_sync_state(&new_state)?;
        Ok(match copy {
            Some(copy_id) => Applied::Conflict { copy_id },
            None => Applied::Adopted,
        })
    }

    /// Put the server's text on disk under `note_id`, byte for byte when
    /// the file already carries that id, moving between root and `.trash/`
    /// and renaming for the title as the editor would.
    fn write_remote_text(&mut self, note_id: &str, text: &str, trashed: bool) -> Result<()> {
        let existing = self.db.get(note_id)?;
        let (fm, body) = note::parse_document(text);
        let now = Utc::now();
        let mut n = Note {
            id: note_id.to_owned(),
            title: note::derive_title(body),
            body: body.to_owned(),
            created: fm.created.unwrap_or(now),
            modified: now,
            pinned: fm.pinned,
            trashed,
            extra_frontmatter: fm.extra,
            path: existing.as_ref().map(|r| r.path.clone()).unwrap_or_default(),
        };
        let dir = if trashed {
            self.dir.trash_dir()
        } else {
            self.dir.root().to_owned()
        };
        let wanted = self.place(&n, &dir);
        if wanted != n.path {
            if existing.is_some() && n.path.exists() {
                self.dir.rename(&n.path, &wanted)?;
            }
            n.path = wanted;
        }
        // Exact bytes keep the hash identical on every device; a file
        // without its id (or with another) gets stamped instead.
        let text = if fm.id.as_deref() == Some(note_id) && fm.created.is_some() {
            text.to_owned()
        } else {
            note::serialize_document(&n)
        };
        self.dir.write_atomic(&n.path, &text)?;
        if let Ok(m) = std::fs::metadata(&n.path).and_then(|m| m.modified()) {
            n.modified = m.into();
        }
        self.index(&n, &note::content_hash(&text))
    }

    /// Keep `text` as a brand-new note titled "… (conflict, device)".
    fn write_conflict_copy(&mut self, text: &str, trashed: bool, device_name: &str) -> Result<String> {
        let (fm, body) = note::parse_document(text);
        let body = conflict_body(body, device_name);
        let now = Utc::now();
        let mut n = Note {
            id: note::new_id(),
            title: note::derive_title(&body),
            body,
            created: fm.created.unwrap_or(now),
            modified: now,
            pinned: fm.pinned,
            trashed,
            extra_frontmatter: fm.extra,
            path: std::path::PathBuf::default(),
        };
        let dir = if trashed {
            self.dir.trash_dir()
        } else {
            self.dir.root().to_owned()
        };
        n.path = self.dir.unique_path(&dir, &n.title, None);
        self.write(&mut n)?;
        Ok(n.id)
    }
}

/// The body with the title marked as a conflict copy: a heading line gets
/// the suffix; anything else gets a heading put in front.
fn conflict_body(body: &str, device_name: &str) -> String {
    let suffix = format!(" (conflict, {device_name})");
    let mut lines: Vec<String> = body.lines().map(str::to_owned).collect();
    let first = lines.iter().position(|l| !l.trim().is_empty());
    match first {
        Some(i) if lines[i].trim_start().starts_with('#') => {
            lines[i] = format!("{}{suffix}", lines[i].trim_end());
            let mut out = lines.join("\n");
            if body.ends_with('\n') {
                out.push('\n');
            }
            out
        }
        _ => {
            let title = note::derive_title(body);
            let mut out = format!("# {title}{suffix}\n\n");
            out.push_str(body.trim_start_matches('\n'));
            out
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::store::View;

    /// One device: its own notes dir and index.
    fn device(name: &str) -> (Store, tempfile::TempDir) {
        let tmp = tempfile::tempdir().unwrap();
        let mut store =
            Store::open(tmp.path().join("notes"), &tmp.path().join("index.db"), name.into()).unwrap();
        // The welcome note would sync too; keep the scenarios clean.
        for n in store.list(&View::All).unwrap() {
            store.delete_forever(&n.id).unwrap();
        }
        (store, tmp)
    }

    fn cycle(store: &mut Store, session: &Session, device: &str) -> (Outcome, Vec<(String, Applied)>) {
        let job = store.sync_job(session.clone(), device.into()).unwrap();
        let out = crate::sync::run(job);
        assert!(out.errors.is_empty(), "sync errors: {:?}", out.errors);
        assert!(!out.unauthorized);
        let applied = store.apply_outcome(&out, device);
        (out, applied)
    }

    fn write(store: &mut Store, id: &str, body: &str) {
        let mut n = store.load(id).unwrap().unwrap();
        n.body = body.into();
        store.save(&mut n).unwrap();
    }

    fn titles(store: &Store, view: &View) -> Vec<String> {
        let mut t: Vec<String> = store.list(view).unwrap().into_iter().map(|n| n.title).collect();
        t.sort();
        t
    }

    /// Runs only with a PocketBase up (`JJB_PB_URL=http://127.0.0.1:8090`).
    #[test]
    fn two_devices_through_a_real_server() {
        let Ok(url) = std::env::var("JJB_PB_URL") else {
            eprintln!("JJB_PB_URL not set; skipping");
            return;
        };
        let email = format!("{}@example.com", note::new_id());
        let session = crate::sync::sign_up(&url, &email, "password123").unwrap();
        let (mut a, _ta) = device("laptop-a");
        let (mut b, _tb) = device("laptop-b");
        a.set_sync_account(&session.account()).unwrap();
        b.set_sync_account(&session.account()).unwrap();

        // A writes a note; B receives it byte for byte.
        let kyoto = a.create().unwrap().id;
        write(&mut a, &kyoto, "# Kyoto

Night one.
");
        let (out, _) = cycle(&mut a, &session, "laptop-a");
        assert_eq!(out.pushed.len(), 1);
        let (out, applied) = cycle(&mut b, &session, "laptop-b");
        assert_eq!(out.incoming.len(), 1);
        assert_eq!(applied, vec![(kyoto.clone(), Applied::Adopted)]);
        let a_text = std::fs::read_to_string(&a.load(&kyoto).unwrap().unwrap().path).unwrap();
        let b_note = b.load(&kyoto).unwrap().unwrap();
        assert_eq!(std::fs::read_to_string(&b_note.path).unwrap(), a_text);
        assert!(b_note.path.ends_with("Kyoto.md"));
        // Quiet afterwards: nothing pending, nothing incoming.
        let (out, _) = cycle(&mut b, &session, "laptop-b");
        assert!(out.pushed.is_empty() && out.incoming.is_empty());
        assert_eq!(a.sync_pending_count(), 0);

        // The plane: A edits offline, B edits and syncs, then A comes back.
        write(&mut a, &kyoto, "# Kyoto

Night one, written on the plane.
");
        write(&mut b, &kyoto, "# Kyoto

Night one, written after landing.
");
        cycle(&mut b, &session, "laptop-b");
        let (out, applied) = cycle(&mut a, &session, "laptop-a");
        assert_eq!(out.conflicts, vec![kyoto.clone()]);
        assert!(out.pushed.is_empty(), "nothing goes up while a conflict is open");
        let copy_id = match &applied[..] {
            [(id, Applied::Conflict { copy_id })] if id == &kyoto => copy_id.clone(),
            other => panic!("expected a conflict copy, got {other:?}"),
        };
        assert_eq!(
            a.load(&kyoto).unwrap().unwrap().body,
            "# Kyoto

Night one, written after landing.
"
        );
        let copy = a.load(&copy_id).unwrap().unwrap();
        assert_eq!(copy.title, "Kyoto (conflict, laptop-a)");
        assert!(copy.body.contains("on the plane"));
        // The copy goes up as its own note and B gets it.
        let (out, _) = cycle(&mut a, &session, "laptop-a");
        assert_eq!(out.pushed.len(), 1);
        cycle(&mut b, &session, "laptop-b");
        assert_eq!(
            titles(&b, &View::All),
            vec!["Kyoto".to_string(), "Kyoto (conflict, laptop-a)".to_string()]
        );

        // Trash travels.
        b.trash(&copy_id).unwrap();
        cycle(&mut b, &session, "laptop-b");
        cycle(&mut a, &session, "laptop-a");
        assert_eq!(titles(&a, &View::Trash), vec!["Kyoto (conflict, laptop-a)".to_string()]);
        assert!(a.load(&copy_id).unwrap().unwrap().trashed);

        // Delete vs edit: the edit wins and the note comes back.
        a.delete_forever(&copy_id).unwrap();
        b.restore(&copy_id).unwrap();
        write(&mut b, &copy_id, "# Kyoto, merged

Both halves.
");
        cycle(&mut b, &session, "laptop-b");
        let (out, applied) = cycle(&mut a, &session, "laptop-a");
        assert!(out.pushed.is_empty(), "the tombstone must not go up over an edit");
        assert_eq!(applied, vec![(copy_id.clone(), Applied::Adopted)]);
        assert_eq!(a.load(&copy_id).unwrap().unwrap().title, "Kyoto, merged");

        // A plain delete goes through.
        a.delete_forever(&copy_id).unwrap();
        let (out, _) = cycle(&mut a, &session, "laptop-a");
        assert_eq!(out.pushed.len(), 1);
        assert!(out.pushed[0].deleted);
        let (_, applied) = cycle(&mut b, &session, "laptop-b");
        assert_eq!(applied, vec![(copy_id.clone(), Applied::Deleted)]);
        assert!(b.load(&copy_id).unwrap().is_none());
        assert_eq!(titles(&b, &View::All), vec!["Kyoto".to_string()]);

        // A fresh install with the same files adopts quietly.
        let (mut c, _tc) = device("laptop-c");
        c.set_sync_account(&session.account()).unwrap();
        let text = std::fs::read_to_string(&a.load(&kyoto).unwrap().unwrap().path).unwrap();
        std::fs::write(c.notes_dir().join("Kyoto.md"), &text).unwrap();
        c.reindex().unwrap();
        let (out, applied) = cycle(&mut c, &session, "laptop-c");
        assert!(out.conflicts.contains(&kyoto), "the create is refused: the note exists up there");
        // (The deleted copy's tombstone comes down too, harmlessly.)
        assert!(applied.contains(&(kyoto.clone(), Applied::Unchanged)));
        assert!(applied.iter().all(|(_, a)| *a == Applied::Unchanged));
        assert_eq!(c.sync_pending_count(), 0);
        assert_eq!(titles(&c, &View::All), vec!["Kyoto".to_string()]);
    }

    #[test]
    fn conflict_title_goes_on_the_heading_or_in_front() {
        assert_eq!(
            conflict_body("# Kyoto\n\nText\n", "laptop"),
            "# Kyoto (conflict, laptop)\n\nText\n"
        );
        assert_eq!(
            conflict_body("\n# Kyoto  \nText", "laptop"),
            "\n# Kyoto (conflict, laptop)\nText"
        );
        assert_eq!(
            conflict_body("Just words\nmore\n", "laptop"),
            "# Just words (conflict, laptop)\n\nJust words\nmore\n"
        );
    }
}
