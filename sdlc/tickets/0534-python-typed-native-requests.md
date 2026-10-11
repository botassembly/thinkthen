# 0534: Give Python typed native requests and Rust admission

Status: OPEN.

Milestone: 0.2

## Outcome

Python calls hand native Rust request values to the engine. Python code no longer writes the request grammar, no longer checks rules that Rust admission owns, and no longer accepts untyped extra keyword arguments. Python file reading passes typed arguments to the native reader.

## Evidence

- Starts from: gaps 1 and 3 of [the 0.2 closure review](../records/2026-10-11-0-2-closure-review.md). `thinkthen/_calls.py:210` and `thinkthen/__init__.py:145-149` assemble `{schema, call}` and pass it as JSON. `__init__.py:149-299` raises host `UsageError`s and accepts `**legacy` on rank, find, annotate, recognize and relate. `src/files.rs:6,44` parses a JSON selection with the parser in `thinkthen_host::source` before it calls the native reader. [The binding guide](../../libraries/BINDING-AUTHOR.md) says native callers construct typed Request values without a JSON round trip. [The thin-binding ruling](../decisions/2026-10-09-thin-first-class-bindings.md) puts every rule in Rust.
- Keeps: every named call, keyword, result type, error class, cancellation and iterator behavior that 0496 qualified. The pandas and Polars frame calls keep their current behavior.
- Changes: The PyO3 layer in `libraries/python/src/` converts Python values into the public Rust Request types. Delete the host request assembly and the host admission checks that Rust repeats. Remove `**legacy` and list each removed keyword in the README's old-to-new table. Pass paths, unit and window as typed arguments to `thinkthen::read_files`, and delete the JSON parse in `src/files.rs`. Lower the Python ceiling.
- Proof: the routine installed Python checks pass. One installed case shows that an invalid input raises the same typed error with the same message that Rust admission gives. One case shows that a removed keyword raises `TypeError`.
- Defers: nothing. The C library keeps the `thinkthen_host::source` parser.
