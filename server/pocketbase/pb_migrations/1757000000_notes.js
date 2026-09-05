/// <reference path="../pb_data/types.d.ts" />
//
// The JotJotBoom sync collection. The server is a dumb blob store: it knows
// a note's id, revision, timestamp and device, and carries the note itself
// as an opaque `blob` it never reads. Title, body, tags and trash state all
// live inside the blob, which is what lets end-to-end encryption wrap the
// same payload later without touching this schema.
migrate(
  (app) => {
    const users = app.findCollectionByNameOrId("users");
    const own = '@request.auth.id != "" && owner = @request.auth.id';
    const collection = new Collection({
      name: "notes",
      type: "base",
      listRule: own,
      viewRule: own,
      createRule: '@request.auth.id != "" && @request.body.owner = @request.auth.id',
      updateRule: own + ' && (@request.body.owner:isset = false || @request.body.owner = @request.auth.id)',
      deleteRule: own,
      fields: [
        {
          name: "owner",
          type: "relation",
          required: true,
          collectionId: users.id,
          maxSelect: 1,
          cascadeDelete: true,
        },
        // The note's own id (the `id:` line of its frontmatter).
        { name: "note", type: "text", required: true, min: 1, max: 64 },
        // Bumped by the server on every write; clients send `base_revision`
        // with an update and are refused (409) when it no longer matches.
        { name: "revision", type: "number", onlyInt: true },
        { name: "device", type: "text", max: 128 },
        // The client's own modified timestamp (RFC 3339).
        { name: "modified", type: "text", max: 64 },
        // The opaque payload. Plain JSON today, ciphertext later.
        { name: "blob", type: "text", max: 20000000 },
        { name: "created", type: "autodate", onCreate: true },
        { name: "updated", type: "autodate", onCreate: true, onUpdate: true },
      ],
      indexes: [
        "CREATE UNIQUE INDEX idx_notes_owner_note ON notes (owner, note)",
        "CREATE INDEX idx_notes_owner_updated ON notes (owner, updated)",
      ],
    });
    app.save(collection);
  },
  (app) => {
    const collection = app.findCollectionByNameOrId("notes");
    app.delete(collection);
  }
);
