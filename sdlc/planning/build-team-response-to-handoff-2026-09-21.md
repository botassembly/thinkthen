# Build-team response to the 2026-09-21 handoff

Status: Approved by Ian on 2026-09-21. Ticket 0053 starts the accepted order. The product-side additions are recorded in `../issues/2026-09-21-the-product-sides-reply-to-the-build-teams-response.md`.

This response reviews `handoff-to-the-build-team-2026-09-21.md` against the current code, ADR 0017, the twenty-five conformance cases, the caller review, and the first quality wave. Ian can overturn every recommendation.

## Where the suggested order is wrong

The handoff puts all eight shape changes before the one-crate merge. That would add engine behavior to the command and immediately move it. ADR 0017 promises that step 1 is a mechanical move with no request, retry, output, diagnostic, exit-code, or page change.

Only two changes should precede the move: request identity and the failed-question outcome. Both define core result types that the private engine bridge must carry.

The move comes next and adds the private runner as a new proof over the shared cases. Backend profiles, size preflight, record-return behavior, cache settings, and threshold warnings should follow the move and precede the public Rust surface. This keeps the move reviewable and prevents the command from growing machinery that belongs in the engine.

The handoff also puts the C door before `recognize` and `relate`. The allocator can be imagined first, but the ABI cannot freeze until the two variable-size result objects and their Rust ownership are real. One Rust result path should exist before one C JSON allocation and free rule binds it.

The quality work cannot all follow the new functions. The failed-question outcome and local size check protect their ordinary multi-question requests. The recognize work must also verify the probability-total repair that already landed and settle the shared-instruction packing change before any public cost claim.

## Placement of the eight changes

| Change | Placement | Reason |
| --- | --- | --- |
| Request identity on every result | Separate ticket before the merge | The engine bridge must carry it from its first day |
| Failed question inside a successful request | Separate ticket before the merge | It creates a public outcome distinct from an unsure `null` |
| Backend profile and local size check | Separate ticket after the move, before the public Rust API | The engine owns preflight and the check changes send behavior and error kind |
| Record returned with its answer | Separate ticket after the move, before the public Rust API | The exact CLI and bulk-return shape still needs one ruling |
| Bounded default cache, config, `--no-cache`, prune, and `status` | Engine tickets after the move, before the public Rust API | The cache must move before its default behavior changes; the quality wave calls the missing surface a blocker |
| One relation-number name | Settle with the `recognize` and `relate` contract | No current result carries a relation |
| Variable-size `recognize` and `relate` results through C | Implement after the Rust result path and public API | C should own one JSON allocation and one free rule over final result objects |
| Backend-aware threshold behavior | Settle with backend profiles after the move, before the public Rust API | A stable profile identity is needed before a useful warning can compare calibration |

No shape change belongs inside the move. The already accepted private conformance runner is a new gate. It changes no product behavior.

## Conformance changes

The contract should grow from the current file. It should not replace the twenty-five cases or freeze a new count.

1. Request identity adds a logical request-digest list to every successful expectation. The validator derives each digest from the exact adapter, address, and request bytes. Per-answer request identity stays on grouped answers.
2. Partial failure adds one salvageable mixed reply with one good answer and one invalid answer. It also keeps whole-response and transport failures, and an unsure case that proves `null` still means unsure. The expected metadata carries the failure count.
3. Size preflight adds profile cases at and one unit past both the request-size limit and the question-count limit. The engine runner proves that each refusal sends no request. The existing recording-read local fault stays.
4. Record return covers `decide`, `choose`, `tag`, `score`, and `annotate` under lines, JSONL, CSV, and TSV. It proves byte-for-byte line preservation, structural object preservation, existing annotate collision behavior, and the ruled text/object annotate shapes.
5. Shared engine cache behavior uses runner scenarios over existing success cases: default miss and hit, `--no-cache`, explicit-folder precedence, cap, and prune. CLI-only tests cover config discovery and `status`. Host paths and cache state do not enter the language-neutral data file.
6. Threshold compatibility waits for the profile policy. The runner then applies the same question through two profiles and pins the warning channel and exact text while proving that the result shape does not change.
7. `recognize` raises verb coverage to nine and adds Unicode global offsets, repeated text with distinct entity ids, no entities, wildcard relations, the exact cut boundary, one name crossing a cut, overlap deduplication, a relation crossing cuts, and the maximum question-count boundary after those rules settle.
8. `relate` raises verb coverage to ten and adds directed and either edges, no legal edge, the record-limit boundary, and the ruled one-or-many-relations behavior. Both new functions pin logical request and pair counts, the relation-number field, details, and variable-size results.
9. C pointer, length, error, null, zero-length, and single-free behavior stays in C binding tests. Those tests consume the same expected JSON as the shared cases.

