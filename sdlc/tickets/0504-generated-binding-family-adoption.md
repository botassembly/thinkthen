# 0504: Move language families onto generated binding contracts

Status: OPEN.

Milestone: 0.2

Depends on: 0516

Reviews: revision 95fade3863f777142ecd49d4abca0efcf1790cce, accept

Reviews: revision c79e3f65e05eb4f12fa46d8f7ab003d477258c9c, accept

Reviews: revision 4d59e7c6a, accept

## Outcome

Migrate every remaining language family through generated request/result types and the selected session interface while preserving named typed calls.

## Evidence

- Starts from: PM architecture asks5/6 and0502 experiment; manual carrier copies cause recurring drift.
- Keeps: All languages, files/images, failure facts, cache/replay and installed package behavior.
- Changes: After0491/0492/0502/0503, land independently reviewed family slices for C#/Java/Kotlin/Scala, Dart/Flutter, Go/C++, Swift/Objective-C/PHP and Rust/dataframe consumers not owned by0496. Claim `libraries/csharp/**`, `libraries/jvm/**`, `libraries/dart/**`, `libraries/flutter/**`, `libraries/go/**`, `libraries/cpp/**`, `libraries/swift/**`, `libraries/objective-c/**`, `libraries/php/**`, `libraries/rust/**` and `libraries/polars/**`. Narrow each slice's ownership before parallel work. Each slice removes duplicated readers only after parity; no per-language paperwork beyond the shared suite.
- Proof: Run complete shared conformance through installed typed public interfaces, including absent/null/failures, unknown output fields, located files, images and cancellation. A not-run required host case is a failure, with supported-platform rulings explicit.
- Defers: Proxy and dropping languages. Size: large rollout; each independent family is a medium or large reviewed slice.

## Revised code-reduction estimate

## 2026-10-09 amendment

The thin, first-class ruling supersedes the layout-only reader fallback and its estimate below. Depend on 0516's measured C# pilot, 0511 admission, 0503 session, 0513 generated results and 0514 guide. Each independently reviewed family implements async, cancellation, cleanup and naming idiom while deleting duplicated rules and readers. Objective-C follows 0518's Foundation target. Direct Python/dataframe ownership stays with 0496. Narrow each family's actual paths before parallel work, retain installed typed parity and measure actual code removed rather than promising a line target.

### Earlier fallback estimate

0502 selects generated native layouts, not schema-generated semantic readers. Withdraw the study's 25,000–35,000-line removal estimate for this rollout. The identifiable C# and Dart layout files contain 1,778 nonblank lines: 884 in `libraries/csharp/src/NativeAbi*.cs` and 894 in `libraries/dart/lib/src/native/abi.dart`. Estimate at most about 1,800 hand-maintained lines becoming generated in these two hosts; generated declarations remain in the package, so this does not imply a net deletion of 1,800 lines. Imports and generator support reduce the maintenance saving.

The experiment's 127 C# reader lines and 1,565 Dart reader lines remain owned-copy logic unless a parity-preserving replacement is demonstrated. PHP reads the header directly; Python stays directly on Rust. Other families may remove duplicate admission and layout code, but the experiment did not measure that scope. Count actual removed hand-written code in each family landing rather than crediting speculative reader deletion. This estimate comes from the named layout files at main `26fc8c900` and the retained 0502 reader inventory.

## Surface assessment amendment

0516 owns the C# implementation and first packaged pilot; this ticket consumes it rather than building C# again. 0518 owns Objective-C. Each remaining family owns its target templates and generated outputs using 0513's common graph, plus native input conversion, scheduling and ownership. Apply all ten guide items through the installed public API. Carry the pilot's distinction between caller cancellation and native settlement into each host; prove scheduler progress and prompt cancellation with the existing held-provider fixture where the host promises async. Do not add an async runtime to synchronous hosts.

Settle 0517's bounded package design before the affected family slice. The JVM slice owns migration from preview to stable foreign-function APIs and declares the supported JDK floor before coding. Java, Kotlin and Scala share transport and ownership while retaining their native public types and scheduling. Packaging later consumes that same runtime contract. Dart/Flutter share one implementation; the real Flutter paths are under `libraries/dart/flutter/`, not `libraries/flutter/`.

Rust and Rust Polars stay direct to Rust and do not acquire a JSON session wrapper. The Rust Polars implementation is `crates/thinkthen/src/public/frame.rs` and `crates/thinkthen/src/public/frame/`; the folders under `libraries/` hold consumers. Its slice must claim those actual implementation paths, public exports, affected native tests and installed consumer files before editing. Preserve column types, null-to-row mapping, original indices and complete-set rank/find. Python pandas and Python Polars stay with 0496.
