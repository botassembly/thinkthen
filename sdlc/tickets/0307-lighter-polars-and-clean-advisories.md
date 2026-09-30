# 0307: Lighter Polars and clean advisories

Status: ready. Lane claude-3. Plan: `sdlc/planning/cleanup-2026-09-30.md`, step 6.

## Outcome

`cargo deny check` passes offline on main. The optional `polars` feature pulls in only what the lazy and streaming door needs. The dependency count with the feature drops well below today's 362 crates.

## Evidence

- Starts from: `lint` on main `574ef900d` fails `cargo deny` advisories: RUSTSEC-2025-0141 (bincode unmaintained), RUSTSEC-2026-0194 and RUSTSEC-2026-0195 (quick-xml), all through the Polars feature landed in 0298.
- Keeps: the Polars eager and lazy door and its tests.
- Changes: Polars features and `deny.toml`; an ignore only with a stated reason when no feature removes the crate.
- Proof: `cargo deny check` passes; `cargo tree -e normal --features polars | sort -u | wc -l` before and after; Polars tests pass.
- Defers: nothing.
