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

## 2026-08-29 — Inline images, magazine style (third pass)
User: the strip "looks like an attachment to an email"; wants pictures sitting
in the text, resizable, with a click-for-options menu.
- **Block editor** (`blocks.rs`): the body is split at image lines into
  text runs and images; every text run is its own text editor widget, images
  render between them exactly where their `![…]` line is. Cursor/selection
  do not cross blocks (images are atomic blocks); ↑/↓ at a block edge hop
  over the picture; Backspace at the start of the block below an image
  deletes it. The body is re-joined on save, so files are unchanged in form.
- `align=left|right` puts the paragraphs that follow the image beside it
  (a two-column region until the next image); `center` is a full-width block
  capped by `w=`. Widths are pixels: drag the `◢` grip (global mouse
  tracking, committed on release) or pick a preset in the menu.
- The `⋯` menu is a popover: 8 frame styles, sits left/centre/right, width
  presets + full, open, remove. Clicking the picture opens it too.
- Rail/top/bottom placements removed. Empty text runs around images
  contribute no lines when joining (a blank line between two adjacent
  images is normalised away — the one known lossy case).
- Gotcha: a text editor emits `ClearSelection` when it loses focus; only
  real interaction (click/drag/edit/move/select) may claim block focus.
- Gotcha: `retro::frame` was height Fill; inside a shrink row that collapses
  to nothing → `frame_sized(.., Length::Shrink, title_size)` for cards.

## 2026-08-29 — Image style feedback
- ASCII now follows the width like every other style: columns are derived
  from the display width (`ascii_layout`), glyph size fills it, and the
  render is cached per column count so resizing re-renders.
- The ⋯ menu has a **caption / title** field. It edits the image's alt text
  (`![caption](…)`), which is what the instant print prints and what the box
  frame shows as its title — one name, stored in plain markdown.

## 2026-08-29 — Undo / redo
- Note-level undo: a step is a snapshot of the whole body + caret (blocks
  are rebuilt on restore). 200 steps per open note (`UNDO_DEPTH`), stacks
  reset when switching notes. Redo cleared by any new edit.
- Grouping: consecutive typing (insert/enter/indent) or consecutive deleting
  within 700 ms is one step; switching between the two, pausing, pasting,
  dock format actions, image insert/remove/style/width/caption edits and
  Backspace-over-image each start a new step (caption typing groups).
- Ctrl+Z / Ctrl+Shift+Z / Ctrl+Y and an Edit menu. The stock text editor has
  no undo of its own, so this is the only undo.

