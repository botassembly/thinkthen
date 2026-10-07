# 0410: Complete all ten dataframe functions and located files

Status: ready. Planning only; implementation follows accepted ticket review.

Milestone: 0.2

Owner: builder.
Ticket review: ACCEPT, 2026-10-06; fresh read-only review. Conflicting image deferrals corrected where found.

## Outcome

pandas, Python Polars and Rust Polars expose all ten functions, equivalent identity/results/errors/storage, and typed located-file routes.

## Evidence

- Starts from: Existing ticket and 0405 audit; landed 0377/0401D/0420 are the current baseline, replacing the older audit-only matrix. PM message `2026-10-06-pm-0-2-is-not-done-every-sdk-consistent-and-the-sdk-ready-for-the-proxy.md`, asks 4 and 7.
- Keeps: Preserve existing bare calls and generic JSON compatibility doors, backend selection from 0377, six error kinds, cancellation, secrecy, count-only usage, and zero-send strict replay. Reuse the Rust engine, C boundary and native file reader; add no host cache, scheduler or second parser.
- Changes: Remove container-only refusals; add Rust Polars filter/rank/find/recognize/relate through the existing engine. Preserve nulls, duplicate index/name, whole candidate/entity sets, spans and relation endpoints. Lazy whole-set helpers explicitly materialize the logical collection where required; do not claim per-morsel streaming equivalence. Fix the deadline test’s invalid demand for two sends before expiry under load.
- Proof: Compare frame and ordinary saved inputs with nulls/duplicates/empty rows and all ten outputs. Reuse native file fixture. Already-expired deadline sends zero; in-flight timing accepts the valid completed prefix and preserves cancellation/facts without new clocks or timeout inflation.
- Defers: Proxy service/screens, unrelated features and Windows Node/C#/JVM packaging remain outside this outcome.

## Dependencies and ownership

0296 owns pandas Series accessor; 0431 owns ordinary Python/Rust file carriers; 0300 owns checked aggregate pricing. Source helpers reuse the native reader, never another parser.

## Vision and answer identity in 0.2

Second PM message `2026-10-06-pm-vision-ships-in-0-2-and-the-sdk-stays-one-endpoint-while-the-proxy-owns-the.md`, asks 1–4, supersedes the image deferral. Adopt 0447 typed single/multiple-image and image-source inputs for decide/choose/score, 0448 route admission, and 0450 stable full-result identity/proxy reservations. Preserve image order/duplicates and absent text lines; ordinary strings/bytes do not imply images. Execute text-only refusals for the other seven functions, with zero sends. This ticket owns its public typed carriers/consumers; shared native image behavior stays in 0447 and SQL adaptation in 0452. Respect 0449’s single endpoint/key/API-type boundary. Known image/result/identity fields cannot remain raw JSON.

## Active dataframe slice, 2026-10-06

Builder: Sol High on `ticket/0410-dataframes-ten-functions`, lane claude-2. Risk: High for Arrow/native ownership, nullable and duplicate row identity, and checked cost aggregation. Main and saved0431 remain read-only; no native engine/core/cache/result edits. Independent frame and optional Series dispatch work is in progress; this is not completion or landing evidence.

## Private slice handoff, 2026-10-06

Implemented Rust filter/rank/find/recognize/relate and explicit whole-set lazy rank/find, plus ordered typed decide/choose/score image columns and native located-file columns. Existing row-wise native/Arrow paths remain the implementation. Consumer checks exercise independent outputs, saved recognition spans/edges, nullable duplicates, complete candidate/entity sets, cancellation, replay and counted zero sends. The deadline proof requires zero sends when already expired and a valid completed prefix during expiry; it requires no second send under load.

Focused offline Polars gate: both Clippy profiles, 29 consumer tests and one doctest passed. The existing 200-record stress case was not run; it remains exclusively under `test-stress --run`. Python installed consumers: 53 passed on pandas 3; the Series/pandas subset also passed 19 on pandas 2. Stress selections were deselected, not claimed passed. Existing Arrow memory-boundary, release, cancellation and replay regressions ran. The additional priced-recognition zero-send refusal passed on pandas 2 and 3. Policy accepted 268 resolved packages; no dependency was added. These are branch checks, not the root's fresh whole-family High review or full landing tests/lint.

Exact native needs: engine-priced checked tally snapshot (0300); original row index remapping for composed native recognition observations; complete-result/2 and explicit named/reference loading from 0443/0456; accepted 0407/0414 context/candidate and question semantics; native selection identity that distinguishes synthetic none from threshold abstention (current real-unit selection flags deliberately do not guess). Score-based/rank-set dataframe adoption and remaining located routes remain open. Python typed image/file carrier integration waits for preserved 0431 interfaces. No legacy result/1 was cast to result/2; no answer IDs, counts or provenance were invented.

No shared `public/mod.rs` export edit is needed for this slice: added methods use the existing exported `PolarsEngine` and existing typed inputs/results. The proposed native tally method needs no new type export. Read-only inspection of committed `cdb452882` did not merge active WIP. Tickets remain open; no landing record or approval is supplied here.
Native consumer handoff, 2026-10-06: `Tally::facts_with_engine(&Engine)` now supplies the checked engine-priced per-call snapshot required by the dataframe aggregate. Input/output availability is independent; unknown output never drops reported input or becomes zero. The dataframe owner still adopts this method and the complete score/rank-set, observation row mapping and selection identity routes; this native constituent does not close column equivalence.


Native consumer views, 2026-10-06: `CompleteFound::selection()` reports the actual mapped unit or explicitly offered synthetic none, including a none tie whose raw backend leading pick names a real unit. `RecordObservation::remap_index` maps retained rows back to original dataframe presentation indices without modifying detail identities, facts, source coordinates or original input. Public tests use independently expected exchanges and retain original observations on coalesced duplicate rows. Complete rank-set and host adoption remain open.

### Added public declarations

```text
enum FindSelection
FindSelection::Unit(usize)
FindSelection::None
fn CompleteFound::selection(&self) -> FindSelection
fn RecordObservation::remap_index(self, usize) -> RecordObservation<'_>
```

Native set-rank handoff: `Engine::rank_set_records_complete_with` and plain/fallible equivalents now supply typed `CompleteSetRank` final/member results through one ordinary engine. Located `RecordInput` originals and explicit contexts use existing composition; member views retain all actual probabilities, identities and partial metadata. Frame adoption remains with its owner.

Native foundation handoff: the existing checked per-call `Tally::facts_with_engine`, observation row remapping and `CompleteFound::selection` are available alongside full complete score/rank-set and composed/located aggregate execution. Typed `CompleteRecognized::source_value` and `CompleteRelated::source_edges` expose actual physical spans and ordered original endpoint occurrences without known-result JSON parsing. Partial reported input survives absent output. The dataframe owner still integrates and checks actual consumers; no column parity/main claim is made.
