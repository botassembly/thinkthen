# 0298 — Rust Polars lazy expressions (F7)

Status: Done. Final High review accepted `365a7c92a4957ff7d2766c90ae00d67273710210`; selected source-consumer proof is complete. Full package and release qualification remain separate. ADR 0107 settles the outcome.

## Outcome

Add decide_expr/choose_expr/score_expr/tag_expr over pinned Polars Rust UDF API. Decide/choose probability Struct only; pass shared core tally, token and per-morsel deadline at build. catch_unwind maps panic to a compute error. Lead with cheap filters then with_columns-shaped judgment and state the filter/head pushdown hazard.

The expression deadline applies to each morsel, not the whole query. RSS/load/churn is opt-in.

## Prerequisites and proposed files

T7 is landed. The Rust implementation is `crates/thinkthen/src/public/frame.rs` and cohesive `frame/` modules, with source tests in `crates/thinkthen/tests/polars/`. `libraries/polars/` contains its README and check script only. The pinned 0.55.2 Cargo feature, root lock, Polars-only exports, selected specification, and measured root ratchet are claimed for this build. The three old package criteria are in `cases.rs`, `deadline.rs`, and `throttle_equality.rs`; default-Max body proof remains separate in `door/batching.rs`.

## Smallest meaningful proof

Lazy/eager parity, score/tag refusal, token stops later morsels, tallies and judged rows match listener. Count exact accepted loopback request bodies and pin exit codes and refusal sentences where applicable; record source and installed-host receipts separately. Run only the affected functional cases, measured ratchets, focused format/policy/pages/tickets/diff. [Shared proof routes](../records/2026-09-29-sql-frame-redesign-proofs.md) name optional stress and later package qualification.

## Evidence

- Starts from: Main `869710193`, accepted ADR 0107, experiment 2038 HANDOFF and saved spikes, and the [preparation](../records/2026-09-29-sql-frame-redesign-preparation.md).
- Keeps: Existing successful values, owned facts, NULL/not-sure, cache identity, six error kinds, offline replay and privacy except the accepted changes.
- Changes: Rust Polars lazy expression family with tally, cancellation and panic boundary.
- Proof: Lazy/eager parity, score/tag refusal, token stops later morsels, tallies and judged rows match listener. Use the [independent corpus](../records/2026-09-29-sql-frame-redesign-corpus.md) and captured wire bodies.
- Defers: Unrelated package/release qualification, provider work, marketing site, token cap and per-record cache; named prerequisites remain.

## What the build taught us

The code is in `public/frame.rs` and its new `frame/lazy.rs`; there is no `libraries/polars/src`. The pinned `polars/lazy` and `polars/streaming` features resolve 166 additional locked packages offline. The root feature assertion had to name both edges, while the policy's all-feature metadata scan needed three named license associations. `cargo deny` already allows BSD for the two zstd crates; a zlib-rs exception in `deny.toml` produced an unused-exception warning, so it was removed. The DuckDB planted extra-license name was changed because zlib-rs is now part of the approved metadata graph; the plant still refuses an unclaimed entry.

Fresh High review found that the first streaming proof assumed a scheduler cut: with one Polars worker, the three-cell frame made one packed request rather than two singleton requests. The corrected streaming expression explicitly selects batch one, so its two non-null rows produce the same two literal singleton bodies whether Polars evaluates one or two morsels; the separate ordinary lazy/eager default-Max witness stays packed. One- and two-worker focused runs pass with exact listener/tally counts. The original held-throttle fixture tried one three-record call: its lazy input scheduler delivered only one held request. Three distinct singleton calls started at a barrier establish the configured two-slot bound with two held arrivals and the third after release, without a timing band. The corrected `cases.rs` selects batch one only for the saved singleton recordings. The deadline fixture uses three held singleton score rows and verifies the deadline kind, two sends, and no third request. The 200-row equality case stays ignored for stress.

The new expression uses an owned engine, question and controls; every evaluated morsel receives a fresh call deadline and shared token/tally. The closure catches a panic as a compute error. No core accounting or eager call implementation was copied. The `frame.rs`/`frame/lazy.rs` split keeps both under 500 nonblank lines. The [build record](../records/0298-rust-polars-lazy-expressions-build.md) holds the exact focused receipts and deferred qualification.
