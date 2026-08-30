# Rich editor — build step 3 plan

Status: planned 2026-08-30; phase 0 spike passed the same day; **all phases landed 2026-08-30**; the rich editor is the only editor since phase 4. Results per phase at the end. Read `DECISIONS.md` for how the app
got here; this document is the plan for the piece the handover called "the
long pole".

## Why

Every recent request — hide the `**`, real heading sizes, a tick in the box,
strike-through on finished tasks, a rule that is a line — runs into the same
wall: iced's `text_editor` lets a highlighter set **colour and font** per
span and nothing else. The wall is in iced's wrapper, not in the engine
underneath it. This plan replaces the wrapper.

## What the engine can do (verified in the checked-out sources)

| Need | cosmic-text 0.19 (`Attrs`) | iced fork (`c003a58`) |
|---|---|---|
| Per-span font size / line height | `metrics_opt: Option<CacheMetrics>` ✓ | not exposed by `highlighter::Format` |
| Strike-through / underline | `text_decoration` ✓ (attr only) | not exposed; renderer (`cryoglyph`) does **not** draw decorations — we draw quads |
| Per-span colour, family, weight, style | ✓ | ✓ (this is all `Format` carries) |
| Per-span metadata (span kind id) | `metadata: usize` ✓ — glyphs carry it, so draw-time overlays know what they sit on | — |
| Draw a raw buffer | — | `renderer.fill_raw(Raw { buffer: Weak<Buffer>, position, color, clip })` ✓, honours per-glyph colour (`cryoglyph::text_render.rs:250`) |
| Share the buffer between editor and renderer | `Editor::new(BufferRef::Arc(Arc<Buffer>))`, mutation via `Arc::make_mut` ✓ | hand the renderer a fresh `Weak` each draw |
| Hit-testing, layout runs | `Buffer::hit`, `layout_runs`, `line_layout` ✓ | — |
| Editing actions (motions, insert, delete, selection) | `cosmic_text::Editor` + `Action` ✓ | iced's `Content::perform` is a thin map onto these |

One `cosmic-text` version in `Cargo.lock` (0.19.0), so a direct dependency
pins to the same engine iced uses. `iced_graphics::text::{font_system,
to_attributes, Raw}` are public.

## Design

### Source of truth stays the markdown text

The buffer's text **is** the note body, byte for byte. Nothing is
transformed before layout; formatting is attributes plus a draw-time overlay
layer. This keeps every existing invariant (files are the truth, tags/links
extraction, autosave, undo snapshots, `blocks.rs` helpers) untouched.

### Obsidian-style live preview, not Typora

- The line holding the caret shows its markers, dimmed (exactly today's
  look). Everything else shows formatted with markers **collapsed**.
- Collapsing = marker span gets `color` transparent and `metrics_opt` with a
  near-zero font size, so it takes ~no width but still exists in the text.
  The caret only ever passes through markers on the active line, where they
  are visible, so there is no "invisible caret step" problem.
- Clicking into another line activates it; the line re-lays out with its
  markers shown and the caret is placed by hit-testing at the click's x.
  That is the same small shift Obsidian has. Acceptable.

