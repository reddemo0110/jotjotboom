# Project Handover — Retro Notes App for COSMIC Desktop

Working name: TBD (suggestions welcome — something terminal/retro flavoured)

## What we're building

A native Linux note-taking app for the COSMIC desktop that delivers a hybrid-markdown notes experience — inline-styled markdown editing, nested tags, wiki-links/backlinks, instant search, three-pane layout — with a retro (CRT/phosphor/terminal) visual twist, and a future account-based cloud sync that comparable apps lack on Linux.

Target platform: COSMIC desktop (1.0+), packaged as Flatpak for the COSMIC Store, plus AUR. Should also run on other Wayland/X11 Linux DEs since libcosmic is cross-platform.

## Locked-in architecture decisions

### Stack

- **Rust + libcosmic** (iced-based, COSMIC's official toolkit). Start from the official template: `cargo generate gh:pop-os/cosmic-app-template`
- Use libcosmic's built-ins wherever possible: header bar, nav model, cosmic-config, system theme integration (light/dark, accent colours)
- libcosmic API moves faster than its docs — read source of existing apps in the cosmic-utils GitHub org for real-world patterns

### Storage

- **SQLite via `rusqlite`, with FTS5 enabled** — this powers instant-as-you-type search
- Schema: `notes` (id, title, body, created, modified, pinned, trashed), `tags`, `note_tags` join table rebuilt on save by re-parsing `#tags` from the body
- **Open decision**: DB-only vs plain `.md` files on disk with SQLite as a derived index. File-based is friendlier to the Linux crowd (grep, Syncthing) — lean this way unless it complicates the editor. Decide early, it affects sync payloads.

### Editor (the long pole — most of the project's effort lives here)

Hybrid markdown editing: raw markdown is the source of truth, rendered inline with styling while syntax markers stay subtly visible (dimmed).

1. Custom editor widget built on `cosmic-text` (COSMIC's shaping/layout/editing engine)
2. Incremental parsing via `tree-sitter-markdown` — reparses only changed regions, stays fast on large notes
3. Map syntax tree → per-span cosmic-text attributes: bold/italic, heading sizes, dimmed syntax markers, monospace code spans
4. Wiki-links `[[note title]]` and backlinks come from the same parse pass: index link targets in SQLite, query the reverse direction for a backlinks panel

### Layout

The three-pane triptych mapped to libcosmic patterns:

- Nav bar (built-in) = nested tag tree; nested tags are `#work/incab` style, split on `/`
- Custom middle column: note list with previews, pinned notes on top
- Right: editor
- libcosmic's responsive/condensed handling gives narrow-window mode for free

### Retro theming

- **Chrome stays fully native COSMIC; retro treatment is confined to the editor surface and themes.** It should feel like a COSMIC app with personality, not a costume.
- Ship bitmap-style editor fonts (Departure Mono / Berkeley Mono / IBM VGA conversion); UI chrome keeps the COSMIC system font
- Themes: honour system theme by default, plus green-phosphor, amber-on-black, and a WordPerfect-blue focus theme
- **CRT shader** on the editor pane via iced's custom wgpu shader support: scanlines + slight barrel distortion + phosphor glow. Must have an on/off toggle and intensity slider.
- Details: block cursor blinking at ~530ms, optional typing sounds, focus mode dimming all but the current paragraph

## Sync design (build later, design for now)

**Local-first is non-negotiable.** SQLite is the source of truth; the app is fully functional offline; sync is a background reconciliation process. Never gate note access behind login.

### Planned backend

- **PocketBase** self-hosted (single Go binary): email/OAuth auth, REST + realtime API. Supabase is the fallback if hosted-with-no-ops is preferred.
- Client auth: libcosmic login screen, bearer token stored in the system keyring via the `secret-service` **crate** — never plaintext config

### Sync protocol

- **Oplog table** in local SQLite: (note_id, revision, modified_at, device_id, content_hash) appended on every save
- Background tokio task: push unsynced revisions, pull remote changes; debounced on-save plus PocketBase realtime subscription for live cross-device updates
- **Conflict handling: per-note last-write-wins + conflict copies.** If both sides changed a note since last sync, keep both and mark one "(conflict from device)". Never silently clobber. CRDTs (loro/automerge) are a possible future upgrade for the note body — explicitly deferred.

### E2E-encryption readiness — HARD CONSTRAINTS from day one

Encryption itself is deferred, but these design rules are not:

1. **The server is a dumb blob store.** Sync payloads are opaque bytes. Title AND body live inside the payload. Server-visible metadata is the minimum sync needs: note id, revision, timestamp, device id.
2. **Search, rendering, indexing, previews are client-side forever.** Never add a server feature that requires reading note content — that is the trap that makes E2E retrofits impossible.
3. **Metadata boundary (decided):** plaintext = note id, revision, timestamps, device id. Encrypted = title, body, tags. Tags stay inside the encrypted payload despite the temptation of server-side tag features — they leak too much.
4. **Key management is explicitly out of scope for now** (passphrase → argon2-derived key, XChaCha20-Poly1305 via `chacha20poly1305` crate, device key exchange, recovery kits). Design docs only, no code, until sync v1 works.
5. **Use the keyring (`secret-service`) for any secret from the very first commit** so the plumbing exists.

## Build order

1. Template scaffold + SQLite + basic plain-text editing, three-pane layout
2. FTS5 search + tag parsing + tag tree nav
3. Hybrid markdown editor widget (cosmic-text + tree-sitter) — the long pole
4. Wiki-links + backlinks panel
5. Retro theming: fonts, themes, CRT shader, focus mode
6. Polish: export (md/html/pdf), pinning, trash, full keyboard navigation
7. (Later phase) PocketBase sync per the design above
8. (Later still) E2E encryption wrapping the existing blob payloads

## Packaging

- Flatpak (COSMIC Store) — add `<id>com.system76.CosmicApplication</id>` to the metainfo provides section
- AUR package
- Build deps on Pop!_OS/Debian: cargo, cmake, just, libexpat1-dev, libfontconfig-dev, libfreetype-dev, libxkbcommon-dev, pkgconf

## Open questions for implementation

- DB-only vs markdown-files-on-disk + SQLite index (lean: files on disk)
- App name
- Whether note list previews render markdown-stripped text or first-N-chars raw
- Whether attachments/images are in scope for v1 (suggest deferring)

