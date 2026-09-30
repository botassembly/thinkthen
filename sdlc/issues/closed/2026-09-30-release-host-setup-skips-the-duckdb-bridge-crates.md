Status: Closed by the quick fix landed as `Land quick fix: release builds clear host setup, apt, offline crates, and library tests`. Found by the second release rehearsal, run 36780048676, on 2026-09-30. Owner: ticket 0128 Phase 3b. Resolution: `host-setup` fetches `databases/duckdb/bridge/Cargo.toml`. `sdlc/scripts/workflows` requires every tracked `Cargo.lock` to have its manifest in the host-setup fetch list, or to appear in `UNFETCHED_LOCKS` with the reason no release build reads it. Four lock files sit in that table: the conformance consumer, the R package's crate, the Rust examples, and one probe. A self-test case drops the bridge from the list and requires the finding.

Kind: bug

Pay when: before the next rehearsal dispatch.

Keeping it stops all four `build` jobs when they reach the DuckDB extension, so no loadable file is built and Debt 026 cannot be checked.

# Release host setup does not fetch the DuckDB bridge's crates

## The problem

The aarch64 Linux `build` job failed inside `sdlc/scripts/release-container` after Ruby built:

```
error: failed to download `cc v1.5.1`
Caused by:
  attempting to make an HTTP request, but --offline was specified
```

The container builds offline from the runner's crate registry, which `host-setup` fills with `cargo fetch --locked` for eight manifests (`release-workflow`, lines 224 to 227). The list omits `databases/duckdb/bridge/Cargo.toml`. That lock file alone pins `cc` 1.5.1; every other lock file pins 1.4.7. The x86-64 Linux job would have failed the same way, but it stopped earlier on the apt simulation note. The macOS jobs reach it too: `release-pack` builds DuckDB through `databases/duckdb/cpp/build.sh`, whose line 47 runs `cargo build --locked --offline` on the bridge. They stopped earlier on the uv check. Local builds pass because a developer's registry already holds `cc` 1.5.1.

## A fix

Add `databases/duckdb/bridge/Cargo.toml` to the fetch list. Add a check to `sdlc/scripts/workflows` that every `Cargo.lock` a release build reads has its manifest in that list, so a new lock file cannot be missed again.
