# 0278 Rust API inventory preparation

Source: `origin/main` `0b06da8c6` (the public source is unchanged from `2d940c606`). The existing integrated checkpoint at `target/codex-builds/integrated-lint-checkpoint/inventory.log` failed with 36 missing and 103 uncontracted declarations; its four mutation plants refused their changes. A bounded warm `cargo +nightly-2026-08-24 public-api --package thinkthen --no-default-features -ss` extraction at `target/codex-builds/0278-inventory/public-api.txt` reproduced the same canonical differences. This diagnostic only documented the Rust crate; it ran no tests, package builds or providers. The raw set has another 169 allowed implicit trait impls, which `inventory.problems` correctly excludes.

## Difference-to-authority map

| Difference | Count | Accepted authority and current source |
| --- | ---: | --- |
| Old eager `Result<T, Error>` signatures absent | 36 missing | [0212](../tickets/0212-rust-library-batching.md) §4 and [ADR 0089](../planning/adr/0089-rust-calls-carry-facts.md) select uniform `Call<T>` for nine verb families, Engine/root and ordinary/`_with`; `crates/thinkthen/src/public/{engine,mod}.rs`. |
| Matching `Result<Call<T>, Error>` signatures | 36 extra | Same 0212/0089 authority; preserve generic bounds, arguments and `where` clauses exactly. |
| Batch selectors, facts, observer declarations and three new bulk verbs | 62 extra | [0212](../tickets/0212-rust-library-batching.md) §§1, 3–6 and [ADR 0089](../planning/adr/0089-rust-calls-carry-facts.md) §§2–6; `public/{options,settings,engine,mod,batch,results/call,results/observation,error}.rs`. This includes the actual enum variant payloads and observer field types, not only type names. |
| Dynamic details bridge | 2 extra | [0230](../tickets/0230-c-json-batching-and-facts.md) §5 authorizes `Engine::details_many[_with]`, including the `Evidence + Serialize` bound; `public/engine.rs`. |
| Saved-profile accessor | 1 extra | [0203](../tickets/0203-calibration-profile-across-libraries.md) §3 authorizes `Details::profile_warning()` with `(tuned_for, running)` order; `public/results.rs`. |
| Explicit CA builder method | 1 extra | [0211](../tickets/0211-private-tls-roots.md) §1 authorizes `EngineBuilder::ca_bundle`; `public/settings.rs`. |
| Choice metadata trait method | 1 extra | [0243](../tickets/0243-rust-choice-descriptions.md) authorizes `Choice::description` with the manual-implementation default; `public/choice.rs`. |

Total: 36 missing and 103 extra. No other missing declaration, unauthorized export or suspicious `Clone`/`Copy` appeared in the selected no-default-features comparison. This is a scoped observation, not an approval of every feature-enabled Rust surface. Historical [0084](../tickets/0084-freeze-the-public-rust-contract.md), [0095](../tickets/0095-add-the-binding-members-to-the-rust-contract.md), [0147](../tickets/0147-recognize-adr.md) and [ADR 0017](../planning/adr/0017-libraries-over-one-bound-core.md) retain their original accepted meanings and checker transformations.

## Builder handoff

Claim `sdlc/scripts/inventory`, this ticket and this record. Add the ticket's separately reviewed retired/added blocks as the final normative delta. The checker should require 36 retired entries present in the old contract and 103 added entries absent from it, subtract then add, and compare the **actual** built listing. The two static ticket blocks are independent of that listing; do not populate them at runtime from `built()`. Retain the old implicit-trait allowances, prohibited builder/error `Clone`/`Copy`, public `Debug` requirement, and four mutation plants. A tiny parser proof should cover a tuple enum variant, a struct variant field, `const fn`, and a nested `Call<Vec<_>>` return, so a dropped declaration cannot make a false green.

After the checker change, run its real no-default-feature extraction, exact declaration comparison and four plants; focused policy, page/ticket, format and diff checks. No package or all-port campaign is needed. If the independent review disputes any listed addition, stop before implementing it and return the exact source/authority mismatch for a coordinator ruling. Existing executable docs or historical tickets are not implementation completion evidence.

## What preparation taught us

The checkpoint's 103 are policy-filtered findings, not the 272 raw set differences; the latter include 169 allowed implicit trait impls. The two post-0212 `details_many` methods and three single-method additions came from later accepted tickets, so a 0212-only update would still fail. The script really uses `--no-default-features`, correcting the earlier intake's description of a default-feature inventory. Its existing parser recognizes methods, enum payloads and fields, but the new independent delta source and a parser-edge proof need explicit treatment in the build.
