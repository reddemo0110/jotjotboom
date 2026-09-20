// SPDX-License-Identifier: GPL-3.0-only

//! The self-hosted backend: a PocketBase server set up from `server/`.
//! Two collections, `notes` and `files`; a hook owns the revision counter
//! and refuses (409) a write based on a stale one.

use super::Session;
use super::backend::{Backend, FileBody, FilePush, FileUpload, NotePush, NoteUpload, Page, RawFile, RawNote};
use anyhow::{Context, Result, anyhow};
use serde::Deserialize;
use std::io::{Read, Write};
use std::path::Path;
use std::time::Duration;
use ureq::unversioned::multipart::{Form, Part};

const USER_AGENT: &str = concat!("JotJotBoom/", env!("CARGO_PKG_VERSION"));
const PAGE: usize = 200;

fn agent() -> ureq::Agent {
    ureq::Agent::config_builder()
        .http_status_as_error(false)
        .timeout_global(Some(Duration::from_secs(30)))
        .user_agent(USER_AGENT)
        .build()
        .into()
}

/// Uploads and downloads can be long: no overall deadline, only patience
/// limits on connecting and on hearing back.
fn transfer_agent() -> ureq::Agent {
    ureq::Agent::config_builder()
        .http_status_as_error(false)
        .timeout_connect(Some(Duration::from_secs(30)))
        .timeout_recv_response(Some(Duration::from_secs(120)))
        .user_agent(USER_AGENT)
        .build()
        .into()
}

fn base(url: &str) -> String {
    url.trim().trim_end_matches('/').to_owned()
}

/// Pull the server's error message out of a failed response.
fn api_error(status: u16, body: &str) -> anyhow::Error {
    #[derive(Deserialize)]
    struct Err {
        #[serde(default)]
        message: String,
        #[serde(default)]
        data: serde_json::Value,
    }
    let parsed: Option<Err> = serde_json::from_str(body).ok();
    let msg = parsed
        .as_ref()
        .map(|e| e.message.clone())
        .filter(|m| !m.is_empty())
        .unwrap_or_else(|| format!("HTTP {status}"));
    // Field-level detail ("email: Value must be unique.") when there is one.
    let detail = parsed
        .and_then(|e| e.data.as_object().cloned())
        .map(|fields| {
            fields
                .iter()
                .filter_map(|(k, v)| v.get("message").and_then(|m| m.as_str()).map(|m| format!("{k}: {m}")))
                .collect::<Vec<_>>()
                .join("; ")
        })
        .filter(|d| !d.is_empty());
    match detail {
        Some(d) => anyhow!("{msg} ({d})"),
        None => anyhow!("{msg}"),
    }
}

#[derive(Deserialize)]
struct AuthResponse {
    token: String,
    record: AuthRecord,
}

#[derive(Deserialize)]
struct AuthRecord {
    id: String,
    #[serde(default)]
    email: String,
}

fn auth_response(url: &str, email: &str, mut resp: ureq::http::Response<ureq::Body>) -> Result<Session> {
    let status = resp.status().as_u16();
    let body = resp.body_mut().read_to_string().context("reading reply")?;
    if status >= 400 {
        return Err(api_error(status, &body));
    }
    let auth: AuthResponse = serde_json::from_str(&body).context("parsing sign-in reply")?;
    Ok(Session {
        url: base(url),
        token: auth.token,
        user_id: auth.record.id,
        email: if auth.record.email.is_empty() {
            email.to_owned()
        } else {
            auth.record.email
        },
    })
}

/// Sign in with email + password.
pub fn sign_in(url: &str, email: &str, password: &str) -> Result<Session> {
    let url = base(url);
    if url.is_empty() {
        anyhow::bail!("enter the server address");
    }
    let resp = agent()
        .post(format!("{url}/api/collections/users/auth-with-password"))
        .send_json(serde_json::json!({ "identity": email.trim(), "password": password }))
        .with_context(|| format!("reaching {url}"))?;
    auth_response(&url, email, resp).context("signing in")
}

