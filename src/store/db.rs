// SPDX-License-Identifier: GPL-3.0-only

//! SQLite index. Entirely derived from the files on disk — dropping it is
//! always safe. Holds the FTS5 table, tag/link graph, and the sync oplog.

#![allow(dead_code)]

use crate::note::{Note, NoteSummary};
use anyhow::{Context, Result};
use chrono::{DateTime, Utc};
use rusqlite::{Connection, OptionalExtension, params};
use std::collections::HashMap;
use std::path::{Path, PathBuf};

const SCHEMA_VERSION: i64 = 1;

pub struct Db {
    conn: Connection,
}

/// What the index knows about a file, used to diff against the disk scan.
#[derive(Debug, Clone)]
pub struct IndexedFile {
    pub id: String,
    pub hash: String,
    pub modified: DateTime<Utc>,
}

/// Full row for a note (body excluded — the file is the source of truth).
#[derive(Debug, Clone)]
pub struct NoteRow {
    pub id: String,
    pub path: PathBuf,
    pub title: String,
    pub created: DateTime<Utc>,
    pub modified: DateTime<Utc>,
    pub pinned: bool,
    pub trashed: bool,
    pub revision: i64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum View {
    All,
    Untagged,
    Trash,
    Tag(String),
}

impl Db {
    pub fn open(path: &Path) -> Result<Self> {
        if let Some(dir) = path.parent() {
            std::fs::create_dir_all(dir).with_context(|| format!("creating {}", dir.display()))?;
        }
        let conn = Connection::open(path).with_context(|| format!("opening {}", path.display()))?;
        conn.execute_batch(
            "PRAGMA journal_mode = WAL;
             PRAGMA synchronous = NORMAL;
             PRAGMA foreign_keys = ON;",
        )?;
        let mut db = Self { conn };
        db.migrate()?;
        Ok(db)
    }

    fn migrate(&mut self) -> Result<()> {
        let version: i64 = self
            .conn
            .query_row("PRAGMA user_version", [], |r| r.get(0))
            .unwrap_or(0);
        if version == SCHEMA_VERSION {
            return Ok(());
        }
        if version != 0 {
            // The index is derived data: on any schema change, rebuild from scratch.
            tracing::info!(
                from = version,
                to = SCHEMA_VERSION,
                "index schema changed; rebuilding"
            );
            self.conn.execute_batch(
                "DROP TABLE IF EXISTS notes_fts;
                 DROP TABLE IF EXISTS links;
                 DROP TABLE IF EXISTS note_tags;
                 DROP TABLE IF EXISTS tags;
                 DROP TABLE IF EXISTS notes;",
            )?;
        }
        self.conn.execute_batch(
            "CREATE TABLE IF NOT EXISTS notes (
                id        TEXT PRIMARY KEY,
                path      TEXT NOT NULL UNIQUE,
                title     TEXT NOT NULL,
                preview   TEXT NOT NULL,
                created   TEXT NOT NULL,
                modified  TEXT NOT NULL,
                pinned    INTEGER NOT NULL DEFAULT 0,
                trashed   INTEGER NOT NULL DEFAULT 0,
                hash      TEXT NOT NULL,
                revision  INTEGER NOT NULL DEFAULT 0
            );
            CREATE INDEX IF NOT EXISTS notes_modified ON notes(trashed, pinned DESC, modified DESC);
            CREATE TABLE IF NOT EXISTS tags (
                id   INTEGER PRIMARY KEY,
                name TEXT NOT NULL UNIQUE
            );
            CREATE TABLE IF NOT EXISTS note_tags (
                note_id TEXT NOT NULL REFERENCES notes(id) ON DELETE CASCADE,
                tag_id  INTEGER NOT NULL REFERENCES tags(id) ON DELETE CASCADE,
                PRIMARY KEY (note_id, tag_id)
            );
            CREATE INDEX IF NOT EXISTS note_tags_tag ON note_tags(tag_id);
            CREATE TABLE IF NOT EXISTS links (
                from_id  TEXT NOT NULL REFERENCES notes(id) ON DELETE CASCADE,
                to_title TEXT NOT NULL COLLATE NOCASE,
                PRIMARY KEY (from_id, to_title)
            );
            CREATE INDEX IF NOT EXISTS links_to ON links(to_title);
            CREATE VIRTUAL TABLE IF NOT EXISTS notes_fts USING fts5(
                id UNINDEXED, title, body, tokenize = 'unicode61 remove_diacritics 2'
            );
            -- Sync oplog: one row per local save. Plaintext columns are the
            -- E2E metadata boundary: id, revision, timestamp, device, hash.
            CREATE TABLE IF NOT EXISTS oplog (
                seq         INTEGER PRIMARY KEY AUTOINCREMENT,
                note_id     TEXT NOT NULL,
                revision    INTEGER NOT NULL,
                modified_at TEXT NOT NULL,
                device_id   TEXT NOT NULL,
                hash        TEXT NOT NULL,
                synced      INTEGER NOT NULL DEFAULT 0
            );",
        )?;
        self.conn
            .pragma_update(None, "user_version", SCHEMA_VERSION)?;
        Ok(())
    }

