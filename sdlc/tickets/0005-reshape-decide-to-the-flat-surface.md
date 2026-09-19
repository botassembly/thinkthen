---
flow: build
priority: 50
opens: crates spec specification/fixtures sdlc/ratchet.json demos/01-refund-gate demos/09-what-leaves-the-machine
---

# 0005: Reshape `decide` to the flat surface

Status: landed

## Outcome

`thinkthen decide QUESTION` works as ADR 0007 and the pages `specification/channels.md`, `threshold.md`, `result.md`, and `decide.md` describe. The earlier grammar is gone from the binary, the executable pages, and the fixtures' names. Nothing about the wire format, the HTTP edge, recording, or replay changes.

## Current Facts

Tickets 0001 to 0004 landed `thinkthen decide if CONDITION` with a full result object on every run, an optional symmetric `--min-prob`, an exit code that carries the answer only under `--status`, `--plan`, `--backend`, and five `THINKTHEN_*` backend variables. The adapter already sends the question text unchanged, so the request bytes and every recording digest stay the same. `sdlc/issues/2026-09-19-review-leftovers-from-the-core-tickets.md` lists thirteen small leftovers, and this ticket rewrites much of the code they touch.

## Scope

- The command is `thinkthen decide QUESTION`. Options may sit before or after the question, and `--` ends option parsing.
- A threshold type in the core replaces the pass mark. It parses `T` and `LOW:HIGH`, applies the inclusive rule, and defaults to 0.5. A percent, a reversed band, a value out of range, and a number that is not finite are usage errors before any request.
- Standard output holds `true`, `false`, or `null`. `--details` prints the result object of `result.md`, with `value`, `question.text`, `threshold`, and `meta.profile`. The view never changes the request.
- The exit code is 0 for yes, 1 for no, and 3 for unresolved on every run. `--status` is gone. `--quiet` suppresses standard output and changes nothing else. The `unassessed` outcome is gone.
- `--plan` becomes `--dry-run`, and the plan document's `backend` field becomes `profile`.
- `--backend` becomes `--profile`. The five `THINKTHEN_*` backend variables are removed, with the empty-variable rule that served them. The ad-hoc rules of ADR 0004 and ADR 0006 stay.
- The short help shows the everyday options. The long help adds the advanced ones.
- `spec/decide-if.md` becomes `spec/decide.md`. The fixtures named `if-urgent.*` become `decide-urgent.*`, with their README.
- Each leftover in the review issue is fixed, or the ticket's record says why it no longer applies.

Excluded: the configuration file, `THINKTHEN_PROFILE`, records, and every other command. Build Settled sections only. Anything a page marks Draft waits, such as `meta.tool` from ADR 0008.

## Acceptance

- A table test pins the rule at p equal to 0, 0.1, 0.5, 0.9, and 1 under no threshold, `0.9`, and `0.1:0.9`. A property test holds the rule for any p and any valid threshold.
- Integration tests cover: the three exit codes with their bare values, `--quiet` printing nothing with the same exit code, `--details` producing the same request bytes as the bare run, `--dry-run` printing six fields with `profile` and opening no connection, and each removed option (`--status`, `--min-prob`, `--plan`, `--backend`) exiting 2.
- A test proves that the removed environment variables no longer change the plan.
- The recording digest of the renamed fixture equals the pinned value from ticket 0004.
- The key and the evidence still never appear in any error or any Debug output.
- The ratchet equals the measured total, and the commit that moves it says what grew or shrank and why.
- Demos 01 and 09 parse against the new binary. Demo 01 stays red until it is recorded live.
- The whole ladder is green, and a second agent reviews the public surface change.
