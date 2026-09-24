# audit and diff move into 0.1

Status: Open. Ian's ruling, 2026-09-24.

Ian moves `thinkthen audit` and `thinkthen diff` from after 0.1 into 0.1. This overturns the timing in `2026-09-23-a-measurement-tier-for-thinkthen-audit-and-diff.md`. Everything else in that issue stands: both commands, their names, subcommands beside `status` and `cache`, and the Beatles Bench prototype as the definition.

## Priority

The ten functions come first. audit and diff land after the ten are done and before 0.1 ships. audit comes before diff if only one fits.

## What the prototype fixes

- `botassembly/beatles-bench`, `scripts/tools/measure.py` (561 lines of Python) and `tests/test_measure.py` (272 lines).
- 14 golden files under `tests/fixtures/audit/golden/`, listed with their command lines in `scripts/tools/README.md`. The Rust output must match their parsed JSON.
- Neither command sends a request or reads a key. Both read ThinkThen's own JSONL output and recordings, plus an answer key for audit.
- audit: agreement with a Wilson interval, both disagreement directions, AUC, calibration, a coverage curve, and a suggested cut tuned on one half and checked on the other.
- diff: what flipped between two runs or two cuts on one run, which way, and exact McNemar on right answers.

## Why now

- The launch story says "hand Jev the facts, and it decides". Buyers then ask how they know it decides right on their data. audit answers that in one command.
- botassembly's bench and optimizer plan to use them. audit checks a ThinkThen judge against human labels before an eval trusts it. diff checks whether an edit made a case better or worse. botassembly adds only a small adapter to write one row per case.

## Asked of the ThinkThen team

Add audit and diff to the 0.1 plan behind the ten functions, with a ticket each. The marketing deck says "after 0.1" until the tickets exist, then changes to 0.1.

Tickets: 0113 `thinkthen audit` on `ticket/0113-audit-command` and 0114 `thinkthen diff` on `ticket/0114-diff-command`, both design drafts pending review. `sdlc/planning/one-line-plan-2026-09-24.md` queues them after the ten functions and before the release build.
