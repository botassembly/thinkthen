# 0540: Remove repeated Rust request code

Status: OPEN.

Milestone: 0.2

Depends on: 0539

## Outcome

Each request rule has one owner in Rust. Results unpack through one accessor per kind. Source and feed framing share one decoder and one validator. Modules change each other's state only through methods.

## Evidence

- Starts from: gap 7 of [the 0.2 closure review](../records/2026-10-11-0-2-closure-review.md).
- Why 0.2: Ian's 2026-10-09 ruling asks for the least code, with every rule in one place in Rust. Each repeat is a second place a rule can drift.
- Keeps: every public behavior, error message and exit code. This ticket changes structure only.
- Changes:
  - Add one accessor per result kind on `RequestValue` and on `pull::Rows`. Replace the 27 hand-written "returned another result kind" checks, starting with `public/frame/typed.rs:84-416`.
  - Merge `public/request/source.rs:90-114` and `public/request/framing.rs:53-85` into one framing decoder with one admission validator and one wording.
  - Give `table::Rows` a reset method, and stop `table/framed.rs:26-30` from writing its fields.
  - Write the stop-at-first-error iterator once (`public/request/source.rs:78,132,173`, `public/request/transport.rs:159`, `cli/asking/judged.rs:91`). Decide which functions stream in one table (`public/request/execution.rs:549-575`, `public/pull.rs:80-87`). Carry rank's error mapping in the definitions table (`cli/check/functions/plans.rs:33`).
  - Lower the Rust source ceiling.
- Proof: the existing outside-in tests pass unchanged. No new test is needed unless a merge changes a message, in which case the test pins the kept message.
- Defers: nothing.