## Recommended public choices

These choices are hard to reverse after bindings ship, so Ian should answer them before the tickets become active.

1. Use `meta.requests`, always an array in logical construction order: input record, evidence group, then chunk. A retry does not add an entry. Two logical requests with identical digests appear twice. Single-question results carry one digest. Keep the existing per-answer `request` where one answer must name its exact request. This revises the handoff's singular `meta.request` wording and needs Ian's explicit approval.
2. Represent a failed named answer as `{"failed":{"kind":"backend","cause":"invalid_distribution"}}` in the value map. Its detailed entry carries `question`, `request`, and the same `failure`, with no `value` or `answer`. Add `meta.failed_questions`. Missing answers, wrong answer kinds, and invalid distributions fail only their own question when the top-level reply and the other answers decode. Malformed top-level JSON, missing or conflicting model identity, unexpected answers, transport, authentication, and a refused whole reply still fail the request and run. An unsure answer keeps `value: null`.
3. In record mode, make `decide`, `choose`, `tag`, and `score` print JSONL rows shaped as `{"input":RECORD,"value":ANSWER}` by default. Preserve `annotate`'s flat enrichment for JSON objects, including its collision refusal. Change `annotate` text records from answers alone to `{"input":TEXT,"value":ANSWERS}`. CSV and TSV records are objects and keep the enrichment rule. `filter`, `rank`, and `find` continue returning records.
4. Call the number on every relation `probability`. Keep `confidence` for the aggregate entity confidence from `recognize`, where it combines several probabilities.
5. Give each backend profile a stable name and declared request limits. Keep one threshold in a question file. Warn loudly when a threshold calibrated for one named profile runs against another. Do not add a per-backend threshold table in 0.1.

The later tickets still have outward choices. They do not belong inside the mechanical merge. The backend-profile ticket must settle profile selection for a custom URL or model, byte limits versus backend token limits, local counting, the question-file calibration field, what uncalibrated means, and the warning channel. The `recognize` and `relate` ticket must settle text cuts, overlap and deduplication, cross-cut names and relations, global offsets, the per-request question maximum, the `relate` record maximum, one or several true relations per pair, and whether SQL exposes relations through `thinkthen_recognize` or a separate `thinkthen_relations`. The C ticket must settle generic JSON request versus per-function exports, error ownership, and null and zero-length returns. Each of those tickets stops for its contract review before code.

## Polars ruling received during this review

Ian chose Polars as Python's data-frame container while this response was under review. Plain Python lists remain first-class, and all width and vectorization stay in Rust. This changes no merge step and adds no shared case. The public Rust ticket must expose one bulk spine that both containers cross once. The later Python ticket proves list and Polars results against the same shared cases, plus zero-copy Arrow crossing, process-wide width, interrupt, and warm-fork behavior from `polars-plan.md`.

## Proposed tickets after Ian answers

1. Put request identity on every result and extend the shared expectations.
2. Preserve good answers when one question fails and add the partial-success case.
3. Fold to one crate through the private bounded bridge and run the existing shared contract through it. Preserve all behavior.
4. Add named backend profiles, local size preflight, and the threshold-calibration warning.
5. Return each record with the value verbs' answer and pin the bulk result shape.
6. Expose cache and counter settings after step 1 has already moved scheduling, recording, and cache locks into the engine. Preserve behavior.
7. Ship the bounded XDG cache, smallest config file, `--no-cache`, prune, and `thinkthen status`.
8. Verify the landed probability tolerance and pack shared instructions once per request. Pin the changed request bytes and cost record.
9. Build `recognize` and `relate` together with their shared cases after their open contract choices settle.
10. Open the public Rust API over all ten functions and the final result shapes.
11. Add width, cancellation, fork repair, and fast dead-address failure at the public engine boundary.
12. Add the C JSON result door and its one free function.
13. Build the Python list and Polars doors over the same Rust bulk spine after experiments 212 through 216 answer the open plugin choice.
14. Close the cache-resume and retry-accounting money findings.
15. Close the public-example, pipeline-outcome, and vocabulary promise findings.
16. Close or waive the remaining quality findings under `quality-plan.md`, then run the separate release pass.

The proposal creates each ticket only when its turn begins. Ian's answer can change the choices or order without leaving stale ticket text.
