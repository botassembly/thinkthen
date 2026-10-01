# The C door's relate reads records with its own JSON reader

Status: closed by ticket 0359. Filed 2026-09-30 by ticket 0354. Owner: queue owner.
Kind: debt
Debt: 031
Severity: low
Paid: 2026-09-30
Pay when: a user sends a relate record nested past 127 levels, or the door's relate reader changes.

`libraries/c/src/door.rs:200` parses each `relate` record with `serde_json::from_str` instead of the crate's shared record reader. A record nested 128 levels deep is refused there as `a relate record is not JSON`, where every other record path says `the JSON nests more than 127 levels of arrays and objects, the most this tool reads`. The door reads only `name` (or `text`) and `kind`, so such a record is rare. Keeping it risks one less exact refusal sentence. The fix routes `entity()` through a public record reader and adds an edge row at 128 levels.

Resolution: ticket 0359. The door reads each relate record through `Entity::from_record`, a hidden crate reader over the one record parser. A record 128 levels deep gets the depth sentence, and a repeated member name gets the duplicate sentence. The door keeps its two relate sentences. An edge table through `thinkthen_relate` pins each row.
