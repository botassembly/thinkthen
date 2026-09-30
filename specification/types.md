# Types on every surface

Status: **Settled** by ADR 0082. This page maps the existing question and result contracts into the command, libraries, C door, and SQL extensions. [Question files](question-file.md) fix the input grammar; [results](result.md), [annotate](annotate.md), [recognize](recognize.md), and [relate](relate.md) fix the complete outputs. [`result.schema.json`](result.schema.json) is their structural JSON check, not a replacement for semantic validation. A unit test generates it from the Rust types that serialize each result (ADR 0112); nobody edits it by hand. The existing [`question-file.schema.json`](question-file.schema.json) remains the hand-kept input schema, and it holds the C door request as `doorRequest`.

## Inputs

| Concept | Type and rule |
| --- | --- |
| Question text | A string, structured object, or list in the question-file grammar. |
| Description | A string or structured `{what, not_for, examples}` object. Descriptions travel into the prompt; dropping one changes the question. Every backend receives them as authored, except the `ollama` built-in, which sends an object's `what` text alone as a temporary workaround for an Ollama bug ([backends.md](backends.md#named-backends)). |
| `true`, `false` | A description of each `decide` outcome. |
| `options`, `labels`, `kinds` | An ordered label set with optional descriptions. A bare list gives labels without descriptions; a map gives each label's description. |
| `levels` | Ordered bands, lowest first, with optional descriptions. The result is numeric. |
| `relations` | A relation name, source kind, target kind, and optional `either`. |
| `threshold` | One cut or, for `decide`, a two-cut band. |
| `on` | A JSON Pointer or pointers for records; a DataFrame uses a column name. |
| `deadline` | No deadline or a nonnegative budget. Zero is already spent. A negative budget other than the C ABI's no-deadline sentinel is refused. |
| `token` | One-shot cancellation handle. |

Every surface that accepts a description string also accepts its structured form unchanged. An ordered label set keeps caller order. For an enum-derived set, member metadata supplies defaults; an explicit map replaces the named label's description, and an unknown mapped label is a usage error. The language forms are fixed in [ADR 0082](../sdlc/planning/adr/0082-one-type-contract-for-every-surface.md); tickets J3 through J6 add forms missing today.

## Answers and failures

| Function | Successful answer | Not sure form |
| --- | --- | --- |
| `decide` | Boolean | `null` |
| `choose` | One caller label | `null` |
| `tag` | Ordered labels above the cut; `[]` is success | No separate not sure form |
| `score` | Number from 0 to K−1, possibly fractional | None |
| `filter` | Kept records | None |
| `rank` | Every record, most likely yes first | None |
| `find` | Selected unit; the libraries and the C door give `{index, unit, probability}` with a zero-based index (R: one-based `place`) | `null` when `none` wins |
| `annotate` | Named answers per input record | `null` for a not sure member |
| `recognize` | Entities with `text`, `start`, `end`, `length`, `kind`, `strength`, plus optional relations, flagged as relate's edges are | Empty `entities` is success |
| `relate` | Edges with `relation`, `source`, `target`, `probability`, and `"either":true` last on an edge of a both-ways rule | Empty edges are success |

`null` never means a failed call or failed annotate question. An annotate member that fails has `{"failed":{"kind":"backend","cause":CAUSE}}` in JSON or a distinct native variant. The score answer is a number; a level name describes a band and is never the answer. A call failure has a kind, retryable flag, and message. The six kinds are `usage`, `backend`, `deadline`, `local`, `cancelled`, and `defect`. The C ABI alone spells them as codes 1 through 6. A binding exposes the names and never turns failure into `null`. A detailed result retains the answer, question, rule, and metadata under `thinkthen.result/1` as [result.md](result.md) specifies.

The C door's `question_json` already carries structured descriptions. Its generic `thinkthen_call` takes one JSON object and returns `{"value":VALUE,"facts":FACTS}` on a successful asking call. `VALUE` is the prior bare JSON or, with `details:true` on the four judgments, a detailed result. These four verbs also accept `records` arrays with runtime labels and ordered answers; their detailed arrays include each whole `input`. `{"usage":true}` remains the engine's direct totals. A failed call returns `NULL`, with final started-call facts available through `thinkthen_error_facts_json`; see the [header](../libraries/c/include/thinkthen.h). The door adds no result structs. SQL keeps `TEXT`, `BOOLEAN`, and `DOUBLE`; the caller constrains labels. Frame columns keep their question-kind type, with one companion `failed` column for per-question markers under J2.

For a SQL `choose` result, constrain the caller's **stored answer column** with a [PostgreSQL domain](../databases/postgresql/README.md#constrain-a-stored-answer), [DuckDB enum cast](../databases/duckdb/README.md#constrain-a-stored-answer), or [SQLite check](../databases/sqlite/README.md#constrain-a-stored-answer). These recipes leave the functions' plain return types unchanged. A stored `NULL` can represent a choice with no selected option: the winner falls below the cut or the top two options tie exactly, even above the cut. SQL input-`NULL` behavior depends on the surface and argument; see each function's contract. Each recipe permits a stored `NULL`. A failed call remains an error or a distinct `try_details` failed envelope; never insert a failure as `NULL` to satisfy the constraint.

The command's narrow annotate continuation emits a `thinkthen.record-error/1`
row for a missing question-set `on` pointer. Its `failure.kind` is the existing
`usage` kind, with `cause: missing_pointer`; it does not add a seventh failure
kind or change library, C, or binding return types. The named `recordError`
definition in [`result.schema.json`](result.schema.json) checks this separate
CLI row. The schema root still checks successful `thinkthen.result/1` rows.

## Offsets

`start` includes the first character and `end` excludes the next character in Rust, C, Python, and TypeScript. `length` is `end - start`. Rust, C, and Python count Unicode scalar values. TypeScript counts UTF-16 code units, matching `String.slice`. R reports one-based inclusive positions. Keep these names and document the unit beside each native entity type. The shared non-BMP [conformance case 41](../conformance/cases.json), `Le café 😀 Maria Chen arrived.`, pins `Maria Chen` at scalar span `[10,20)`, TypeScript UTF-16 span `[11,21)`, and R positions `11..20`. The same case checks the C door and each later port; no new recording is needed.

## Parity

[`fixtures/types/`](fixtures/types/) checks the generated result schema against valid and invalid examples and checks selected requests through the real C JSON door against the offline conformance backend. A schema verdict alone does not prove that the door accepts or emits the same shape. Port integration runs this corpus and the shared conformance cases through the public binding. The question-file schema and parser keep their [separate parity check](fixtures/question-file/README.md).
