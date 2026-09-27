# One type contract for every surface

Status: Open. The queue owner places batch J in the work plan. Ian can overturn any rule in section 4.

The Codex coordinator wrote a type-system review on 2026-09-27 for the language-port experiments. Claude revised it the same day against main at `7a1ba6cc` and corrected its claims about the schema, the C door, Rust, TypeScript and Python. This issue holds the revised version. The port experiments wait on J1: they need one written contract to map into each language.

## Plan placement

Add batch J to `sdlc/planning/work-plan-2026-09-27.md`. Leave the existing tickets as they are.

- **J1** is ready now. It edits only `specification/`, a new ADR and a fixture corpus, so it collides with no running lane.
- **J2** is a defect and is ready now. It edits the Python frame code and the Rust Polars door before B12d starts.
- **J3 to J6** each run in the same lane right after that library's batching ticket, B12b to B12f, because they edit the same files.
- **J7** is ready after J1.
- **J8** is one integration ticket per port, after J1. Rule 2 gates each one.
- Add J1 to J8 to the "Every item" table as to do, and update the Counts line.

## 1. The verdict

The type model is already right, and Rust holds it. The question-file schema already exists. The C door already carries descriptions. What 0.1 lacks is the same model, told the same way, in every library. Five defects and gaps follow from that. The worst is that a failed `annotate` question changes a DataFrame column's type at run time.

The language ports should not invent their own typing. They should map one written contract into their idiom. That contract is a spec page plus a result schema, and it is ticket J1 in section 6.

## 2. The model every surface maps to

### Inputs

| Concept | Type | Notes |
|---|---|---|
| Question text | a string, an object, or a list | This is the question-file grammar. |
| Description | a string, or a structured object with `what`, `not_for` and `examples` | Rust's `Description` builder and the schema already carry the structured form. Every surface should accept both forms. |
| `true` / `false` | a description each | This input is for `decide` only. |
| Label set: `options`, `labels`, `kinds` | ordered labels, each with an optional description | A bare list means labels with no descriptions. |
| `levels` | ordered bands, lowest first, each with an optional description | The answer stays a number. |
| `relations` | name → source kind, target kind, and an `either` flag | |
| `threshold` | a cut, or a band of two cuts (`0.1:0.9`) | `decide` takes a cut or a band. `choose` and `tag` take one cut. |
| `on` | one JSON Pointer or several | In a DataFrame, `on` names a column. |
| `deadline` | none, or a non-negative budget | Zero means the budget is spent. A negative budget is refused. |
| `token` | a one-shot cancel handle | |

### Answers

| Verb | Answer | "Not sure" |
|---|---|---|
| decide | yes or no, with a probability in details | `null` |
| choose | one label | `null` |
| tag | the labels over the cut | an empty list |
| score | a number from 0 to K−1, which may be fractional | not applicable |
| filter | the kept records | not applicable |
| rank | every record, most likely yes first | not applicable |
| find | one unit | `null` when `none` is offered |
| annotate | one field per question, holding one of the answers above or a failure marker | `null` |
| recognize | entities with `text`, `start`, `end`, `length`, `kind` and `strength`, plus relations | not applicable |
| relate | edges with `relation`, `source`, `target` and `probability` | not applicable |

### Errors

Every call fails with one of six kinds: `usage`, `backend`, `deadline`, `local`, `cancelled` and `defect`, numbered 1 to 6. Each failure also carries a retryable flag and a message. Every language shows the kind by name. The number is for the C ABI only.

### Three rules that hold everywhere

1. **`null` means not sure. It never means failed.** A failure is its own value: `{"failed": {"kind", "cause"}}` in JSON. A typed language gives it a separate type. `specification/annotate.md` line 112 states this rule.
2. **Score is a number.** Levels describe the bands. If a caller wants a band name back, a helper can map the number to it. The band name is never the answer.
3. **Descriptions are data.** They go into the prompt, and that is their job. A surface that drops them changes the answers.

## 3. What main has today (corrections to the first draft)

