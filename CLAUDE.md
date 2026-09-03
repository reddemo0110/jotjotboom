# JotJotBoom — agent notes

Hybrid-markdown notes app for the COSMIC desktop, in Rust + libcosmic.
Read `Project Handover — …md` for the locked architecture and
`DECISIONS.md` for how the open questions were settled and why.
`RICH-EDITOR-PLAN.md` is the plan for build step 3 (the cosmic-text editor).

## Build / run / test

Toolchain lives in `~/.cargo/bin` (rustup); make sure it's on `PATH`.

- `cargo build` / `cargo run` — debug build, launches on the current Wayland session
- `cargo test` — unit tests (note format, SQLite/FTS5 index, store round-trips)
- `just run` — release build + run; `just check` — clippy pedantic
- `just install-user` — release build + per-user install (~/.local: binary,
  launcher entry with absolute Exec, icon, metainfo); `just uninstall-user`
- `RUST_LOG=jotjotboom=debug cargo run` for tracing output
- Visual check without a human: `tools/xshot.py out.png [--script ...]`
  runs the app on Xwayland, drives it via the `JJB_SCRIPT` hook (`step:arg` form, e.g. `--script 'new;type:Hello;wait:1000'`; steps:
  new, type, search, select, pin, trash, folder, format, selectall, dock,
  themes, theme, image, imgframe, imgalign, imgwidth, imgcaption, imgmenu, imgdrag, imgmove, linkdrag, fold, font, pairing, size, docksize, section, tagmenu, renametag, nav, togglebox, marker, measure, follow, icon, coffee, tagicon, pickdir, iconset, attach, tagdrag, tagmove, addspace, weight, fontfor, buffet, sel, cell, editcell, fpick, fpickover, pickdone, draft, fill, tsel, quit, wait, exit; `;` separates steps — write `\;` inside text),
  and captures the window with X auto-repeat switched off. It runs against a
  fresh scratch notes dir (`JJB_NOTES_DIR`); pass `--notes-dir ~/Documents/JotJotBoom`
  only when the real notes are needed for the picture — steps like `new`/`type`/`attach` write files. Portal screenshots hang unattended, and the in-app `JJB_SCREENSHOT`
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
- `src/markdown.rs` — per-line markdown scanner and span → colour/font table.
- `src/editor/` — the rich editor: `content.rs` (cosmic-text buffer + the
  editor API the app uses), `style.rs` (span → attributes, active line vs
  hidden markers), `widget.rs` (drawing, caret, mouse, keys, IME, Ctrl+click).
- `src/images.rs` — image reference format, assets store, retro pixel treatments.
- `src/links.rs` — link cards: `[text](url)` / bare address / attached file on a line of its own; Open Graph scrape (`ureq`), preview cache under `assets/.links/` (derived, disposable), `JJB_LINK_FIXTURE=page.html` serves a local page instead of the network.
- `src/blocks.rs` — the note body as text/image blocks (one editor per text run).
- `src/glyph.rs` — folder icons a tag can wear instead of `#`: seven styles (Boxicons, Iconoir, Solar, Myna UI, Majesticons, Pixelarticons, Duoicons; bundled as SVG paths from Iconify, Boxicons is the fallback), chosen in Options → Icon. Generated — regenerate rather than hand-edit the bodies.
- `src/icon.rs` — the launcher icon generated from a palette and written into
  the user's hicolor theme.
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
