# Decisions log

Running record of decisions made during implementation. The handover doc
(`Project Handover — …md`) holds the locked-in architecture; this file
records the open questions as they get settled, plus anything discovered
while building. Newest at the bottom.

## 2026-08-29 — Open questions from the handover, settled

### App name: **JotJotBoom** (working name, from the project folder)
App id `io.github.jotjotboom.JotJotBoom`, binary `jotjotboom`. Change with a
single `grep -rl` / `sed` pass — it appears in `Cargo.toml`, `justfile`,
`build.rs`/xdgen metadata, and `src/app.rs` (`APP_ID`).

### Storage: **markdown files on disk + SQLite as a derived index**
- Notes live as `<title>.md` under a user-visible notes directory
  (default `~/Documents/JotJotBoom/`; configurable via cosmic-config).
- Each file starts with a small YAML frontmatter block carrying what the
  filesystem can't: `id` (UUIDv7), `created`, `pinned`. Title is the H1 /
  first line. Body below is the source of truth.
- Filename tracks the title (slug-safe); renames happen on save with a
  numeric suffix on collision. Trash = moved to `<notes dir>/.trash/`.
- SQLite (`$XDG_DATA_HOME/jotjotboom/index.db`) holds `notes`, `tags`,
  `note_tags`, `links`, `oplog`, and an FTS5 table. It is rebuilt from disk
  on startup (mtime + content hash diff) and written through on every save,
  so deleting the DB is always safe.
- Why files: grep/Syncthing/git-friendly for Linux users, per the handover's
  lean. Why an index anyway: FTS5 search, backlinks, tag tree, and the oplog
  the sync design needs.
- Sync payload implication: the opaque blob is the full file text
  (frontmatter + body). Title/body/tags all live inside it — satisfies the
  E2E metadata boundary. Plaintext side-channel is id/revision/timestamp/
  device only.

### Note list previews: **markdown-stripped text**
First ~120 chars of the body with markdown syntax removed, whitespace
collapsed. Computed on index write, stored in the `notes` table so the list
never re-parses.

### Attachments / images: **deferred** (not in v1)
Wiki-link and tag parsing will ignore `![…](…)` so nothing breaks when they
arrive later.

## 2026-08-29 — Environment notes
- Machine runs COSMIC 1.7 on Wayland; the app can be launched locally.
- No sudo available from the agent session. Rust installed via rustup into
  `~/.cargo`; `just` and `cargo-generate` via `cargo install`. `cmake` is
  not installed system-wide — if a crate demands it, drop a Kitware release
  tarball into `~/.local` or `sudo pacman -S cmake`.
