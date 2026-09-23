---
flow: build
priority: 38
opens: crates/thinkthen specification demos sdlc/planning sdlc/ratchet.json
---

# 0057: Correct help and the CLI contract

Status: landed

## Outcome

Help states the defaults and exit behavior a script needs before its first run. The specification agrees with the binary on partial question failures, empty inputs, ties, and detailed examples. A timeout of zero is a usage error before a key, input, or request.

## Current facts and decisions

Ticket 0054 made exit 6 the partial-question-failure outcome, but two specification sentences still say that code is unused or that every backend failure ends the run. A record-mode command exits 0 when it completes without a partial or whole-run failure, whatever the individual answers say; annotate exits 6 when it completes with failed-question markers. Their help still presents single-document answer exit codes without that boundary. Help also hides the 0.5 threshold, width 4, the backend address option, and `find`'s empty exit-3 result. `--timeout 0` currently reaches the engine and fails as a backend timeout.

The binary already has coherent behavior for four specification contradictions: an empty document is a usage error while an empty line or JSONL stream succeeds; CSV and TSV require a header; an exact `choose` tie is unresolved; detailed result and rank rows use the complete result shape. This ticket decides that those behaviors satisfy the existing settled rules and corrects the stray sentences. Ian can overturn these decisions.

## Scope

Write ADR 0031 to make `--url` the one advanced option shown in short help, because it decides whether a first run reaches the default hosted service. Correct the command help, the affected specification sentences, and one existing how-to that teaches a shell gate. Refuse a zero timeout during argument validation. Keep the command names, option names, request bytes, successful output, record framing, answer exit codes, and partial-failure implementation unchanged.

Excluded: diagnostic rewrites beyond the zero-timeout sentence, backend profiles, result-shape changes, cache behavior, scheduling, and paid calls.

## Acceptance

- Help names the default threshold of 0.5. Long help names the default width of 4, preserving the settled rule that `--jobs` is advanced. The backend address option appears on the short screen. `find` help says that `none` prints nothing and exits 3.
- ADR 0031 and the channels specification state the narrow short-help exception for `--url`; every other advanced option remains long-help only.
- Every record-capable verb says that a record run exits 0 when it completes without a partial or whole-run failure, and that printed values carry the individual answers. Annotate also names exit 6 for a completed run with failed questions. The existing gate how-to shows a one-document exit-code gate and a record gate that tests filtered output or a count.
- `--timeout 0` exits 2 with one fixed tool-owned sentence before reading a key, opening an input path, or opening a connection. Compiled cases cover an ordinary command and `find`, each with an unreadable `--input` path, plus counted listeners and a positive-timeout regression.
- The exit-code tables and annotate pages agree that mixed good and failed questions print good answers and failed markers, then exit 6; a reply with no usable answer exits 4.
- The specification says that empty line and JSONL streams succeed, empty CSV and TSV inputs fail for a missing header, an empty document is a usage error, and an exact choose tie is unresolved.
- Detailed examples are complete or explicitly marked as excerpts, and ranked detailed rows keep `value: null`.
- Compiled tests pin the changed help and zero-timeout behavior. A counted loopback listener proves the zero-timeout case sends no request. The full repository ladder and `git diff --check` pass without a key or outside network access.

## Dependencies

Tickets 0054 and 0056, ADR 0007's existing short-help rule, the accepted build queue, and the four issue pages cited by that queue for exit 6, record-mode exit 0, hidden defaults, and specification contradictions.

## Complexity

- Contract: 2
- State and timing: 0
- Reach: 2
- Proof: 1
- Cost of error: 1
- Total: 6
- Minimum level floor: none
- Final level: 3
- Reasons: the visible help and contract span every record-capable command, timeout validation has two argument homes and must precede input access, and exit 0 must stay distinct from annotate's exit 6. Exact compiled help, unreadable-input, and counted-listener tests directly prove the risky behavior.
- Selected model: `gpt-5.6-sol` with medium reasoning.

Re-score if the work changes an exit code other than zero timeout, alters successful JSON, changes request bytes, or adds a dependency.

## Review

Independent design review rejected the first draft because it called every completed record run exit 0 despite annotate's partial-failure exit 6, moved the advanced `--jobs` option onto short help without an ADR, routed a level-3 ticket to Luna, attributed ticket decisions to Ian, and did not prove timeout validation preceded input access in both argument homes. The first repair qualified record completion, kept `--jobs` in long help, recorded the four specification choices as ticket decisions, corrected the routing, and required ordinary-command and `find` tests with unreadable inputs and counted listeners. Re-review found that `--url` also needs an ADR exception to ADR 0007 and that two introductory sentences retained the old ambiguity. The second repair includes ADR 0031 and uses the qualified exit wording throughout. The same reviewer accepted the final design and confirmed the level-3 Sol Medium route.

Code review found that the record-mode warning appeared only in long help and that two how-to sentences omitted the partial and whole-run failure qualification. The repair puts the exact warning on the first `decide -h` screen, pins that surface in the compiled test, keeps every record-capable verb's long-help coverage, and qualifies both how-to sentences. The focused help test, compiled demo runner, lint rung, specification rung, formatting check, ratchet check, and `git diff --check` pass. The same reviewer accepted the repair with no remaining findings.

## Implementation note

Implemented in the ticket worktree. Argument validation now rejects `--timeout 0` before environment, input-file, or connection access in both shared argument homes. Compiled tests cover both homes with an unreadable input path, a counted loopback listener, the exact exit-2 diagnostic, and a positive-timeout request. Help tests pin the threshold and jobs defaults, the `--url` short-help exception, record-run exit behavior, annotate exit 6, and `find --none` exit 3. ADR 0031, the affected specification pages, and how-to 01 carry the accepted contract.

The source ratchet rises from 26,009 to 26,179 non-blank Rust lines. The growth is the compiled boundary coverage, including the short-help placement assertion, the small parser-edge validation, and the module boundary that keeps each source file below 500 lines. Existing help and backend harnesses supply the shared machinery; no duplicate production path or dependency was added. Independent review is required by the repository rule before this ratchet increase lands.

Red evidence: the zero-timeout test reached the unreadable input and exited 5, short help omitted `--url`, and help omitted the accepted record-run sentence. Green evidence: the focused timeout and help tests pass, followed by the install, lint, test, and specification rungs. The specification rung runs 26 command examples, 7 transform examples, replay checks, and all 19 green how-tos. `git diff --check` also passes.
