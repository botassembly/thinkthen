# 0478: Supply context for recognition stages

## Contract and sources

This implementation starts from main `93f667042`, after PostgreSQL Request adoption. [Accepted ADR 0126](../planning/adr/0126-recognition-stage-context-and-boundary-proposals.md) owns the stage-context contract; its boundary mode belongs to 0479. The existing canonical Request owns serialized admission. The existing saved reader owns version-one question declarations. These are repository contracts, not evidence about model accuracy.

## Findings and actions

`RecognitionStageContext` defines the three exact optional strings once and withholds them in Debug. Canonical Request options and the saved declaration use this same type. Saved schema projection derives the type through the existing schema generator. Call members overlay saved members independently. Each executed stage resolves its context against the existing record/shared fallback on an engine clone. Empty strings remove the stage fallback; omitted controls preserve the existing path.

The canonical description retains supplied members, including empty strings, and omits an empty object. Complete reading exposes the retained typed object. The reading digest clears stage controls before hashing; aggregate identity retains the resolved declaration and actual observations. Request bodies and ordinary question identities carry each effective stage context.

Boundary admission prepares exact adapter bodies with boundary context. The existing static kind-menu probe uses kind/edge context. Future edge and relation bodies depend on earlier answers and retain their stage-local checks before sends; earlier stages may have sent when a later body fails. No claim covers unknown future request sizes, paid backend accuracy, generated host adoption, release qualification or boundary-only mode.

The counted CLI case compares complete request objects with the existing saved exchange fixture after replacing only stage state. It preserves authored kind order by reading `question_json`; constructing the saved object through sorted `serde_json::Value` changed generated wording and the test refused it. The native case proves a changed boundary sends one request while unchanged kind answers remain reusable; clearing both contexts uses the existing no-context body. The exact-byte case covers escaped Unicode context on both admitted text adapters and refuses one byte below the necessary body limit with zero additional requests. Invalid selected record context still refuses while all stages override it.

## Commands and evidence

Commands run from the ticket worktree. Scoped Cargo commands use `systemd-run --user --scope -p MemoryMax=8G -p MemorySwapMax=1G` with `CARGO_NET_OFFLINE=true CARGO_BUILD_JOBS=2 CARGO_TARGET_DIR=target/0478`.

- `git fetch origin main`; `git switch ticket/0478-recognize-stage-context`; `git rebase origin/main`: clean start at the cited main revision.
- `CARGO_NET_OFFLINE=true python3 sdlc/scripts/policy.py`: passed. Existing source-size warnings remain. CLI arguments grew by the three literal controls, native Recognize by its typed setters, and Request execution by shared overlays; those changes stay at their existing owning boundaries and below enforced caps.
- `cargo check -p thinkthen --all-targets`: passed.
- `cargo test -p thinkthen stage_context`: passed parser, counted CLI saved/replay and native stage/cache cases. An initial native test incorrectly treated the listener's draining `requests()` accessor as cumulative; correcting its evidence collection made the intended checks pass.
- `THINKTHEN_WRITE_SCHEMA=1 cargo test -p thinkthen --lib schema`: regenerated Request and result schemas and failed as its write contract requires.
- `cargo test -p thinkthen --lib schema`: passed all six selected schema cases after regeneration.
- `cargo test -p thinkthen --test backend stage_context`: passed saved/call/record precedence and exact encoded size cases.
- `cargo fmt`; `git diff --check`: passed.

## Added public declarations

The literal declarations below describe the implemented surface for the owning ticket's API inventory contract. They introduce no second wire grammar.

```rust
pub struct RecognitionStageContext {
    pub boundary: Option<String>,
    pub kind_edge: Option<String>,
    pub relation: Option<String>,
}
impl RecognitionStageContext {
    pub const fn is_empty(&self) -> bool;
}
impl Recognize {
    pub fn with_stage_context(self, context: RecognitionStageContext) -> Self;
    pub fn boundary_context(self, text: &str) -> Self;
    pub fn kind_edge_context(self, text: &str) -> Self;
    pub fn relation_context(self, text: &str) -> Self;
}
impl RecognitionReading<'_> {
    pub fn stage_context(&self) -> &RecognitionStageContext;
}
pub RequestOptions::stage_context: Option<RecognitionStageContext>;
```

## What the build taught us

The existing aggregate context and ordinary answer cache already supply the right stage boundary. Resolving a stage on an engine clone changes that stage's request state without adding scheduler policy or a parallel cache domain. Exact fixture comparison also protects the authored order of kind descriptions; parsing a saved question through a sorted generic object can change its actual question wording.

API inventory ran with `python3 sdlc/scripts/inventory` under the same scoped Cargo environment. It refused precisely the new stage-context exports absent from the ticket contract, plus the new type's `Default`, `Deserialize<'de>` and `Serialize` implementations. It checked 1,838 existing declared items and refused its four planted faults. Ordinary withheld `Debug`, `Clone` and `PartialEq` are accepted extra traits by the existing inventory rule. The owning ticket supplies these literal additions alongside this implementation.

```rust
impl Default for RecognitionStageContext;
impl<'de> serde::Deserialize<'de> for RecognitionStageContext;
impl serde::Serialize for RecognitionStageContext;
```

The lane measurement covered `target`, `libraries` and `databases`: 41,436,762,112 bytes, or 38.59 GiB, below the 40 GiB lane cap. Ticket-owned `target/0478` held 2,050,256,896 bytes before the inventory build. Full gates reuse the lane's existing warm output after this ticket-owned scratch is removed. `RequestFunction::allows_option` classifies `stage_context` as recognize-only so generated tool option schemas match native admission.

The aggregate complete serializer intentionally omits `meta.question_sha256`. The reading-digest assertion therefore belongs to the existing canonical saved-reader contract test, where it compares actual digests before and after stage controls. The native test instead pins changed aggregate answer IDs and reused unaffected requests.

`cargo test -p thinkthen recognize` passed the affected saved-reader, backend, native complete and recognition parser cases after the final source changes. `cargo test -p thinkthen --lib schema` also passed after the recognize-only applicability update. Policy passed again after the completed code and schema changes. All calls used saved responses or owned loopback listeners; no paid calls or release operations ran.

## Settings and source ceiling

The settings reference names the three literal CLI controls, the saved object and native canonical Request controls. It marks other typed SDK adoption pending and leaves unrelated historical context/kind rows to their existing owner. The existing ratchet measured 173,788 nonblank Rust source lines against its prior 173,298 ceiling. The ceiling now equals the actual measurement. The growth adds the shared context type, edge adapters and outside-in precedence/body/cache checks. Duplication review compared saved parsing with canonical Request deserialization, execution with the existing aggregate-context helper, and new tests with the existing saved exchange fixture. Those paths share the typed contract, existing context scheduler and existing fixture responses; no second parser or cache domain was added.
