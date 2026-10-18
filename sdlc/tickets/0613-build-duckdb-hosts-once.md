# 0613: Build the DuckDB hosts once per release run and share them

Status: OPEN.

Milestone: 0.2.1

Phase: 1

## Outcome

- The build job uploads each pinned DuckDB host it compiled as a build output named by version, target and SHA.
- Every smoke job that needs a DuckDB host downloads that output. No smoke job compiles DuckDB.
- The Intel-only `duckdb-hosts` cache step, the `THINKTHEN_DUCKDB_HOST_CACHE_HIT` switch and the cache-ban policy exception are deleted.

### Proof

- `sdlc/scripts/lint` passes on the M5 through pm job with the cache step and its policy exception gone.
- The workflow tickets 0613, 0620 and 0622 land together. One dispatched nonpublishing rehearsal at the next checkpoint (0691) proves them: its logs show no DuckDB compile in any smoke job, and every DuckDB smoke passes using the uploaded host.

## Evidence

Release smoke jobs on the Intel macOS runner ran into the six-hour cap while handling the DuckDB hosts. The build job already compiles the pinned DuckDB hosts. `65acd9a5e` ("Reuse Intel DuckDB hosts within the release run") then added an `actions/cache` keyed by run id for the Intel target only (`.github/workflows/release.yml`, the `duckdb-hosts` step in the `smoke` job). That patches one target through a cache, which the workflow policy otherwise bans. Run 39102241482 saved the cache, but the restore and a complete Intel smoke have not run.