    /// path -> what we indexed, for diffing against a disk scan.
    pub fn indexed_files(&self) -> Result<HashMap<PathBuf, IndexedFile>> {
        let mut stmt = self
            .conn
            .prepare("SELECT path, id, hash, modified FROM notes")?;
        let rows = stmt.query_map([], |r| {
            Ok((
                PathBuf::from(r.get::<_, String>(0)?),
                IndexedFile {
                    id: r.get(1)?,
                    hash: r.get(2)?,
                    modified: parse_ts(&r.get::<_, String>(3)?),
                },
            ))
        })?;
        Ok(rows.collect::<std::result::Result<_, _>>()?)
    }

    /// Insert or replace everything we index about a note.
    pub fn upsert(
        &mut self,
        note: &Note,
        preview: &str,
        hash: &str,
        tags: &[String],
        links: &[String],
        body: &str,
    ) -> Result<()> {
        let tx = self.conn.transaction()?;
        tx.execute(
            "INSERT INTO notes (id, path, title, preview, created, modified, pinned, trashed, hash, revision)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, 0)
             ON CONFLICT(id) DO UPDATE SET
                path = excluded.path, title = excluded.title, preview = excluded.preview,
                created = excluded.created, modified = excluded.modified, pinned = excluded.pinned,
                trashed = excluded.trashed, hash = excluded.hash",
            params![
                note.id,
                path_str(&note.path),
                note.title,
                preview,
                fmt_ts(note.created),
                fmt_ts(note.modified),
                note.pinned as i64,
                note.trashed as i64,
                hash,
            ],
        )?;
        tx.execute("DELETE FROM note_tags WHERE note_id = ?1", params![note.id])?;
        for tag in tags {
            tx.execute(
                "INSERT OR IGNORE INTO tags (name) VALUES (?1)",
                params![tag],
            )?;
            tx.execute(
                "INSERT OR IGNORE INTO note_tags (note_id, tag_id) SELECT ?1, id FROM tags WHERE name = ?2",
                params![note.id, tag],
            )?;
        }
        tx.execute("DELETE FROM links WHERE from_id = ?1", params![note.id])?;
        for link in links {
            tx.execute(
                "INSERT OR IGNORE INTO links (from_id, to_title) VALUES (?1, ?2)",
                params![note.id, link],
            )?;
        }
        tx.execute("DELETE FROM notes_fts WHERE id = ?1", params![note.id])?;
        tx.execute(
            "INSERT INTO notes_fts (id, title, body) VALUES (?1, ?2, ?3)",
            params![note.id, note.title, body],
        )?;
        tx.execute(
            "DELETE FROM tags WHERE id NOT IN (SELECT tag_id FROM note_tags)",
            [],
        )?;
        tx.commit()?;
        Ok(())
    }

