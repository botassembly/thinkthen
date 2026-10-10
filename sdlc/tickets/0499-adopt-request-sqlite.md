# 0499: Run SQLite through the shared request contract

Status: OPEN.

Milestone: 0.2

Depends on: 0511
Depends on: 0513

Reviews: revision 95fade3863f777142ecd49d4abca0efcf1790cce, accept

Reviews: revision b0fc3e13e4451a1abeb38dfc0a9ecc29abd06905, reject

Reviews: revision 83e9c8a21fc757d2903ba170f35e206d35d9393e, reject

Reviews: revision 7714682107b4202d5faf796f2f9dc9c66ed4c186, reject

Reviews: revision 8a67f22504a9adf6986c3c14009efe283e2371d8, accept

Reviews: revision c0565d2c568769a1066fd6a751fc8a0568854ffc, accept

Reviews: revision a087f6dc3, accept

Reviews: revision ebe2a7ea15349037eb939a7aad66bfe38698b7ae, accept

Reviews: revision 5f41abc5a408b3600f42426b5611e8132fd3ed8c, reject

Reviews: revision c97589232ea2a69aad6f111558b42349a62eef8c, accept

Reviews: revision 791fe5e8f5c72e83ec8ff796a87b80d229afe988, accept

## Outcome

SQLite runs every judgment through the shared Request contract and reads generated result types, so Rust owns all admission and result rules. SQLite keeps only its host types, SQL authority and cancellation. Its SQL conventions (NULL, binary images, JSON values and in-database descriptions) belong to 0519.

## Evidence

- Starts from: [the 2026-10-09 binding decision](../decisions/2026-10-09-thin-first-class-bindings.md). Slices A and B moved the ten complete functions and the legacy scalar, many, table, rank-set, image, document and keyed routes onto Request.
- Keeps: SQL authority, the selected model, files and facts. All ten functions and their input, result, error, cache and replay behavior. Composed records keep file and line locations and the difference between eager admission and incremental execution. Already-composed feeds keep selector, context, attachment, function and image-route validation, cancellation, request-size admission and failure prefixes. The canonical request schema and ordinary Request filtering stay unchanged. Missing stays distinct from null, and permitted unknown result fields are tolerated. A composed native filter feed may retain every complete observation for SQL's existing result carrier.
- Changes: Remaining slices, in order:
  - Canonical saved-selector admission through Request. Claim `databases/sqlite/**` paths per slice. Name any shared Request path and coordinate it with its writer before editing.
  - Document the supported feed combinations and refuse incompatible ones before sending.
  - Remove unused compatibility execution after parity. Keep the SQL preparation, descriptor and admission helpers that PostgreSQL imports. DuckDB's dependency on the shared dispatcher goes in 0495; coordinate any SQLite-only deletion with it.
  - The reviewed public declarations `RequestFeed::from_records`, `RequestFeed::eager`, `RequestFeed::with_image_inputs` and `AdmittedRequest::record_reading` land in the inventory together with their implementation, never ahead of it.
  - SQLite's public SQL function names stay unchanged, so it has no old-name removal. Record handwritten code removed and added in the landing record.
- Proof: The full shared cases pass through the installed extension, including files and images, context and options, original positions, facts, failures, invalid input with zero sends, and cancellation. Raw JSON pass-through does not count.
- Defers: SQL conventions to 0519. DuckDB to 0495. Proxy behavior is out of scope.

## Progress

### Added public declarations

```text
fn RequestFeed::from_records(impl Into<String>, impl Iterator<Item = Result<RecordInput<QuestionInput>, Error>> + 'a) -> RequestFeed<'a>
fn RequestFeed::eager(self) -> RequestFeed<'a>
fn RequestFeed::with_image_inputs(self) -> RequestFeed<'a>
fn RequestFeed::with_all_filter_results(self) -> RequestFeed<'a>
fn AdmittedRequest::record_reading(&self) -> Result<RecordReading, Error>
fn AdmittedRequest::with_resolved_definition(self, RequestDefinition) -> Result<AdmittedRequest, Error>
```

- 2026-10-08 started
- 2026-10-08 landed 1499da2eae04b0db9ab71dc4172c01daea1ef651; next: Partial slice A moves all ten complete SQLite functions onto Request. Installed 255 cases, four counted regressions, root tests and specifications pass; corrected inventory completes the final lint stage. Keep 0499 open for legacy scalar, many, table, rank-set, image, document and keyed routes, plus saved-selector admission. PostgreSQL may now reuse the reviewed bridge; shared Request ownership transfers to 0493.
- 2026-10-08 landed 6f6e5f96e; next: Slice B moves the remaining legacy SQLite judgment routes through Request. Installed legacy cases, 255 shared SQL cases and owning facts pass after shared deadline repair 0509. Keep 0499 open for canonical saved-selector admission; DuckDB dispatcher removal belongs to 0495.
- 2026-10-10 landed 819e5e6c7; next: Canonical saved-selector admission is landed along with complete and legacy SQLite Request routes. Installed saved-file and feed checks preserve zero-send refusals and successful calls. Finish remaining feed documentation and final installed qualification; preserve helpers still used by PostgreSQL and DuckDB.
- 2026-10-10 landed 0b6bf683e; next: SQL feed controls and whole-input recognition are documented accurately. Add CSV/TSV path readers through the shared source-format contract, then qualify the installed extension; retain reachable PostgreSQL and DuckDB helpers.
- 2026-10-10 landed b9fba40a8; next: SQLite and PostgreSQL use native whitespace and count admission while retaining early byte refusal before copying. Adopt shared CSV/TSV path readers; DuckDB chunk preflight and final installed qualification remain.
