# The C door's relate reads records with its own JSON reader

Status: open. Filed 2026-09-30 by ticket 0354. Owner: queue owner.
Kind: debt
Debt: 029
Severity: low
Pay when: a user sends a relate record nested past 127 levels, or the door's relate reader changes.

`libraries/c/src/door.rs:200` parses each `relate` record with `serde_json::from_str` instead of the crate's shared record reader. A record nested 128 levels deep is refused there as `a relate record is not JSON`, where every other record path says `the JSON nests more than 127 levels of arrays and objects, the most this tool reads`. The door reads only `name` (or `text`) and `kind`, so such a record is rare. Keeping it risks one less exact refusal sentence. The fix routes `entity()` through a public record reader and adds an edge row at 128 levels.
