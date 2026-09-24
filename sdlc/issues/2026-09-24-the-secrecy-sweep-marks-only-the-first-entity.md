# The secrecy sweep marks only the first entity

Status: Open

The shared secrecy sweep in `crates/thinkthen/tests/backend/secrecy.rs` (lines 61 to 74 at ticket 0088's landing) puts its secret marker in the first entity only. A leak of any later entity's name or kind passes every secrecy test.

## Reproduction

Found by the final review of ticket 0088 (`sdlc/records/0088-review-final.md`, finding F2). Plant an `eprintln!` of the last relate entity's name (`Acme`) in the relate command and run:

    $ cargo test --locked -p thinkthen --test backend secrecy

All five secrecy tests pass. A planted print of the first entity's name fails four of them.

## Smallest fix

Put a second marker in the other entity and require both markers to stay out of every output, diagnostic, and stored file.
