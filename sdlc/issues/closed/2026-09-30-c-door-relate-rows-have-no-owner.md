# The C door's relate rows still join JSON text, and no ticket owns them

Status: Closed by ticket 0346, landed as `Land 0346: the C door joins no JSON text, and each door test process keeps its own folder`. Resolution: the door's relate, annotate and record-array values serialize each row the crate wrote as a raw JSON value, and `libraries/c/tests/door/golden.rs` pins every byte, now with a both-ways edge's `"either":true`.

Kind: debt

Pay when: 0314 slice 4's C bridge family builds typed `annotate` and record-array rows, or before the 0.1 release candidate.

Debt: 012

Severity: medium

Paid: 2026-09-30

Keeping it risks the C door's relate output drifting from the typed result schema the other rows follow.

## The problem

Ticket 0314 slice 2 left `annotate`, record arrays and `relate` joining the JSON text the public types give in the C door, and deferred typed rows "for 0304's single batching path" (`sdlc/tickets/0314-rust-result-schema.md:50`). Slice 4 of ticket 0304 moved `relate` onto that path. The prep note (`sdlc/planning/after-slice-3-prep.md`, section 1, item 7) recommended slice 4 take the relate rows and 0314 slice 4 take `annotate` and record arrays. Slice 4 kept its scope to the question cache and did not take them.

## What should happen

The ticket that builds 0314 slice 4's C bridge family takes the relate rows beside `annotate` and record arrays, and pins the door bytes in `libraries/c/tests/door/golden.rs`.