    pub fn remove(&mut self, id: &str) -> Result<()> {
        let tx = self.conn.transaction()?;
        tx.execute("DELETE FROM notes WHERE id = ?1", params![id])?;
        tx.execute("DELETE FROM notes_fts WHERE id = ?1", params![id])?;
        tx.execute(
            "DELETE FROM tags WHERE id NOT IN (SELECT tag_id FROM note_tags)",
            [],
        )?;
        tx.commit()?;
        Ok(())
    }

    /// Record a local save in the oplog and bump the note's revision.
    pub fn append_oplog(
        &mut self,
        id: &str,
        modified: DateTime<Utc>,
        device_id: &str,
        hash: &str,
    ) -> Result<i64> {
        let tx = self.conn.transaction()?;
        tx.execute(
            "UPDATE notes SET revision = revision + 1 WHERE id = ?1",
            params![id],
        )?;
        let revision: i64 = tx.query_row(
            "SELECT revision FROM notes WHERE id = ?1",
            params![id],
            |r| r.get(0),
        )?;
        tx.execute(
            "INSERT INTO oplog (note_id, revision, modified_at, device_id, hash) VALUES (?1, ?2, ?3, ?4, ?5)",
            params![id, revision, fmt_ts(modified), device_id, hash],
        )?;
        tx.commit()?;
        Ok(revision)
    }

    pub fn get(&self, id: &str) -> Result<Option<NoteRow>> {
        self.conn
            .query_row(
                "SELECT id, path, title, created, modified, pinned, trashed, revision FROM notes WHERE id = ?1",
                params![id],
                row_to_note,
            )
            .optional()
            .map_err(Into::into)
    }

    pub fn list(&self, view: &View) -> Result<Vec<NoteSummary>> {
        let (sql, tag): (&str, Option<&str>) = match view {
            View::All => (
                "SELECT id, title, preview, modified, pinned, trashed FROM notes WHERE trashed = 0 ORDER BY pinned DESC, modified DESC, rowid DESC",
                None,
            ),
            View::Trash => (
                "SELECT id, title, preview, modified, pinned, trashed FROM notes WHERE trashed = 1 ORDER BY modified DESC, rowid DESC",
                None,
            ),
            View::Untagged => (
                "SELECT id, title, preview, modified, pinned, trashed FROM notes
                 WHERE trashed = 0 AND id NOT IN (SELECT note_id FROM note_tags)
                 ORDER BY pinned DESC, modified DESC, rowid DESC",
                None,
            ),
            View::Tag(t) => (
                "SELECT n.id, n.title, n.preview, n.modified, n.pinned, n.trashed FROM notes n
                 JOIN note_tags nt ON nt.note_id = n.id JOIN tags t ON t.id = nt.tag_id
                 WHERE n.trashed = 0 AND (t.name = ?1 OR t.name LIKE ?1 || '/%')
                 GROUP BY n.id ORDER BY n.pinned DESC, n.modified DESC, n.rowid DESC",
                Some(t.as_str()),
            ),
        };
        let mut stmt = self.conn.prepare(sql)?;
        let rows = match tag {
            Some(t) => stmt.query_map(params![t], row_to_summary)?,
            None => stmt.query_map([], row_to_summary)?,
        };
        Ok(rows.collect::<std::result::Result<_, _>>()?)
    }

