# 0511: Let core own the whole request grammar

Status: OPEN.

Milestone: 0.2

Reviews: revision 4cd756859, reject

Reviews: revision c79e3f65e05eb4f12fa46d8f7ab003d477258c9c, accept

Reviews: revision d5ec74f1047c04be948eec94e7971870872f405d, accept

Reviews: revision a087f6dc3, accept

Reviews: revision 2726d5c41, accept

Reviews: revision cde3e7649c1ad0dbb2f5f5c5da379f0f0a6efe6a, reject

Reviews: revision 9e9721d83f74690ba8c1768d8f104f0f162c6d5d, accept

Reviews: revision 0b1012c750da66b1356b8aa346545278c1225ca8, accept

Reviews: revision 2da0d0ecf6fe6df5dc53280d710b80823bc8e402, accept

Reviews: revision 4b505569df6b3e8fded623df2ad8edd4203ec619, accept

Reviews: revision a4c93ae43c5ef2263698dc91aa1361b1ed3569d4, accept

Reviews: revision 22a58d373187f746d2999220e945f8c1ef71ea42, accept

Reviews: revision 93ed5726b36931b6c25fb2107b989ac8f6c32c9b, accept

Reviews: revision 2d400ced676655cce01f60d5c9f8dcf90cbc1f5c, accept

Reviews: revision d3e1e145cf7662ebf1f8096dd3f0b0ef5ada67e9, accept

Reviews: revision 263ff4fd11f5f33349055b63b6842e27bcee9b34, accept

Reviews: revision 564dbbae3f6f0112c44d8220fa163fb7c07b827f, accept

Reviews: revision ad878dacedc6fbfa5f0ae4e2538d7bab83c85842, accept

Reviews: revision f147a00f8ee6a789ec761f4e5356f9d95eebf38a, accept

Reviews: revision 0ac0e2e51ee88baee4c939a863c7d5c775413c55, accept

Reviews: revision 011355dcf90f4602663bf0f5e2d11e7bdf34dee1, accept

Reviews: revision d8ed6cda41b9f3fe23fab737a08695c2b1500408, accept

Reviews: revision 20b1bb55a3e4f28c26e9704ca9e78b07ab0a8b0b, accept

Reviews: revision 513e220aabd9053a07586ed6b4aa1d2c50af8406, accept

Reviews: revision 11f51eda194c1f80e8cfa46fff1c159008577772, accept

Reviews: revision 127d97271e1f49497bfb46fb05c59b98a4a124d7, accept

## Outcome

One public Request edge admits every request through shared Rust grammar, limits, defaults and validation. Every surface calls it, and core refusals reach the caller. No binding, extension or host package restates an engine rule.

## Evidence

- Starts from: [the 2026-10-09 binding decision](../decisions/2026-10-09-thin-first-class-bindings.md). The audits found the complete-call input grammar in `databases/sqlite/src/complete_native` and `libraries/r/thinkthen/src/rust/src/complete`, pulled into other crates by `#[path]` includes. The C door's verb-key grammar lives in `libraries/c/src/call.rs`. Find and relate limits are restated in DuckDB, SQLite, PostgreSQL and C. The label grammar is restated in Ada `thinkthen.adb` and COBOL `tt_shape.c`. Settings, batch, deadline and relate-rule checks are restated in Ruby, R and TypeScript.
- Keeps: The 0491 Request contract, its generated schema, error kinds and every currently accepted input. Pure core keeps its typed semantic rules and imports no Request, decoder, reader or runtime handle, under ADR 0125. Surfaces keep host conversion, pointer representability and file-authority checks. Legacy C translation, accepted duplicate-key behavior and diagnostics stay.
- Changes: Move the shared grammar behind the public Request edge and delete each restated copy. Slices, in order:
  - The first slice removes duplicate SQL record-descriptor validation through existing Request conversion and admission. Claim `crates/thinkthen/src/public/request/input.rs`, `crates/thinkthen/src/public/request/composition.rs`, `crates/thinkthen/src/public/request/transport.rs`, `databases/sqlite/src/complete_native/inputs.rs`, `databases/sqlite/src/complete/request.rs` and `crates/thinkthen/tests/request_contract/projections.rs`.
  - Later slices name their consumers before starting. Remove cross-crate `#[path]` includes after those consumers migrate.
