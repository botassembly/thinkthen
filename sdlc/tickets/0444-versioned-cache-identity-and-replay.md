# 0444: Version cache keys and preserve offline replay

Status: in progress. Native cache behavior and reviewed host test-oracle reconciliation are implemented, with all four full library checks passing. Integrated source tests, lint and specification checks remain before completion.

Milestone: 0.2

Owner: builder.
Ticket review: ACCEPT, 2026-10-06; fresh read-only review. Conflicting image deferrals corrected where found.

## Outcome

Saved answers carry enough validated identity to rebuild their key without a call. Equivalent endpoint spellings share keys; a changed reported model never serves an old online answer.

## Evidence

- Starts from: Current main c64b71859; 0405 audit is historical and predates 0377/0420. PM message `2026-10-06-pm-0-2-is-not-done-every-sdk-consistent-and-the-sdk-ready-for-the-proxy.md`, asks 10.
- Keeps: Preserve existing bare calls and generic JSON compatibility doors, backend selection from 0377, six error kinds, cancellation, secrecy, count-only usage, and zero-send strict replay. Reuse the Rust engine, C boundary and native file reader; add no host cache, scheduler or second parser.
- Changes: Implement 0442’s versioned pure key, canonical posting URL, requested/concrete model identity and validated offline store conversion. Preserve original rows on failed atomic migration. Read-only replay indexes in memory. Unknown/damaged/incomplete records refuse locally before any send. Final reconciliation owns `libraries/python/tests/**`, `libraries/typescript/tests/**`, `libraries/ruby/tests/**`, `libraries/r/tests/**`, `libraries/python/ratchet.py.json`, `libraries/typescript/ratchet.mjs.json`, `libraries/ruby/ratchet.rb.json`, `libraries/r/ratchet.R.json`, comment only in `crates/thinkthen/src/core/pack/key.rs`, and this ticket, `sdlc/records/0444-versioned-cache-identity-and-replay.md` and `sdlc/planning/team-0-2-2026-10-04.md`.
- Proof: Two processes share keys for trailing-slash/host-case variants; distinct paths differ. Rebuild saved keys offline. Pin valid v1 conversion/idempotence/read-only bytes and tampered/missing-field zero-send refusal. Pinned models cache; opaque selector returning model1 then model2 goes live again and cannot reuse model1. Multiple historical concrete versions refuse ambiguous replay with zero sends. Keep jev-latest refresh and no key/address leakage.
- Defers: Proxy service/screens, unrelated features and Windows Node/C#/JVM packaging remain outside this outcome.

## Dependencies and ownership

0442 defines the deliberate storage compatibility change. 0443 supplies cache-control policy and provenance. No automatic repair send, selector-to-model guessing, migration command or silent downgrade promise.

## Image and observation identity

0447 image evidence enters canonical shared state: actual media and ordered immutable image bytes under the reviewed serialization version, preserving the v2 key’s framed fields. Paths/labels/timestamps remain excluded. Pin byte/media/order changes, text/image separation, relocation equality and zero-send key rebuild/replay. 0450 observation IDs persist outside request identity; derive validated legacy IDs before rekeying and retain them atomically. Read-only replay changes no bytes. If normalization collapses distinct saved snapshots onto one key, refuse before committing rather than choose one silently.

0449 removes SDK group interpretation; retain conservative requested/reported-model comparison, known mutable-alias refresh and ambiguous offline replay refusal. Do not add discovery, target maps or routing. A mutable selector that echoes itself cannot prove freshness: caller no-cache/refresh or provider no-store is required, and future explicit proxy calls bypass local reuse until an admitted policy-version contract proves safety. Do not claim the key formula alone fixes echoed aliases.

When this change is pushed to main, notify the experiments team through pm with the commit, changed behavior and affected experiment 0035 steps. They rerun only affected steps without waiting for release. Note the notification in this ticket’s single landing record.

Main implements versioned text/image identity, validated atomic migration, read-only replay, persistent observations, partial reported usage, original exchanges, held-model diagnostics and bounded timing history; the distinct contracts and native evidence remain in specification/cache.md and records/0443-sdk-call-identity-and-cache-policy.md. This reconciliation completes the four host test oracles and retained cancellation/public-contract assertions, with final integrated qualification still pending.

### Added public declarations

