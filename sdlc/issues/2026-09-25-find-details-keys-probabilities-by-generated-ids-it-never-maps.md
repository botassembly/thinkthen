# find --details keys probabilities by generated ids it never maps

Status: Open

Reported from a user's benchmark on 2026-09-25 and checked against main at `535cb3e7`. Verdict: confirmed. One correction: the report's second suggested fix named a `--id` option, and `find` has none.

## What the user sees

The user fed ten JSONL records to `find ... --jsonl --field /input --details`. The records carried their own ids `u01` to `u10`. The answer keyed the pick and every probability as `u001` to `u010`. Nothing in the output says which record each key means. The user had to rebuild the map from the input order. Their own `u01` and the generated `u001` look almost the same, which invites a wrong match.

## What the code and specification do

- The core builds each id from the unit's position: `format!("u{:03}", place + 1)` (`crates/thinkthen/src/core/find.rs:172-174`).
- `--details` prints the pick and a `probabilities` map in wire order (`core/find.rs:199-228`). The `value` holds the selected record, so the pick can be read back. The other probabilities cannot.
- `specification/find.md:41` says the answer holds "the selected generated unit id" and every probability "in input order". It never states the rule that `uNNN` is the one-based input position.
- Only ADR 0030 line 10 says "Unit ids are `u001` onward". `specification/result.md:83` shows an example with no mapping.
- `thinkthen find --help` says `--details` prints "the full result object" and says nothing about ids.

So the output matches the specification. The specification and help leave the id rule for the reader to guess.

## Why it matters

A user reads `--details` to see how close the runners-up were. To do that, the user must know which record `u007` is. Today that knowledge lives only in an ADR. A user whose records carry similar ids can join the wrong rows and draw a wrong conclusion without any error.

## Fix options

1. State the rule in `specification/find.md` and in the `--details` help line: `uNNN` is the one-based input position, zero-padded to three digits. This changes no output.
2. Also add the position to the answer, for example an `index` beside each probability, or a `units` list of `{id, index}` in input order. This widens the result schema in `specification/result.md`.

Recommendation: option 1 now, as a small documentation fix. Option 2 changes a settled public schema and needs a ticket and a second reviewer. Take it only if a user still misjoins after the rule is written down. Ian can overturn this choice.
