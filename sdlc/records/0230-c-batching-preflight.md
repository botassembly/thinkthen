# 0230 C JSON batching preflight

Status: source investigation at main `cbaa5fbd`; no implementation or proof run. Accepted ADR 0089 is on the unlanded 0212 branch at `a713e462`, whose current source is `e306674f`. Read the final reviewed 0212 result before building. The C JSON door is B12b, not the typed C ABI.

## Current path and needed change

`libraries/c/include/thinkthen.h:235–274` promises bare success JSON, `NULL` failure, five envelope keys and ten verbs. `src/call.rs::split` partitions envelope from question with `ENVELOPE`; `Question::from_json` handles the remaining question grammar. Top-level `batch` already belongs to that question and its calibration identity. `call.rs::answer` uses `Engine::details_with` for decide/choose/score/tag; `filter_with` and `annotate_with` are consumed by `collect`, rank/find return eager values, and `door::recognize`/`door::relate` carry the other two shapes. `src/settings.rs` parses `new_with`; it has no batch key. `src/ffi.rs::thinkthen_call_opts` owns the returned `CString` and records failures with `Held::settle`. `src/failures.rs::Failure::from(Error)` currently drops `Error::facts`; `Last` retains code, retryability and message in an engine/thread table. The borrowed `error_message` pointer survives until that thread's next failure on this engine or engine free; `UNBUILT` is the null-engine build slot. Add failure facts to this same storage/lifetime, not process counters or a global slot.

| Verb | Existing success value | Batch/facts route |
| --- | --- | --- |
| decide | boolean or null | One document keeps this value; proposed `records` returns an array of these values through `decide_many`. |
| choose | label or null | Proposed `records` returns an array; keep one complete question, options and descriptions for all rows. |
| score | number | Proposed `records` returns an array of weighted numbers. |
| tag | ordered label array, including `[]` | Proposed `records` returns an array of label arrays; never turn an empty inner array into failure. |
| filter | kept-record array | Exhaust bulk iterator; facts count judged rows, including suppressed ones. |
| rank | sorted-record array | Eager whole-input call; ties and order stay as Rust returns. |
| find | selected unit or null | Existing candidate planner; call facts, no record batching selector. |
| annotate | named result object per input row | Exhaust iterator; keep explicit failed-member marker, not null. |
| recognize | entities and optional relations object | Existing multi-step planner, no invented context route. |
| relate | edges object | Existing relation planner, no invented record grouping. |

The ten rows are **current** JSON output shapes; proposed automatic facts wrap a successful asking result once. `{"usage":true}` stays the direct process snapshot. C already owns complete `Vec<String>`/`Vec<Box<RawValue>>` inputs before calling the library. The pending 0212 default-Max caller-owned iterator pause concerns incremental host iterators, not this already-materialized C input. Do not claim 0212 is landed or infer that its current source settles the pending choice.

## File and corpus inventory

Prospective direct source: `libraries/c/src/{call,door,settings,ffi,failures}.rs` and `libraries/c/include/thinkthen.h`; a coherent private child may carry success/failure JSON writers if needed. Current nonblank sizes at this base: call 249, settings 130, ffi 458, failures 261, door 149. `ffi.rs` has only 42 lines before the usual 500-line cap, so prefer an existing error-storage helper over crowding it. Direct consumers and documentation: `libraries/c/tests/door/{bytes,cases,settings}.rs`, `libraries/c/tests/c/{driver,opts,cancel,nulls,threads,settings}.c`, `libraries/c/examples/functions.c`, `libraries/c/README.md` and `libraries/c/DESIGN.md`. `bytes.rs` currently compares a bare reply with CLI bytes; after the public JSON decision it should compare `value` with the old bytes and facts separately. Keep the ten operation fixtures in `tests/door/cases.rs` rather than creating ten copied listener suites.

The JSON contract also requires `specification/result.schema.json` (a named C-call success definition and typed facts), `specification/fixtures/types/corpus.json` valid/invalid rows, and its `self-test`, which runs both schema and a real C door. Add the C question request's closed `call` options to the question-file/C request parity proof without changing the saved question's top-level `batch`. Review `conformance/cases.json`, `specification/types.md`, `specification/{question-file,settings,result}.md` and `sdlc/issues/2026-09-26-batching-design.md` for applicable public promises. The result schema currently types bare verb definitions and detailed `thinkthen.result/1`, not a `{value,facts}` C wrapper. `tests/door/cases.rs` is already large; add a small child for distinct new boundaries if needed. Claim exact files before implementation.

Reuse the shared listener/captured-wire fixtures and C installed driver from the existing C check route. The 0226 Linux C package/lifetime proof is evidence for that earlier guard, not changed 0230 binary proof. Keep `batch 1` request-body/digest comparison, no-send refusal, 413 split, error facts and old caller `NULL` behavior as separate observations; do not reproduce the full B12a planner table. The pending 0212 source compile migration is separate from 0230 public JSON behavior. A reviewed 0212 landing is required before this C implementation starts.

## Public choice and review limit

Recommend automatic `{"value":...,"facts":...}` on successful asking calls and an additive borrowed `thinkthen_error_facts_json` for started failures. The former changes bare success JSON and requires caller migration; the latter preserves `NULL` and its existing error accessors. A non-NULL error envelope would make old failure checks misclassify errors. Independent design review should decide whether the proposed success wrapper and `"call"` request spelling follow Ian's B12b outcome or require an explicit outward-facing choice. Do not silently omit failed-call facts, add a mandatory second call for success, or change the typed C result ABI.
