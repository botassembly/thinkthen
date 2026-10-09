# 0499: Move SQLite onto the shared request contract

Status: OPEN.

Milestone: 0.2

Reviews: revision 95fade3863f777142ecd49d4abca0efcf1790cce, accept

Reviews: revision b0fc3e13e4451a1abeb38dfc0a9ecc29abd06905, reject

Reviews: revision 83e9c8a21fc757d2903ba170f35e206d35d9393e, reject

Reviews: revision 7714682107b4202d5faf796f2f9dc9c66ed4c186, reject

Reviews: revision 8a67f22504a9adf6986c3c14009efe283e2371d8, accept

Reviews: revision c0565d2c568769a1066fd6a751fc8a0568854ffc, accept

## Outcome

Adopt shared Request and generated results in SQLite through named typed public calls.

## Evidence

- Starts from: PM architecture asks2/5 requires one ticket per direct binding; current SQLite adapter repeats admission/result construction.
- Keeps: SQL authority, selected model, files and facts; all ten functions, input/result/error and cache/replay behavior.
- Changes: Depends on0491, 0502 and the reviewed generation decision. Translate host arguments into Request, decode generated types, and remove the old copy after parity. Claim `databases/sqlite/**` and its installed typed consumer cases. Preserve absent versus null and tolerate permitted unknown result fields.
- Proof: Full shared cases execute through the installed host's typed interface, including files/images where supported, context/options, original positions, facts, failures, invalid-input zero sends and cancellation. Raw JSON pass-through is insufficient.
- Defers: Proxy and changing platform rulings without evidence. Size: medium surface migration.

## Native feed bridge

SQLite's existing composed records retain file and line locations and distinguish eager admission from incremental execution. Compatibility translations reuse the reviewed shared Request feed bridge and native legacy projections. Reuse annotation's existing per-member evidence selection and declaration admission. A composed native filter feed may explicitly retain every complete observation for SQL's existing result carrier; ordinary Request filtering and the canonical schema stay unchanged. Remaining SQLite adapter work claims its database paths, not shared Request implementation. Any further native extension must first coordinate with the recognition writer and name its exact paths.

Already-composed feeds must preserve selector, context, attachment, function and image-route validation, cancellation, request-size admission and failure prefixes. Document which combinations are supported and refuse incompatible combinations before sending. Remove unused compatibility execution after parity; retain the SQL preparation, descriptor and admission helpers PostgreSQL imports. This preserves native composition, the canonical request schema and SQL authority.

### Added public declarations

```text
fn RequestFeed::from_records(impl Into<String>, impl Iterator<Item = Result<RecordInput<QuestionInput>, Error>> + 'a) -> RequestFeed<'a>
fn RequestFeed::eager(self) -> RequestFeed<'a>
fn RequestFeed::with_image_inputs(self) -> RequestFeed<'a>
fn RequestFeed::with_all_filter_results(self) -> RequestFeed<'a>
fn AdmittedRequest::record_reading(&self) -> Result<RecordReading, Error>
```

### Reviewed API scope

The reviewed branch declares `RequestFeed::from_records`, `RequestFeed::eager`, `RequestFeed::with_image_inputs` and `AdmittedRequest::record_reading`. Keep their exact inventory declarations with the implementation when it lands. Publishing those declarations ahead of the code makes independent main-based inventory checks fail; this paragraph retains design approval without claiming shipped exports.

## Progress

- 2026-10-08 started
- 2026-10-08 landed 1499da2eae04b0db9ab71dc4172c01daea1ef651; next: Partial slice A moves all ten complete SQLite functions onto Request. Installed 255 cases, four counted regressions, root tests and specifications pass; corrected inventory completes the final lint stage. Keep 0499 open for legacy scalar, many, table, rank-set, image, document and keyed routes, plus saved-selector admission. PostgreSQL may now reuse the reviewed bridge; shared Request ownership transfers to 0493.
- 2026-10-08 landed 6f6e5f96e; next: Slice B moves the remaining legacy SQLite judgment routes through Request. Installed legacy cases, 255 shared SQL cases and owning facts pass after shared deadline repair 0509. Keep 0499 open for canonical saved-selector admission; DuckDB dispatcher removal belongs to 0495.
