// SPDX-License-Identifier: GPL-3.0-only

//! The rest of the notes folder: pictures and attached files under
//! `assets/`, and the `.folders` list. They ride in a second collection,
//! `files`, under the same bargain as notes — the server sees a key (a hash
//! of the path, never the name), a revision, a timestamp and the device; the
//! path and content hash sit in an opaque [`Meta`], the bytes in a protected
//! upload.
//!
//! Unlike notes, the bytes never pass through the app thread: this half of
//! the cycle reads and writes `assets/` itself, from the blocking thread,
//! and hands the store only bookkeeping ([`FileState`]), the names it had to
//! change, and the server's `.folders` to merge. Rules (DECISIONS.md,
//! 2026-09-20): a file is never overwritten by a different one — the local
//! file steps aside under a free name and the notes that show it are
//! pointed at the new name; deleting an asset does not travel.

use super::{Session, api_error};
use anyhow::{Context, Result, anyhow};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use std::time::{Duration, Instant};
use ureq::unversioned::multipart::{Form, Part};

pub const FOLDERS_PATH: &str = ".folders";
const ASSETS_PREFIX: &str = "assets/";
/// Downloads land here first; dot-named, so the scan never sees it.
const INCOMING_DIR: &str = "assets/.incoming";
/// The `files` collection's upload limit (see the migration).
pub const MAX_SIZE: u64 = 256 * 1024 * 1024;
/// A cycle stops moving files after this long and asks for another, so a
/// first sync of a big `assets/` never holds the notes up for minutes.
const BUDGET: Duration = Duration::from_secs(20);
const PAGE: usize = 200;

/// The opaque part of a file record.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Meta {
    pub v: u32,
    /// Path inside the notes folder, `/`-separated.
    pub path: String,
    /// blake3 of the bytes.
    pub hash: String,
    #[serde(default)]
    pub size: u64,
}

/// What the server can see of a path.
pub fn key_for(path: &str) -> String {
    blake3::hash(format!("jjb-file:{path}").as_bytes())
        .to_hex()
        .to_string()
}

/// A path we are willing to write: `.folders`, or something under `assets/`
/// with no hidden or climbing components.
pub fn safe_path(path: &str) -> bool {
    if path == FOLDERS_PATH {
        return true;
    }
    path.strip_prefix(ASSETS_PREFIX).is_some_and(|rest| {
        !rest.is_empty()
            && !path.contains(['\\', '\0'])
            && rest
                .split('/')
                .all(|c| !c.is_empty() && !c.starts_with('.'))
    })
}

/// What we know about one file: what the server agreed to, and a cache of
/// the local hash so an unchanged file is never read twice.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct FileState {
    pub path: String,
    /// Empty until the file has been up (or come down).
    pub record_id: String,
    pub revision: i64,
    /// Content hash as last agreed with the server; empty for none.
    pub hash: String,
    /// Hash of the local file when it had this size and mtime.
    pub local_hash: String,
    pub size: u64,
    /// Nanoseconds since the epoch.
    pub mtime: i64,
}

#[derive(Debug, Clone, Default)]
pub struct FilesJob {
    pub notes_dir: PathBuf,
    /// Last `updated` seen in the `files` collection.
    pub cursor: String,
    pub known: Vec<FileState>,
}

#[derive(Debug, Clone, Default)]
pub struct FilesOutcome {
    /// Rows to remember (only the ones that changed).
    pub states: Vec<FileState>,
    /// Local files that stepped aside for a different file of the same
    /// name: (old path, new path). Notes showing the old one follow it.
    pub renamed: Vec<(String, String)>,
    /// The server's `.folders`, to be merged with ours.
    pub folders_remote: Option<String>,
    /// The `.folders` text that went up: the new merge base.
    pub folders_pushed: Option<String>,
    pub cursor: String,
    pub up: usize,
    pub down: usize,
    /// Ran out of time; run another cycle soon.
    pub more: bool,
    pub errors: Vec<String>,
}

#[derive(Debug, Clone)]
struct Local {
    abs: PathBuf,
    size: u64,
    mtime: i64,
    hash: String,
}

