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

## 2026-08-29 — Verification tooling (after step 1 landed)
- Unattended UI checks: `tools/xshot.py out.png --script '...'` runs the app
  on Xwayland, drives it via the `JJB_SCRIPT` env hook (`src/debug_script.rs`),
  and grabs the window with XGetImage. Chosen because every other route failed
  without a human: portal screenshots hang waiting for approval, XTEST under
  Xwayland blocks on the libei portal, and iced's own `window::screenshot`
  (the `JJB_SCREENSHOT` hook) silently drops text-editor contents and menu
  labels — it looked like two rendering bugs that didn't exist.
- Both hooks are env-gated and cost nothing when unset; they stay in the
  binary for now so the CRT/editor work can be checked the same way.
- Steps 1 and 2 of the build order are done together: FTS5 search, tag
  parsing and the nested tag tree fell out of the storage design cheaply.

## 2026-08-29 — Retro direction: Option A (full terminal), btop-inspired
User feedback after seeing step 1: "needs more retro with a hint of btop".
Two directions were mocked up on a design canvas; the user picked **A**:
everything below the COSMIC header bar lives in btop-style frames (1px
rounded border, title cut into the top edge, right-hand badge), monospace
throughout, tag tree drawn with `└─` connectors. This supersedes the
handover's "retro confined to the editor surface" rule — the header bar and
window controls stay native COSMIC, the content area does not.
- Themes: green phosphor (default), amber, WordPerfect blue, and a
  "COSMIC" variant that derives its palette from the system theme. Chosen
  under View, persisted in cosmic-config (`theme`).
- Title face: VT323 (SIL OFL, bundled in `resources/fonts/`); body stays the
  system monospace for now — bitmap editor fonts are still a step-5 item.
- Built-in libcosmic nav bar replaced by our own two frames (`views`, `tags`)
  so the nav could be styled; `nav_model()` returns None.
- Still to come from the mockup: CRT scanline/glow overlay (needs a custom
  shader widget — step 5) and focus mode (needs the hybrid editor — step 3).

## 2026-08-29 — Dock and folders
- **Dock**: a floating pill at the bottom of the content area with the
  markdown format actions (bold, italic, code, H1, H2, bullet, to-do,
  wiki-link, tag, rule) and a `+`. Wrapping actions wrap the selection or
  insert an empty pair with the cursor inside; line actions toggle the
  prefix on the current line (and swap one list/heading prefix for another).
  Focus returns to the editor afterwards.
- **Folders are tags.** `+` → `folder` creates a tag with no notes yet. They
  are persisted in `<notes dir>/.folders` (one per line) — not in the
  index, which stays disposable — and merged into the tag tree with a count
  of 0. Creating a note while a folder is selected pre-fills `#folder` on
  its own line; a line made only of tags is neither the title nor "content",
  so an abandoned pre-filled note is still dropped and its title stays
  Untitled until the user types one.
- Folder names are normalised to tag form (lowercase, spaces → `-`,
  `parent/child` allowed). Removing a folder is `Store::remove_folder`, not
  yet exposed in the UI.

## 2026-08-29 — Inline markdown rendering and eight retro themes
- **Rendering without a custom editor.** iced's `text_editor` accepts a
  per-span highlighter (colour + font, no size). `markdown.rs` scans each
  line — headings, quotes, lists/tasks, fences, rules, `**`/`*`/`` ` ``/`~~`
  emphasis, `[[wiki]]`, `[md](links)`, `#tags` — and paints bold as bold,
  italic as italic, headings in the accent, tags/links in the second accent,
  done tasks dimmed, and every syntax marker in a near-invisible "ghost"
  colour. The markers still occupy space (the cursor walks over them), and
  headings cannot be larger than body text. Both limits go away with the
  cosmic-text editor in step 3; this gets 90% of the hybrid-editor feel today.
  View → "Show markdown syntax" paints markers dim instead of ghosted.
