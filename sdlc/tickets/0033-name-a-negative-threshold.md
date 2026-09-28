---
flow: build
priority: 46
opens: crates/thinkthen/src/args.rs crates/thinkthen/tests/backend/refusals.rs sdlc/ratchet.json
---

# 0033: Name a negative threshold

Status: landed

## Outcome

A negative value beside `--threshold` reaches the tool's threshold validation and receives the same safe `thinkthen:` refusal as the equivalent `--threshold=VALUE` spelling. The diagnostic names `--threshold` and gives no clap tip that turns the value into a question or record argument.

## Current Facts

Finding 6 in `sdlc/issues/closed/2026-09-19-hands-on-test-pass-one.md` records the defect. On current main, `thinkthen decide q --threshold -0.1` exits 2 through clap with `unexpected argument '-0' found` and advises `-- -0`. Following that advice would make the value positional input instead of a threshold.

The same split affects every built command's `--threshold` argument. A space followed by `-0.1` falls out of clap on `decide`, `choose`, `filter`, `rank`, and `score`. The equals spelling `--threshold=-0.1` already reaches the shared parser on all five commands and exits 2 with the exact safe sentence `thinkthen: --threshold: a single cut is above zero and at most one`.

`specification/threshold.md` already settles the accepted ranges and requires a usage error before any request. The core parser already refuses a negative cut. The defect is confined to the CLI value boundary.

## Scope

- Let each CLI argument named `--threshold`, including the hidden refusal arguments on commands that take no threshold, accept a following negative number. Use clap's narrow negative-number setting so a following option name remains an option.
- Preserve the shared threshold parser, its validation order, exit 2, and the exact negative-cut sentence.
- Keep the space and equals spellings behaviorally identical.

Excluded: accepting a negative threshold, accepting arbitrary hyphen-leading threshold text, changing threshold ranges or precedence, changing question-file thresholds, changing any other option's clap behavior, changing valid requests or answers, and rewriting the settled threshold specification.

## Acceptance

- Red first: an exact compiled-binary matrix shows that `--threshold -0.1` on `decide`, `choose`, `filter`, `rank`, and `score` currently produces clap's `unexpected argument` output instead of the tool-owned diagnostic.
- The same five space-separated cases then exit 2, print no standard output, and print exactly `thinkthen: --threshold: a single cut is above zero and at most one\n` on standard error. Matching `--threshold=-0.1` cases pin the same result.
- Each case runs with a marker key against a counted loopback listener through the real non-dry-run path. The listener observes zero connections and zero requests, and neither normal nor debug output contains the key or the input marker.
- A threshold followed by another option keeps clap's existing missing-value behavior rather than consuming that option as threshold text. Existing valid cut and band cases, invalid-threshold cases, question-file cases, command-specific threshold refusals, shared secrecy coverage, and help snapshots remain green.
- The ratchet equals the measured total, and the whole ladder passes with the key and base address unset.

## Dependencies

Ticket 0032 is the landed sequencing predecessor in the plan and is not a technical dependency. ADR 0007 and `specification/threshold.md` already settle the threshold grammar, ranges, status, and before-request boundary. No new architecture decision is needed.

## Complexity

- Contract score: 1
- State and timing score: 0
- Reach score: 1
- Proof score: 1
- Cost of error score: 1
- Total: 4
- Minimum level floor: none
- Final level: 2
- Reasons: one settled public validation rule must behave consistently across five command parsers; the implementation stays at one CLI owner, while the integration matrix proves every public argument home, exact output, no request, and secrecy; a wrong diagnostic misdirects a user but is local and reversible.
- Selected model: `gpt-5.6-luna` with high reasoning

## Review

- Design review: accepted. The reviewer confirmed the five command homes, the narrow clap setting, exact safe diagnostic, keyed zero-request proof, following-option regression, existing ADR coverage, and level 2 Luna High route.
- Code review: accepted with no findings. The reviewer confirmed the narrow setting on all five argument homes, exact space-and-equals matrix, empty output and safe diagnostics, keyed zero-connection and zero-request proof, following-option behavior, regression coverage, and exact ratchet.
