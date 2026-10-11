# 0538: Move the remaining DuckDB routes to the shared request

Status: OPEN.

Milestone: 0.2

Reviews: revision fe9bf662f0234e8177eecd2a0f0adea96b36e5d0, accept

## Outcome

Every DuckDB function builds its call through the shared Request and owned session. No DuckDB route calls the engine directly or writes request JSON by hand.

## Evidence

- Starts from: gap 4 of [the 0.2 closure review](../records/2026-10-11-0-2-closure-review.md). 0495 moved the complete and file paths. About 13 routes still call the engine directly: `databases/duckdb/bridge/src/ffi/scalar/ffi.rs:105,229,254,343`, `portable_many/ffi.rs:287,311,322,332`, `portable/ffi.rs:162`, `listed.rs:75`, `images/ffi.rs:200,206` and `complete_listed/ffi.rs:23`.
- Keeps: every DuckDB function name, argument, NULL rule, JSON value, file admission bound and description that 0470, 0495 and 0519 qualified. Both DuckDB 1.5.4 and 1.5.5 builds.
- Changes: route the scalar, portable, listed, image and complete-listed functions through the shared request. Delete the direct engine calls and their JSON assembly. Lower the DuckDB ceiling.
- Proof: the routine installed DuckDB checks pass on both versions.
- Defers: nothing.

## PM amendment, 2026-10-11

The packed try-details route keeps its direct engine call in 0.2. It returns answered rows beside rows that failed on backend or settings errors, and the owned session stops at the first atomic failure and has no failure value for each row. A recovery entry point and error value for each row would be new public design, so the idea "the shared request session recovers failures row by row" carries it after 0.2. This route stays the only direct engine call in DuckDB, and a code comment names that idea.

Ordinary details may add one small native accessor that returns the existing Details serialization, so DuckDB keeps its JSON values. The accessor adds no new format.

Migrate every other route as the Changes line says. The ticket closes when those routes use the shared request and the routine installed DuckDB checks pass on both versions.

### Planned public declarations

```text
fn CompleteDecision::to_scalar_json(&self) -> Result<String, Error>
fn CompleteChoice::to_scalar_json(&self) -> Result<String, Error>
fn CompleteTags::to_scalar_json(&self) -> Result<String, Error>
fn CompleteScore::to_scalar_json(&self) -> Result<String, Error>
```

These accessors serialize the retained scalar Details result without introducing a new format or copying its metadata rules into DuckDB.

Annotate ruling. DuckDB annotate takes the shared Request behavior: an invalid row such as one missing `/body` stops the call before any send, and the query returns the same bad-row error. Today the query already fails on that row. Only the side effect changes: valid neighbours are no longer sent and cached first. Update the installed DuckDB regression to expect the error with zero sends. Packed batch counts for valid input stay unchanged. Add no public admission helper and no second direct engine call. `Engine::check_record_limit` may keep eager quotas. Note the change in the DuckDB README.

## Progress

- 2026-10-11 started
- 2026-10-11 next: Await the PM ruling on preserving DuckDB packed try-details recovery while migrating ordinary routes to owned requests.
- 2026-10-11 next: Apply the approved packed try-details exception and existing Details serializer, migrate every other DuckDB route, and finish installed checks on both versions.
- 2026-10-11 next: Installed checks exposed annotation neighbour-cache behavior that shared sessions cannot preserve without a pure admission helper; await a bounded API ruling while fixing quota and NULL-group validation.
