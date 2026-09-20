# JotJotBoom sync server

Sync runs against a self-hosted [PocketBase](https://pocketbase.io) — one Go
binary, its own SQLite database, no other services. The app never lets the
server read a note: each note travels as an opaque `blob` keyed by the
note's id, and the only things the server can see are the id, a revision
counter, a timestamp and which device wrote it.

## Set up

1. Download the PocketBase release for your server from
   <https://github.com/pocketbase/pocketbase/releases> (0.23 or newer) and
   unpack it into a directory.
2. Copy `pb_migrations/` and `pb_hooks/` from this folder next to the
   binary. The migrations create the `notes` and `files` collections on
   start; the hook keeps the revision counters honest. (Upgrading an
   existing server: copy both folders again and restart.)
3. Create the admin login and start the server:

   ```sh
   ./pocketbase superuser upsert you@example.com 'a long passphrase'
   ./pocketbase serve --http 0.0.0.0:8090
   ```

   Put it behind HTTPS (Caddy, nginx, a Tailscale node, …) before pointing
   another machine at it. `./pocketbase serve --https` can also terminate
   TLS itself with Let's Encrypt when the box has a public name.
4. In JotJotBoom, open Options → Sync, enter the server address
   (`https://notes.example.com`), an email and a password, and press
   **Create account**. Every other device then signs in with the same
   details. Accounts are ordinary PocketBase `users`; close signup in the
   admin UI (Collections → users → API rules → Create) once everyone has
   one.

## What syncs

- Notes, including pinned state and trash: the whole file goes in the blob.
- Pictures and attached files (`assets/`), up to 256 MB each, and
  `.folders` (tags created without a note). The server stores them under
  a hash of their path, never their name; downloads need a signed-in
  token.
- Not the link-preview cache (`assets/.links/`) — every device rebuilds
  its own. Deleting a file from `assets/` by hand does not remove it
  elsewhere.

## Conflicts

Last write wins per note, and nothing is ever silently thrown away: if two
devices changed the same note between syncs, the server's copy keeps the
note's id and the local text is kept beside it as a new note titled
"… (conflict, *hostname*)".

Files follow the same spirit: if two devices hold different files under one
name, the local one is renamed (`photo-2.jpg`) and the notes that show it
are pointed at the new name, so every note keeps its own picture.