/// Create the account, then sign in.
pub fn sign_up(url: &str, email: &str, password: &str) -> Result<Session> {
    let url = base(url);
    if url.is_empty() {
        anyhow::bail!("enter the server address");
    }
    if password.chars().count() < 8 {
        anyhow::bail!("the password needs at least 8 characters");
    }
    let mut resp = agent()
        .post(format!("{url}/api/collections/users/records"))
        .send_json(serde_json::json!({
            "email": email.trim(),
            "password": password,
            "passwordConfirm": password,
        }))
        .with_context(|| format!("reaching {url}"))?;
    let status = resp.status().as_u16();
    let body = resp.body_mut().read_to_string().context("reading reply")?;
    if status >= 400 {
        return Err(api_error(status, &body)).context("creating the account");
    }
    sign_in(&url, email, password)
}

/// Validate the token and get a fresh one. `Ok(None)` means the server
/// refused it (expired, revoked, account gone) — sign in again.
pub fn refresh(session: &Session) -> Result<Option<Session>> {
    let resp = agent()
        .post(format!("{}/api/collections/users/auth-refresh", session.url))
        .header("Authorization", format!("Bearer {}", session.token))
        .send_empty()
        .with_context(|| format!("reaching {}", session.url))?;
    let status = resp.status().as_u16();
    if status == 401 || status == 403 || status == 404 {
        return Ok(None);
    }
    auth_response(&session.url, &session.email, resp)
        .map(Some)
        .context("refreshing the sign-in")
}

#[derive(Deserialize)]
struct NoteRecord {
    id: String,
    #[serde(default)]
    note: String,
    #[serde(default)]
    revision: i64,
    #[serde(default)]
    device: String,
    #[serde(default)]
    modified: String,
    #[serde(default)]
    updated: String,
    #[serde(default)]
    blob: String,
}

impl From<NoteRecord> for RawNote {
    fn from(r: NoteRecord) -> Self {
        RawNote {
            record_id: r.id,
            note_id: r.note,
            revision: r.revision,
            device: r.device,
            modified: r.modified,
            blob: r.blob,
        }
    }
}

#[derive(Deserialize)]
struct FileRecord {
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

impl From<FileRecord> for RawFile {
    fn from(r: FileRecord) -> Self {
        RawFile {
            record_id: r.id,
            key: r.key,
            revision: r.revision,
            meta: r.meta,
            locator: r.data,
        }
    }
}

#[derive(Deserialize)]
struct ListResponse<T> {
    items: Vec<T>,
}

pub struct PocketBase {
    session: Session,
    agent: ureq::Agent,
    transfers: ureq::Agent,
    /// Protected files are fetched with a short-lived token of their own.
    file_token: Option<String>,
}

impl PocketBase {
    pub fn new(session: Session) -> Self {
        Self {
            session,
            agent: agent(),
            transfers: transfer_agent(),
            file_token: None,
        }
    }

    fn records_url(&self, collection: &str) -> String {
        format!("{}/api/collections/{collection}/records", self.session.url)
    }

    fn bearer(&self) -> String {
        format!("Bearer {}", self.session.token)
    }

