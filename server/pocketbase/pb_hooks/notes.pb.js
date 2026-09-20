/// <reference path="../pb_data/types.d.ts" />
//
// Revision bookkeeping for the `notes` and `files` collections: the server
// owns the revision counter so two devices can never both believe they
// wrote last.

onRecordCreateRequest((e) => {
  e.record.set("revision", 1);
  e.next();
}, "notes", "files");

onRecordUpdateRequest((e) => {
  const current = e.record.original().getInt("revision");
  // A number from JSON, a string from a multipart upload.
  const raw = e.requestInfo().body.base_revision;
  const base = raw === undefined || raw === null || raw === "" ? NaN : Number(raw);
  if (!isNaN(base) && base !== current) {
    throw new ApiError(409, "revision conflict", { revision: current });
  }
  e.record.set("revision", current + 1);
  e.next();
}, "notes", "files");
