---
flow: build
priority: 68
opens: crates/thinkthen/src/core/result.rs crates/thinkthen/tests specification transforms probes demos sdlc/scripts/test sdlc/ratchet.json sdlc/planning
---

# 0068: Name stored-result metadata cached

Status: landed

## Outcome and authority

Detailed results emit `meta.cached` instead of `meta.replayed`, and existing offline consumers still read historical rows. Ian authorized execution of the reviewed engine plan and creation of bounded tickets. Its metadata rename can proceed independently of the remaining outcome-vocabulary issue. Ticket 0065 has landed at `ba60f04`; this ticket changes no cache behavior. ADR 0036 records the spelling, exact preserved meaning, prerelease schema choice, and reader compatibility before implementation.

## Current facts and design

`core/result.rs` serializes `Meta` and `AnnotateMeta` with `replayed` after `requests_sent`. The engine supplies the same boolean for explicit replay and caches; annotation combines group flags with AND. Change the output-field name only. Use `cached` directly in the two serialization structs and initialize it from the unchanged `RequestMeta.replayed`; this avoids two extra attribute lines in a file already near its enforced size cap. Leave private engine/RequestMeta names, propagation, usage accounting, output membership, and `--replay` intact.

New `thinkthen.result/1` rows emit only `cached` at the same field position. It means the answer came entirely from stored exchanges, not merely that this particular caller sent nothing. Explicit replay remains true; live/coalesced-live remains false; mixed stored/live annotation remains false. Request counts and all other metadata retain their current meanings.

Cost, trials, and the find-probe input reader accept new `cached` and legacy `replayed`. A present `cached` key wins even when false; only absence permits fallback. Preserve existing validation: trials/find reject invalid canonical values, while cost treats only boolean true as stored. Keep transform/probe aggregate output names unchanged. Never use `//` for this boolean fallback. The replay checker normalizes both field names without rewriting captured rows and requires new replay output to contain `cached:true`.

## Scope and exclusions

Production source is limited to `core/result.rs` serializers and its existing expectations. Update affected backend test expectations and the executable replay-demo test fixture; add only focused metadata assertions to existing tests. Update `specification/result.md`, `specification/recording.md`, current how-tos 12/27/28, and relevant existing transform/probe descriptions. Consumer changes are limited to cost/trials transforms and tests, `probes/find-0040/run` plus its self-test and one small `stored-result.jq` input normalizer, and `probes/replay-check.sh`. The probe's `fixture.py`, `cases.json`, and `hashes.sha256` are frozen preregistration evidence and must remain byte-identical. Normalize only transient result metadata before the original fixture reader runs; preserve its validation and derived output. Never update a historical pin to accommodate this rename. A small new `transforms/cost/test.sh` may be wired into the existing test rung if the how-to budget cannot hold the compatibility cases. Update directly affected field-name comments only; preserve unrelated comments. No bulk replacement across the repository.

Excluded: engine/state/concurrency changes, all recording files and captured probe/experiment rows, historical SDLC records, transform output renames, outcome vocabulary, site/library/database work, dependencies, policy changes, paid calls, and publication. The active library team receives the canonical contract through the durable handoff, not edits to its branch.

## Acceptance

- Observe failing tests before changing serializers and consumers. Exact `Meta` and `AnnotateMeta` expectations show `cached` after `requests_sent`, no `replayed` alias, and otherwise unchanged output. All supported detailed command forms use those serializers.
- Existing counted-listener coverage proves live and retry results remain false with actual sends; explicit replay, named/default cache hits, and cache-lock waiters remain true with zero sends; mixed annotation keeps its AND rule. Reuse tests rather than constructing another engine or recorder.
- Cost/trials/find-reader cases cover new-only and legacy-only true/false, mixed inputs, and both-key disagreement with canonical false winning. Trials/find reject canonical null/string despite a valid legacy boolean. Existing validation and aggregate outputs remain unchanged.
- Replay checks pass against untouched captured rows. New replayed rows must carry `cached:true`; normalization must not hide changes to unrelated content. Preserve recording bytes/digests and historical measurements.
- Run focused core/result and backend metadata tests, transform tests, find-probe self-test, and affected how-tos. Coordinator runs all four gates and diff checks after independent code review. Independent review covers the public field and any measured ratchet adjustment.
- Amend the queue and relevant ADR/spec links, record the printed-output change for marketing/library integration, and document actual evidence and limitations.

## Complexity

Contract 1; State/timing 0; Reach 1; Proof 2; Cost of error 1; Total 5. Minimum floor: none. Final level: 2. Reasons: one explicit output rename plus offline historical-reader compatibility; no persistent-state or request behavior changes. Selected implementation: `swe2-implementer` (`swe-2-high`), Ian's approved bounded-worker experiment. Separate `sol-reviewer` sessions review design and code. Stop and re-score if state, credentials, or a new compatibility policy becomes necessary.

## Review

Independent Sol design and code reviews: ACCEPT, including the frozen-probe read boundary and annotation no-alias proof. The final output structs use `cached` directly; engine/RequestMeta bookkeeping is unchanged, and the 500-line cap remains intact. The coordinator's sequential four-rung ladder passed 568 Rust tests, one intentional child-harness ignore, doctests, replay checks, transform/probe tests, and nineteen how-tos at the exact 33,840-line ceiling. Frozen preregistration files and captured recordings/rows are unchanged. Hosted run `35756403716` was cancelled after Ian disabled Actions; he selected local full gates as the verification authority. New main `692ba59` was integrated without conflicts and the coordinator reran the complete ladder successfully on the combined tree. Integrated revision `22a4193` was fast-forwarded to main and pushed under that local-gate authority. The matching record retains the earlier overlapping-worker test failures and their evidence limitations.
