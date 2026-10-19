# 0699: Align installed error fixtures with approved provider redaction

Status: OPEN.

Milestone: 0.2.1

Phase: 1

Depends on: 0624

## Outcome

Installed backend-error checks expect the public allow-listed diagnostic established by 0624 and prove that unrelated provider text remains withheld. No product behavior, public names, types or diagnostic policy changes.

Search the existing library and database checks for the same stale raw-provider-message expectation and repair the whole class. Keep every error-kind, retryability, facts, position, send-count and replay assertion. Use existing loopback fixtures and installed-package checks; add no scanner or checker.

## Changes

Existing backend-error fixture assertions under `databases/*/tests/` and `libraries/*/` that still expect unrelated provider text in the public message.

## Proof

The existing SQLite installed-artifact routine check passes against its exact-source package and loopback backend, including both image failure cases. Every additional changed binding or database runs its affected existing installed check through pm job on the declared build machine. Report unavailable platform proof explicitly. Fresh review checks the preserved assertions and the complete searched class.

## Evidence

The 0627 installed SQLite run on b7563ca93 reached two unchanged assertions in databases/sqlite/tests/test_images.py, at lines 90 and 197. Both expected a public message ending in provider message: private-backend-body; the core correctly returned the status-503 message without that unrelated provider text. The earlier source-mode run separately failed on macOS wc padding and is not repaired by this ticket.