## 2026-08-29 — Theme pass 2 (user feedback)
- Muted the loud backgrounds: Handhelder is now real-DMG olive
  (#a9b58a) instead of emulator lime; Little Blue's #0000aa softened to
  #1c2874 with gentler yellow/cyan; Escape from C64 desaturated slightly.
- New **Ghostwriters**: plain white (#ffffff / #fbfbfb panels) with
  #0c0c0c ink — never pure black — and a muted steel blue for tags/links.
- Five COLOURlovers-derived themes (hex values from the palettes, dark
  grounds derived from their deepest colour): Big Fish in Little China
  (Giant Goldfish), Blade Thinker (Thought Provoking), Cheer Up, Karate Kid
  (Cheer Up Emo Kid), The Abyss Five (Ocean Five), Adrift in Dreamscape
  (Adrift in Dreams). Blurbs credit the source palette.

## 2026-08-29 — Little Fish in Big China
Copy of Big Fish in Little China with the tangerine accent muted
(#fa6900 → #c9773f); every other colour identical. User request.

## 2026-08-29 — Shortcuts for everything
- One bind map (`key_binds`) is the single source: menus print the shortcut
  beside each item (libcosmic does this from the map), header/dock tooltips
  append it ("Bold · Ctrl + B"), and Ctrl+Shift+H opens a full-window
  overlay generated from the same map; × or Esc closes it.
- Layout: File — Ctrl+N new, Ctrl+Shift+N folder, Ctrl+Shift+I image,
  Ctrl+F search, Ctrl+Shift+P pin, Ctrl+Shift+D trash. Edit — Ctrl+Z,
  Ctrl+Shift+Z / Ctrl+Y. Format — Ctrl+B/I/E, Ctrl+1/2 headings, Ctrl+L
  bullet, Ctrl+T to-do, Ctrl+K link, Ctrl+Shift+3 tag (# lives on 3),
  Ctrl+R divider. View — Ctrl+Shift+1/2 panes, Ctrl+Shift+0 editor only,
  Ctrl+Shift+C themes, Ctrl+Shift+M markdown syntax, Ctrl+Shift+H this.
  Pane toggles moved from Ctrl+1/2 to Ctrl+Shift+1/2 to free the digits for
  headings. A new Format menu mirrors the dock.

## 2026-08-29 — Save indicator
- The editor badge no longer flips "editing… / saved hh:mm". It is a quiet
  `✓` (autosave is assumed); clicking it shows "✓ saved hh:mm" for four
  seconds. If a save fails the badge becomes "✗ not saved — last saved
  hh:mm" (tooltip has the error) and the edit stays pending with a retry
  every ~5 s until a save succeeds.

## 2026-08-29 — App icon and per-user install
- Icon: 128px SVG, no text (icon renderers can't be trusted to have our
  fonts): phosphor-green terminal frame with a two-dot "jot jot" title notch,
  three text lines, a block cursor, and an 8-point burst breaking out of the
  frame for "boom". Reads at 48px.
- `just install-user` installs without sudo into `~/.local` (binary,
  desktop entry with an absolute `Exec` because `~/.local/bin` is not on this
  machine's PATH, icon under hicolor/scalable/apps, metainfo) and refreshes
  the desktop/icon caches. The template's `install` recipe had the icon
  destination pointing at the directory, not the file — fixed.

## 2026-08-30 — Comic frame and drag-to-move pictures
- New frame style `comic` ("comic book"): a 45° halftone screen (5 px
  cells) over the photo, slightly desaturated (chroma × 0.8) to account for
  the ink — dot size follows the tone at each cell centre, paper shows
  between the dots, and each dot is a darker ink of the colour under it.
  Two heavier versions were built and thrown out: posterise + Sobel + black
  dots (muddy), then the full Photoshop/Toonify pipeline (bilateral smooth,
  auto-levels, cel bands, hysteresis outlines, Ben-Day shadows) — the user
  judged the flattening and outlines "terrible" on real photos, so the
  treatment is just the halftone. The panel is a thick ink border on a
  newsprint gutter; a caption becomes a yellow narration box in the
  top-left corner. Not palette-dependent.
- A picture is moved by dragging it (press, then travel > 6 px; a plain click
  still opens the ⋯ menu). While dragging, the note switches to a "drop
  mode": each body line is drawn as plain text with a top half (drop above)
  and a bottom half (drop below), so the drop line — an accent rule tagged
  "▼ picture drops here" — is drawn exactly where the picture will land, no
  pixel-to-line guessing. Dropping mid-paragraph splits it; the end of the
  note is reachable via a strip under the last line. Esc cancels; the move
  is one undo step. Rejected: estimating the target line from cursor y over
  the live editors (wrapping makes it lie) and a ghost image that follows
  the pointer (window-to-editor coordinates are not available to `view`).
- Script hook: `imgdrag:n:line` shows drop mode with the line before body
  line `line`; `imgmove:n:line` performs the move.

## 2026-08-30 — Flat layout: hairlines, not gaps; foldable sub-tags
- The user sent a reference screenshot: panes are flat and butt against each
  other, separated by one-pixel lines, no gutters, no boxes. The btop-style
  frames (Option A) are retired for the three columns: `retro::pane` is a
  flat pane with the title (still VT323 in the accent) as a header row and
  no border; `retro::vrule` / `retro::hrule` are the hairlines. The sidebar
  sits on `bg`, list and editor on `panel`, so the tonal step reads like
  the reference's darker sidebar. `frame_sized` stays for the inline "box frame"
  image style. `GAP` is gone.
- Sub-tags fold: a parent tag gets a ▸/▾ chevron; clicking it hides or
  shows everything beneath it. Folded tags persist in cosmic-config
  (`collapsed_tags`, full paths). Default is expanded. Indent is 14 px per
  level instead of the old `└─` prefix. Script step `fold:<tag>`.

## 2026-08-30 — Dock sizes, clean previews, text size, editor fonts
- Dock size: Small / Medium / Large / WOW! (`retro::DockSize`), chosen in
  the Appearance drawer, persisted as `dock_size`. The dock is now a
  `flex_row`, so the big sizes wrap onto extra lines instead of running
  off the pane.
- Titles and previews only ever contain typed text: `strip_inline_markup`
  swallows an image's `{frame=… w=…}` attributes with the image, the title
  is the first line that still says something afterwards, and the preview
  starts after that line (not after "the first non-blank line", which was
  wrong when a note opened with a picture). `SCHEMA_VERSION` bumped to 2 so
  the derived index rebuilds and stale previews disappear.
- Editor text size: − / + at the foot of the sidebar, 1 px a step,
  10–48 px, persisted as `editor_font_size` (0 = default 15). Applies to
  every text block and the drag-mode lines.
- Editor fonts: eight bundled faces (OFL/UFL, `resources/fonts`) plus the
  system monospace — IBM Plex Mono, Fira Mono, Ubuntu Mono, Anonymous Pro,
  Space Mono, Courier Prime, B612 Mono, VT323. Static Regular/Bold/Italic
  files were chosen over variable fonts because cosmic-text picks faces by
  weight and would render a variable font's bold as regular. Picked in the
  Appearance drawer, persisted as `editor_font`. ~3.8 MB in the binary.
- Bug found on the way: `iced::font::load` issued from `init()` is
  silently dropped when the compositor does not exist yet (winit's
  `Action::LoadFont` is `if let Some(compositor)`), so the VT323 title font
  had never actually rendered under the xshot harness. Fonts are now also
  loaded on `window::Event::Opened`; already-loaded slices are skipped.
- Script steps: `font:<key>`, `fontsize:+n|-n`, `docksize:<key>`.

## 2026-08-30 — Appearance drawer: Colour / Font / Size, beside the note
- The drawer is now three foldable sections (▸/▾ headers, Colour open by
  default): Colour (themes), Font (the nine editor faces), Size. Size has a
  card per text pane — Sidebar, Notes list, Editor — each with a live
  sample in that pane's font and − / + (1 px a step; sidebar and list
  9–30 px, editor 10–48 px), then the dock size row. Persisted as
  `sidebar_font_size`, `list_font_size`, `editor_font_size` (0 = default;
  13 / 13 / 15). The − / + footer that lived at the foot of the sidebar is
  gone — the user judged text size an occasional setting, not a control.
- The views pane's fixed height now follows the sidebar size so "Trash"
  never clips.
- `core.window.context_is_overlay = false`: libcosmic lays the drawer out
  beside the content instead of over the note being written. On a narrow
  window the editor column gives way to the drawer; on a wide one both fit.
- Script steps: `size:<sidebar|list|editor>:±n`, `section:<colour|font|size>`.

## 2026-08-30 — Rename a tag everywhere
- Right-click a tag in the sidebar → a small popover with "rename…"; that
  swaps to an inline input pre-filled with the full path (`travels/japan`).
  Enter commits, Esc / click-away cancels. `note::rename_tag` rewrites
  `#old` and every `#old/…` in a body (same scanner as `extract_tags`, so
  code fences, inline code and `#oldx` are left alone; the user's casing on
  the unrenamed remainder is kept). `Store::rename_tag` applies it to every
  file (trash included), re-indexes each changed file, and renames folder
  entries under the old path. The open note is flushed first and reloaded
  afterwards if it carried the tag; the current view and the folded set are
  remapped. Unit-tested at both layers.
- Script steps: `tagmenu:<tag>`, `renametag:<old>:<new>`.

## 2026-08-30 — Arrow navigation, real rules, clickable task boxes, format review
- ↑ / ↓ that nothing consumed (the editor is not focused — e.g. right after
  clicking a note) open the note above / below, like a mail client, and the
  list snaps so the selection stays in view. Implemented on the event
  subscription's `Status::Ignored`, so the editor's own caret movement is
  untouched.
- `---` / `***` / `___` lines are blocks of their own (`Segment::Rule`,
  `Block::Rule`), drawn as a full-width 2 px line in the muted colour.
  Typing one (Enter after `---`) or pressing the rule button re-splits the
  block live with the caret preserved (`Blocks::resplit`); Backspace at the
  start of the text below removes it, like an image. Fenced `---` stays
  text.
- Task boxes: clicking `[ ]` / `[x]` flips it (one undo step). `[x]` is
  drawn in the accent, bold; the finished text is the theme foreground at
  45 % alpha. What the stock iced text editor cannot do — a real tick
  glyph, a strike-through line, hidden markers, heading sizes — is exactly
  the custom rich-editor milestone (build step 3); the highlighter can only
  set colour and font per span.
- Format buttons reviewed: bold / italic / code / link now unwrap when the
  selection is already wrapped; heading / bullet / to-do apply to every
  line a selection touches (the first line decides add vs remove) and
  recognise `* ` / `+ ` / `[X]` prefixes; the rule button always puts the
  rule on a line of its own.
- Script steps: `nav:±n`, `togglebox:<line>:<col>`.

## 2026-08-30 — `[]` makes a task; pick your own "done" mark
- Typing `]` right after `[` at the start of a line (indent and a `- `
  allowed) expands to `- [ ] ` (`Blocks::expand_task_shorthand`).
- A task box is `[ ]` open, or `[` + one mark + `]` done. The mark lives
  in the file: `- [✓] milk`, `- [🦆] milk`. Everything that reads boxes —
  the highlighter, previews, the click toggle, the dock's to-do toggle —
  goes through `note::task_box`, which accepts any single mark (≤ 12
  bytes, no spaces), so `[x]` from other apps still works. Chosen in the
  Appearance drawer's new Tasks section (x, ✓, ✔, –, •, ★, 🦆, 🔥, 💀, 🍕,
  🐈, 🚀, 👍, 🍺) with a live sample; persisted as `task_marker`
  (empty = x). The hint says plainly that other markdown apps only count
  `x` as done — the user chose visible ticks over strict GFM.
- Script step: `marker:<mark>`.

## 2026-08-30 — The measure: a centred text column
- Like Craft and other reading-first editors (the user sent references), the note text
  lives in a column that stops growing at a chosen width and centres with
  margins; a narrower pane just wraps the text to fit. Implemented as a
  `max_width` container around the block column inside the editor's
  scrollable, so images, rules and drop-mode lines follow the same measure.
- Four widths, rated in fish in the Size section: 🐟 560 px, 🐟🐟 720 px
  (default), 🐟🐟🐟 920 px, 🐟🐟🐟🐟 no limit. Persisted as `text_width`.
- Script step: `measure:<key>`.

## 2026-08-30 — Designer font pairings
- Research (Typewolf, Pagecloud, Inspotype, Fontfabric, Readium's libre
  font list) agrees on the recipe: a display or serif for headings over a
  calm sans for UI, or one superfamily throughout; Atkinson Hyperlegible
  and Lexend are the accessibility picks; Lato + a serif, PT Sans + PT
  Serif, and Plex are the canonical free pairs. Only families with static
  Regular/Bold/Italic files were bundled (cosmic-text ignores variable
  axes): IBM Plex Serif, Lato, PT Sans, PT Serif, Atkinson Hyperlegible,
  Ubuntu, Spectral, DM Serif Display, Special Elite. Skipped for being
  variable-only: Playfair Display, Libre Baskerville, Merriweather, Source
  Sans/Serif, Literata, Lexend, Cormorant. Fonts are now ~13 MB of the
  binary.
- Eight pairings (`retro::PAIRINGS`), each setting three faces — pane
  titles, sidebar + list, note: JotJotBoom (default: VT323 / system mono /
  system mono), Plex, Editorial (Spectral + Lato), Magazine (DM Serif
  Display + Lato), ParaType (PT Serif + PT Sans), Hyperlegible, Ubuntu,
  Typewriter (Special Elite + Courier Prime). Persisted as `title_font`,
  `ui_font`, `editor_font`; "Restore default fonts" re-applies the first.
  Titles are set bold except for faces that only ship a Regular. The
  single "Editor font only" list stays underneath for mixing.
- Script step: `pairing:<key>`.

## 2026-08-30 — Rich editor: phase-0 spike passed
- On branch `rich-editor`. A throwaway widget (`src/editor/spike.rs`,
  `JJB_SPIKE=1`) proved every unverified assumption in the plan: per-span
  sizes, seam-free collapsed markers (transparent, 0.5 px), overlays from
  layout metadata, strikes from cosmic-text's own decoration spans, drawn
  through `fill_raw`. Findings and exact API notes are in
  `RICH-EDITOR-PLAN.md`. `cosmic-text = "0.19"` is now a direct dependency
  (same version iced uses). Next: phase 1, the real widget behind a
  `rich_editor` config flag.

## 2026-08-30 — Rich editor phase 1: the widget, behind a flag
- `Content` became an enum over iced's content and the new `RichContent`
  so the two editors coexist; new blocks pick the variant from the
  `rich_editor` flag (config, Appearance → Font toggle, `rich:on|off`).
  Default off until phase 2 renders more than the stock editor did.
- The widget copies iced's `text_editor` behaviour (click kinds, key
  table, focus operation, caret blink) but draws the shared buffer with
  `fill_raw`, which is what lets phase 2 add sizes, collapsed markers and
  overlays. Details in `RICH-EDITOR-PLAN.md`.

## 2026-08-30 — Rich editor phase 2: the formatted look
- Behind the same `rich_editor` flag the note now renders formatted:
  sized headings, hidden markers everywhere but the caret's line, drawn
  bullets and task boxes (`x` shown as a ✓, other marks as themselves),
  quote bars, code backgrounds, strike-throughs. Files are unchanged. The
  two hiding modes (collapsed vs transparent-at-width) and the overlay
  order are recorded in `RICH-EDITOR-PLAN.md`.

## 2026-08-30 — Rich editor phase 3: Ctrl+click and hover
- Ctrl+click on a `[[wiki link]]` opens that note (by title), on a `#tag`
  selects that tag; plain click keeps placing the caret so editing a link
  never fights with following it. Hover shows a hand over task boxes, and
  over links/tags with Ctrl held (plus an underline). Task clicks count
  from the list marker onwards, matching where the box is drawn.

## 2026-08-30 — Quote in the dock
- `> ` lines (the bar with italic text) get a dock button ❝, a Format-menu
  entry and Ctrl+Shift+Q, toggling like the other line prefixes.

## 2026-08-30 — Rich editor: raw markdown on double-click only
- Single click places the caret and keeps the line rendered; the line's
  markdown is revealed by double-click, by typing a marker character
  (`* _ ` ~ [ ] > #`), or by a Backspace/Delete that would eat a hidden
  marker (deleting plain text keeps the line rendered), and hides again
  when the caret leaves. Plain letters keep the line rendered. After `[]`
  expands to a task box, or a box is toggled, the line renders at once
  (`RichContent::render_now`), so a new to-do shows its box before its text.

## 2026-08-30 — Rich editor phase 4: only editor, merged
- IME preedit wired like iced's editor. The stock `text_editor` path,
  the highlighter, the flag and the spike are deleted; `Content` is the
  cosmic-text content. v0.2 = first release where the note is rendered
  formatted. Fallback if something goes badly wrong: tag
  `v0.1-pre-rich-editor` still builds the old editor.

## 2026-08-30 — Image picker: thumbnails in a grid
- The in-app picker defaults to a grid of 104×78 thumbnails (decoded off
  the UI thread to 160 px, cached per path for the session) with folders
  as chips above; a button flips to the old list. Script step
  `pickdir:<path>`.

## 2026-08-30 — 8-bit folder icons
- Emoji were rejected as folder icons ("I don't like the emoji style").
  `src/pixel.rs` holds 21 8×8 one-colour pixel glyphs (coffee, book,
  camera, home, briefcase, music, heart, star, plane, food, bulb, code,
  money, gift, leaf, gear, flag, pin, bug, game pad, beer), drawn in the
  theme's tag colour: as crisp SVG rects in the sidebar and the picker, as
  quads in the editor over the tag's hash. Right-click a tag → "icon…"
  opens the grid; "none" restores the `#`. Assignments are per full tag
  path (a leaf-only match also applies) and persist as `tag=icon` entries
  in `tag_icons`. Coffee words default to the coffee glyph, so the ☕ egg
  became the first icon. Script step `tagicon:<tag>:<key|none>`.

## 2026-08-30 — Coffee
- Three easter eggs, all opt-in by curiosity: a tag whose leaf is a coffee
  word (`coffee`, `espresso`, `flatwhite`, …) wears a ☕ instead of its `#`
  in the sidebar and in rendered notes (`coffee::is_coffee_tag`); typing
  `coffee` into the search box unlocks the hidden "Long Black" theme
  (crema on espresso, `coffee_unlocked` in config, listed after the
  sixteen); Ctrl+Shift+Enter raises a full-window neon sign — a braille-dot
  cup (`coffee::FRAMES`, generated from a pixel grid) whose steam drifts
  through four frames every 170 ms with the odd tube dimming, pink cup,
  cyan steam, VT323 "COFFEE" — any key or click puts it out.

## 2026-08-30 — Settings gear in the sidebar, Alt+,
- The drawer (titled Options) is opened from a ⚙ in the top-right of the
  views pane — tooltip "Settings" — and with Alt+, (Ctrl+Shift+C retired);
  the ◐ left the dock, which is now format actions only.

## 2026-08-30 — Editor header, H1 by default, drop line for new images
- The editor pane no longer repeats the note title in its header (the
  first line is the title); the ✓ save badge stays on the header's right.
- A new note starts as `# ` with the caret after it, so its first line is
  a heading. A body that is only heading markers counts as blank, so an
  untouched new note is still deleted on close.
- Dragging an image in from a file manager shows the same "picture drops
  here" line as moving one: the DnD destination reports pointer motion in
  window coordinates; each text widget records its drawn bounds, the app
  maps y to a line through the widget's own layout (`line_at_y`), and the
  widget draws the rule at that line. On drop the image is inserted at the
  caret and then moved to the target line. Images and rules between text
  blocks get the indicator from the block view, as in drag mode.

## 2026-08-30 — The icon: dot dot hash, in the theme's colours
- Rejected the phosphor-frame icon (title notch, three lines, cursor and a
  burst all merge below 32 px). Six minimal directions were sheeted at
  128→16 px; the user chose "dot dot boom", then a sticker cut (rejected:
  too wide, overlapped dock neighbours), then "dot dot hash" — two jots
  and a `#`, the sign the app runs on — with the jots on the hash's lower
  bar.
- Geometry follows the macOS app-icon grid the reference app uses: tile
  at 824/1024 of the canvas with clear margin, a continuous-curve
  squircle, no outline; the marks sit on the golden ratio (boom/hash
  height = jot × φ², jot→jot × φ = jot→hash). `src/icon.rs` bakes the
  geometry and fills the colours from a palette: the tile is the theme's
  background (lifted 10 % toward the text colour at the top), the marks
  are the note-writing colour — the icon is the note pane in miniature.
  (First cut used the accent as the tile; the user wanted bg + text.)
- The launcher icon is chosen in Appearance → Icon: any theme's colours,
  or "follow the colour theme" (default). The app writes the SVG to
  `~/.local/share/icons/hicolor/scalable/apps/<app id>.svg` and refreshes
  the cache, so the dock and app library follow; the running window's own
  icon updates on next launch. `resources/…/icon.svg` ships the Phosphor
  version for fresh installs. Script step `icon:<theme|follow>`.

## 2026-08-30 — Example notes and the brochure
- `examples/notes/`: eight sample notes (letter, journals, packing list,
  recipe, coffee log, index) with fourteen Unsplash photographs, credited in
  `examples/README.md`; `just install-examples` copies them into the notes
  folder without overwriting. They double as the fixtures for screenshots.
- `docs/brochure.html`: a single-file page (images embedded) telling the
  app's story and features, shot from the examples in the phosphor theme.
  Also published as an artifact for sharing.