#[derive(Deserialize)]
struct Record {
    id: String,
    #[serde(default)]
    key: String,
    #[serde(default)]
    revision: i64,
    #[serde(default)]
    updated: String,
    #[serde(default)]
    meta: String,
    /// The stored file name, for the download URL.
    #[serde(default)]
    data: String,
}

#[derive(Deserialize)]
struct ListResponse {
    items: Vec<Record>,
}

struct RemoteFile {
    record_id: String,
    revision: i64,
    stored_name: String,
    meta: Meta,
}

fn remote(r: Record) -> Result<RemoteFile> {
    let meta: Meta = serde_json::from_str(&r.meta).context("decoding file details")?;
    if !safe_path(&meta.path) || key_for(&meta.path) != r.key {
        anyhow::bail!("refusing a file with a bad path ({})", meta.path);
    }
    Ok(RemoteFile {
        record_id: r.id,
        revision: r.revision,
        stored_name: r.data,
        meta,
    })
}

/// Uploads and downloads can be long: no overall deadline, only patience
/// limits on connecting and on hearing back.
fn agent() -> ureq::Agent {
    ureq::Agent::config_builder()
        .http_status_as_error(false)
        .timeout_connect(Some(Duration::from_secs(30)))
        .timeout_recv_response(Some(Duration::from_secs(120)))
        .user_agent(super::USER_AGENT)
        .build()
        .into()
}

fn mtime_ns(meta: &std::fs::Metadata) -> i64 {
    meta.modified()
        .ok()
        .and_then(|t| t.duration_since(std::time::UNIX_EPOCH).ok())
        .map_or(0, |d| i64::try_from(d.as_nanos()).unwrap_or(i64::MAX))
}

pub fn hash_file(path: &Path) -> Result<String> {
    let mut file =
        std::fs::File::open(path).with_context(|| format!("opening {}", path.display()))?;
    let mut hasher = blake3::Hasher::new();
    std::io::copy(&mut file, &mut hasher).with_context(|| format!("reading {}", path.display()))?;
    Ok(hasher.finalize().to_hex().to_string())
}

/// Every syncable file: `.folders` and the visible contents of `assets/`.
/// (relative path, absolute path, metadata)
fn scan(root: &Path) -> Vec<(String, PathBuf, std::fs::Metadata)> {
    fn walk(dir: &Path, rel: &str, out: &mut Vec<(String, PathBuf, std::fs::Metadata)>) {
        let Ok(entries) = std::fs::read_dir(dir) else {
            return;
        };
        for entry in entries.flatten() {
            let Ok(name) = entry.file_name().into_string() else {
                continue;
            };
            // No symlinks: `symlink_metadata` reports the link itself.
            let Ok(meta) = std::fs::symlink_metadata(entry.path()) else {
                continue;
            };
            let rel = format!("{rel}{name}");
            if name.starts_with('.') || !safe_path(&rel) {
                continue;
            }
            if meta.is_dir() {
                walk(&entry.path(), &format!("{rel}/"), out);
            } else if meta.is_file() {
                out.push((rel, entry.path(), meta));
            }
        }
    }
    let mut out = Vec::new();
    let folders = root.join(FOLDERS_PATH);
    if let Ok(meta) = std::fs::symlink_metadata(&folders)
        && meta.is_file()
    {
        out.push((FOLDERS_PATH.to_owned(), folders, meta));
    }
    walk(&root.join("assets"), ASSETS_PREFIX, &mut out);
    out
}

/// `assets/photo.jpg` → `assets/photo-2.jpg`, the first one not taken.
fn free_name(root: &Path, rel: &str) -> String {
    let (dir, name) = rel.rsplit_once('/').map_or(("", rel), |(d, n)| (d, n));
    let (stem, ext) = match name.rsplit_once('.') {
        Some((s, e)) if !s.is_empty() => (s, format!(".{e}")),
        _ => (name, String::new()),
    };
    (2..)
        .map(|n| {
            if dir.is_empty() {
                format!("{stem}-{n}{ext}")
            } else {
                format!("{dir}/{stem}-{n}{ext}")
            }
        })
        .find(|candidate| !root.join(candidate).exists())
        .unwrap_or_else(|| rel.to_owned())
}

