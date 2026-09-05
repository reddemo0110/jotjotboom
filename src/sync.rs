// SPDX-License-Identifier: GPL-3.0-only

//! Cloud sync against a self-hosted PocketBase server (`server/`).
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
//! the app thread, see `Store::apply_remote`.

use anyhow::{Context, Result, anyhow};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Duration;

/// Keyring key for the bearer token.
pub const TOKEN_KEY: &str = "sync-token";
const USER_AGENT: &str = concat!("JotJotBoom/", env!("CARGO_PKG_VERSION"));
const PAGE: usize = 200;

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
    /// The server's own change time — the pull cursor.
    pub updated: String,
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
}

fn agent() -> ureq::Agent {
    ureq::Agent::config_builder()
        .http_status_as_error(false)
        .timeout_global(Some(Duration::from_secs(30)))
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
struct Record {
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

#[derive(Deserialize)]
struct ListResponse {
    items: Vec<Record>,
}

fn remote(r: Record) -> Result<Remote> {
    Ok(Remote {
        envelope: Envelope::decode(&r.blob).with_context(|| format!("note {}", r.note))?,
        record_id: r.id,
        note_id: r.note,
        revision: r.revision,
        device: r.device,
        modified: r.modified,
        updated: r.updated,
    })
}

fn records_url(session: &Session) -> String {
    format!("{}/api/collections/notes/records", session.url)
}

/// Every record changed at or after `cursor`, oldest first.
fn pull(agent: &ureq::Agent, session: &Session, cursor: &str) -> Result<Vec<Remote>> {
    let mut out = Vec::new();
    let filter = if cursor.is_empty() {
        String::new()
    } else {
        format!("updated >= \"{cursor}\"")
    };
    for page in 1.. {
        let mut req = agent
            .get(records_url(session))
            .header("Authorization", format!("Bearer {}", session.token))
            .query("sort", "updated,id")
            .query("perPage", PAGE.to_string())
            .query("page", page.to_string())
            .query("skipTotal", "1");
        if !filter.is_empty() {
            req = req.query("filter", &filter);
        }
        let mut resp = req.call().context("pulling changes")?;
        let status = resp.status().as_u16();
        let body = resp.body_mut().read_to_string()?;
        if status >= 400 {
            return Err(api_error(status, &body)).context("pulling changes");
        }
        let list: ListResponse = serde_json::from_str(&body).context("parsing pulled changes")?;
        let n = list.items.len();
        for r in list.items {
            match remote(r) {
                Ok(r) => out.push(r),
                Err(err) => tracing::warn!(%err, "skipping unreadable remote note"),
            }
        }
        if n < PAGE {
            break;
        }
    }
    Ok(out)
}

/// The server's copy of one note, by note id.
fn fetch_by_note(agent: &ureq::Agent, session: &Session, note_id: &str) -> Result<Option<Remote>> {
    let mut resp = agent
        .get(records_url(session))
        .header("Authorization", format!("Bearer {}", session.token))
        .query("filter", format!("note = \"{}\"", note_id.replace('"', "")))
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

enum PushResult {
    Landed(Record),
    /// Someone else wrote first; here is their copy.
    Conflict(Option<Remote>),
}

fn push_one(agent: &ureq::Agent, session: &Session, device_id: &str, p: &Pending) -> Result<PushResult> {
    let blob = p.envelope.encode();
    let (mut resp, creating) = match &p.record_id {
        None => (
            agent
                .post(records_url(session))
                .header("Authorization", format!("Bearer {}", session.token))
                .send_json(serde_json::json!({
                    "owner": session.user_id,
                    "note": p.note_id,
                    "device": device_id,
                    "modified": p.modified,
                    "blob": blob,
                }))?,
            true,
        ),
        Some(id) => (
            agent
                .patch(format!("{}/{id}", records_url(session)))
                .header("Authorization", format!("Bearer {}", session.token))
                .send_json(serde_json::json!({
                    "device": device_id,
                    "modified": p.modified,
                    "blob": blob,
                    "base_revision": p.base_revision,
                }))?,
            false,
        ),
    };
    let status = resp.status().as_u16();
    let body = resp.body_mut().read_to_string()?;
    match status {
        200 => Ok(PushResult::Landed(serde_json::from_str(&body).context("parsing push reply")?)),
        // Stale base revision (hook), or the note already exists up there
        // (a reinstall that lost its sync state, another device's copy).
        409 => Ok(PushResult::Conflict(fetch_by_note(agent, session, &p.note_id)?)),
        400 if creating && body.contains("validation_not_unique") => {
            Ok(PushResult::Conflict(fetch_by_note(agent, session, &p.note_id)?))
        }
        // The record went away under us: start over as a create next time.
        404 if !creating => Ok(PushResult::Conflict(None)),
        _ => Err(api_error(status, &body)),
    }
}

/// One sync cycle. Never panics on a network failure: the outcome carries
/// what worked and what did not, so a flaky link only delays things.
pub fn run(job: Job) -> Outcome {
    let mut out = Outcome {
        cursor: job.cursor.clone(),
        ..Default::default()
    };
    let agent = agent();

    // The token first: a dead one makes every other call fail quietly
    // (PocketBase treats an unknown token as a guest with nothing to see).
    let session = match refresh(&job.session) {
        Ok(Some(s)) => s,
        Ok(None) => {
            out.unauthorized = true;
            return out;
        }
        Err(err) => {
            out.errors.push(format!("{err:#}"));
            return out;
        }
    };
    out.session = Some(session.clone());

    // Pull.
    let mut incoming_ids = std::collections::HashSet::new();
    match pull(&agent, &session, &job.cursor) {
        Ok(records) => {
            for r in records {
                if r.updated > out.cursor {
                    out.cursor = r.updated.clone();
                }
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
        match push_one(&agent, &session, &job.device_id, p) {
            Ok(PushResult::Landed(r)) => {
                if r.updated > out.cursor {
                    out.cursor = r.updated.clone();
                }
                out.pushed.push(Pushed {
                    note_id: p.note_id.clone(),
                    record_id: r.id,
                    revision: r.revision,
                    hash: p.hash.clone(),
                    trashed: p.envelope.trashed,
                    deleted: p.envelope.deleted,
                });
            }
            Ok(PushResult::Conflict(Some(r))) => {
                out.conflicts.push(p.note_id.clone());
                if r.updated > out.cursor {
                    out.cursor = r.updated.clone();
                }
                out.incoming.push(r);
            }
            Ok(PushResult::Conflict(None)) => out.conflicts.push(p.note_id.clone()),
            Err(err) => out.errors.push(format!("{}: {err:#}", p.note_id)),
        }
    }
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
        let s = Session {
            url: "u".into(),
            token: String::new(),
            user_id: "id".into(),
            email: String::new(),
        };
        assert_eq!(s.account(), "u|id");
    }
}
