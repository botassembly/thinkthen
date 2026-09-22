# Review leftovers from the core tickets

Status: Closed on 2026-09-22. Merged into 2026-09-22-small-leftovers-from-early-reviews.md.

Filed 2026-09-19 by the steering agent from the second reviews of tickets 0001 and 0002. None blocks landing. Each is small and arguable, so no reviewer fixed it.

1. Three decode errors carry the wire name of a question as a `String`. A place number rendered as the wire name would carry the same text and need no parsing.
2. Several public items in the core have no caller outside the crate yet. Tickets 0003 and 0004 decide which of them the binary needs. Whatever still has no outside caller after 0004 becomes crate-private.
3. `Reply` is a named triple. A tuple would save about thirty lines and lose the names.
4. `encode` copies the evidence and each condition into owned strings. A borrowing wire struct would avoid the copy and tie the test structs to a lifetime.
5. `EncodeError` cannot fire today. It stays because the alternative is an `unwrap` with a suppression.
6. `Probability` and `PassMark` repeat about thirty lines of newtype shape. A macro would read worse.

`DecodeError::Malformed` also covers a response with no model name. The steering agent left that merged on purpose. The binary maps every decode error to exit code 4, and the message already differs.

## Added from the reviews of tickets 0003 and 0004

7. Standard input has no size cap. The specification sets none today.
8. An oversized or cut-short response body is retried like a transport failure.
9. A defect and a render failure share exit code 70.
10. The demo runner splits a harvested `--replay` folder name on white space, so a folder with a space in its name would be checked wrong.
11. `EntryError::Unwritable` cannot fire, and `EntryError::Schema` echoes the schema string of a hand-edited file.
12. `record_and_replay.rs` repeats one record call five times, and the wrong-assertion fixture copies a whole demo page.
13. `backend.rs` is 411 lines and its first doc line joins two jobs.
