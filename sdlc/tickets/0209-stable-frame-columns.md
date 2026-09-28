---
flow: build
priority: 209
opens: sdlc/tickets/0209-stable-frame-columns.md sdlc/records/0209-frame-preflight.md sdlc/records/0209-design-review.md
---

# 0209: Keep frame answer columns typed

Status: proposed design on `ticket/0209-stable-frame-columns`; no runtime file is claimed or changed. Owner: Codex. Ian's ruling in the [type-contract issue](../issues/2026-09-27-one-type-contract-for-every-surface.md), ADR 0082, and landed J1 ticket 0204 set the outcome. Ian can overturn the frame representation described here.

## Outcome

`annotate` over a pandas or Polars DataFrame, and the Rust `PolarsEngine::annotate_frame`, return one column per question whose type does not depend on a backend failure. They add one `failed` column. A cell is null when that row has no failed question; otherwise it maps each failed question name to the existing `{"failed":{"kind":"backend","cause":CAUSE}}` marker. Refuse a question named `failed` and an input frame that already has that column before any request. Preserve the caller's columns, row order and pandas index. Keep list and Series calls, partial-result rules, error kinds, retryability, request bytes and result JSON unchanged.

## Evidence

- Starts from: The type-contract issue's rule 7, ADR 0082, `specification/types.md`, and [source preflight](../records/0209-frame-preflight.md) against main `30d340d1`. Current Python and Rust frame constructors deliberately turn any question column with one `Annotated::Failed` into text. Existing focused tests assert that behavior; it is a confirmed source defect, not an inferred port gap.
- Keeps: The engine's `Annotated` values and `AnnotatedRecord::value_json()`, exact failure marker and six named error kinds, nullable unresolved `decide` and `choose`, successful empty `tag`, and the rule that an all-failed response ends the call. A Series call still raises for its one failed question, because it has no frame companion column.
- Changes: The three frame doors produce stable question columns and one failure column. Update the old widened-column tests and the frame-specific documentation and conformance comparison. Amend ADR 0047 item 10's old widened/text table when implementation is accepted; ADR 0082 has superseded that result rule. No C ABI, generic result schema, or question-file grammar changes.
- Proof: Reuse shared `17-annotate-partial` and the existing two-row malformed-reply frame boundary. Pin dtypes and values for a failed `choose` beside a not-sure `decide`, answered and failed `score`, and successful `tag` list versus failed null. Check per-row failure mapping, pandas 2 and 3 indexes, Polars and Rust Polars frame schemas, empty and all-success frames, pre-send reserved-name and input-column refusals, and retained all-failed call error. Use selected existing tests and case 17 only; full port and stress gates wait for the related batch.
- Defers: Categorical encoding for `choose` is optional under rule 7; this ticket uses stable UTF-8/string, including when labels are known. J5 owns Python Enum/Literal and sharper stubs. B12d owns Python batching; J8 owns the port corpus. No new frame test framework or paid call belongs here.

## Stable columns and row semantics

| Question kind | pandas dtype | Polars dtype, Python and Rust | Answered value | Unresolved or failed answer cell |
| --- | --- | --- | --- | --- |
| `decide` | nullable `boolean` | `Boolean` | `True` or `False` | null for not sure; null plus `failed` marker for failure |
| `choose` | nullable `string` | `String` | chosen label | null for no fit; null plus `failed` marker for failure |
| `score` | nullable `Float64` | `Float64` | numeric position, including fractional | null plus `failed` marker for failure; no ordinary not-sure state |
| `tag` | `object` containing `list[str]` or null | `List(String)` | ordered labels, including `[]` for a successful empty answer | null plus `failed` marker for failure; `[]` never means failed |

Always append `failed` last, after the question columns, including for an empty frame or a run with no failures. pandas holds a sparse `dict[str, dict]` in an `object` column for failed rows, and `None` for other rows. Pinned Rust `polars` 0.55.2 has no public `DataType::Map`. The proposed Polars column is a nullable `Struct` with one field per question in set order. Each question field is a nullable `Struct<failed: Struct<kind: String, cause: String>>`, preserving the full marker. The outer `failed` cell has explicit parent null validity when no question failed. Null sibling fields in a nonnull cell mean those questions did not fail; null children alone do not make the parent null.

For questions `urgent: score` and `team: choose`, an answered row has `urgent=0.8`, `team="billing"`, and `failed=None` in pandas and Polars. A partial row whose `urgent` reply failed with `missing_probability` while `team` was not sure has `urgent=null` and `team=null` in both doors. pandas has `failed={"urgent":{"failed":{"kind":"backend","cause":"missing_probability"}}}`. The proposed Polars row has `failed={"urgent":{"failed":{"kind":"backend","cause":"missing_probability"}},"team":null}`. Thus the answer columns keep `Float64` and `String` throughout; the companion distinguishes the not-sure `team` from failed `urgent`. The Polars row exposes the extra null `team` key. **Ian must choose whether that fixed-key columnar encoding satisfies the issue's sparse map wording.** Recommend accepting it: native typed Struct cells support the pinned Arrow and Rust Polars doors, preserve the exact marker under each failed question, and keep one stable schema. If absent sibling keys are required in Polars row dictionaries, return for a different public representation decision. Do not silently use JSON text or flatten the marker.

For zero rows, construct typed empty answer columns and an empty `failed` column with its complete nested Struct schema in both Polars doors; pandas has typed empty answer Series and an empty `object` Series. For all-success rows, set every outer Polars `failed` validity bit to null, so extracting that column returns actual null cells, not nonnull structs of null children. A row with one failed question has a valid outer cell and only that question's nested marker present.

The `failed` cell distinguishes a failed `decide` or `choose` from a not-sure null in its answer column. A whole-call `Backend`, `Deadline`, `Cancelled`, or other error still raises and returns no frame. A question named `failed`, an input frame already holding `failed`, or another answer-name collision is a `usage` refusal with zero backend sends. These checks run before the worker sends. No extra failure column is added to standalone Series returns or to the list/record `annotate` form.

## Proposed implementation boundary

The [preflight](../records/0209-frame-preflight.md) inventories every constructor and test adapter. After fresh design review and Ian's representation ruling, claim the exact Rust Polars `public/frame.rs` and `frame/column.rs`, Python binding `src/frame.rs`, Arrow output files, Python `thinkthen/__init__.py`, and named frame tests and docs. Add the pinned `polars` 0.55.2 `dtype-struct` feature for the native companion column; measure its lock and compile effect before approval. Keep Python's core dependency-free and use the existing Arrow writer and pandas dtype adapter. The current `libraries/python/src/arrow/write.rs` is already 540 nonblank lines; move its existing `SchemaNode` and metadata-copy section into `src/arrow/write/schema.rs`, then add the nested failure writer in the existing column section. Claim the new module registration and ADR 0047 correction before those edits. No source edit belongs to this design phase.

## Review route

A fresh read-only reviewer checks the stable type/null table against ADR 0082 and the type issue, especially the Polars struct map interpretation, categorical deferral, all-failed rule, pre-send refusals, adapter inventory and proof distinctness. The coordinator accepts or amends the design, claims runtime files, then resumes this builder. A fresh code reviewer and focused host proofs precede landing. No paid provider call is requested.