    /// Full-text search within a view. Matches title or body; prefix-matches
    /// the last word so results update as you type.
    pub fn search(&self, query: &str, view: &View) -> Result<Vec<NoteSummary>> {
        let Some(fts_query) = fts_query(query) else {
            return self.list(view);
        };
        let trashed = matches!(view, View::Trash) as i64;
        let mut stmt = self.conn.prepare(
            "SELECT n.id, n.title, n.preview, n.modified, n.pinned, n.trashed
             FROM notes_fts f JOIN notes n ON n.id = f.id
             WHERE notes_fts MATCH ?1 AND n.trashed = ?2
             ORDER BY bm25(notes_fts, 4.0, 1.0), n.modified DESC
             LIMIT 200",
        )?;
        let rows = stmt.query_map(params![fts_query, trashed], row_to_summary)?;
        let mut out: Vec<NoteSummary> = rows.collect::<std::result::Result<_, _>>()?;
        match view {
            View::Untagged => {
                let tagged = self.tagged_ids()?;
                out.retain(|n| !tagged.contains(&n.id));
            }
            View::Tag(t) => {
                let ids = self.ids_with_tag(t)?;
                out.retain(|n| ids.contains(&n.id));
            }
            View::All | View::Trash => {}
        }
        Ok(out)
    }

    fn tagged_ids(&self) -> Result<std::collections::HashSet<String>> {
        let mut stmt = self
            .conn
            .prepare("SELECT DISTINCT note_id FROM note_tags")?;
        let rows = stmt.query_map([], |r| r.get::<_, String>(0))?;
        Ok(rows.collect::<std::result::Result<_, _>>()?)
    }

    fn ids_with_tag(&self, tag: &str) -> Result<std::collections::HashSet<String>> {
        let mut stmt = self.conn.prepare(
            "SELECT DISTINCT nt.note_id FROM note_tags nt JOIN tags t ON t.id = nt.tag_id
             WHERE t.name = ?1 OR t.name LIKE ?1 || '/%'",
        )?;
        let rows = stmt.query_map(params![tag], |r| r.get::<_, String>(0))?;
        Ok(rows.collect::<std::result::Result<_, _>>()?)
    }

    /// All tags with the number of (non-trashed) notes carrying them, sorted by name.
    pub fn tags(&self) -> Result<Vec<(String, usize)>> {
        let mut stmt = self.conn.prepare(
            "SELECT t.name, COUNT(n.id) FROM tags t
             JOIN note_tags nt ON nt.tag_id = t.id JOIN notes n ON n.id = nt.note_id
             WHERE n.trashed = 0 GROUP BY t.name ORDER BY t.name",
        )?;
        let rows = stmt.query_map([], |r| {
            Ok((r.get::<_, String>(0)?, r.get::<_, i64>(1)? as usize))
        })?;
        Ok(rows.collect::<std::result::Result<_, _>>()?)
    }

    /// Notes that link to `title` (case-insensitive).
    pub fn backlinks(&self, title: &str) -> Result<Vec<NoteSummary>> {
        let mut stmt = self.conn.prepare(
            "SELECT n.id, n.title, n.preview, n.modified, n.pinned, n.trashed FROM links l
             JOIN notes n ON n.id = l.from_id WHERE l.to_title = ?1 AND n.trashed = 0
             ORDER BY n.modified DESC",
        )?;
        let rows = stmt.query_map(params![title], row_to_summary)?;
        Ok(rows.collect::<std::result::Result<_, _>>()?)
    }

    pub fn find_by_title(&self, title: &str) -> Result<Option<String>> {
        self.conn
            .query_row(
                "SELECT id FROM notes WHERE title = ?1 COLLATE NOCASE AND trashed = 0 ORDER BY modified DESC LIMIT 1",
                params![title],
                |r| r.get(0),
            )
            .optional()
            .map_err(Into::into)
    }

    pub fn count(&self) -> Result<usize> {
        Ok(self
            .conn
            .query_row("SELECT COUNT(*) FROM notes", [], |r| r.get::<_, i64>(0))?
            as usize)
    }
}

/// Build an FTS5 query from free text: each word becomes a required term, the
/// last word is a prefix match. Quoted to neutralise FTS operators.
fn fts_query(query: &str) -> Option<String> {
    let words: Vec<&str> = query.split_whitespace().collect();
    if words.is_empty() {
        return None;
    }
    let mut parts = Vec::with_capacity(words.len());
    for (i, w) in words.iter().enumerate() {
        let escaped = w.replace('"', "\"\"");
        if i + 1 == words.len() {
            parts.push(format!("\"{escaped}\"*"));
        } else {
            parts.push(format!("\"{escaped}\""));
        }
    }
    Some(parts.join(" "))
}

