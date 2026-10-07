# 0410: Complete all ten dataframe functions and located files

Status: reviewed and qualified. All three source and installed dataframe consumers passed 248 required cases each; fresh whole-family High review ACCEPT at `9c8ba1817`. Final full gates and coordinator landing remain.

Milestone: 0.2

Owner: builder.
Ticket review: ACCEPT, 2026-10-06; fresh read-only review. Conflicting image deferrals corrected where found.

## Outcome

pandas, Python Polars and Rust Polars expose all ten functions, equivalent identity/results/errors/storage, and typed located-file routes.

## Evidence

- Starts from: Existing ticket and 0405 audit; landed 0377/0401D/0420 are the current baseline, replacing the older audit-only matrix. PM message `2026-10-06-pm-0-2-is-not-done-every-sdk-consistent-and-the-sdk-ready-for-the-proxy.md`, asks 4 and 7.
- Keeps: Preserve existing bare calls and generic JSON compatibility doors, backend selection from 0377, six error kinds, cancellation, secrecy, count-only usage, and zero-send strict replay. Reuse the Rust engine, C boundary and native file reader; add no host cache, scheduler or second parser.
- Changes: Remove container-only refusals; add Rust Polars filter/rank/find/recognize/relate through the existing engine. Preserve nulls, duplicate index/name, whole candidate/entity sets, spans and relation endpoints. Lazy whole-set helpers explicitly materialize the logical collection where required; do not claim per-morsel streaming equivalence. Fix the deadline test’s invalid demand for two sends before expiry under load. Ownership: `crates/thinkthen/src/public/frame.rs` `crates/thinkthen/src/public/frame/**` `crates/thinkthen/tests/polars/**` `libraries/polars/**` `libraries/python/**` (settled shared SDK fixture stays read-only) `libraries/r/thinkthen/src/rust/src/complete/inputs.rs` `libraries/r/ratchet.json` `libraries/typescript/ratchet.json` `libraries/ruby/ratchet.json` `sdlc/scripts/release-workflow` `sdlc/scripts/surfaces` `sdlc/surfaces.txt`.
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

## Complete dataframe API, 2026-10-07

The complete dataframe facade is additive. `thinkthen.frames.Engine` shares a selected existing engine and exposes the ten named calls over pandas or Polars Series of text or explicit typed `complete.Item` inputs. `FrameCompleted` exposes the unchanged native `Completed`, its typed facts/results/inputs, and original frame, index, name and nullable positions separately. Optional `series.tt.complete(engine=...)` selects the same facade. Files use the existing typed native `complete.Files` reader and produce located dataframe results. Lazy whole-set calls collect the logical Series once. Bare released returns retain their existing shape.

Rust `Engine` complete dataframe helpers accept actual text Series or typed nullable record columns, delegate the existing complete native calls, and return typed native rows with original nullable positions separately. Source columns use the one native reader. No native null input, rewritten answer/call ID, host cache/scheduler, money arithmetic or second parser is added. Existing shared case runners/projectors execute each actual frame consumer. Root accepted this additive API within the reviewed outcome; the final whole-ticket review checks its declarations and implementation.

Rust typed input columns associate a nullable `Vec<RecordInput<T>>` with an actual Series. The public helper validates row count and null occurrence positions before calling the native engine. The Series supplies presentation identity; the explicit typed carriers supply selected input, context, images and physical provenance. Native judgments retain compact occurrence ordinals, while the returned position map retains actual frame rows. A bare native Vec call does not establish dataframe support. Each shared Rust Polars case exercises this public association with a real frame column.

The current native priced recognition collection supersedes the earlier priced refusal. The focused Polars tests pass 31 cases; the separate existing stress case remains ignored in this routine run. The pricing case captures four actual requests, compares their saved question/state bodies, and pins 196 reported input tokens and $0.000002. The scripted listener's old `count()` stays zero while its `requests()` captures every send; this consumer uses the actual captured length. No listener or proof tool was changed. Question-key expectations follow the independent result/2 formula in `specification/cache.md` and retain original request bytes. Annotation context now succeeds through the native implementation.

Python presentation preserves MultiIndex names and captures original index/name before completion or batch iteration. Completed native facts, IDs and compact ordinals remain unchanged. The actual archive consumer uses the same Polars method calls and shared native fixture parser/projectors as the source consumer. Offline policy accepts 268 resolved packages. Shared Surface admission, typed rank member adoption and the new shared rank-set case remain prerequisites for final dataframe acceptance; these focused checks do not claim the full matrix or ticket completion.

`FrameCompleted.source` and `FrameBatch.source` preserve the actual supplied Series, DataFrame, LazyFrame or Files carrier. The selected column supplies captured presentation metadata. Duplicate present rows can coalesce to one native request while retaining two actual native results and two original frame positions; the frame adapter does not force additional sends.

Missing dataframe columns raise the retained Usage error with a content-free message before any native send. pandas and Polars consumers count this refusal separately from invalid rows, cancellation and already-expired deadlines.

The installed Rust Polars archive exposed a native prerequisite: canonical decide/choose/score results omitted images when the native reader supplied `QuestionInput::Images`. The existing native ancillary-image helper will copy that carrier's existing ordered typed image collection, as it already does for record images. File image results retain actual bytes/media/dimensions and the existing physical filename, with absent text line coordinates. No new parsing, schema, input, ID, pricing or surface policy is introduced. Root assigned this helper hunk and the existing native file-image regression to this slice. The preliminary archive matrix executed all 247 current rows; 15 image-only propagation rows failed and all others passed. Final acceptance still requires the new shared rank-set row and complete reruns.