- **The schema already exists.** Ticket 0069 added `specification/question-file.schema.json` (Draft 2020-12). A parity corpus holds the parser and the schema to one verdict. What is missing is a schema for the *answers* and for the JSON door's request.
- **The C door already carries descriptions.** Every question crosses as `question_json` in the question-file grammar, so maps and structured descriptions pass unchanged. The ABI needs no description arrays and no result structs. Offsets count code points, as in Rust and Python.
- **Rust has typed labels.** The `Choice` trait and the `choices!` macro give `choose` and `tag` enums, and `Annotated` is a real union that includes `Failed`. The gap is that a `choices!` variant cannot carry a description.
- **TypeScript is typed well on the output side.** `AnnotatedField` includes `FailedField`, and `ThinkThenError.kind` is a union of the six names. The gap is the input side: `options`, `labels` and `levels` accept only `string[]`, so a TypeScript caller cannot send descriptions.
- **Python** ships `py.typed` and a full `.pyi` stub. It has no runtime dependencies and imports neither pandas nor Polars. It accepts a list or a dict for every label set. The gaps: it does not accept `Enum` or `Literal`, the stub types label sets as plain `str`, and `annotate` returns `dict[str, Any]`.
- **Ruby** passes a hash through, so descriptions work. **R** needs the same check.
- **The defect:** in a pandas or Polars frame, a failed `annotate` question widens its column to text (`libraries/python/README.md` line 27, and the Rust Polars door). A column's type then depends on whether the backend failed that day. A boolean column can turn into a text column mid-pipeline.

## 4. The rules (answers to the eight questions)

1. **Python stays standard-library only. Pydantic is an optional extra, as Ian suggested.** The core accepts `Enum` subclasses and `Literal[...]` as label sets with no dependency, and it reads descriptions from an enum member's docstring or from a `description` attribute. `pip install thinkthen[pydantic]` adds two things. First, the core accepts a Pydantic model or an `Annotated[..., Field(description=...)]` type as a label set or question set. Second, `thinkthen.pydantic.row_model(question_set)` builds a typed model for `annotate` rows, where each field is `bool | None`, `Literal[...] | None`, `float` or `list[...]`, or `Failed`. The package imports Pydantic only when a caller uses these.
2. **The typed layer gates integration, in part.** A port cannot land in `libraries/` without three things. It must name the outcome and the six error kinds, keep `null` apart from failure, and carry descriptions in at least the map form. Building label sets from enums can follow in the port's next ticket.
3. **When enum metadata and a map disagree, the explicit map wins for each label.** The enum supplies the labels and their default descriptions. A map entry replaces that label's description. A map that names a label the enum lacks is a usage error. The two are never merged silently.
4. **The C ABI stays JSON.** It already carries everything. Each port parses answers against the result schema from rule 5, so a port has one contract to parse and no ABI to grow.
5. **One contract, in the spec.** Add `specification/types.md`, which holds section 2 of this page. Add `specification/result.schema.json`, which covers the bare answers, the failure marker, details, and the door's request. Keep both hand-written, and hold them to the parser with a parity corpus, as 0069 does. Do not generate them from Rust: that adds a build step for no gain. Every library's conformance run checks its outputs against the corpus. A port's stage-two proof uses the same corpus.
6. **The SQL extensions return plain types: TEXT, BOOLEAN and DOUBLE.** The caller owns the enum. `specification/` gets one recipe per database for constraining an answer column: a `CHECK` or a `DOMAIN` in PostgreSQL, an `ENUM` cast in DuckDB, and a `CHECK` in SQLite. A DuckDB function that returns an ENUM depends on bind-time types, so it waits for 0201's C++ API and gets its own row then.
7. **DataFrame columns keep one type per question kind, always.** `decide` gives a nullable boolean, `choose` a string (categorical when the labels are known), `score` a float, and `tag` a list of strings. A failure goes in one companion column named `failed` per frame. Each cell of that column holds a map from question name to marker, or null when nothing failed. A question named `failed` is refused. This fixes the defect in section 3.
8. **The order:** J1 (the contract), then the frame defect, then each library's gaps, then the ports. Section 6 gives the tickets.

Two more rules the first draft did not state:

- **Offsets:** keep the type names. Each library's entity doc states its unit: code points in Rust, C and Python, UTF-16 in TypeScript, and one-based inclusive in R. One shared conformance case with a character outside the Basic Multilingual Plane (such as an emoji) pins each unit. Renaming types to `Utf16Span` adds noise, and the test already catches drift.
- **Structured descriptions:** every surface that accepts a description string also accepts the `{what, not_for, examples}` object and passes it unchanged.

