# Small leftovers from early reviews

Status: Closed on 2026-09-25 by commit c67f97e88. That commit merged this file into `2026-09-25-test-harness-and-review-leftovers.md` and deleted it. This copy restores it so the four files merged here keep their target. Earlier status: Open. Low priority. Anyone closing a nearby ticket takes an item.

This merges four leftover files from the reviews of tickets 0001, 0002, 0003, 0004, 0006, 0010, and 0019, filed 2026-09-19. Each source file is closed and points here. None blocks anything. Items 14 to 18 were never confirmed by the sweep of 2026-09-21, so the taker checks the claim before fixing it.

## From the core tickets

1. Three decode errors carry a question's wire name as a `String`. A place number rendered as the wire name would carry the same text and need no parsing.
2. Several public items in the core have no caller outside the crate. Whatever still has none becomes crate-private.
3. `Reply` is a named triple. A tuple would save about thirty lines and lose the names.
4. `encode` copies the evidence and each condition into owned strings. A borrowing wire struct would avoid the copy and tie the test structs to a lifetime.
5. `EncodeError` cannot fire today. It stays because the alternative is an `unwrap` with a suppression.
6. `Probability` and `PassMark` repeat about thirty lines of newtype shape. A macro would read worse.
7. Standard input has no size cap, and the specification sets none.
8. An oversized or cut-short response body is retried like a transport failure.
9. A defect and a render failure share exit code 70.
10. The demo runner splits a harvested `--replay` folder name on white space, so a folder with a space in its name is checked wrong.
11. `EntryError::Unwritable` cannot fire, and `EntryError::Schema` echoes the schema string of a hand-edited file.
12. `record_and_replay.rs` repeats one record call five times, and the wrong-assertion fixture copies a whole demo page.
13. `backend.rs` is 411 lines and its first doc line joins two jobs.

## From ticket 0006

14. `sdlc/scripts/live` reads `THINKTHEN_LIVE_LEDGER`, which lets any caller point the spend limit at a throwaway file. The variable exists only for the script's own test. Accept it under the test alone, or reach the ledger another way.
15. `sdlc/scripts/demos` captures an empty folder name when a page writes `--replay` with the folder in backticks, and it then skips the folder guard in silence. It should fail loudly on an empty name.
16. `crates/thinkthen/tests/harness/mod.rs` opens with `#![allow(dead_code)]`, and `rust-standards.md` says no test file pastes a suppression at its top.
17. `sdlc/records/0006-...` says the ratchet is 4531 and fourteen demos are red. The ratchet was 4520 and thirteen were red.
18. `--url ""` reports the ad-hoc message where a blank-address message would be clearer.

## From ticket 0010

19. `demos/27-test-with-no-network/README.md` copies `triage.sh` into a block that asserts nothing, so the listing rots in silence when the script changes. A listing of a committed file is printed by a block the gate runs.
20. A block that starts with `set +e` lets an earlier failure inside it pass unnoticed, because only the final assertion is checked. The demos script could refuse a `set +e` block that never prints or pins an exit code.

## From the security ticket

21. `--field` and `--options` echo a pointer as it was typed, with no control-character check. A label already gets that check from ticket 0013, and the same check is the fix.
22. A transport failure that can never succeed is still retried. A header the HTTP library refuses is attempted three times. Cosmetic.
