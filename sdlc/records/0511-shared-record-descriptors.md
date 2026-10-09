# Shared complete-call record descriptors

## Source

This slice starts from `db7917daa0cb19b09e4ad51ffec58a97ee923b1c` and follows the accepted review amendment on ticket 0511. The product change is in rebased source commit `605c34e02`. Integration uses published main `6af931685f82d64668508661b5604d7814903462`, including the reader repairs, host ceilings and Facts schema additions. The rebase preserved those root changes; its only conflict was the source ceiling, resolved through the existing ratchet measurement.

`public/request/input.rs` exposes the existing complete-call descriptor conversion as `RequestItem::from_record_descriptor`. `public/request/transport.rs` owns its closed envelope grammar and existing diagnostics. It converts literal text, document text, ordered JSON, context, shortlist replacement, recognition controls and integer-array images into Request values. Pixel validation belongs to shared composition and runs once. Explicit context remains a typed value until Request execution admits it against the prepared question; a missing declaration in the initial composition helper had refused the existing declared-object-context case.

`public/request/composition.rs` exposes the existing typed composition path as `RequestItem::compose_record`. It preserves the legacy image-only behavior when reading projections are supplied. `databases/sqlite/src/complete_native/inputs.rs` delegates record conversion and composition to those public methods. SQL retains its reader-error decoding, source identities, file authority, format conversion and host envelope controls. The existing SQLite `complete/request.rs` path still supplies composed records through `RequestFeed::from_records`, so shared Request execution retains verb admission, cancellation, declaration checks, image routes and engine limits. Pure core imports no decoder or runtime authority.

The public Request wire schema and legacy C translation are unchanged. C retains its accepted duplicate-key behavior and diagnostics. PostgreSQL, DuckDB, R and other cross-crate include users retain their current entry points; their named migration slices and the removal of cross-crate includes follow separately. This slice introduces no binding-wide lint or cross-language conformance fanout.

## Proof

The bounded Request contract run passed all 16 tests. The added cases cover the legacy originals, explicit empty context, complete shortlist replacement, malformed envelope diagnostics, duplicate-field refusals, shared record-size refusal, ordered duplicate image attachments, image-only projection compatibility and native malformed-pixel diagnostics. Existing tests retain native answers, identity, filter projections, declaration refusals, cancellation and completed-prefix behavior.

The SQLite debug build, focused Request Clippy, SQLite Clippy, four SQLite complete-request tests and complete facts checks passed. `policy.py` passed. The composition file crosses the source-size warning threshold because the public method directly delegates to its existing implementation; the source commit explains this growth.

Complete SQL parity on `e33fc8a8140f1f651cdf8b4f6b52cea2ecf2be73` passed 254 cases and exposed the declared-object-context regression. The corrected source passed all 14 existing context cases, including that failure and undeclared-object refusals, through `python3 databases/sqlite/tests/complete/parity.py sqlite CASE`. The complete-request and facts checks also passed again against the final SQLite artifact. The unaffected parity results remain applicable; the full parity suite was not restarted after the context correction. These outputs are in `sqlite-parity-final.log`, `sqlite-context.log`, `sqlite-request.log` and `sqlite-facts.log` under `target/0511/`.

After integration, the 16 Request contract tests, SQLite build, all 14 context cases, four complete-request tests, complete facts checks, policy and whitespace checks passed again. The root source ceiling equals the existing tool's measured 176291 non-blank lines. The increase moves previously unmeasured SQL grammar into the core crate and adds four edge behavior cases; it does not retain the removed SQL validation copies.

Inventory checked all 1898 declared items and refused all four planted changes. Its only differences are the expected additions `fn RequestItem::compose_record(&self, &RecordReading) -> Result<RecordInput<QuestionInput>, Error>` and `fn RequestItem::from_record_descriptor(&str) -> Result<RequestItem, Error>`. Those functions cross the Rust crate boundary from the shared SQL adapter to the approved Request edge. Their declaration amendment belongs to PM; no ticket or inventory script was edited.

The specification gate passed on the first source commit. Its 24 green demos and executable pages exercise unchanged command paths. The later correction moves pixel validation within the new descriptor path and does not alter a command consumer, so that specification evidence remains applicable.

The focused commands were `cargo test --locked --offline -p thinkthen --test request_contract -- --test-threads=2`, `cargo clippy --locked --offline -p thinkthen --test request_contract -- -D warnings`, `cargo build --manifest-path databases/sqlite/Cargo.toml --locked --offline`, `cargo clippy --manifest-path databases/sqlite/Cargo.toml --locked --offline --all-targets --all-features -- -D warnings`, `python3 databases/sqlite/tests/test_complete_request.py`, `python3 databases/sqlite/tests/complete/facts.py sqlite`, `CARGO_NET_OFFLINE=true python3 sdlc/scripts/policy.py` and `sh sdlc/scripts/spec`.

Heavy build checks used systemd scopes with `MemoryMax=8G`, `MemorySwapMax=1G`, two build jobs and offline Cargo. The specification gate used the lane lock and an owned Cargo home whose settings retain the warm build profile and enforce offline operation. Proof output and its OWNER file are under `target/0511/`. Runtime cases use owned loopback listeners or saved exchanges. SQLite runtime checks use the existing pinned 3.50.0 host through `LD_LIBRARY_PATH`; the initial system 3.45.1 invocation refused loading and supplies no product evidence. No paid call, remote build, release action or reader change was made.

## Storage ownership

The assignment reported about 41.42 GiB before cleanup. The backend harness producer in `crates/thinkthen/tests/backend/harness/mod.rs` creates `target/tmp/spawn-home-PID-N` folders from its process identity and monotonically increasing spawn number. Every selected PID was absent before removal, and the lane had no active harness writer. `gio trash` removed only 96,535 completed harness homes containing 2,693,627,904 allocated bytes. All other `target/tmp` entries and canonical warm build folders were retained. The direct measurement after cleanup was 40,802,300 KiB across `target`, `libraries` and `databases`, or 38.912 GiB. The final direct measurement was 41002216 KiB, or 39.103 GiB, below the lane cap.

## Final integration

Integration with the rewritten ticket plan retained the accepted source. The 16 Request tests, policy and whitespace checks passed; the measured root ceiling is 176320. The API inventory exposed five already-landed declarations omitted by the 0499 ticket rewrite. Restoring their existing declaration block and recording the two approved descriptor methods repairs the inventory without changing product code. The earlier SQLite consumer evidence remains applicable; no new full cross-surface run is claimed.

## What the build taught us

Keep lexical compatibility at the public Request edge and delegate semantic validation to existing typed constructors and composition. Retain host authority separately from engine rules. Preserve existing proof while its exercised paths are unchanged, and recheck the affected consumer when a source correction changes its behavior. Measure allocated storage before a build; process-named harness homes provide a narrow cleanup boundary without discarding warm artifacts.