- **Nine palettes**: eight retro (green phosphor, amber, WordPerfect blue,
  paper white, plasma orange, Commodore 64, Game Boy, synthwave) plus a
  COSMIC one derived from the system theme. Picker lives in the context
  drawer (View → Theme colours, or the `◐` on the dock) as swatch cards
  drawn in each theme's own colours.
- Gotcha recorded: `JJB_SCRIPT` splits on `;` — escape as `\;` in note
  text or the tail of the step is silently dropped. Multi-line `type:` now
  types one line per tick so the highlighter sees keystroke-like edits.

## 2026-08-29 — Theme names are 80s action-movie puns (user request)
Keys in config are unchanged (`phosphor`, `amber`, …); only labels/blurbs
moved. Contrast pass at the same time: `dim` lifted on phosphor, amber,
WordPerfect, plasma and C64 (previews/timestamps were ~3:1), Game Boy accent
separated from its foreground.

## 2026-08-29 — Collapsible panes, dock inside the editor
- Views/tags column and notes list each toggle (header buttons, View menu,
  Ctrl+1 / Ctrl+2); Ctrl+0 / View → Editor only collapses both or brings
  both back. Persisted (`hide_nav`, `hide_list`, inverted so the default
  is shown).
- The dock moved into the editor frame as a centred pill at its foot; the
  `+` section opens as a second pill underneath so the main row never
  overflows. Notes list narrowed 340 → 320 so the dock fits at the 1024px
  minimum-ish width.
- Icon lookup only searches the Cosmic/Pop themes (Adwaita-only names such
  as `view-pin-symbolic` silently render nothing) — use `pin-symbolic`,
  `sidebar-places-symbolic`, `view-list-symbolic`.

## 2026-08-29 — Images
User picked *all* frame styles from the canvas, so every one is a per-image
option; placement is a setting with all three feasible layouts.
- **Format**: whole-line `![alt](assets/x.png){frame=tint size=m}`; the
  braces are Pandoc-style attributes, ignored by other markdown tools.
  Default frame/size are omitted so a plain `![alt](path)` stays plain.
- **Assets** live in `<notes dir>/assets/`, copied on import with readable
  unique names (`my-photo.png`, `my-photo-2.png`). Import via drag-and-drop
  onto the window or the `⧉` dock button (portal file picker). Clipboard
  paste is not wired yet (`images::import_bytes` is ready for it).
- **Treatments** (`images.rs`): tint (luma → bg…accent ramp), dither
  (Floyd–Steinberg to bg/mute/dim/fg), pixel (nearest to ~96px and back),
  bezel (every 3rd row darkened + vignette, in a rounded shell), ASCII
  (72 cols, accent text), print (off-white card + VT323 caption), film
  (sprocket rows + frame number), box (btop frame + size badge). Images are
  downscaled to ≤720px, processed on a blocking thread, cached by
  path|mtime|style|theme; theme-dependent styles re-render on theme change.
- **Placement**: View → Images: right rail (default) / top strip / bottom
  strip, persisted; a note can override with `images: top` in frontmatter.
  Sizes S/M/L per image. Each card has chips: cycle frame, cycle size, open
  in the system viewer. Inline flow waits for the custom editor.

## 2026-08-29 — Image picker and drag-and-drop, second pass
- The xdg-portal file chooser (`cosmic::dialog::file_chooser`) never
  returned on this COSMIC 1.7 session — no error, no dialog, the future just
  hangs — so the `⧉` button now opens an **in-app picker** in the context
  drawer: Home / Pictures / Downloads shortcuts, `..`, folders first, then
  image files. Self-contained, no portal dependency.
- winit's `FileDropped` only fires on X11. On Wayland libcosmic delivers
  drops through `widget::dnd_destination`; the editor frame is now wrapped
  in `dnd_destination_for_data::<UriList>` (`text/uri-list`, percent-decoded
  `file://` URLs) with a "drop to add" badge while hovering. Both paths stay
  so the X11 capture harness and real Wayland use both work.
- Lesson: the script harness proves logic, not integration with the
  compositor/portals — anything touching those needs a hands-on check.
