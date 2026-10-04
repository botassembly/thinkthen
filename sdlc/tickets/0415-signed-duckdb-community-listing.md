# 0415: Prepare a signed DuckDB community extension listing

Status: ready. Planning record only; fresh ticket review precedes implementation.

Milestone: later

## Outcome

After 0.2 core lands, prepare the extension for DuckDB community CI and review a concrete listing submission. Ian approved the separate signed-extension follow-up. Keep dbt v1 as the documented 0.2 route.

## Evidence

- Starts from: [0403](0403-duckdb-extension-for-dbt-v2.md), [ADR 0081](../planning/adr/0081-duckdb-cpp-api.md), experiment 0011 named in 0403, and Ian’s 2026-10-04 approval of the separate post-core signed-listing follow-up. 0403 produces the DuckDB 1.5.4 compatible build; signing/listing is a separate outcome. Unsigned compatibility alone cannot make dbt v2 load the extension.
- Keeps: existing unsigned extension assets, supported standalone DuckDB routes and the documented dbt v1 route for 0.2.
- Changes: prepare a signed duckdb community extension listing. Product work starts only after the contract and ticket receive fresh review.
- Proof: Offline reproducible community build rehearsal, source/package/ABI checks and signed-loader plan. Record exact supported versions and target families. Obtain required authorization before external submission, enrollment or spend.
- Defers: No submission, public commitment or spend is authorized by this planning ticket. 0403 version compatibility remains separate.

## What Ian can overturn

The supported DuckDB version policy, community CI build integration and target families. No external action or new cost follows from this planning record.
