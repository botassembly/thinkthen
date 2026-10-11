# 0424: Replay DuckDB recognize and relate with a zero request cap

Status: COMPLETE.

Opened as: 2026-10-11. Fresh High source review accepted; both DuckDB host versions passed replay and spend checks. Full tests and lint run on the landing commit.
Milestone: 0.2

## Outcome

Strict recorded recognize and relate answers work in DuckDB when thinkthen_max_requests_total is zero or already spent. Replay misses still fail without sending. Actual live attempts and retries continue to reserve the existing shared process quota before network dispatch.

## Evidence

- Starts from: actual public v0.1.2 DuckDB 1.5.5 strict replay failures under a zero process cap, reported in `inbox/thinkthen/2026-10-05-experiments-0033-0-2-replay-gap-duckdb-recognize-and-relate-refuse-a-zero-request-cap.md`. Other functions replayed; the OS denied IPv4/IPv6 sockets and final usage remained zero. Current source retains the premature host within_total check in nested and relate routes. Ian prioritizes actual SQL bugs ahead of files.
- Keeps: all result shapes, named-file and cache permissions, missing-record refusal, staged recognize/relate limits, cancellation and worker lifetime; keys never enter SQL; the process-wide atomic actual-send reservation and retry accounting stay unchanged. Ordinary live calls with zero quota still send nothing.
- Changes: remove or bypass the premature host row-budget refusal for these staged calls while preserving max_requests_total in their native CallOptions. The native engine decides whether an actual send is needed after replay/cache lookup and enforces its shared quota at dispatch. Cover both current portable nested route and retained grouped bridge route, plus explicit relate. Do not loosen global quotas or use SQL spelling alone as evidence of replay mode. Update the relevant SQL replay guidance and error-timing contract.
- Proof: use existing owned recording/loopback host fixtures. Record minimal recognition and relation answers, then load strict replay without keys with zero total and an already-spent positive total; assert stored answers and zero new requests/input/output tokens. Replay may increase cache_answers. A missing recording fails without sending. A zero-total live call fails with zero counted sends. Retain positive-total, retry and shared-process quota cases. Reuse the existing core atomic-reservation tests. Run focused DuckDB bridge/host tests, one fresh High source review, then full tests and lint on the landing commit.
- Defers: general host precollection and row-budget redesign; new replay tools or network guards; paid calls, benchmarks and whole-SQL cost accounting.

## Spending risk

Risk: High. This changes host admission timing, never the final spending authority. Existing engines::options_for passes max_requests_total to the core. Core SendBudget reserves the actual process request count at dispatch. Confirm that every changed nested/relate path still passes this option before removing its preliminary refusal. Do not change core quota arithmetic or permit a live attempt when the total is zero.
