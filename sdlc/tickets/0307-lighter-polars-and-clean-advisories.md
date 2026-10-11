# 0307: Lighter Polars and clean advisories

Status: COMPLETE.

Opened as: 2026-10-11. Lane claude-3. Plan: `sdlc/planning/cleanup-2026-09-30.md`, step 6.

## Outcome

`cargo deny check` passes offline on main. The optional `polars` feature pulls in only what the lazy and streaming door needs. The dependency count with the feature drops well below today's 362 crates.

## Evidence

- Starts from: `lint` on main `574ef900d` fails `cargo deny` advisories: RUSTSEC-2025-0141 (bincode unmaintained), RUSTSEC-2026-0194 and RUSTSEC-2026-0195 (quick-xml), all through the Polars feature landed in 0298.
- Keeps: the Polars eager and lazy door and its tests.
- Changes: Polars features and `deny.toml`; an ignore only with a stated reason when no feature removes the crate.
- Proof: `cargo deny check` passes; `cargo tree -e normal --features polars | sort -u | wc -l` before and after; Polars tests pass.
- Defers: nothing.

## What the build taught us

- Both advisories came through `polars/streaming`. `polars-stream` always turns on `polars-io/file_cache`, and `file_cache` turns on `cloud`. `cloud` brings `object_store` with its `http` feature, and `object_store` needs `quick-xml` 0.39. `cloud` also turns on `polars-io/serde`, which turns on `polars-utils/serde` and so `bincode`. No Polars feature below `streaming` avoids that chain. The patched `quick-xml` 0.41 is cached, but `object_store` 0.13.2 pins 0.39.
- The door uses no streaming code. Its expressions are plain column functions, and lazy `collect()` runs them on Polars' in-memory engine. The `polars` feature now selects `polars/lazy` alone. A consumer who wants the streaming engine enables `polars/streaming` in their own manifest; `libraries/polars/README.md` says so.
- Crates with the feature, `cargo tree -e normal --features polars | sort -u | wc -l`: 362 before, 220 after. The root lock went from 355 to 244 packages.
- `cargo deny --offline check advisories bans licenses` passes with no ignore. The license exceptions for `zlib-rs`, `zstd-safe`, and `zstd-sys` left `policy.py` with their crates.
- `tests/polars/lazy.rs` lost its streaming collect. The batch-one expression now runs through lazy `collect()` and still proves exact singleton bodies and the shared tally. The morsel-cut proof of ADR 0107 F7 goes with the streaming engine. Restoring it means restoring `polars/streaming` and three advisory ignores.
- A dependency feature named only on the command line, such as `--features polars/streaming`, builds under `--locked` without a lock entry for the crates it adds. The lane cannot use that to test streaming with pinned versions.
