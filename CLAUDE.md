# JotJotBoom — agent notes

Hybrid-markdown notes app for the COSMIC desktop, in Rust + libcosmic.
Read `Project Handover — …md` for the locked architecture and
`DECISIONS.md` for how the open questions were settled and why.

## Build / run / test

Toolchain lives in `~/.cargo/bin` (rustup); make sure it's on `PATH`.

- `cargo build` / `cargo run` — debug build, launches on the current Wayland session
- `cargo test` — unit tests (note format, SQLite/FTS5 index, store round-trips)
- `just run` — release build + run; `just check` — clippy pedantic
- `just install-user` — release build + per-user install (~/.local: binary,
  launcher entry with absolute Exec, icon, metainfo); `just uninstall-user`
- `RUST_LOG=jotjotboom=debug cargo run` for tracing output
- Visual check without a human: `tools/xshot.py out.png [--keys ctrl+n --type 'text']`
  runs the app on Xwayland, drives it via the `JJB_SCRIPT` hook (steps:
  new, type, search, select, pin, trash, folder, format, selectall, dock,
  themes, theme, wait, exit; `;` separates steps — write `\;` inside text),
  and captures the window with X auto-repeat switched off. Portal screenshots hang unattended, and the in-app `JJB_SCREENSHOT`
  hook (iced `window::screenshot`) silently drops editor text and menu labels.

libcosmic is pulled from git; its API moves. When in doubt, read the checkout
under `~/.cargo/git/checkouts/libcosmic-*/` rather than trusting docs.

## Layout

- `src/note.rs` — document format (frontmatter + markdown body) and pure
  functions: title derivation, preview stripping, `#tag` and `[[link]]`
  extraction, filename slugs. Fully unit tested; keep it that way.
- `src/store/` — `fs.rs` (notes dir, atomic writes, `.trash/`), `db.rs`
  (rusqlite schema, FTS5 search, tag/link queries, oplog), `mod.rs` (the
  `Store` that ties them: reindex-from-disk, create/save/trash/restore).
- `src/app.rs` — the libcosmic `Application`: framed views/tags/notes/editor,
  dock, theme picker drawer, autosave, key binds.
- `src/retro.rs` — palettes, btop-style `frame`, style classes, swatches.
- `src/markdown.rs` — per-line markdown scanner + the editor highlighter.
- `src/images.rs` — image reference format, assets store, retro pixel treatments.
- `src/blocks.rs` — the note body as text/image blocks (one editor per text run).
- `src/config.rs` — cosmic-config entry (`notes_dir`, `device_id`).
- `src/secrets.rs` — keyring wrapper; unused until sync, deliberately present.

## Rules that matter

- Files on disk are the source of truth; `index.db` is derived and disposable.
  Never add state that lives only in SQLite (except the sync oplog).
- The sync payload will be the full file text; title/body/tags must stay
  inside it. Server-visible metadata is id/revision/timestamp/device only.
- Retro styling belongs to the editor surface and themes only. Chrome stays
  stock COSMIC.
- Autosave is debounced (`AUTOSAVE_DELAY`); anything that switches or
  drops the current note must call `close_current()` / `flush()` first.
