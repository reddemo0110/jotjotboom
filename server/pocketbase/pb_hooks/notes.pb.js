/// <reference path="../pb_data/types.d.ts" />
//
// Revision bookkeeping for the `notes` collection: the server owns the
// revision counter so two devices can never both believe they wrote last.

onRecordCreateRequest((e) => {
  e.record.set("revision", 1);
  e.next();
}, "notes");

onRecordUpdateRequest((e) => {
  const current = e.record.original().getInt("revision");
  const base = e.requestInfo().body.base_revision;
  if (typeof base === "number" && base !== current) {
    throw new ApiError(409, "revision conflict", { revision: current });
  }
  e.record.set("revision", current + 1);
  e.next();
}, "notes");
