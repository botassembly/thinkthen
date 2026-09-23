# 0055: Fold to one crate through the private engine

Date: 2026-09-21

Status: landed

## Result

The workspace now contains one package named `thinkthen`. Its private `core`, `engine`, and `cli` modules follow one inward dependency direction. The core keeps pure judgment rules. The engine owns both scheduling state machines, scoped request workers, transport, retries, recording, and cache locks. The command keeps arguments, credentials, framing, its detached reader, output, messages, and exit codes.

The binary requires the default `cli` feature. A build without default features produces the library without `clap` or `csv-core`. Packaged builds need no sibling crate. Library releases unwind panics, while the command release keeps abort behavior. The package exposes no new normal public API.

The command now runs all shared conformance cases through the private engine. Successful cases check their complete expected results. All six fault kinds cross the same boundary. The production scheduler test proves that no engine worker survives its call. Requests, recordings, output, messages, exit codes, ordering, concurrency bounds, and user pages did not change.

The pure-core checks retain the complete accepted Clippy ban table and exact dependency roots. A token-based source check catches reverse references and aliases through Rust use trees, comments, raw identifiers, direct paths, external-crate declarations, and outer wildcard imports. The checker carries hostile plants and allowed controls.

## Review and proof

Independent design review rejected the first ticket because it omitted the shared conformance runner. The corrected design was accepted.

Independent code review rejected the first implementation for five gaps in engine ownership and proof. Later passes found import spellings that evaded the core boundary check. The repairs moved scheduling state into the engine, restored the accepted policy, strengthened conformance and lifetime tests, restored behavioral doctests, and replaced spelling-based import matching with token and use-tree checks. The same reviewer accepted the final implementation and confirmed that the earlier findings stayed closed.

With the key and base-address variables unset, `install`, `lint`, `test`, and `spec` all exited zero. The test rung passed 172 library tests, 210 backend tests, the edge and question-file suites, two doctests, policy and transform checks, and local-listener integration tests. The spec rung passed 26 specification checks, seven transform checks, every recorded replay, and all nineteen green how-tos. The package proof built a standalone generated crate with and without command features and checked both panic strategies. The exact Rust ceiling is 26,012 nonblank lines. No network or paid call ran.
