# Debug follow-ups from the 0102 review

Status: open. Found 2026-09-24 by the 0102 review (`sdlc/records/0102-review.md`). Owner: Claude.

## User text still printed under `{:?}`

Neither path reaches a log or an error today, and no secrecy test covers either one.

- `core/adapters/systemone/request.rs` `Request { state: Json, .. }` derives `Debug`, and `state` is the evidence. Redact `state` in `Debug` or drop the derive. Ticket 0076 now edits `engine/request.rs`, so this fix waits for 0076 to land.
- `choose --options` builds `Question::Choose` labels from the record (`cli/asking.rs` `Asks::of`). `Question` and `Labels` print those labels under `{:?}`, and `Sending.question` holds one. Question text prints on purpose. Labels read from a record are evidence and should be withheld.

## Small notes

- `Token`'s `Debug` hides `byte_start` and `byte_end` behind `..`. They are positions, not secrets, and showing them would help debugging.
- `Record(<withheld>)` shows neither the byte length nor whether the record is text or JSON. The other redactions show the length.
- A shared `Withheld(usize)` newtype with its own `Debug` would turn each of the six `format_args!("<{} bytes withheld>", len)` sites into one line. The review estimates about 11 lines saved.
