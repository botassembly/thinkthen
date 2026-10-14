# 0583: Experiment: one DuckDB build across 1.4, 1.5 and 2.0

Status: OPEN.

Milestone: 0.3

## Outcome

Learn whether one ThinkThen DuckDB extension build, made against DuckDB's stable C extension API, loads and passes the extension's SQL checks on DuckDB 1.4, 1.5 and the 2.0 pre-release.

## Evidence

- Starts from: `duckdb/`, `extension_config.cmake` and `extension-ci-tools/` on main, and ticket 0577's community listing work. Design: the marketing repo's `products/thinkthen/0.3/prd/09-publishing-channels.md`, "One version rule".
- Keeps: 0577's 0.2 listing and main's DuckDB build untouched. No product code changes, no paid model calls, no publishing, and no agent or machine time taken from 0.2 work.
- Changes: `sdlc/records/0583*` only. Work in an owned numbered folder under the workspace experiments folder with `OWNER` reading `thinkthen/0583`; containers start with `docker run --rm --label pm.repo=thinkthen --label pm.ticket=0583`.
- Proof: for each DuckDB version, the build loads and the extension's existing SQL tests pass, or the record names the first missing C API call. Record build count, build time and binary size against today's per-version builds.
- Stop rule: one build passes on all three versions, or the C API lacks a call the extension needs.
- Decision it changes: Ian's decision 17 (support 1.4 only if free) and PRD 09's build-once rule. Tell the coordinator at once if anything found helps 0577.
- Machine: the Beelink.
- Defers: any change to the shipped extension goes to a 0.3 product ticket after the record lands.
