# 0527: Make Rust and Rust Polars thin and first-class

Status: OPEN.

Milestone: 0.2

Depends on: 0511
Depends on: 0513

Reviews: revision a087f6dc3, accept

Reviews: revision 829ffbd655bfa88202b57031823b2edb4ea31836, accept

## Outcome

A Rust caller uses one named API with typed Rust values and Rust-owned complete results. A Rust Polars caller passes native columns and gets native columns back, with column types, nulls and original row positions intact. Both call Rust directly and gain no JSON session wrapper.

## Evidence

- Starts from: the [2026-10-09 binding decision](../decisions/2026-10-09-thin-first-class-bindings.md) and the [0521 assessment](../records/0521-surface-contract-assessment.md). The Rust Polars implementation lives in `crates/thinkthen/src/public/frame.rs` and `crates/thinkthen/src/public/frame/`, outside the folders earlier tickets claimed. `libraries/rust/` and `libraries/polars/` hold consumers. This ticket splits Rust and Rust Polars out of 0504.
- Keeps: Column types, the mapping from nulls to rows, original indices, and whole-set rank and find. All ten functions and their input, result, error, cache and replay behavior. Missing stays distinct from null.
- Changes: Meet the caller acceptance and the Rust and Rust Polars sections of `../../libraries/BINDING-AUTHOR.md`. This ticket owns:
  - admission of typed Rust values and Polars columns through 0511's shared Request, with no restated rule;
  - Rust-owned complete results from 0513's common graph, with no JSON result reader;
  - typed `Result` and `Error` values carrying Rust error kinds and facts;
  - native cancellation with no required async runtime, and scoped cleanup through Rust ownership;
  - rustdoc for the public calls;
  - the consumer READMEs, with a short old-to-new call mapping;
  - removal of old public names after installed parity.
  One public API is one coherent family of named typed calls. Claim `crates/thinkthen/src/public/frame.rs`, `crates/thinkthen/src/public/frame/**`, the public exports they touch, the affected native tests, `libraries/rust/**` and `libraries/polars/**`. Narrow to the actual files per slice before editing.
- Proof: The full shared cases run through the existing `libraries/rust` and `libraries/polars` consumers, with and without the Polars feature. Polars cases check column types, null rows, original indices and complete-set rank and find, and invalid input with zero sends. Record handwritten code removed and added in the landing record.
- Defers: Python pandas and Python Polars belong to 0496. The proxy needs no 0.2 ticket.
