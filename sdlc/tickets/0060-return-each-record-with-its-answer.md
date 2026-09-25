---
flow: build
priority: 39
opens: crates/thinkthen conformance specification demos sdlc/ratchet.json
---

# 0060: Return each record with its answer

Status: landed

## Outcome

The value verbs keep each streamed record beside the answer it produced. A default record-mode row from `decide`, `choose`, `tag`, or `score` is compact JSON shaped as `{"input":RECORD,"value":ANSWER}`. In record mode, `annotate` keeps enriching object records and uses the same wrapper for text and other non-object records. One document keeps its current output for every JSON shape and for text. Requests, answers, thresholds, order, and exit behavior do not change.

## Current facts and decisions

Record-mode `decide`, `choose`, `tag`, and `score` currently print an answer alone. `annotate --lines` prints only its answer map. A later command cannot tell which answer belongs to which record without asking for full details or joining two streams. `filter`, `rank`, and `find` already return records and remain unchanged.

Ian approved one default bulk shape before the public libraries freeze. Under `--lines`, `--jsonl`, `--csv`, and `--tsv`, the four value verbs print one compact JSONL object per answer. `input` is the parsed record: a line is a JSON string, JSONL preserves the JSON value structurally, and CSV or TSV produces an object of string cells. `value` is exactly the bare answer the command printed before this ticket. Key order is `input`, then `value`.

`annotate` keeps its established enrichment rule for an object record: original members first, then named answers, with the existing collision refusal before a request. CSV and TSV records are objects and follow that rule. A line, JSON scalar, or JSON array becomes `{"input":RECORD,"value":ANSWERS}`. A one-document annotation keeps its established output, including enrichment when that document is a JSON object.

Explicit views still mean what they say. `--details` prints the existing full result with `input`. `--raw` and `--quiet` keep their current behavior. The new wrapper is only the default record-mode view. One text document still prints one bare answer, including `true`, `false`, `null`, a choice, a tag list, or a score.

Default record output remains JSONL, including output produced from CSV and TSV input. CSV and TSV are input formats only. `choose --raw` keeps its established plain-text view on one document, lines, and JSONL. This ticket adds no CSV or TSV output mode.

Ian can overturn the wrapper keys, object-enrichment rule, or explicit-view precedence.

## Scope

Add one small serializable record-and-value type in the core result layer and use it at the existing output boundary only when `Reading::streams()` is true. Keep the one-document annotation path unchanged. In record mode, make non-object annotations retain their input beside the answer map while object enrichment stays structurally equivalent. Extend the host-neutral shared cases over typed records and values through the production serializer. Use compiled command tests for the four CLI framings and five affected verbs. Bring the result, records, verb, and affected how-to pages level.

Excluded: `filter`, `rank`, `find`, request construction, recording identity, cache keys, retry or scheduling behavior, answer types, thresholds, detailed output, new flags, table output, `recognize`, `relate`, and library bindings.

## Acceptance

- Default record mode for `decide`, `choose`, `tag`, and `score` emits one compact JSONL row per input shaped exactly as `{"input":RECORD,"value":ANSWER}` in input order. Lines retain their text as a JSON string. JSONL values remain structurally equal. CSV and TSV cells remain strings inside an input object.
- Default record-mode `annotate` enriches JSON objects, CSV rows, and TSV rows exactly as today. Lines, JSON scalars, and JSON arrays emit `{"input":RECORD,"value":ANSWERS}`. Existing member-collision refusal still occurs before any request.
- A single document keeps its established bare value or annotation shape. Exact tests cover one-document annotation of text, a JSON scalar, a JSON array, and a JSON object. `--details`, `--raw`, and `--quiet` keep their established shapes and channels. `filter`, `rank`, and `find` remain byte-for-byte unchanged.
- Empty, unresolved, and failed values remain distinguishable: `null` is a value, an empty tag list is `[]`, and an annotate failed marker remains inside the value map. Each still retains its input in the new wrapper where the rule applies.
- Request bytes, request digests, recording paths, cache behavior, input order, worker width, exit codes, and stopped-run wording do not change. Existing committed recordings replay without replacement.
- Host-neutral shared cases pin `{"input","value"}` over typed records and values through the production serializer. They do not absorb CLI framing. Compiled tests cover all four framings, the five affected verbs, single-document compatibility, explicit views, object collision, parallel order, replay, and secrecy. The four repository gates and `git diff --check` pass without a key or outside network access.

## Dependencies

Ticket 0059, the A3 build-queue entry, `sdlc/planning/build-team-response-to-handoff-2026-09-21.md`, `sdlc/planning/go-ahead-for-the-build-team-2026-09-21.md`, and `sdlc/issues/closed/2026-09-21-two-function-flows-lose-the-record-between-stages.md`.

## Complexity

- Contract: 2
- State and timing: 1
- Reach: 2
- Proof: 2
- Cost of error: 1
- Total: 8
- Minimum level floor: none
- Final level: 3
- Reasons: the output contract changes across five public commands and four framings, but it stays at one existing serialization boundary and changes no request or engine behavior.
- Selected model: `gpt-5.6-sol` with medium reasoning.

Re-score if the work changes a request, detailed result, input parser, scheduler, exit code, or public option.

## Review

Independent design review rejected the first draft because three sentences could break one-document annotations, put CLI table parsing in the host-neutral conformance layer, and call `choose --raw` JSONL. The repair limits the new annotation wrapper to `Reading::streams()`, separates typed shared cases from compiled framing tests, and preserves the explicit raw view.

The same reviewer accepted the repaired design with complexity 8, level 3, and Sol Medium. No material contradiction remains.

## Implementation evidence

The compiled record matrix failed first because every value verb printed its old bare answer and non-object annotations printed only their answer map. The production serializer, the five typed shared cases, and the compiled framing matrix then passed. The matrix covers four value verbs across lines, JSONL, CSV, and TSV; annotate across lines, JSONL arrays and scalars, CSV, and TSV; and all four one-document annotation shapes.

Independent code review rejected the first help text because the shared `--lines` and `--jsonl` descriptions falsely promised the value wrapper for `filter`, `rank`, and object annotations. A compiled help test over `decide`, `annotate`, and `filter` reproduced the omission. The shared text now names the three different output rules and the test passes. Output behavior did not change in this repair.

The same reviewer accepted the repair. The final install, lint, test, and specification rungs pass with the key and base-address variables unset. The test rung passes 185 library tests, 234 backend tests, and every other compiled suite. The specification rung passes all 19 green how-tos. The ratchet is exact at 28,528 nonblank Rust lines, and `git diff --check` passes. No outside network or paid call ran.