fn row_to_summary(r: &rusqlite::Row<'_>) -> rusqlite::Result<NoteSummary> {
    Ok(NoteSummary {
        id: r.get(0)?,
        title: r.get(1)?,
        preview: r.get(2)?,
        modified: parse_ts(&r.get::<_, String>(3)?),
        pinned: r.get::<_, i64>(4)? != 0,
        trashed: r.get::<_, i64>(5)? != 0,
    })
}

fn row_to_note(r: &rusqlite::Row<'_>) -> rusqlite::Result<NoteRow> {
    Ok(NoteRow {
        id: r.get(0)?,
        path: PathBuf::from(r.get::<_, String>(1)?),
        title: r.get(2)?,
        created: parse_ts(&r.get::<_, String>(3)?),
        modified: parse_ts(&r.get::<_, String>(4)?),
        pinned: r.get::<_, i64>(5)? != 0,
        trashed: r.get::<_, i64>(6)? != 0,
        revision: r.get(7)?,
    })
}

fn path_str(p: &Path) -> String {
    p.to_string_lossy().into_owned()
}

pub fn fmt_ts(t: DateTime<Utc>) -> String {
    t.to_rfc3339_opts(chrono::SecondsFormat::Micros, true)
}

pub fn parse_ts(s: &str) -> DateTime<Utc> {
    DateTime::parse_from_rfc3339(s)
        .map(|d| d.with_timezone(&Utc))
        .unwrap_or_else(|_| DateTime::<Utc>::UNIX_EPOCH)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn note(id: &str, title: &str) -> Note {
        Note {
            id: id.into(),
            title: title.into(),
            body: format!("# {title}\nbody"),
            created: Utc::now(),
            modified: Utc::now(),
            pinned: false,
            trashed: false,
            extra_frontmatter: vec![],
            path: PathBuf::from(format!("/tmp/{id}.md")),
        }
    }

    #[test]
    fn fts5_is_available_and_searches() {
        let mut db = Db::open(Path::new(":memory:")).unwrap();
        let a = note("a", "Alpha note");
        let b = note("b", "Beta");
        db.upsert(
            &a,
            "",
            "h1",
            &["work".into()],
            &[],
            "# Alpha note\nhello world #work",
        )
        .unwrap();
        db.upsert(
            &b,
            "",
            "h2",
            &[],
            &["Alpha note".into()],
            "# Beta\nsee [[Alpha note]] wörld",
        )
        .unwrap();
        assert_eq!(db.count().unwrap(), 2);
        let hits = db.search("hel", &View::All).unwrap();
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].id, "a");
        // diacritics folded
        assert_eq!(db.search("world", &View::All).unwrap().len(), 2);
        assert_eq!(db.list(&View::Untagged).unwrap().len(), 1);
        assert_eq!(db.list(&View::Tag("work".into())).unwrap()[0].id, "a");
        assert_eq!(db.backlinks("alpha NOTE").unwrap()[0].id, "b");
        assert_eq!(db.tags().unwrap(), vec![("work".to_string(), 1)]);
        let rev = db.append_oplog("a", Utc::now(), "dev", "h1").unwrap();
        assert_eq!(rev, 1);
        db.remove("a").unwrap();
        assert!(db.tags().unwrap().is_empty());
        assert!(db.search("hel", &View::All).unwrap().is_empty());
    }

    #[test]
    fn fts_query_building() {
        assert_eq!(fts_query("  "), None);
        assert_eq!(fts_query("foo bar").unwrap(), "\"foo\" \"bar\"*");
        assert_eq!(fts_query("a\"b").unwrap(), "\"a\"\"b\"*");
    }
}
