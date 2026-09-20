/// <reference path="../pb_data/types.d.ts" />
//
// Everything in the notes folder that is not a note: pictures and attached
// files under `assets/`, and the `.folders` list. Same bargain as `notes`:
// the server sees a key (a hash of the file's path, never the name), a
// revision, a timestamp and the device. The name and content hash live in
// the opaque `meta`; the bytes are a protected upload called "blob".
migrate(
  (app) => {
    const users = app.findCollectionByNameOrId("users");
    const own = '@request.auth.id != "" && owner = @request.auth.id';
    const collection = new Collection({
      name: "files",
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
        // blake3 of the file's path inside the notes folder.
        { name: "key", type: "text", required: true, min: 1, max: 64 },
        // Bumped by the server on every write, as for notes.
        { name: "revision", type: "number", onlyInt: true },
        { name: "device", type: "text", max: 128 },
        { name: "modified", type: "text", max: 64 },
        // Opaque: path, content hash, size. Plain JSON today, ciphertext later.
        { name: "meta", type: "text", max: 100000 },
        // The bytes. Protected: downloads need a short-lived file token.
        { name: "data", type: "file", maxSelect: 1, maxSize: 268435456, protected: true },
        { name: "created", type: "autodate", onCreate: true },
        { name: "updated", type: "autodate", onCreate: true, onUpdate: true },
      ],
      indexes: [
        "CREATE UNIQUE INDEX idx_files_owner_key ON files (owner, key)",
        "CREATE INDEX idx_files_owner_updated ON files (owner, updated)",
      ],
    });
    app.save(collection);
  },
  (app) => {
    const collection = app.findCollectionByNameOrId("files");
    app.delete(collection);
  }
);
