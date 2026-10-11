# 0415: Submit the ThinkThen DuckDB community extension

Status: COMPLETE.

Opened as: 2026-10-11. The reviewed adapter landed and the approved community submission is open at https://github.com/duckdb/community-extensions/pull/2936. DuckDB acceptance, native CI, signing and publication remain external. See record 0415 for source and platform results.

Milestone: 0.2

## Outcome

Rehearse the community build offline, then submit thinkthen to DuckDB's community extensions under imaurer. This work runs beside 0.2 and does not hold its release. Keep dbt v1 documented; claim dbt v2 only once a signed listing is live.

## Evidence

- Starts from: landed 0403 builds for DuckDB 1.5.4/1.5.5, ADR 0081, experiment 0011 and Ian's later 2026-10-05 listing authorization in the PM message about files and local runtimes.
- Keeps: unsigned standalone assets, existing load/install behavior and dbt v1.
- Changes: community build integration and a listing pull request. Account and sole maintainer: Ian Maurer (imaurer). Extension: thinkthen. License: MIT. Source: botassembly/thinkthen at the rehearsed commit. Follow current stable DuckDB in community CI, rebuild on new releases and retain 0403's dbt versions. Include every community target that builds; name any exclusion.
- Proof: existing source, package, ABI and loader checks plus offline community build rehearsal. Review the concrete listing and pin its source to the rehearsed commit before submission. Native community CI supplies its own platform results.
- Defers: DuckDB acceptance/signing/publication is external. Any new cost, enrollment, account or change to the approved identity returns to Ian; no expected cost.

## What Ian can overturn

Version/target policy and submission details. The approved outside pull request needs no second permission request while those details stay unchanged.

0.2 ships without DuckDB community acceptance/signing. Integrate the signed listing in a 0.2 point release when DuckDB makes it available and its checks pass.
