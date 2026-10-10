# 0527: Make Rust and Rust Polars thin and first-class

Status: COMPLETE.

Milestone: 0.2

Depends on: 0511
Depends on: 0513

Reviews: revision a087f6dc3, accept

Reviews: revision 829ffbd655bfa88202b57031823b2edb4ea31836, accept

Reviews: revision 4255c4ab84a30d82b94b246eaa0324d5baa0cdaa, accept

Reviews: revision ede92df4e1f92f508a398db7d176c5feb75a14f7, accept

Reviews: revision a598f47447532f9cb0f23abafdd310835e797680, accept

Reviews: revision 1bce8899d87d7b230e6efa3f23153a884c04659f, accept

Reviews: revision ac40d46ab5244a1cfff128d101da92a67a025395, accept

Reviews: revision bb869d4a7bdb51aa4fe8d75119c579038f0d7737, accept

Reviews: revision 1b566d22367686db993dfcff475bc8186ef87e49, accept

Reviews: revision 76b153289292d50474becb86cc7f55049579efde, accept

Landed: fb35e20

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

## Progress

- 2026-10-10 landed 0399d25ec; next: All ten basic named Polars paths use native Requests. Typed score complete conversion is in review; remaining typed complete groups, bounded pull methods and installed consumers stay open.
- 2026-10-10 landed eb0124b21; next: Typed complete score calls are landed through native Requests. Complete choose/tag groups are in progress; other typed groups, bounded pull methods and installed consumers stay open.
- 2026-10-10 landed 40b8fcfa3; next: Typed complete score, choose and tag calls are landed with owned originals and nullable positions preserved. Finish the other complete groups, bounded pull adoption and installed consumers.
- 2026-10-10 landed 794ca5563; next: Typed complete decide, choose, score, tag, annotate and recognize use native Requests with original ownership preserved. Finish whole-set typed collections, explicit probability projection, bounded pull adoption and installed consumers.
- 2026-10-10 landed 3df0957cb; next: Typed complete rank and rank-set preserve original ownership after sorting and cutoff selection. Finish typed filter/find/relate, bounded pull adoption and installed consumers.
- 2026-10-10 landed 1372c7342; next: Typed complete filter preserves passing and rejected observations with original ownership. Finish typed find and relate, bounded pull adoption and installed qualification; release-only checks remain held.
- 2026-10-10 landed b055b9656; next: All typed complete Polars functions use native Requests, including aggregate find and relate with owned originals. Finish bounded pull adoption and installed qualification; compatibility retirement and release-only checks remain held.
- 2026-10-10 landed d42b1593f; next: All named, typed complete and borrowed row Polars methods use shared Request admission. Focused ownership and lifecycle checks pass; final installed qualification and compatibility retirement remain held.