## 5. Per-language idiom for label sets

Each form maps to the same ordered set of labels with optional descriptions.

- **Rust:** slices and pairs today. Add a description to `choices!`: `Billing => "billing": "Charges and refunds"`.
- **Python:** a list, a dict, an `Enum`, or a `Literal`. With the extra, also a Pydantic model or `Field`.
- **TypeScript:** `string[]`, `Record<string, Description>` (insertion order holds for string keys), or a `readonly` tuple of literals. JavaScript checks the same forms at run time.
- **Ruby:** an array or a hash. **R:** a character vector, or a named character vector.
- **Go:** `[]Label{{Name, Description}}` plus a `Labels(...string)` helper. A Go map loses order, so it is not accepted.
- **Java and Kotlin:** `List<String>`, `LinkedHashMap`, or any `Enum` with a `@Description` annotation. **Scala:** a `LabelSet[A]` given. **C#:** `string[]`, `IDictionary`, or an `Enum` with `[Description]`. **Swift:** `[String]`, `KeyValuePairs`, or a `CaseIterable` enum with a description. **PHP 8.1+:** a list, an ordered array, or a backed enum that implements a `Described` interface. **Zig:** a slice of `Label` structs, or an enum with a declared description table. **Ada:** a `Label_Set` built from vectors or from a discrete type. **COBOL:** a table of label and description groups with a count, and 88-levels for fixed sets.

## 6. Work for 0.1

| Id | Work | Component | Size |
|---|---|---|---|
| J1 | ADR and spec: `types.md`, `result.schema.json`, a parity corpus, and the offset case. Rules 1 to 8 recorded. | Pages / Contract | M |
| J2 | Frame columns keep their type, and failures go in the `failed` column (Python pandas and Polars, and the Rust Polars door). | Libraries | M |
| J3 | TypeScript accepts label maps and structured descriptions. | Libraries | S |
| J4 | Rust `choices!` carries descriptions. | Libraries | S |
| J5 | Python accepts `Enum` and `Literal`, the stub gains `Literal` kinds and a typed annotate row, and the `[pydantic]` extra is added. | Libraries | M |
| J6 | Check Ruby and R for maps, structured descriptions, the failure marker and named kinds. Fix what fails. | Libraries | S |
| J7 | A SQL recipe for constraining answer columns, one per database. | Databases / Pages | S |
| J8 | Each port integration ticket meets rule 2 and passes the J1 corpus. | Libraries | one per port |

## 7. Note to the language-port team

The core team has agreed on one type contract, and ticket J1 will write it as `specification/types.md` plus `specification/result.schema.json`. Until J1 lands, section 2 of this issue is the contract. Keep your stage-one plumbing: ownership, UTF-8, budgets and cancel. It is good. For stage two, map the contract into your language's idiom and add nothing of your own.

- Name the outcome (yes, no, not sure) and the six error kinds, and keep the retryable flag. Raw integers stay inside the binding.
- Keep `null` apart from failure. A failed `annotate` field is its own type or object, never null.
- Carry descriptions. Accept the map form at least, and pass structured descriptions unchanged. Section 5 gives your language's forms.
- Parse `annotate`, `recognize` and `relate` from the door's JSON against the result schema. Do not ask for new C structs. The ABI stays JSON by rule 4.
- Name your offset unit in the entity docs, and run the shared offset case.
- Integration into `libraries/` waits for J1 and requires rule 2. Enum-derived label sets can come in your next ticket.
- Keep working in your local experiments. Do not edit main. When the core blocks you, file an issue in `sdlc/issues/`. Cite experiments as "local experiment NNN".

*Sources: `specification/annotate.md` (lines 30, 48 and 112), `specification/question-file.schema.json`, ticket and record 0069, `libraries/c/include/thinkthen.h`, `libraries/python/thinkthen/__init__.pyi`, `libraries/python/README.md` line 27, `libraries/typescript/index.d.ts`, `libraries/ruby/lib/thinkthen.rb`, `crates/thinkthen/src/public/{choice,annotated,question}.rs`, and the experiment reports for 273, 274 and 289 to 295.*