```text
struct ReportedUsage
impl Serialize for ReportedUsage
const fn ReportedUsage::input_tokens(self) -> Option<u64>
const fn ReportedUsage::output_tokens(self) -> Option<u64>
const fn Details::reported_usage(&self) -> Option<ReportedUsage>
fn CompleteDecision::reported_usage(&self) -> Option<ReportedUsage>
fn CompleteChoice::reported_usage(&self) -> Option<ReportedUsage>
fn CompleteTags::reported_usage(&self) -> Option<ReportedUsage>
fn CompleteScore::reported_usage(&self) -> Option<ReportedUsage>
fn CompleteFilter::reported_usage(&self) -> Option<ReportedUsage>
fn CompleteRank::reported_usage(&self) -> Option<ReportedUsage>
fn CompleteFound::reported_usage(&self) -> Option<ReportedUsage>
fn CompleteRecognized::reported_usage(&self) -> Option<ReportedUsage>
fn CompleteRelated::reported_usage(&self) -> Option<ReportedUsage>
fn Details::question_sources(&self) -> &[QuestionSource]
fn Details::observations(&self) -> &[Observation]
```

Focused public/CLI cases in native_store cover read-only bytes/mtime, independent expected framing/legacy identity, v1 migration/idempotence, collision rollback, malformed unrelated entries in all modes, changed reported models and offline ambiguity, 13/5/8 per-question missing pieces, coalescing/refresh observation identity, and original body conversion/replay. These are constituent checks, not whole-ticket acceptance. Root owns the experiments notification after main landing.

```rust
fn Details::question_sources(&self) -> &[QuestionSource]
fn Details::observations(&self) -> &[Observation]
```

Native correction: v2 explicit images retain the `thinkthen.image-question-key/2`
framing domain alongside image-state/1. Ordinary text keeps question-key/2.
The existing validated state digest selects the domain during migration and
snapshot replay; an image-looking ordinary JSON value does not infer images.
A prior-failing independently framed public recording regression now passes,
including original ordered duplicate attachments and zero-send strict replay.
All 24 image and nine storage cases plus affected Clippy passed before the native foundation landed.

Lane1 legacy-key diagnosis: the saved C integration log's two failures are
`cases::portable::c_json_records_keep_fixture_questions_and_keys_in_one_request`
and `cases::every_applicable_shared_case_passes_through_the_door`. Committed C
consumer be26fdc13 computes LF-joined v1 keys in tests/door/cases/wire.rs; the
shared-case planner uses that same helper. Native public calls reproduce the
distinction for the portable questions and shared case 01: wire questions remain
byte-identical, while metadata keys match independently framed question-key/2.
The accepted cache contract, committed in e3d665981, explicitly rules under
Validation and migration: "`meta.requests` uses v2 question keys after adoption,
requiring one controlled example/fixture update by 0444." ADR 0120 decisions 3–4
retain original v1 validation before rekeying and committed offline fixtures.
The added public regression replays original valid v1 rows without sends or
file writes; existing recording/1 conversion regressions remain passing.
No native compatibility exception or C expectation edit is made here. Root must
coordinate the authorized metadata-oracle adoption with the C/corpus owners;
original v1 stored keys and request-envelope validation remain distinct.

Authorized C/shared legacy oracle adoption (0426 integration): the Rust public
consumer and C share one independent specification/cache.md framed v2 helper.
Reported model is supplied explicitly from each saved response; the final URL
is the actual owned loopback fixture endpoint. Canonical legacy request-digest
placeholders still identify their original request, then map to these v2 keys.
No production hash, saved v1 row/recording, question bytes or packed counts change.
Synthetic case 40 previously requested jev-1.13.0 but reported jev-latest while
expecting a cache hit. Under v2 freshness that mismatch correctly resends; this
controlled synthetic fixture now reports jev-1.13.0 and its detailed model
expectation agrees. Its exact request and two-calls/one-send/one-cache-answer
expectations stay intact. Captured historical mismatches are preserved.
Lane1 CLI oracle correction on actual native-foundation candidate 1bbcc0bb1:
the full-test replay failure reproduces in shared case 01 (v2 1c38… versus
legacy v1 2bc2…). Current facade/staged metadata now uses the accepted shared
oracle from 54c557c3e, including each saved response's literal reported model.
The unchanged v1 helper stays with historical captured-fixture validation;
original exchange digest placeholders, questions and recording bytes stay
distinct. The all-selected command wire run also reproduced 30 key failures
from substituting the requested model for different saved reported models;
its endpoint mapping now reuses that same shared oracle. No C/corpus or
production key/hash change, regenerated expectation, ignore or weakened
identity assertion is introduced.