struct Cycle<'a> {
    agent: ureq::Agent,
    session: &'a Session,
    device_id: &'a str,
    root: PathBuf,
    started: Instant,
    locals: HashMap<String, Local>,
    /// path → what the server agreed to, updated as the cycle goes.
    synced: HashMap<String, FileState>,
    file_token: Option<String>,
    /// `.folders` came down: merge before pushing ours.
    folders_incoming: bool,
    out: FilesOutcome,
}

impl Cycle<'_> {
    fn records_url(&self) -> String {
        format!("{}/api/collections/files/records", self.session.url)
    }

    fn bearer(&self) -> String {
        format!("Bearer {}", self.session.token)
    }

    fn out_of_time(&mut self) -> bool {
        let over = self.started.elapsed() > BUDGET;
        if over {
            self.out.more = true;
        }
        over
    }

    fn list(&self, cursor: &str) -> Result<Vec<Record>> {
        let mut out = Vec::new();
        for page in 1.. {
            let mut req = self
                .agent
                .get(self.records_url())
                .header("Authorization", self.bearer())
                .query("sort", "updated,id")
                .query("perPage", PAGE.to_string())
                .query("page", page.to_string())
                .query("skipTotal", "1");
            if !cursor.is_empty() {
                req = req.query("filter", format!("updated >= \"{cursor}\""));
            }
            let mut resp = req.call().context("listing files")?;
            let status = resp.status().as_u16();
            let body = resp.body_mut().read_to_string()?;
            if status == 404 {
                anyhow::bail!(
                    "the server has no `files` collection yet — add the new migration and hook from server/pocketbase"
                );
            }
            if status >= 400 {
                return Err(api_error(status, &body)).context("listing files");
            }
            let list: ListResponse =
                serde_json::from_str(&body).context("parsing the file list")?;
            let n = list.items.len();
            out.extend(list.items);
            if n < PAGE {
                break;
            }
        }
        Ok(out)
    }

    fn fetch_by_key(&self, key: &str) -> Result<Option<RemoteFile>> {
        let mut resp = self
            .agent
            .get(self.records_url())
            .header("Authorization", self.bearer())
            .query("filter", format!("key = \"{key}\""))
            .query("perPage", "1")
            .query("skipTotal", "1")
            .call()?;
        let status = resp.status().as_u16();
        let body = resp.body_mut().read_to_string()?;
        if status >= 400 {
            return Err(api_error(status, &body));
        }
        let list: ListResponse = serde_json::from_str(&body)?;
        list.items.into_iter().next().map(remote).transpose()
    }

    /// Protected files are fetched with a short-lived token of their own.
    fn token(&mut self) -> Result<String> {
        #[derive(Deserialize)]
        struct Token {
            token: String,
        }
        if let Some(t) = &self.file_token {
            return Ok(t.clone());
        }
        let mut resp = self
            .agent
            .post(format!("{}/api/files/token", self.session.url))
            .header("Authorization", self.bearer())
            .send_empty()?;
        let status = resp.status().as_u16();
        let body = resp.body_mut().read_to_string()?;
        if status >= 400 {
            return Err(api_error(status, &body)).context("asking for a download token");
        }
        let t: Token = serde_json::from_str(&body).context("parsing the download token")?;
        self.file_token = Some(t.token.clone());
        Ok(t.token)
    }

    /// Fetch a record's bytes into the incoming dir, checked against the
    /// hash its meta promises.
    fn download(&mut self, r: &RemoteFile) -> Result<PathBuf> {
        let token = self.token()?;
        let incoming = self.root.join(INCOMING_DIR);
        std::fs::create_dir_all(&incoming).context("creating the incoming dir")?;
        let tmp = incoming.join(format!("{}.part", r.record_id));
        let mut resp = self
            .agent
            .get(format!(
                "{}/api/files/files/{}/{}",
                self.session.url, r.record_id, r.stored_name
            ))
            .query("token", &token)
            .call()?;
        let status = resp.status().as_u16();
        if status >= 400 {
            let body = resp.body_mut().read_to_string().unwrap_or_default();
            return Err(api_error(status, &body));
        }
        let mut reader = resp.body_mut().as_reader();
        let mut file = std::fs::File::create(&tmp)?;
        let mut hasher = blake3::Hasher::new();
        let mut buf = vec![0u8; 64 * 1024];
        loop {
            let n = reader.read(&mut buf)?;
            if n == 0 {
                break;
            }
            hasher.update(&buf[..n]);
            file.write_all(&buf[..n])?;
        }
        file.sync_all().ok();
        if hasher.finalize().to_hex().as_str() != r.meta.hash {
            std::fs::remove_file(&tmp).ok();
            anyhow::bail!("the download does not match its checksum");
        }
        Ok(tmp)
    }

    /// Download `r` into its place and remember it as agreed.
    fn adopt(&mut self, r: &RemoteFile) -> Result<()> {
        let tmp = self.download(r)?;
        let abs = self.root.join(&r.meta.path);
        if let Some(parent) = abs.parent() {
            std::fs::create_dir_all(parent)?;
        }
        std::fs::rename(&tmp, &abs).with_context(|| format!("placing {}", abs.display()))?;
        let meta = std::fs::metadata(&abs)?;
        self.locals.insert(
            r.meta.path.clone(),
            Local {
                abs,
                size: meta.len(),
                mtime: mtime_ns(&meta),
                hash: r.meta.hash.clone(),
            },
        );
        self.agree(r);
        self.out.down += 1;
        Ok(())
    }

    fn agree(&mut self, r: &RemoteFile) {
        let s = self.synced.entry(r.meta.path.clone()).or_default();
        s.path.clone_from(&r.meta.path);
        s.record_id.clone_from(&r.record_id);
        s.revision = r.revision;
        s.hash.clone_from(&r.meta.hash);
    }

    /// Fold one server record into the folder.
    fn reconcile(&mut self, r: &RemoteFile) -> Result<()> {
        let path = &r.meta.path;
        if path == FOLDERS_PATH {
            // Never written from here: the store merges it with ours.
            let tmp = self.download(r)?;
            let text = std::fs::read_to_string(&tmp);
            std::fs::remove_file(&tmp).ok();
            self.out.folders_remote = Some(text.context("reading the folder list")?);
            self.folders_incoming = true;
            self.agree(r);
            return Ok(());
        }
        let Some(local) = self.locals.get(path).cloned() else {
            return self.adopt(r);
        };
        if local.hash == r.meta.hash {
            self.agree(r);
            return Ok(());
        }
        let untouched = self
            .synced
            .get(path)
            .is_some_and(|s| !s.hash.is_empty() && s.hash == local.hash);
        if !untouched {
            // Two different files under one name. Ours steps aside (and goes
            // up under its new name); theirs takes the name it came with.
            let new_rel = free_name(&self.root, path);
            let new_abs = self.root.join(&new_rel);
            std::fs::rename(&local.abs, &new_abs)
                .with_context(|| format!("moving {} aside", local.abs.display()))?;
            tracing::info!(
                from = path,
                to = new_rel,
                "a different file of the same name came down"
            );
            self.locals.remove(path);
            self.locals.insert(
                new_rel.clone(),
                Local {
                    abs: new_abs,
                    ..local
                },
            );
            self.out.renamed.push((path.clone(), new_rel));
        }
        self.adopt(r)
    }

    fn pull(&mut self, cursor: &str) {
        let records = match self.list(cursor) {
            Ok(r) => r,
            Err(err) => {
                self.out.errors.push(format!("{err:#}"));
                return;
            }
        };
        let mut newest = cursor.to_owned();
        let mut complete = true;
        for rec in records {
            if rec.updated > newest {
                newest.clone_from(&rec.updated);
            }
            let r = match remote(rec) {
                Ok(r) => r,
                Err(err) => {
                    tracing::warn!(%err, "skipping unreadable remote file");
                    continue;
                }
            };
            if self
                .synced
                .get(&r.meta.path)
                .is_some_and(|s| s.record_id == r.record_id && s.revision == r.revision)
            {
                continue;
            }
            if self.out_of_time() {
                complete = false;
                break;
            }
            if let Err(err) = self.reconcile(&r) {
                complete = false;
                self.out.errors.push(format!("{}: {err:#}", r.meta.path));
            }
        }
        // Anything left behind is listed again next time.
        if complete {
            self.out.cursor = newest;
        }
    }

    fn upload(&mut self, path: &str, local: &Local) -> Result<()> {
        let state = self.synced.get(path).cloned().unwrap_or_default();
        let meta = serde_json::to_string(&Meta {
            v: 1,
            path: path.to_owned(),
            hash: local.hash.clone(),
            size: local.size,
        })?;
        let key = key_for(path);
        let modified =
            chrono::DateTime::<chrono::Utc>::from_timestamp_nanos(local.mtime).to_rfc3339();
        let base = state.revision.to_string();
        // `.folders` goes up from memory so the merge base is exactly what
        // the server got.
        let folders_text = if path == FOLDERS_PATH {
            Some(std::fs::read(&local.abs)?)
        } else {
            None
        };
        let data = match &folders_text {
            Some(bytes) => Part::bytes(bytes),
            None => Part::file(&local.abs)?,
        }
        .file_name("blob")
        .mime_str("application/octet-stream")
        .map_err(|e| anyhow!("{e}"))?;
        let mut form = Form::new()
            .text("device", self.device_id)
            .text("modified", &modified)
            .text("meta", &meta)
            .part("data", data);
        let creating = state.record_id.is_empty();
        let mut resp = if creating {
            form = form.text("owner", &self.session.user_id).text("key", &key);
            self.agent
                .post(self.records_url())
                .header("Authorization", self.bearer())
                .send(form)?
        } else {
            form = form.text("base_revision", &base);
            self.agent
                .patch(format!("{}/{}", self.records_url(), state.record_id))
                .header("Authorization", self.bearer())
                .send(form)?
        };
        let status = resp.status().as_u16();
        let body = resp.body_mut().read_to_string()?;
        let taken =
            status == 409 || (status == 400 && creating && body.contains("validation_not_unique"));
        if status == 200 {
            let rec: Record = serde_json::from_str(&body).context("parsing the upload reply")?;
            let s = self.synced.entry(path.to_owned()).or_default();
            path.clone_into(&mut s.path);
            s.record_id = rec.id;
            s.revision = rec.revision;
            s.hash.clone_from(&local.hash);
            if let Some(bytes) = folders_text {
                self.out.folders_pushed = Some(String::from_utf8_lossy(&bytes).into_owned());
            }
            self.out.up += 1;
            Ok(())
        } else if taken {
            // Someone else holds this name (or wrote first): fold theirs in;
            // whatever of ours is left goes up next cycle.
            self.out.more = true;
            match self.fetch_by_key(&key)? {
                Some(r) => self.reconcile(&r),
                None => Ok(()),
            }
        } else if status == 404 && !creating {
            // The record went away under us: create it next time.
            if let Some(s) = self.synced.get_mut(path) {
                s.record_id.clear();
                s.revision = 0;
                s.hash.clear();
            }
            self.out.more = true;
            Ok(())
        } else {
            Err(api_error(status, &body))
        }
    }

    fn push(&mut self) {
        let mut paths: Vec<String> = self
            .locals
            .iter()
            .filter(|(path, local)| self.synced.get(*path).is_none_or(|s| s.hash != local.hash))
            .map(|(path, _)| path.clone())
            .collect();
        // Small things first, so one huge file never starves the rest.
        paths.sort_by_key(|p| self.locals[p].size);
        for path in paths {
            if path == FOLDERS_PATH && self.folders_incoming {
                continue;
            }
            // Reconciling an earlier upload may have moved this one.
            let Some(local) = self.locals.get(&path).cloned() else {
                continue;
            };
            if local.size > MAX_SIZE {
                self.out.errors.push(format!(
                    "{path}: too big to sync ({} MB)",
                    local.size / (1024 * 1024)
                ));
                continue;
            }
            if self.out_of_time() {
                break;
            }
            if let Err(err) = self.upload(&path, &local) {
                self.out.errors.push(format!("{path}: {err:#}"));
            }
        }
    }
}

