# A JSON record nested 128 levels deep is refused as "not valid JSON"

Status: closed by ticket 0354. Filed 2026-09-30 from the release QA break-it round. Owner: queue owner.
Kind: bug
Blocks 0.1: yes. The refusal gives a false reason and stops the batch.

Found by release QA on main `f0308dd9d`. A `--jsonl` record nested 127 levels deep is accepted. The same record nested 128 levels deep stops the run at exit 2 with `the record is not valid JSON`.

The record is valid JSON. The specification names no depth limit and uses that sentence only for malformed records. The limit is most likely `serde_json`'s default recursion limit of 128.

Fix: keep a bounded depth, since unbounded recursion risks a stack overflow. Name the limit in `specification/records.md`, and refuse a deeper record with its own sentence that names the limit, on every surface that parses records. Add an edge-table row at 127 and 128 levels deep.

Resolution: ticket 0354 slice A. A JSON record nests at most 127 levels of arrays and objects, named in `specification/records.md`. A deeper record is refused with `the JSON nests more than 127 levels of arrays and objects, the most this tool reads` on the command, the Rust library and the C door. Edge rows pin 127 as accepted and 128 as refused.