    /// Every record of `collection` changed at or after `cursor`, oldest
    /// first, as raw JSON pages.
    fn list<T: for<'de> Deserialize<'de>>(&self, collection: &str, cursor: &str) -> Result<Vec<T>> {
        let mut out = Vec::new();
        for page in 1.. {
            let mut req = self
                .agent
                .get(self.records_url(collection))
                .header("Authorization", self.bearer())
                .query("sort", "updated,id")
                .query("perPage", PAGE.to_string())
                .query("page", page.to_string())
                .query("skipTotal", "1");
            if !cursor.is_empty() {
                req = req.query("filter", format!("updated >= \"{cursor}\""));
            }
            let mut resp = req.call()?;
            let status = resp.status().as_u16();
            let body = resp.body_mut().read_to_string()?;
            if status == 404 && collection == "files" {
                anyhow::bail!(
                    "the server has no `files` collection yet — add the new migration and hook from server/pocketbase"
                );
            }
            if status >= 400 {
                return Err(api_error(status, &body));
            }
            let list: ListResponse<T> = serde_json::from_str(&body).context("parsing the reply")?;
            let n = list.items.len();
            out.extend(list.items);
            if n < PAGE {
                break;
            }
        }
        Ok(out)
    }

    /// The one record of `collection` where `field` is `value`.
    fn find<T: for<'de> Deserialize<'de>>(&self, collection: &str, field: &str, value: &str) -> Result<Option<T>> {
        let mut resp = self
            .agent
            .get(self.records_url(collection))
            .header("Authorization", self.bearer())
            .query("filter", format!("{field} = \"{}\"", value.replace('"', "")))
            .query("perPage", "1")
            .query("skipTotal", "1")
            .call()?;
        let status = resp.status().as_u16();
        let body = resp.body_mut().read_to_string()?;
        if status >= 400 {
            return Err(api_error(status, &body));
        }
        let list: ListResponse<T> = serde_json::from_str(&body)?;
        Ok(list.items.into_iter().next())
    }

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
}

impl Backend for PocketBase {
    fn authorize(&mut self) -> Result<Option<Session>> {
        // A dead token makes every other call fail quietly (PocketBase
        // treats an unknown token as a guest with nothing to see).
        let fresh = refresh(&self.session)?;
        if let Some(s) = &fresh {
            self.session = s.clone();
        }
        Ok(fresh)
    }

    fn pull_notes(&mut self, cursor: &str) -> Result<Page<RawNote>> {
        let records: Vec<NoteRecord> = self.list("notes", cursor).context("pulling changes")?;
        let mut newest = cursor.to_owned();
        let items = records
            .into_iter()
            .map(|r| {
                if r.updated > newest {
                    newest.clone_from(&r.updated);
                }
                r.into()
            })
            .collect();
        Ok(Page { items, cursor: newest })
    }

    fn push_note(&mut self, up: &NoteUpload) -> Result<NotePush> {
        let (mut resp, creating) = match up.record_id {
            None => (
                self.agent
                    .post(self.records_url("notes"))
                    .header("Authorization", self.bearer())
                    .send_json(serde_json::json!({
                        "owner": self.session.user_id,
                        "note": up.note_id,
                        "device": up.device,
                        "modified": up.modified,
                        "blob": up.blob,
                    }))?,
                true,
            ),
            Some(id) => (
                self.agent
                    .patch(format!("{}/{id}", self.records_url("notes")))
                    .header("Authorization", self.bearer())
                    .send_json(serde_json::json!({
                        "device": up.device,
                        "modified": up.modified,
                        "blob": up.blob,
                        "base_revision": up.base_revision,
                    }))?,
                false,
            ),
        };
        let status = resp.status().as_u16();
        let body = resp.body_mut().read_to_string()?;
        let theirs = |pb: &Self| -> Result<NotePush> {
            let found: Option<NoteRecord> = pb.find("notes", "note", up.note_id)?;
            Ok(NotePush::Conflict(found.map(Into::into)))
        };
        match status {
            200 => {
                let r: NoteRecord = serde_json::from_str(&body).context("parsing push reply")?;
                Ok(NotePush::Landed {
                    record_id: r.id,
                    revision: r.revision,
                    cursor: Some(r.updated),
                })
            }
            // Stale base revision (hook), or the note already exists up there
            // (a reinstall that lost its sync state, another device's copy).
            409 => theirs(self),
            400 if creating && body.contains("validation_not_unique") => theirs(self),
            // The record went away under us: start over as a create next time.
            404 if !creating => Ok(NotePush::Conflict(None)),
            _ => Err(api_error(status, &body)),
        }
    }

