# 0715: Delete the PostgreSQL removed-function stubs

Status: OPEN.

Milestone: 0.2.1

Phase: 3

Depends on: 0688

## Outcome

- Changes: `databases/postgresql/src/removed.rs`, `databases/postgresql/src/lib.rs`, `databases/postgresql/check.sh`, `databases/postgresql/tests/function_descriptions.py`.
- `databases/postgresql/src/removed.rs` and its `mod removed;` line in `src/lib.rs` are deleted. The extension no longer defines `thinkthen_warm`, `thinkthen_probability`, or the old `thinkthen_decide`, `thinkthen_choose`, `thinkthen_score` and `thinkthen_tag` overloads that only refuse.
- The `removed_forms_refuse_before_send` check in `check.sh` and the removed-overload assertion in `tests/function_descriptions.py` are deleted.
- Calling any of those signatures gives PostgreSQL's own "function does not exist" error.

### Proof

- At the 0619 container checkpoint, in the arm64 container on the M5: `sh databases/postgresql/check.sh 16`, `17` and `18` pass, and `SELECT thinkthen_warm('a', 'b')` fails with `function thinkthen_warm(unknown, unknown) does not exist`. Before the change it fails with `thinkthen_warm was removed; pack records with thinkthen_decide_many`.

## Evidence

- Split from 0644, which keeps the DuckDB stubs and workspace. 0714 takes SQLite. The three land in any order.
- `databases/postgresql/src/removed.rs` holds "plain refusals for pre-0.1 forms replaced by named and keyed calls", each declared through `crate::descriptions::describe!`. `databases/postgresql/check.sh` asserts their sentences in `removed_forms_refuse_before_send`, and `tests/function_descriptions.py` asserts the description of `thinkthen_decide(text,text[])`.
- That is backward-compatibility code, which the 2026-10-12 no-compatibility-before-1.0 ruling forbids.
- The pinned PostgreSQL servers run in the Linux container, so the installed check runs at the 0619 checkpoint and not natively on the M5. 0647 also rewrites `check.sh`; whichever lands second rebases on the first.