/// The files half of one sync cycle.
pub fn run(session: &Session, device_id: &str, job: FilesJob) -> FilesOutcome {
    let known: HashMap<String, FileState> =
        job.known.into_iter().map(|s| (s.path.clone(), s)).collect();
    let mut errors = Vec::new();
    let mut locals = HashMap::new();
    for (rel, abs, meta) in scan(&job.notes_dir) {
        let (size, mtime) = (meta.len(), mtime_ns(&meta));
        let cached = known
            .get(&rel)
            .filter(|s| {
                rel != FOLDERS_PATH
                    && s.size == size
                    && s.mtime == mtime
                    && !s.local_hash.is_empty()
            })
            .map(|s| s.local_hash.clone());
        let hash = match cached.map_or_else(|| hash_file(&abs), Ok) {
            Ok(h) => h,
            Err(err) => {
                errors.push(format!("{rel}: {err:#}"));
                continue;
            }
        };
        locals.insert(
            rel,
            Local {
                abs,
                size,
                mtime,
                hash,
            },
        );
    }

    let mut cycle = Cycle {
        agent: agent(),
        session,
        device_id,
        root: job.notes_dir.clone(),
        started: Instant::now(),
        locals,
        synced: known.clone(),
        file_token: None,
        folders_incoming: false,
        out: FilesOutcome {
            cursor: job.cursor.clone(),
            errors,
            ..Default::default()
        },
    };
    cycle.pull(&job.cursor);
    cycle.push();

    // Bookkeeping: every file we looked at, where anything changed.
    let Cycle {
        locals,
        synced,
        mut out,
        ..
    } = cycle;
    for (path, local) in &locals {
        let agreed = synced.get(path).cloned().unwrap_or_default();
        let state = FileState {
            path: path.clone(),
            local_hash: local.hash.clone(),
            size: local.size,
            mtime: local.mtime,
            ..agreed
        };
        if known.get(path) != Some(&state) {
            out.states.push(state);
        }
    }
    // Agreed with the server but not on disk (`.folders` before its merge).
    for (path, state) in synced {
        if !locals.contains_key(&path) && known.get(&path) != Some(&state) {
            out.states.push(state);
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn only_assets_and_the_folder_list_are_safe() {
        assert!(safe_path(".folders"));
        assert!(safe_path("assets/pic.jpg"));
        assert!(safe_path("assets/trip/pic.jpg"));
        for bad in [
            "",
            "assets/",
            "note.md",
            "assets/../note.md",
            "assets/.links/x.json",
            "assets/.incoming/x.part",
            "assets//x",
            "/etc/passwd",
            "assets\\x",
            ".trash/x.md",
        ] {
            assert!(!safe_path(bad), "{bad}");
        }
    }

    #[test]
    fn keys_hide_the_name() {
        let k = key_for("assets/pic.jpg");
        assert_eq!(k.len(), 64);
        assert!(!k.contains("pic"));
        assert_ne!(k, key_for("assets/pic-2.jpg"));
    }

    #[test]
    fn free_names_count_up() {
        let tmp = tempfile::tempdir().unwrap();
        std::fs::create_dir_all(tmp.path().join("assets")).unwrap();
        assert_eq!(free_name(tmp.path(), "assets/pic.jpg"), "assets/pic-2.jpg");
        std::fs::write(tmp.path().join("assets/pic-2.jpg"), b"x").unwrap();
        assert_eq!(free_name(tmp.path(), "assets/pic.jpg"), "assets/pic-3.jpg");
        assert_eq!(free_name(tmp.path(), "assets/README"), "assets/README-2");
    }

    #[test]
    fn the_scan_skips_hidden_things() {
        let tmp = tempfile::tempdir().unwrap();
        let root = tmp.path();
        std::fs::create_dir_all(root.join("assets/.links")).unwrap();
        std::fs::create_dir_all(root.join("assets/trip")).unwrap();
        std::fs::write(root.join("assets/.links/card.json"), b"{}").unwrap();
        std::fs::write(root.join("assets/trip/pic.jpg"), b"jpg").unwrap();
        std::fs::write(root.join("assets/doc.pdf"), b"pdf").unwrap();
        std::fs::write(root.join(".folders"), b"work\n").unwrap();
        std::fs::write(root.join("Note.md"), b"# Note\n").unwrap();
        let mut found: Vec<String> = scan(root).into_iter().map(|(rel, ..)| rel).collect();
        found.sort();
        assert_eq!(found, [".folders", "assets/doc.pdf", "assets/trip/pic.jpg"]);
    }
}