The focused file-image result regression failed on the missing canonical image field before the helper change. Afterward, all five existing native composition tests and native complete Clippy pass. The file case exercises all three concrete native calls, exact saved image bytes, source coordinates and strict result schemas. Offline policy still accepts 268 resolved packages.

Final adoption uses the settled native closed `pandas` and `python-polars` Surface values. A frame-owned thin bridge forwards eager completion and native batch creation to the same selected native engine with that explicit value. The existing complete facade still owns request construction, decoding, cancellation, lifetime and close. The bridge does not mutate the shared ordinary engine or introduce a native engine, cache, scheduler or parser. Actual typed ordered rank members and their facts/provenance remain the SDK's native carriers, including independent optional usage dimensions; call facts remain the invocation total.

## Current qualification, 2026-10-07

Source pandas and Python Polars consumers passed all 248 required shared cases at `01db05a0b`, with strict frame typing and no skipped required cells. Non-file eager pandas cases use the actual `Series.tt.complete` accessor. Preloading the selected library before the existing cancellation timer corrected seven zero-send fixture cancellations; the native cancellation and timer remain unchanged.

Rust source and the rebuilt public consumer linked to the unpacked crate each passed all 248 required shared cases at `9e9739b2c`, with no skipped cells. The bounded shared preparation helper corrected premature fixture admission; ordinary SDK checks retain their order and the native frame batch enforces its actual pull limit. The host-setup declaration now includes the conventional Polars consumer manifest. Policy and full pre-review lint passed at `535b6d94a`, including the existing registry plants, workspace Clippy, documentation and API inventory. Fresh whole-family review and final landing gates remain open. Installed pandas and Python Polars release-wheel consumers also each passed 248 cases at `944ce9862`; the unchanged Python runtime qualification is retained while the final wheel rebuild and affected checks cover the additive helper.

The coordinator assigned the bounded shared fixture correction: an additive private preparation helper reuses the existing typed composer and native reader. Ordinary SDK `iter`/`read` retain their engine record-limit checks in the same order. The actual frame batch receives prepared typed inputs and owns pull-time admission. The existing host-setup manifest list will include the conventional Polars consumer. No core/public API, corpus, additional engine or runner is changed.

The coordinator assigned the existing feature registry declaration to admit exactly the Polars consumer manifest, lock and `src/main.rs`. Other feature files and a Rust binding misdeclared as a feature remain refused. The final release wheel rebuilt successfully; installed affected record-limit and ordered rank-member cases passed on both Python dataframe surfaces with strict typing, and all seven frame regressions passed. Full lint exposed this registry declaration after its earlier guards passed; full lint will rerun after the bounded correction.

The shared helper adds thirteen nonblank Rust lines. Both R and Python ratchets include that same existing module, so their measured ceilings are 3431 and 9487. Python runtime carriers and ordinary admission semantics are unchanged.

TypeScript and Ruby also include the same shared complete fixture module in their measured Rust totals. Their ceilings rise by the same thirteen lines to 2328 and 2773; no host implementation changed.

Review candidate qualification: all three source and installed public dataframe consumers passed 248 required shared cases each with no skips. The final rebuilt wheel retained its unchanged full-matrix qualification and passed affected record-limit, ordered rank-member, strict frame type and seven frame regression checks. Existing registry plants, policy and full pre-review lint passed at `535b6d94a`. This checkpoint changes only the true qualification note and assigned metadata ownership. No family landing record or final test/spec completion is claimed before the fresh whole-family review.

## Reviewed family checklist, 2026-10-07

Fresh native Sol High review ACCEPT `9c8ba1817322b8b013237ee1c36f95c3da7fd16d` against `e2641d87c`; no blocking findings. One family record covers 0410, 0296 and dataframe adoption of the shared semantic tickets.

- [x] All ten named functions on actual pandas, Python Polars and Rust Polars consumers; 248 required source and installed cases per surface, no skips.
- [x] 0296: eager pandas cases use the actual complete Series accessor; original frame, null positions, duplicate index/name and MultiIndex metadata survive separately from native identities.
- [x] 0300 dataframe adoption: native checked once-rounded prices and independent partial token dimensions; no host money arithmetic or rewritten facts.
- [x] 0407/0414 dataframe adoption: per-record and shared contexts remain separate from evidence, file coordinates and native identities.
- [x] 0411/0418 dataframe adoption: native cache/record/replay and ordered typed rank members preserve their own results, facts and provenance without extra judgments or usage.
- [x] Typed file/image inputs, physical spans/edges, eager/lazy native Surface identity, cancellation, secrecy and zero-send refusals use existing native carriers and fixtures.
- [x] Existing policy, registry plants and full pre-review lint; fresh whole-family High review.
- [ ] Full test/lint/spec on the fixed candidate containing this checklist and the single family record; coordinator landing follows their success.

Cross-surface 0300, 0407, 0411, 0414 and 0418 outcomes remain with their other family/SQL/MCP owners. This checklist establishes dataframe adoption only. Platform/release qualification and publication remain separate.