- Proof: A lint fails when a binding restates an engine semantic limit or a `#[path]` reaches outside its crate. It ignores legitimate host checks and mere mentions of a constant. Shared conformance cases drive one over-limit and one malformed input through each surface and get the same core error kind.
- Defers: Result reading goes to 0513. The CLI and MCP pipelines go to 0512.

The first slice adds these public Request-edge methods for the separate SQL crate's shared admission and composition. The PM approved recording them on 2026-10-09 within this outcome.

### Added public declarations

```text
struct ImageAdmission
fn Relate::admit_record_count(usize) -> Result<(), Error>
const fn Relate::max_record_count() -> usize
fn ImageAdmission::new(usize) -> Result<ImageAdmission, Error>
fn ImageAdmission::push(&mut self, usize) -> Result<(), Error>
fn ImageInput::admit_length(usize) -> Result<(), Error>
fn Question::admit_find_units<'a>(&self, impl IntoIterator<Item = Result<&'a str, Error>>) -> Result<(), Error>
fn RequestItem::from_record_descriptor(&str) -> Result<RequestItem, Error>
fn RequestItem::compose_record(&self, &RecordReading) -> Result<RecordInput<QuestionInput>, Error>
fn RequestItem::with_options_descriptor(self, &str) -> Result<RequestItem, Error>
fn EngineBuilder::from_settings_json(&str) -> Result<EngineBuilder, Error>
fn Engine::plan_request<'a>(&self, &'a AdmittedRequest, RequestEnvironment<'a>) -> Result<PlanEstimate, Error>
enum TableFormat
TableFormat::Csv
TableFormat::Tsv
struct TableReader<R>
fn TableReader::new(impl Into<String>, R, TableFormat) -> Result<TableReader<R>, Error>
impl Iterator for TableReader
type TableReader::Item = Result<SourceRecord<RawRecord>, Error>
RequestSource::framing: Option<RequestFraming>
struct RequestSourceReader<R>
fn RequestSourceReader::new(impl Into<String>, R, RequestFraming, ReaderOptions) -> Result<RequestSourceReader<R>, Error>
impl Iterator for RequestSourceReader
type RequestSourceReader::Item = Result<SourceRecord<RawRecord>, Error>
fn Engine::try_plan_source_with<Q: DetailQuestion + ?Sized>(&self, &Q, impl IntoIterator<Item = Result<SourceRecord<String>, Error>>, CallOptions<'_>) -> Result<PlanEstimate, Error>
fn RequestSource::validate_reading(&self) -> Result<(), Error>
fn RequestSource::read_framed<'a>(&self, CallOptions<'a>) -> Result<Box<(dyn Iterator<Item = Result<SourceRecord<RawRecord>, Error>> + 'a)>, Error>
```

## Progress

