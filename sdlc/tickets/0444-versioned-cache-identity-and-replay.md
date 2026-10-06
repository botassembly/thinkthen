# 0444: Version cache keys and preserve offline replay

Status: in progress. Native implementation in lane0 on ticket/0443-native-complete-results; host adoption and final landing checks remain open.

Milestone: 0.2

Owner: builder.
Ticket review: ACCEPT, 2026-10-06; fresh read-only review. Conflicting image deferrals corrected where found.

## Outcome

Saved answers carry enough validated identity to rebuild their key without a call. Equivalent endpoint spellings share keys; a changed reported model never serves an old online answer.

## Evidence

- Starts from: Current main c64b71859; 0405 audit is historical and predates 0377/0420. PM message `2026-10-06-pm-0-2-is-not-done-every-sdk-consistent-and-the-sdk-ready-for-the-proxy.md`, asks 10.
- Keeps: Preserve existing bare calls and generic JSON compatibility doors, backend selection from 0377, six error kinds, cancellation, secrecy, count-only usage, and zero-send strict replay. Reuse the Rust engine, C boundary and native file reader; add no host cache, scheduler or second parser.
- Changes: Implement 0442’s versioned pure key, canonical posting URL, requested/concrete model identity and validated offline store conversion. Preserve original rows on failed atomic migration. Read-only replay indexes in memory. Unknown/damaged/incomplete records refuse locally before any send.
- Proof: Two processes share keys for trailing-slash/host-case variants; distinct paths differ. Rebuild saved keys offline. Pin valid v1 conversion/idempotence/read-only bytes and tampered/missing-field zero-send refusal. Pinned models cache; opaque selector returning model1 then model2 goes live again and cannot reuse model1. Multiple historical concrete versions refuse ambiguous replay with zero sends. Keep jev-latest refresh and no key/address leakage.
- Defers: Proxy service/screens, unrelated features and Windows Node/C#/JVM packaging remain outside this outcome.

## Dependencies and ownership

0442 defines the deliberate storage compatibility change. 0443 supplies cache-control policy and provenance. No automatic repair send, selector-to-model guessing, migration command or silent downgrade promise.

## Image and observation identity

0447 image evidence enters canonical shared state: actual media and ordered immutable image bytes under the reviewed serialization version, preserving the v2 key’s framed fields. Paths/labels/timestamps remain excluded. Pin byte/media/order changes, text/image separation, relocation equality and zero-send key rebuild/replay. 0450 observation IDs persist outside request identity; derive validated legacy IDs before rekeying and retain them atomically. Read-only replay changes no bytes. If normalization collapses distinct saved snapshots onto one key, refuse before committing rather than choose one silently.

0449 removes SDK group interpretation; retain conservative requested/reported-model comparison, known mutable-alias refresh and ambiguous offline replay refusal. Do not add discovery, target maps or routing. A mutable selector that echoes itself cannot prove freshness: caller no-cache/refresh or provider no-store is required, and future explicit proxy calls bypass local reuse until an admitted policy-version contract proves safety. Do not claim the key formula alone fixes echoed aliases.

When this change is pushed to main, notify the experiments team through pm with the commit, changed behavior and affected experiment 0035 steps. They rerun only affected steps without waiting for release. Note the notification in this ticket’s single landing record.

Native WIP partial usage: optional reported dimensions now remain independent through decoding, wire shares, SQLite/JSONL lookup, zero-send replay and aggregate receipts. Explicit recording retains exact request and response bodies (no headers) in the answers' SQLite transaction. The Imajev fixture reports input887 and no output; no output zero is synthesized. Result/1 retains its full-count usage projection, and result/2 metadata and typed accessors retain partial counts. Versioned key/migration execution is now concrete WIP; whole native review, held-model warning integration, schema/corpus adoption and final landing remain open.

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

Native WIP storage slice: framed question-key/2 includes the normalized existing posting URL and requested/reported models, outside batch and persistent IDs. Writable v1 stores validate every constituent and normalize in one rollback-capable transaction; read-only replay indexes the same validated snapshot in memory. Conflicting normalized histories and ambiguous historical models refuse before sends. Existing image digest/key domains stay available for original v1 validation. SQL hit validation covers text and image constituents in the hit's read snapshot. Undecodable saved answers refuse rather than trigger a repair send. Explicit original exchange bodies now survive SQLite-to-fixture conversion; no request header or transient ID is saved. Counts beyond SQLite integer bounds refuse storage atomically rather than silently become unknown.

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
All 24 image and nine storage cases plus affected Clippy pass. Whole review and
main landing remain open.

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