    fn list_files(&mut self, cursor: &str) -> Result<Page<RawFile>> {
        let records: Vec<FileRecord> = self.list("files", cursor).context("listing files")?;
        let mut newest = cursor.to_owned();
        let items = records
            .into_iter()
            .map(|r| {
                if r.updated > newest {
                    newest.clone_from(&r.updated);
                }
                r.into()
            })
            .collect();
        Ok(Page { items, cursor: newest })
    }

    fn file_by_key(&mut self, key: &str) -> Result<Option<RawFile>> {
        let found: Option<FileRecord> = self.find("files", "key", key)?;
        Ok(found.map(Into::into))
    }

    fn download(&mut self, file: &RawFile, dest: &Path) -> Result<()> {
        let token = self.token()?;
        let mut resp = self
            .transfers
            .get(format!(
                "{}/api/files/files/{}/{}",
                self.session.url, file.record_id, file.locator
            ))
            .query("token", &token)
            .call()?;
        let status = resp.status().as_u16();
        if status >= 400 {
            let body = resp.body_mut().read_to_string().unwrap_or_default();
            return Err(api_error(status, &body));
        }
        let mut reader = resp.body_mut().as_reader();
        let mut out = std::fs::File::create(dest)?;
        let mut buf = vec![0u8; 64 * 1024];
        loop {
            let n = reader.read(&mut buf)?;
            if n == 0 {
                break;
            }
            out.write_all(&buf[..n])?;
        }
        out.sync_all().ok();
        Ok(())
    }

    fn upload(&mut self, up: &FileUpload) -> Result<FilePush> {
        let data = match up.body {
            FileBody::Bytes(bytes) => Part::bytes(bytes),
            FileBody::Path(path) => Part::file(path)?,
        }
        // Never the real name: that lives in the opaque meta.
        .file_name("blob")
        .mime_str("application/octet-stream")
        .map_err(|e| anyhow!("{e}"))?;
        let base = up.base_revision.to_string();
        let mut form = Form::new()
            .text("device", up.device)
            .text("modified", up.modified)
            .text("meta", up.meta)
            .part("data", data);
        let creating = up.record_id.is_none();
        let mut resp = match up.record_id {
            None => {
                form = form.text("owner", &self.session.user_id).text("key", up.key);
                self.transfers
                    .post(self.records_url("files"))
                    .header("Authorization", self.bearer())
                    .send(form)?
            }
            Some(id) => {
                form = form.text("base_revision", &base);
                self.transfers
                    .patch(format!("{}/{id}", self.records_url("files")))
                    .header("Authorization", self.bearer())
                    .send(form)?
            }
        };
        let status = resp.status().as_u16();
        let body = resp.body_mut().read_to_string()?;
        match status {
            200 => {
                let r: FileRecord = serde_json::from_str(&body).context("parsing the upload reply")?;
                Ok(FilePush::Landed {
                    record_id: r.id,
                    revision: r.revision,
                })
            }
            409 => Ok(FilePush::Taken),
            400 if creating && body.contains("validation_not_unique") => Ok(FilePush::Taken),
            404 if !creating => Ok(FilePush::Gone),
            _ => Err(api_error(status, &body)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn api_errors_read_the_server_message() {
        let e = api_error(
            400,
            r#"{"data":{"email":{"code":"x","message":"Value must be unique."}},"message":"Failed to create record.","status":400}"#,
        );
        assert_eq!(
            e.to_string(),
            "Failed to create record. (email: Value must be unique.)"
        );
        assert_eq!(api_error(502, "<html>").to_string(), "HTTP 502");
    }

    #[test]
    fn urls_are_normalised() {
        assert_eq!(base(" https://x.example/ "), "https://x.example");
    }
}
