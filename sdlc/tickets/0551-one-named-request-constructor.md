# 0551: Build every named call through one Rust constructor

Status: OPEN.

Milestone: 0.2

Reviews: revision f23c075965dc12a966847fee80c13bfab3b29385, accept

Reviews: revision a758c55ced8ba8c07bcf4e6b46c32619afcca5e0, accept

## Outcome

One Rust constructor turns a function name, a question, an input and options into a `Request`. No binding or database extension has its own mapping from function names to calls or input kinds. No binding writes the request envelope by hand.

## Evidence

- Starts from: the [2026-10-11 adversarial review](../records/2026-10-11-0-2-adversarial-review.md), findings A1 and B9.
  - `RequestCall` is an enum of ten variants that all carry `RequestArguments` (`crates/thinkthen/src/public/request.rs:232`).
  - The name-to-call mapping is copied in these places:
    - `typescript/src/request_node.rs:102`
    - `python/src/inputs.rs:290`
    - `ruby/src/ffi/request.rs:159`
    - `libraries/r/thinkthen/src/rust/src/ffi/request/inputs.rs:118`
    - `c/src/call/legacy.rs:83`
    - `databases/sqlite/src/complete/selector.rs:30`
    - the PostgreSQL bridge
  - DuckDB picks the call from the kind of question (`databases/duckdb/bridge/src/ffi/request.rs:60-77`).
  - JVM (`Engine.java:134`) and C++ (`client.hpp:84`) write the envelope JSON themselves. The JVM also keeps a hand-written `Json.java`.
- Keeps: the wire request format, every installed case, and every public call name in C# and the JVM.
- Changes: one Rust constructor replaces the copies.
  - Add `RequestFunction: FromStr` and one constructor that picks the input kind from the function.
  - Add a second constructor that takes a question kind, and route DuckDB through it.
  - Delete each copy of the mapping.
  - Have the JVM build the envelope with the generated `Inputs.Request` (`Inputs.java:327`). C++ uses its generated input types, as C# does (`csharp/src/OwnedCall.cs:82`). Delete `Json.java` once the generated readers cover it.
  - Collapse the 20 near-identical call methods in C# `OwnedCall.cs:83` and JVM `Engine.java:114` where the language allows it.
- Proof: a search finds no function-name or question-kind match outside the Rust core. Routine installed checks pass for every touched language.
- Defers: nothing.

PM amendment, 2026-10-11. The proof bans duplicated named-request construction. No code outside the Rust core maps a function name or question kind to a `RequestCall`, `RequestInput` or input kind, or builds either type by hand. Three kinds of mapping may stay:

- DuckDB's mappings to its compact ABI tags.
- The SQLite and PostgreSQL mappings that select settings or check a question's kind, such as `databases/postgresql/src/forms.rs`. Idea "SQL extensions take settings selection and question-kind checks from the core" carries them.
- `Json.java`, as the generic byte and value codec that the generated JVM inputs and results use. It holds no request shape. Its deletion clause in Changes no longer applies.

## Progress

- 2026-10-11 started
- 2026-10-11 next: Request constructor mappings are migrated, while the PM must reconcile the literal search proof with retained question-kind mappings used only for settings selection and ABI tags.
- 2026-10-11 next: Final request construction code passed fresh review under the approved mapping exceptions and awaits the 0549 landing before its accepted core slice and whole-ticket checks.
- 2026-10-11 next: Fresh source search found PHP still selecting input kinds and Call classes by function; checkpoint fa83d9035 removes both through shared construction with the R decoder dependency, but core measures 187637 against unchanged 187611 and PHP measures 1916 against unchanged 1902, so fresh code review is running while compiler and installed checks remain held behind the canonical campaign.
