# Build record: ticket 0130, the Polars feature in thinkthen

Date: 2026-09-25. Ticket: `sdlc/tickets/0130-polars-feature-in-thinkthen.md`. Host: Beelink, under the shared heavy lock.

## What moved

- The door moved from `libraries/polars/src/` to `crates/thinkthen/src/public/frame.rs` and `frame/column.rs`, behind the optional `polars` feature. It is off by default.
- Its tests moved to `crates/thinkthen/tests/polars/`. Four `[[test]]` tables carry `required-features = ["polars"]`.
- `libraries/polars/` keeps only `check.sh` and `README.md`. Its line in `sdlc/surfaces.txt` reads `feature`.
- `thinkthen::Error` replaces the door's own error type.

## Sizes

- Production: 349 nonblank lines (frame.rs 202, column.rs 147). The budget was 355. The door was 399. The first draft came to 371. `single` then dropped its method-name argument and builds the name from the question kind.
- Tests: 737 lines.
- Ratchet: 61972 on main after the second merge, 63063 on this branch, a rise of 1091. The budget was 1,125. The removed door and its crate held 1,152 lines, so the repository's Rust fell by 61. The number was measured after the merge, not merged by hand.
- Lock: 90 packages added, no existing version changed. Their `.crate` files come to 8.09 MB.

## Lane times

The base came from a `git archive` copy of c8ca9a65 in a private scratch folder.

| Lane | Cold | Warm |
| --- | --- | --- |
| Base `libraries/polars/check.sh` | 74 s | 11 s |
| New `libraries/polars/check.sh` | 112 s | 12 s |

The cold rise is 38 s against a limit of 60 s. The warm rise is 1 s against a limit of 30 s. Earlier lint and test baselines are not reported. Another agent overwrote a shared helper script mid-run, and those runs measured the wrong worktree. Every later script and log lived in this ticket's own scratch folder.

## Plants

Each plant was restored and its file touched afterward. Every one went RED for the stated reason.

| Row | Plant | Result |
| --- | --- | --- |
| 1 | Drop the null check | RED, exit 101 |
| 1 | Drop the kind check | RED, exit 101 |
| 2 | Cast every caller column to `String` | RED, exit 101 at `door.rs:242`. The first try did not compile and was redone with `columns()` |
| 3 | A failed cell becomes null | RED, exit 101 |
| 3 | A failed row becomes null | RED, exit 101 |
| 3 | Read the first chunk only | RED, exit 101 |
| 4 | Unsure reads as false | RED, exit 101 |
| 5 | Drop the call options | RED, exit 101 (deadline test) |
| 6 | Decide row by row | RED, exit 101 (throttle test) |
| 7 | Empty `CARGO_HOME` | Exit 77, "not run" |
| 7 | A failing test | RED, exit 101 |
| 7 | The sentinel key | RED, exit 101 |
| 8 | Misformat `column.rs`, then `door.rs` | `cargo fmt --check` refused each |
| 9 | Re-export `kind_word` | RED, exit 1: the inventory names it |
| 10 | `default` holds `polars`; `polars` not optional; a `polars-core` dev-dependency | Each refused by `policy.py` |
| 11 | `--all-features` in the test rung | Refused by `policy.py` |
| 12 | Deny without the `xxhash-rust` exception | Exit 4. `policy.py` without `slotmap` refused |
| 13 | A stray `lib.rs` in the feature folder | Refused by `surfaces --registry` |
| 14 | An ignored Polars test | Refused by `policy.py` |

The public inventory with the feature off matched main (exit 0).

## Rungs

After merging origin/main at b6a8f491:

| Rung | Exit | Time |
| --- | --- | --- |
| install | 0 | 1 s |
| lint | 0 | 138 s |
| test | 0 | 112 s |
| spec | 0 | 21 s |
| surfaces | 1 | 671 s |

Every surface passed except `databases/duckdb`. Its `tools/source_checks.py` check R5-25 needs its `deny.toml` to equal the root text with one exception added. The root file's exception list now holds the four Polars entries. DuckDB's copy now takes the root file whole and adds `zlib-rs` as the last list item. The R5-25 check and `check.sh`'s removal plant changed to match. The R5-25 check lost one line, so `databases/duckdb/ratchet.py.json` falls from 1821 to 1820. A run of `databases/duckdb/check.sh` on its own then passed its source checks and deny step. It next failed at "the stock CLI loads the extension" with "the backend refused the connection", because that run gave it the closed port 9 in place of the surfaces rung's loopback backend. The final surfaces rung below is the proof.

The rungs above ran inside this ticket's own `flock` wrapper, before the coordinator's note against that arrived. The rung scripts take the heavy lock themselves. The final run below called them directly.

## Warnings

The four Polars exceptions go unused in lanes whose trees never meet Polars: the Ruby and TypeScript surfaces, the `surfaces --registry` loop, and the other bindings' copies. cargo-deny reports `license-exception-not-encountered` there as a warning, and no lane fails on it. Stop rule 2 was not crossed.

## Final run

A second merge of origin/main (3a86d814) came first, then the ratchet was measured again. The rung scripts were called directly, without a wrapper, at a9f17c1f:

| Rung | Exit | Time |
| --- | --- | --- |
| install | 0 | 417 s |
| lint | 0 | 119 s |
| test | 101, then 0 on one rerun | 734 s, then 282 s |
| spec | 0 | 116 s |
| surfaces | 0, every surface passed, DuckDB and Polars included | 597 s |

The one test failure came from main's code, and this ticket did not touch it. `crates/thinkthen/tests/backend/profile.rs` has two tests that call `structured_tag()`. Each rewrites the same file, `profiles/structured-tag`, and they run in parallel. One test read the file while the other was rewriting it, so it saw an empty file: "the question file is not valid JSON: the JSON at line 1 column 0 is not one". The rerun passed. The fix belongs to the owner of that test: give each test its own file name.