The affected Relate replay example independently reproduces its original
v1 missing-pair key from unchanged state/question bytes; documented v2 framing
gives the current 9be378… key. Only that diagnostic's expected hash changes;
the complete page passes all four blocks and its historical recording stays
unchanged. Focused replay case passes, the whole private CLI corpus passes
seven tests (two existing child entrypoints remain runner-owned), all 55 wire
cases are selected with 46 passes and the same nine in-process/repacked cases,
and all ten public native-store tests pass, including byte/mtime read-only
replay, distinct v1/v2 identity, mismatches and original body conversion.
Library/backend Clippy, offline policy (268 packages), formatting and ratchet
pass. Additional `cargo clippy -p thinkthen --tests -- -D warnings` fails on
the unchanged image test's unfulfilled `clippy::unwrap_used` expectation at
`core/adapters/systemone/images/local_tests.rs:16`; no allowance, ignore or
out-of-scope image edit is made. Root retains this broader lint finding.

Measured source grows 17 nonblank Rust lines (143995 → 144012): reuse the
already accepted shared metadata map, keep the historical validator separate,
and remove the wire runner's duplicated model assumption. The four affected
Rust files have 487, 298, 465 and 476 nonblank lines; no new framework or
dependency is needed. Checks use empty owned configuration, stripped key
variables, offline two-job 10G/1G scopes and lane/shared locks. The count-only
private-name scan checks 35 patterns: zero path hits, one starting/current
tracked-file hit with an unchanged hit set, zero edited-file hits. Root retains
that existing scan issue and reruns full gates on the actual new merge
candidate; overall native/host parity remains open.

The bounded cache-contract reconciliation from c0551ed17 reproduces all 23
collected workspace failures. Fixture corrections follow cache.md:24,54–60:
matching-model cache tests serve their literal requested model; current keys
use the accepted shared v2 oracle with the saved reported model; unversioned
fixtures and legacy SQLite rows retain independently checked LF-framed v1
keys. The original pinned v1 conversion keys remain asserted. Conversion
matches expected state/origin rather than historical hash order. Conflicting
histories now assert local refusal and unchanged sources; the existing lock
test retains a writer's distinct row. Damaged-answer cache/replay tests assert
zero sends, stopped facts and preservation instead of the superseded resend.
Hostile fixtures retain their secrecy and request-count checks while pinning
the safe v2 validation diagnostic. Original saved recordings remain unchanged.

Two production defects are corrected: SQLite snapshot validation names the
actual SQLite source, and refusal of a damaged JSONL fixture validates before
creating SQLite. The latter independently failed the public cache regression
by leaving an empty competing store. Corrupt image cache and replay both
assert exit 5, empty output, zero additional sends and unchanged database
bytes. The separate usage-decoy failure is test isolation: the two live CLI
cases in native_named_questions inherited XDG_STATE_HOME. Their existing
helper now clears the environment and reuses the platform child helper for
owned configuration/cache/usage, retaining exact requests and completed-prefix
assertions. No key formula, result serializer, C/corpus or active lane0 file
changes are made.

All 26 focused cases pass with a clean usage guard. Final existing workspace
`cargo nextest run --locked --offline --workspace --all-targets --no-fail-fast
--test-threads 2` passes all 1,616 tests with the same 26 existing skips and a
clean usage guard. Policy checks 268 resolved packages; formatting, affected
`cargo clippy -p thinkthen --tests --locked --offline -- -D warnings`, and
exact ratchet pass. Checks use stripped key variables, empty owned config,
bwrap-isolated state, offline two-job 10G/1G scopes and lane/shared locks.
Measured source grows 106 nonblank Rust lines (144011 → 144117): 27 production
lines for source context and pre-creation validation, 79 test lines for the
independent legacy/v2 assertions and owned state checks. The shared v2 oracle
is loaded once per backend test binary, removing its duplicate module import;
legacy framing stays separate. Every changed Rust file remains at or below
500 nonblank lines. Count-only private-name checks retain one unchanged
starting tracked-file hit, zero path hits and zero edited-file hits. No
collected failure or legacy-key contract conflict remains; root retains that
starting scan issue, narrow confirmation and the final actual landing ladder.
Overall parity remains open.
Held-model warning constituent WIP: cache lookup checks a missed exact key for validated historical answers on the same configured URL, literal requested model, selected state and wire question. Excluded mismatches trigger a fixed command warning and a call-scoped `Facts::held_model_mismatch()` getter, with optional true-only complete-facts serialization. Stored model/address values never enter the warning. Exact corrected hits, unrelated questions and offline historical replay remain silent; ambiguous replay still refuses before sends. Primitive keys, grouping, image domains and accepted observation identity are unchanged. The getter is recorded in 0443's canonical inventory delta. Bounded timing history and final whole review/host adoption remain open.

