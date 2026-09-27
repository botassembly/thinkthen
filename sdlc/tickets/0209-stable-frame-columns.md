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

Always append `failed` last, after the question columns, including for an empty frame or a run with no failures. pandas holds a `dict[str, dict]` in an `object` column for failed rows, and `None` for other rows. Polars has no native public map dtype in the pinned Rust `polars` 0.55.2 `DataType`; use a nullable `Struct` whose fields are the set's question names in set order, each a nullable marker struct with `kind` and `cause`. The top-level `failed` cell is null when none failed; a nonnull cell has a marker only under failed names. Null sibling fields mean absent map entries. A reader selecting one question uses `failed.<name>` and tests for null. Keep the exact JSON marker shape in Python's dict and the existing bare list/record outputs; the Polars struct is the columnar representation of the same logical mapping. The independent reviewer must confirm that fixed-key struct is a faithful reading of Ian's map rule before runtime files are claimed. If absent keys rather than null sibling fields are required in Polars row dictionaries, that is a public representation choice for Ian; do not silently replace the map with JSON text.

The `failed` cell distinguishes a failed `decide` or `choose` from a not-sure null in its answer column. A whole-call `Backend`, `Deadline`, `Cancelled`, or other error still raises and returns no frame. A question named `failed`, an input frame already holding `failed`, or another answer-name collision is a `usage` refusal with zero backend sends. These checks run before the worker sends. No extra failure column is added to standalone Series returns or to the list/record `annotate` form.

## Proposed implementation boundary

The [preflight](../records/0209-frame-preflight.md) inventories every constructor and test adapter. After fresh design review, claim the exact Rust Polars `public/frame.rs` and `frame/column.rs`, Python binding `src/frame.rs`, Arrow output files, Python `thinkthen/__init__.py`, and named frame tests and docs. Add `polars`'s `dtype-struct` feature only if the pinned crate requires it for the native companion column; measure any lock/dependency effect before approval. Keep Python's core dependency-free and use the existing Arrow writer and pandas dtype adapter. The current `libraries/python/src/arrow/write.rs` is already 540 nonblank lines; split one coherent existing schema/column section into a private sibling when adding the failure struct instead of lengthening that file further. Claim its module registration and the ADR 0047 correction before those edits. No source edit belongs to this design phase.

## Review route

A fresh read-only reviewer checks the stable type/null table against ADR 0082 and the type issue, especially the Polars struct map interpretation, categorical deferral, all-failed rule, pre-send refusals, adapter inventory and proof distinctness. The coordinator accepts or amends the design, claims runtime files, then resumes this builder. A fresh code reviewer and focused host proofs precede landing. No paid provider call is requested.
