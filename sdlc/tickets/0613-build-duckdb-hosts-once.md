# 0613: Build the DuckDB hosts once per release run and share them

Status: OPEN.

Milestone: 0.2.1

Phase: 1

Reviews: revision 7a2c32b, accept

Reviews: revision 9b49152, accept

Reviews: revision 8dba1ef, accept

Reviews: revision d61872d, accept

Reviews: revision 93e5590, accept

Reviews: revision ed9716b, accept

Reviews: revision 33ae41e, accept

## Outcome

- Changes: `.github/workflows/release.yml`, `databases/duckdb/tools/setup.sh`, `sdlc/scripts/release-container`, `sdlc/scripts/release-workflow`, `sdlc/scripts/workflows`.
- The build job uploads each pinned DuckDB host it compiled as a build output named by version, target and SHA.
- Every smoke job that needs a DuckDB host downloads that output. No smoke job compiles DuckDB.
- The Intel-only `duckdb-hosts` cache step, the `THINKTHEN_DUCKDB_HOST_CACHE_HIT` switch and the cache-ban policy exception are deleted.

### Proof

- `sdlc/scripts/lint` passes on the M5 through pm job with the cache step and its policy exception gone.
- 0613 lands on its own after the M5 gate. One dispatched nonpublishing rehearsal at the next checkpoint (0691), after 0620 and 0622 land, proves the workflow tickets together: its logs show no DuckDB compile in any smoke job, and every DuckDB smoke passes using the uploaded host.

## Evidence

Release smoke jobs on the Intel macOS runner ran into the six-hour cap while handling the DuckDB hosts. The build job already compiles the pinned DuckDB hosts. `65acd9a5e` ("Reuse Intel DuckDB hosts within the release run") then added an `actions/cache` keyed by run id for the Intel target only (`.github/workflows/release.yml`, the `duckdb-hosts` step in the `smoke` job). That patches one target through a cache, which the workflow policy otherwise bans. Run 39102241482 saved the cache, but the restore and a complete Intel smoke have not run.

## Progress

- 2026-10-18 started
- 2026-10-19 next: Fresh review accepts ed9716bfb after the complete hosted-workflow self-test, global workflows, children and policy passed in M5job1792421328845-87188; routing fixtures now read actual manifest pins and isolate deferred legacy Ruby setup, while production hosted Ruby readers remain incompatible until0620, and the standalone closing gate is next.
- 2026-10-19 next: The standalone closing gate at af70f418d failed after105.4s through the same stale shared sccache temporary directory as0612, with receiptc0f6f86a902e226cf0275260233a689b33f2e02e retained; accepted workflow checks remain green and the shared compiler-isolation repair will land before the next native closing run.
- 2026-10-19 next: The integrated native closing job1792422734847-66148 failed at hosted parameter_cases apple-check after its unchanged5s limit, with receipt87570ed25307bc91f83628a2cc380572c4dbb8fc retained; a fresh developer is diagnosing the cause before any fixture change or rerun while independent0629 and0635 work continues.

## Amendment, 2026-10-18

0613 lands alone after the M5 gate; 0622 starts after it and batches with 0620. The hosted transfer proof moves to the 0691 checkpoint rehearsal.