- 2026-10-09 started
- 2026-10-09 landed 2dbccdf0e; next: Shared SQL record descriptors are landed. Continue named consumer migrations and remove remaining duplicated request grammar and cross-crate includes; this first slice does not complete the whole ticket.
- 2026-10-09 landed 48363919e; next: SQL and shared binding record descriptors now delegate original/image composition to Request. Preserve the documented legacy tagged-context and ordered-description translations until canonical grammar expresses them; finish remaining consumer grammar and cross-crate includes.
- 2026-10-09 landed 75544b8f7bf62d904b9aaf061a090245a03e599e; next: Ordered shortlist admission is shared. Finish typed engine settings, canonical plan preview and retained context translation before host migrations.
- 2026-10-09 landed 13f861f58bf0e7af5719fdd6c85587937ebd1117; next: Typed native settings now own validation and C construction. Finish canonical Request planning and remaining consumer translations.
- 2026-10-09 landed 0ac45ca65cda6e240b8dabb32ed1c098991cafc5; next: Canonical Request planning is landed. Fix the authored rank structural schema gap exposed by generated C# inputs; finish combined checks and consumer translations.
- 2026-10-09 landed 0ac45ca65cda6e240b8dabb32ed1c098991cafc5; next: Canonical planning and typed settings are landed. Source inspection confirms rank uses existing decide/score authored shapes and rank sets use questions; no new rank grammar is needed. Finish combined checks and remaining consumer translations.
- 2026-10-09 landed 46e331cea5aa378fcf5f404af14b22055fb392fb; next: Reserved proxy settings now preserve presence and refuse safely before environment capture. Finish combined checks and remaining consumer translations before whole closure.
- 2026-10-09 landed ced19b30f; next: Native settings and generated proxy input types agree. Rerun combined checks after the generated-file repair and finish consumer translations.
- 2026-10-10 landed 649fd2412; next: Native JSONL and line session framing is landed. Shared CSV/TSV reader is in repair review; session table framing and remaining consumer grammar stay open.
- 2026-10-10 landed d8ee56da4; next: Shared authorized CSV/TSV reader is landed with terminal iterator errors fixed. Descriptor table sessions are being implemented from the reviewed logical-row design; remaining consumer grammar stays open.
- 2026-10-10 landed 19ba6aecc; next: Native CSV and TSV sessions are landed with bounded complete-row descriptors, shared parsing and completed-prefix failure handling. Finish remaining consumer grammar and adoption; large-input boundaries remain release-only.
- 2026-10-10 landed 9d76c6903; next: All native consumer locks include the shared CSV dependency and resolve offline without version changes. Shared table readers and sessions are landed; finish remaining consumer grammar/adoption.
- 2026-10-10 landed 639f7a77c; next: Shared explicit CSV/TSV path framing and authorized handle readers are landed with generated inputs. Adopt them in SQL and finish shared source-plan admission and borrowed pull before compatibility retirement.
- 2026-10-10 landed cf55fa095; next: Native source planning preserves original-byte bounds and unread-tail behavior; C and Python imports and the unused R helper are removed. Finish SQL adoption, shared borrowed pull and remaining admission copies before final qualification.
- 2026-10-10 landed 6b89b6252; next: Shared borrowed find admission now validates SQL collections before copying text. Path framing, source planning and borrowed row execution are also landed. Finish remaining consumer admission copies and policy enforcement; final installed qualification remains held.
- 2026-10-10 landed 848daa393; next: SQL and C image bounds now delegate to native borrowed admission, preserving host guards and error order. Shared find, source framing, source planning and borrowed row execution are landed. Finish remaining reachable semantic copies and consumer adoption; qualification remains held.
- 2026-10-10 landed e102ea646; next: Core and C share relation record-count admission; canonical C relation intake is lazy and bounded with preserved cancellation and owned results. Finish aggregate byte-bound sharing, remaining eager C find intake and consumer adoption; release qualification remains held.
- 2026-10-10 landed c0d1bf0d4; next: Native aggregate byte limits share SourceBudget. Finish eager canonical C find intake and remaining consumer adoption; release qualification remains held.
- 2026-10-10 landed a3c59f4e5; next: C find and relate use native bounded fallible intake; count and aggregate byte rules share their native owners. Focused actual C callers pass under AddressSanitizer. Finish the remaining consumer and boundary audit; release qualification remains held.
- 2026-10-10 landed d697fc147; next: SQLite and PostgreSQL relation admission now share native raw count limits; PostgreSQL derives bounded intake from core. Focused callers preserve diagnostics and send nothing on refusals. Remaining compatibility and boundary enforcement await adopted callers and installed qualification.
- 2026-10-10 landed 26e11eca8; next: SQLite inline and JSON relation definitions now share native rule validation. Focused callers and fresh review preserve exact refusals and zero sends; compatibility retirement and boundary enforcement remain.