The near-zero-size trick is the one thing not yet proven (shaping at ~0.5 px,
glyph cache). It is the first spike. Fallback if it misbehaves: markers stay
dimmed on all lines (today's behaviour) and we still get sizes, strike,
ticks, rules, backgrounds — most of the value.

### Rendering layer (spans → attrs → overlays)

Per line, the existing `markdown::scan_line` (already unit-tested, fence-
aware) produces spans. Each span kind maps to:

| Kind | Attrs | Overlay (quads / mini-buffers drawn by the widget) |
|---|---|---|
| Heading 1/2/3 | size ×1.6 / ×1.35 / ×1.15, bold, accent | — |
| Bold / italic / both | weight / style | — |
| Code span / block | mono family, accent2 | rounded background quad behind the run |
| Marker (`**`, `#`, fences) | active line: dim; other lines: collapsed | — |
| Tag, link | accent2 | underline quad on hover; Ctrl+click opens |
| Quote | dim italic | vertical bar quad in the gutter |
| List marker `- ` | collapsed off-line | • drawn at the run's x |
| Task box `[ ]` / `[✓]` | collapsed off-line | box quad; the user's mark (✓, 🦆…) drawn inside from a 1-glyph buffer; click toggles (hit-test on the box rect) |
| Done text | fg at 45 % | strike quad through the run's baseline |
| Strike `~~x~~` | — | strike quad |
| Rule `---` | collapsed | full-width line quad (rules leave `blocks.rs` and become inline again) |

Images stay as blocks in `blocks.rs` (they are widgets, not text) — the
rich editor replaces only the `text_editor` inside `Block::Text`.

### Widget shape

`src/editor/` (new module):

- `content.rs` — `RichContent`: owns `cosmic_text::Editor<'static>` over
  `Arc<Buffer>` and the span cache. Exposes the **same API the app already
  uses** from `text_editor::Content`: `with_text`, `text`, `line_count`,
  `line(i)`, `cursor`, `selection`, `move_to`, `perform(Action)`. Keeping
  that surface means `blocks.rs` (toggle, expand, resplit, move_image),
  `apply_format`, undo snapshots and the drag mode need only a type swap.
- `widget.rs` — `RichEditor` implementing `iced_core::Widget`, adapted from
  the fork's `text_editor.rs` (1 688 lines: focus, click/drag/double-click
  selection, keyboard → `Action`, scroll-into-view, IME preedit). Draws via
  `fill_raw` + quads (caret, selection, overlays). Height = sum of laid-out
  line heights (`Length::Shrink` inside the note's scrollable, like today).
- `style.rs` — span kind → `Attrs` for the active/inactive cases, taking
  the palette, editor font, text size and task mark from settings.

### What does not change

Files, index, tags, links, autosave, undo, the block model for images, the
drop-mode drag, the Appearance drawer, script hooks. `markdown.rs` keeps
its scanner; the `Highlighter` impl is deleted at the end.

## Deviations from the handover

- **No tree-sitter.** The handover planned `tree-sitter-markdown` for
  incremental parsing. The per-line scanner is already fast (it runs on
  visible lines only, as iced's highlighter does), fence state is tracked,
  and nothing we render needs multi-line structure beyond fences. Tree-
  sitter would add a C build dependency for no visible gain. Revisit only if
  tables or nested lists need real structure.

## Phases

0. **Spike (1 session)** — a throwaway widget that draws one raw buffer
   with: per-span sizes (a heading), a collapsed marker (the ~0 px trick),
   a strike quad, a drawn task box. Screenshot via `tools/xshot.py`.
   Go/no-go on the collapse trick; everything else is known to work.
1. **Core widget (2–3 sessions)** — `RichContent` + `RichEditor` with
   parity to `text_editor`: typing, motions, selection (mouse + keyboard),
   caret, focus, placeholder, scroll-into-view, Enter/Backspace edge cases
   the app relies on (`Message::Editor` block-edge logic). Swap it into
   `text_block` behind a config flag `rich_editor` (default off) so the old
   path stays available while it bakes. Existing unit tests must pass with
   the new content type.
2. **Rendering (1–2 sessions)** — the table above: attrs, active-line
   markers, overlays, sizes, task boxes with the chosen mark, inline rules.
   Flip `rich_editor` on by default.
3. **Interactions (1 session)** — click-to-toggle boxes, Ctrl+click links
   and tags, `[]` expansion and the dock actions through `perform`, hover
   cursors.
4. **Polish + removal (1 session)** — IME preedit, double-click word /
   triple-click line, delete the highlighter path and the config flag,
   `DECISIONS.md`, install.

Roughly 6–8 working sessions. Phase 1 is the risk-bearing one; phases 2–4
are mostly the fun part.

## Risks and answers

- *Collapse trick shapes badly* → fallback above; nothing else depends on it.
- *`Arc::make_mut` clones the buffer* → only if another strong ref exists;
  the renderer holds a `Weak`, so mutation is in place. Never keep a strong
  clone across frames.
- *Per-frame attrs cost* → apply to visible lines only (iced does the same);
  cache spans per line keyed by line text.
- *Losing iced's IME handling* → it lives in the vendored widget; port it in
  phase 4, not phase 1.
- *Two editors during the transition* → the config flag; both compile until
  phase 4 removes one.

## Phase 0 result (2026-08-30) — GO

`src/editor/spike.rs`, enabled with `JJB_SPIKE=1`, draws one raw buffer in
the editor pane. Verified in a capture (`tools/xshot.py`):

- per-span size works: a `# ` heading at 1.6× with its own line height;
- the collapse trick works: markers set transparent at 0.5 px leave **no
  visible seam** — `**bold**`, `` `code` ``, `# `, `~~…~~`, `[`/`]` all
  vanish while staying in the text;
- code-span background, task box (drawn quad, mark glyph inside) and
  strike-throughs land exactly where the layout says;
- cosmic-text 0.19 already computes decoration spans per layout run
  (`run.decorations`, with the font's strike offset/thickness), so strikes
  and underlines are a quad per span — no glyph maths of our own.

API notes for phase 1 (the fork's iced 0.14 / cosmic-text 0.19):

- font system: `iced::advanced::graphics::text::font_system().write().raw()`
  gives the `cosmic_text::FontSystem`;
- `Buffer::new(fs, metrics)`, then `set_size(w, h)` and
  `set_rich_text(spans, &default, Shaping::Advanced, None)` take **no**
  font system; `shape_until_scroll(fs, prune)` does;
- `to_attributes(Font)` / `to_color(Color)` convert iced types;
- draw with `renderer.fill_raw(Raw { buffer: Arc::downgrade(&buf), position,
  color, clip_bounds })` — the renderer honours per-glyph colours; draw
  quads *before* it for backgrounds and *after* it for strikes;
- `Attrs::metadata(usize)` survives onto `LayoutGlyph.metadata`, which is
  how overlays find their glyphs.

Known nit: a decoration span includes leading spaces, so a strike on
" walk the duck" starts a few px early; trim at render time.

## Phase 1 result (2026-08-30) — core widget behind the flag

- `src/editor/content.rs`: `Content` is now an enum `{ Iced, Rich }` with
  the app's whole editor API (`perform`, `move_to`, `cursor`, `line`,
  `line_count`, `text`, `selection`). `blocks.rs` and `app.rs` needed only
  a type swap. `RichContent` holds a `cosmic_text::Editor` over
  `BufferRef::Arc`; actions map exactly as iced's graphics editor maps
  them; `update()` (from layout) sets width/metrics and applies per-span
  colour/font from `markdown::scan_line` + `style_for`, so phase 1 looks
  identical to the stock editor. Unit-tested against the same edit
  sequences iced's content produces.
- `src/editor/widget.rs`: `RichEditor` — focus (incl. the `focus(id)`
  operation the app uses), click / double / triple, drag-select, key table
  (copy/cut/paste, motions with Ctrl jump, Tab indent, Esc unfocus; Ctrl+
  letter left to the app), blinking caret, selection quads, placeholder,
  `Length::Shrink` height with `min_height`. No internal scrolling: the
  note's scrollable does that. IME preedit is phase 4.
- Flag: `rich_editor` (config), toggle in Appearance → Font ("Rich editor
  (experimental)"), script step `rich:on|off`. Flipping rebuilds the open
  note's blocks keeping caret and focus.
- Verified with the xshot harness: a note typed through the widget renders
  and saves byte-identically; 38 tests pass.
- Not yet: rich attributes/overlays (phase 2), IME, scroll-into-view of the
  caret inside the outer scrollable (the stock editor did not do this
  either across blocks).

## Phase 2 result (2026-08-30) — rich rendering

- `src/editor/style.rs` maps span kinds to attributes with two looks per
  line: the caret's line (while focused) shows dimmed markers exactly as
  the stock editor did; every other line hides them. Hiding is either
  *collapsed* (transparent, 0.5 px: `**`, `# `, backticks, `~~`, `[[`,
  link URLs) or *transparent at full width* (`- `, `[x] `, `> `) so the
  overlay pass can draw in that exact space and hit-testing / caret columns
  are untouched.
- Headings get real per-line metrics (×1.6 / ×1.35 / ×1.15) via the line's
  base attrs, so the marker and any bold run share the size.
- `RichContent::overlays()` walks the layout runs and returns what to
  paint: code-span backgrounds, code-block rows, task boxes (with the mark
  between the brackets; `x` is shown as ✓), bullets, quote bars, and
  strikes from cosmic-text's own decoration spans (leading blanks trimmed).
  The widget paints backgrounds → selection → glyphs → strikes → boxes and
  marks (`fill_text`) → bullets → caret.
- Per-line styling cache (hash of text + fence state + active flag) so a
  caret move re-styles two lines, not the note.
- Scanner tweaks: a task box span now includes its trailing space; the
  `> ` of a quote is `Kind::QuoteMarker`.
- Not in phase 2: inline rules (still `Block::Rule` widgets, which already
  draw a line), click-to-toggle on the *drawn* box is inherited from the
  caret-column logic (works because the glyphs keep their width), Ctrl+click
  on links (phase 3), IME (phase 4).

## Phase 3 result (2026-08-30) — interactions

- `RichContent::overlays().hotspots`: one per contiguous run of link or
  tag glyphs (target text taken from the line's bytes; a wiki link's alias
  is dropped) and one per drawn task box (rect padded by 3 px).
- Widget: `ModifiersChanged` is tracked; Ctrl+click on a link/tag publishes
  `on_link(Link::Note | Link::Tag)` instead of moving the caret; the
  pointer becomes a hand over a box always and over links/tags while Ctrl
  is held, and the hovered link is underlined in accent2.
- App: `FollowLink` opens the note by title (`Store::find_by_title`) or
  selects the tag view. Script step `follow:<title>`.
- `toggle_task_at_cursor` now accepts a click anywhere from the list marker
  to the closing bracket, since the rich editor draws the box over the
  marker's space. Both editors benefit.

## Follow-up (2026-08-30) — reveal on double-click, not on click

The user found "markers appear wherever I click" distracting. Now a
single click keeps the line rendered and just places the caret; the raw
markdown shows only for a *revealed* line — revealed by a double-click or
by starting to edit it (so typed markers stay visible) — and it re-renders
the moment the caret leaves that line. Triple-click still selects the
line; double-click no longer selects a word.

## Phase 4 result (2026-08-30) — the only editor

- IME: `ModifiersChanged` and `InputMethod` events handled; the widget
  reports caret rectangle + preedit through `request_input_method` on
  every redraw while focused, commits arrive as `Edit::Paste`.
- The stock path is gone: `Content` is a plain alias of `RichContent`;
  iced's `text_editor` widget, the `Highlighter` impl, `retro::editor_style`,
  the `rich_editor` flag/toggle/script step, and the phase-0 spike were
  deleted. `markdown.rs` keeps the scanner and `style_for`.
- 39 tests pass. Merged to `main` and tagged `v0.2`.
- Later ideas, not started: inline rules instead of `Block::Rule`
  widgets; nested-list indentation guides; a "show all markers" mode;
  performance pass for very long notes (styling is per visible layout,
  overlays are computed for every run each draw).
