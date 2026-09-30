# Writing a thinkthen binding

This guide is for people who write or change a language binding under `libraries/`. It states the rules of ADR 0112 (Rust owns the result schema) and ticket 0291 (the remaining language doors). The specification is the contract; this page points at it.

## Rust owns every type

The engine serializes every result. `specification/result.schema.json` is generated from those Rust types by `crates/thinkthen/src/schema_tests.rs`, and nobody edits it by hand. A binding reads each result, facts object, detail row and plan into the host's ordinary JSON value: a dict, map, hash, list, `JsonNode` or equivalent. A reader ignores a member it does not know, per `specification/result.md`.

A binding adds exactly three things:

1. **Named outcomes.** `YES`, `NO` and `UNSURE` with the header's values 1, 0 and 2, in the host's enum or constant idiom. JSON `true`, `false` and `null` map to them.
2. **Named error kinds.** `usage`, `backend`, `deadline`, `local`, `cancelled` and `defect`, with the C codes 1 to 6, in the host's enum, exception class or constant idiom. The retryable flag and the message ride along.
3. **Null versus failure.** One helper reads an annotate answer as unresolved, a value, or a failure with its `kind` and `cause`. A failure is always the one-member object `{"failed": {...}}`; no answered value is an object. A static language returns a three-case type. A dynamic language returns the JSON value and offers `failed(member)`, which returns the failure object or nothing. No binding turns a failure into null. The helper reads only the answer names of a row's `value` and `answers`, never the record members.

Everything else stays host JSON. Do not add typed facts classes, typed entity or edge classes, closed key sets or strict validators. They break the compatibility rule, and each new member then needs a release of every binding.

## Calls and settings

- Parse settings once, in the core. The C door takes engine settings through `thinkthen_engine_new_with` (for example `max_requests_total`) and per-call settings through the portable `thinkthen.settings/1` object. A binding passes the host's values through as JSON and never re-implements the grammar.
- Convert host values to C at the edge only: strings as NUL-terminated UTF-8, texts as pointer and byte-length arrays, the budget as `int64_t` milliseconds with `THINKTHEN_NO_DEADLINE` for none. A computed budget clamps at zero.
- `deadline_ms` names the budget in the host's idiom on every call that sends.
- `find` returns the door's value unchanged: `{"index", "unit", "probability"}` for the selected unit, or null. The native libraries return the same three members.
- Score and tag answers carry no probability. A binding that offers a probability option on decide or choose refuses it on score and tag with the usage kind before anything is sent.
- `plan` previews a judgment call through `thinkthen_plan_json` (or the crate's `Engine::plan_with` for a Rust-native binding) and returns the result schema's `plan` object as host JSON. It needs no key, reads no cache and sends nothing. It is never a second send and never a mutable last-result slot.

## Proof

- **One shared corpus.** `specification/fixtures/types/` holds the result corpus and `conformance/cases.json` the behavior corpus. A binding keeps one host conversion table for them and adds no private copy of a case.
- **Zero sends are counted.** Prove that a plan or a refusal sends nothing by counting requests at a loopback listener, never by reading a dry-run flag.
- **Minimal child environment.** Run each test program with a cleared environment: the loopback address, a fresh cache folder and, only when the case sends, a fake key. A plan case runs with no key at all.
- **Source and installed receipts stay separate.** A check against the source tree and a check against an unpacked release archive are two results, each named.
- **Exact members and exports.** A new C symbol goes into `libraries/c/include/thinkthen.h` first. `libraries/c/tests/door/main.rs` and `sdlc/scripts/check-c-exports.py` derive the export list from the header, so the shared and static libraries must export exactly the header's names. Each binding that declares symbols by hand (C#, PHP, JVM, Dart, Ada, COBOL) updates its declarations in the same change, and each package validator's member list is checked, not assumed.
