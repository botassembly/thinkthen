# Run facts ticket B5 owes a cause, a retryable flag and the stop record

Status: Open. Filed 2026-09-26 by the queue owner from local experiment 273, report 11, issue 2, with report 01, issue 6. An amendment to batching ticket B5, run facts, which is in 0.1. It blocks 0.1 only as part of B5.

## What happens

Exit codes mix failures a retry can cure with failures it cannot.

- Exit 4 covers 429, 503 and a timeout. It also covers 401, an unknown model, `max_tokens_exceeded`, a missing key and a malformed reply.
- Exit 5 covers a full disk, and also invalid UTF-8 and a malformed question file.
- Exit 2 covers a flag typo and one bad record.

Only the English sentence on standard error tells them apart. `specification/channels.md` line 14 says a script never parses standard error. A queue worker must retry every exit 4, burning quota on a revoked key, or retry none, losing work to a 429.

After a stop, a `filter` consumer also cannot tell from standard output how far the run got, because dropped records leave no trace.

The library already has `Error::retryable()` (`crates/thinkthen/src/public/error.rs`). The command has nothing like it.

## What B5 says today

ADR 0048 item 10 defines the `thinkthen.run/1` line that `--facts` writes. Its fields are `records`, `requests_sent`, `cache_answers`, `input_tokens`, `output_tokens`, `seconds` and `model`. None carries a cause, a retry signal or a stop position.

## Checked on main

Verified: ADR 0048 item 10 lists the fields as quoted, and `Error::retryable()` exists in the public library. The exit-code mix comes from the report's runs.

## What to amend in B5

Add to the `thinkthen.run/1` line of a stopped run:

1. A stable cause identifier.
2. A `retryable` flag computed by the same rule as the library's.
3. The record number where the run stopped.

A distinct exit code for a retryable failure is the other route. It changes the exit table, so the queue owner records the choice. Report 11, issue 10, asks for a catalog of cause identifiers, and the severity 3 roll-up lists it.

## Done when

B5 lands with these fields, or its ticket records why a different route was chosen.
