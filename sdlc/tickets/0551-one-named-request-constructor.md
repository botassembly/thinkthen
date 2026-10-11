# 0551: Build every named call through one Rust constructor

Status: OPEN.

Milestone: 0.2

## Outcome

One Rust constructor turns a function name, a question, an input and options into a `Request`. No binding or database extension has its own mapping from function names to calls or input kinds. No binding writes the request envelope by hand.

## Evidence

- Starts from: the [2026-10-11 adversarial review](../records/2026-10-11-0-2-adversarial-review.md), findings A1 and B9.
  - `RequestCall` is an enum of ten variants that all carry `RequestArguments` (`crates/thinkthen/src/public/request.rs:232`).
  - The name-to-call mapping is copied in these places:
    - `typescript/src/request_node.rs:102`
    - `python/src/inputs.rs:290`
    - `ruby/src/ffi/request.rs:159`
    - `r/.../ffi/request/inputs.rs:118`
    - `c/src/call/legacy.rs:83`
    - `databases/duckdb/bridge/src/ffi/request.rs:60`
    - the SQLite and PostgreSQL bridges
  - JVM (`Engine.java:134`) and C++ (`client.hpp:84`) write the envelope JSON themselves. The JVM also keeps a hand-written `Json.java`.
- Keeps: the wire request format and every installed case.
- Changes:
  - Add `RequestFunction: FromStr` and one constructor that picks the input kind from the function.
  - Delete each copy of the mapping.
  - Have JVM and C++ use the generated input types, as C# does (`csharp/src/OwnedCall.cs:82`). Delete `Json.java` once the generated readers cover it.
  - Collapse the 20 near-identical call methods in C# `OwnedCall.cs:83` and JVM `Engine.java:114` where the language allows it.
- Proof: a search finds no function-name match outside the Rust core. Routine installed checks pass for every touched language.
- Defers: nothing.