## Final test-oracle adoption, 2026-10-07

The bounded design found no remaining native cache defect. This slice owns `libraries/python/tests/**`, `libraries/typescript/tests/**`, `libraries/ruby/tests/**`, `libraries/r/tests/**`, their measured host ratchets, and the stale comment in `crates/thinkthen/src/core/pack/key.rs`. SQL0435 owns its active DuckDB reconciliation. Original v1 fixtures and whole-request exchange digests remain unchanged.

The four existing test key helpers take the reported model explicitly from each authored saved exchange response or the exact authored loopback response rule. They use the settled NUL domain and unsigned 64-bit big-endian UTF-8 lengths while retaining original compact state/question bytes and numeric qN order. Python uses the existing standard `JSONDecoder.raw_decode` to retain raw member spans inside its test oracle; malformed or noncompact objects refuse explicitly. This is a test-only standard parser use, with no new production parser, grammar or framework. Existing raw-member helpers in the other hosts are retained. Broad equivalence assertions may compare independently matched native result keys; existing narrow Rust exact framing/legacy oracles remain independent.

Run the actual four existing full library checks with required toolchains and no exit77. Their internal shared-case checks remain intact; retain prior SDK source/installed 248 qualification and add no separate unchanged archive campaign. Fresh whole-change review precedes the single0444 record and final full test/lint/spec.

The helper changes add 22 measured Python, 10 TypeScript, 4 Ruby and 10 R test lines. Existing raw-member extraction is reused; R factors its existing host SHA call into one byte-hashing helper so whole-request digests keep their original prefix without duplicating tool invocation. The TypeScript listener retains its actually authored response model beside each captured body. No dependency, production hash or SQL source changes.

The full Python check exposed a cache-hit tally test whose default requested model differed from its authored listener's reported model. Its engine and authored listener now explicitly use the same pinned `jev-1.13.0` model, retaining the intended cache-hit and tally assertions. Using `jev-latest` would force refresh under the retained native policy. Other listener calls keep their existing reported-model default. The same check exposed an unrelated stale built-in backend list; its refusal expectation now includes the supported `openai` backend. Both corrections change test setup or expected text only.

The full TypeScript check exposed two stale test contracts. Legacy scalar details remain result/1 while the CLI emits result/2; their answer and original metadata fields still compare exactly, both schema versions are asserted, and reader-position checks remain. The export test now recognizes the existing `export * as CompleteTypes` declaration alongside classes, functions and constants. The measured TypeScript test total grows by seven lines to retain these checks explicitly. Ruby's exact public-name test now includes its shipped `Complete` namespace and `Engine#complete` method. R's explicit native arity and positional-refusal checks now pin the reviewed 15-parameter signature including `refresh_cache`; arbitrary invalid names/types, partial-name refusal and zero-send checks remain. No runtime export, result decoder or native serializer changes.

The R vendored consumer exposed a held-cancellation test race: cancelling after 150 ms could precede the request. The reviewed two-file correction advances once through the existing public poll, requires no completed row, waits for the owned listener's exact next-request acknowledgement through its existing 30-second wait guard, and then permits cancellation through the existing stdin `continue` protocol. Preserve the one-second request timeout, held reply, exact native facts and independent one-send assertion, cancellation/error/secrecy checks, and existing pre-pull zero-send protocol. Only the created consumer is killed and reaped on handshake failure; outer backend cleanup remains. Other host routing and production APIs remain unchanged. Reusing the existing child lifecycle adds five measured Python lines and replaces three R timer lines; totals are 7740 Python and 3285 R.


Code review: ACCEPT, 2026-10-07; fresh whole-change read-only Sol review of 2b9ce8b25904a4180ce82afbc8b33c864bd1285f. The separately reviewed 0460 planner is adopted from main f723dde1d; affected host tests retain exact preview and runtime assertions while pinning the conservative occurrence bound.

Reviews: accept
