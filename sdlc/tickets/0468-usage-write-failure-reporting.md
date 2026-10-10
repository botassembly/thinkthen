# 0468: Report failed persistent usage writes to SDK callers

Status: COMPLETE. The sweep found that the deferred writer-failure issue still has a deterministic silent-failure path.

Milestone: 0.2

Owner: builder.
Severity: medium accounting correctness.

Reviews: revision 19abe6542, accept

Reviews: revision 95fade3863f777142ecd49d4abca0efcf1790cce, accept

Reviews: revision 0b41278a00fa05f0088366b20d077682422a9758, accept

Reviews: revision a35156540a08bf97f4dc807a6f91d7dd9ca872f3, accept

Reviews: revision a9b090f8cb6ae8441ff8fd8ff53c6c17e93902c9, accept

Reviews: revision 0c51ef0101898d4d7e58b5123f73ac95c07b3b98, accept

Reviews: revision e67543f4499430721c0080ac6ea8252c99bac307, accept

Reviews: revision cb45c252eb3ba5ecdf4e30ae2d2d5b01a6354d26, accept

Reviews: revision 2ed3348b4501b89a35ee8ebf07c71b30126b7258, accept

Reviews: revision ada5e4f3e88b2120a5bf4efc304a89d8c87fb478, accept

Reviews: revision 70c687a3641d7a2daf32b94822c4813bc5f45151, accept

Reviews: revision 7b972e7c7c150caa85c9f6346149e7b7ff56cc1f, accept

Landed: 142a02c

## Outcome

SDK and SQL callers can discover failed persistence of usage counts without exposing evidence, credentials or private filesystem paths. In-memory call facts remain correct; persistence failure never silently claims durable totals are complete.

## Evidence

- Starts from: issue 2026-09-30-a-usage-write-that-fails-after-a-good-start-is-silent-on-the-libraries.md; engine/usage.rs latches Queue.failed, and engine/facade/finish.rs discards finish's failure. CLI has a warning but the public Engine::finish_usage returns no failure. The after-sprint sweep promotes this known runtime defect into 0.2.
- Keeps: Count-only persistence, private files, the existing usage-lock deadline, no additional model call, existing in-memory facts and successful answers. No secret or authored content in warnings.
- Changes: Add nonblocking Engine::usage_persistence() and Engine::finish_usage_status() with the existing usage-lock deadline, returning UsagePersistence; retain void finish_usage() for compatibility. States are Disabled (no usage path), Pending (queued/writing), Written (this engine’s current deltas drained), and latched Failed (writer/queue failure, taking precedence). Written says nothing about future calls or other engines/processes. Only usage-lock acquisition has a deadline; other filesystem work can take longer. Give all language engines explicit equivalent methods and the C JSON door a status request. Add thinkthen_usage_status() to all three SQL extensions without changing thinkthen_usage() output; PostgreSQL aggregates built engines and DuckDB retains failure when engines are evicted. Only fixed safe advice is exposed. Never infer durable status from call facts or turn a valid answer into a backend failure.
  Implement the shared state and facts contract first. Claim `crates/thinkthen/src/engine/usage.rs`, `crates/thinkthen/src/engine/usage/**`, `crates/thinkthen/src/engine/facade/finish.rs`, `crates/thinkthen/src/engine/facade.rs`, `crates/thinkthen/src/engine/call_facts.rs`, `crates/thinkthen/src/public/engine.rs`, `crates/thinkthen/src/public/options.rs`, `crates/thinkthen/src/public/mod.rs`, `crates/thinkthen/src/public/results.rs`, `crates/thinkthen/src/public/results/call.rs`, `crates/thinkthen/src/public/results/complete_facts.rs` and existing usage/facts consumer tests. Name any further implementation seam individually. Generated-family adoption tickets carry this settled observation into host methods in 0.2. Subsequent C/SQL status slices name their adapter files when assigned; no current claim covers every library, database or public module.
- Proof: Fresh design/ticket and code reviews. Existing writer failure fixtures plus public consumers prove Pending during a held write, Written after successful finalization, latched Failed after an owned write failure, Disabled without storage, safe advice, unchanged call facts and exact request counts. SQL status remains callable before exit; exit hooks alone are insufficient. Applicable full checks at landing; no hosted run or paid call.
- Also changes: Under the 2026-10-08 binding architecture ruling, expose the safe persistence state in the shared facts JSON with its observation point. A result's earlier Pending snapshot cannot imply a later asynchronous write succeeded. Settle this representation before generated result readers, and have their adoption tickets consume it rather than adding independent per-binding logic.
- Shared representation: complete invocation facts contain `usage_persistence` with `state` (`disabled`, `pending`, `written` or `failed`), `observed_at: "facts_snapshot"` and optional fixed failure `advice`. Freeze the observation when public Facts reads its owning call facts; engine methods continue to report live state. Aggregate tallies have no invented persistence observation. The enum and state calculation have one owner in engine usage counters.
- Additional exact seams: `crates/thinkthen/src/engine/facade/state.rs`, `crates/thinkthen/src/public/results/tally.rs`, `crates/thinkthen/src/public/results/complete.schema.json` and `sdlc/ratchet.json`. Bind call facts to their owned counters at shared state construction, derive the complete schema from its serializer and measure actual source growth.
- The existing schema generator also owns `specification/result.schema.json`; regenerate its additive complete-facts definitions from the same serializer rather than editing either schema by hand.
- Existing fork regression seam: `crates/thinkthen/src/engine/facade/fork_tests.rs`. Prove status and finalization avoid an inherited held queue and write only the child's owned deltas, using the existing busy-parent fixture.
- Defers: No monthly spending policy, ledger change, telemetry, new durable store or proof framework. Release management remains held.

### Added public declarations

```text
enum UsagePersistence
UsagePersistence::Disabled
UsagePersistence::Pending
UsagePersistence::Written
UsagePersistence::Failed
const fn UsagePersistence::advice(self) -> Option<&'static str>
fn UsagePersistence::aggregate(impl IntoIterator<Item = UsagePersistence>) -> UsagePersistence
fn Engine::usage_persistence(&self) -> UsagePersistence
fn Engine::finish_usage_status(&self) -> UsagePersistence
const fn Facts::usage_persistence(&self) -> Option<UsagePersistence>
impl Serialize for UsagePersistence
```

## What the build taught us

The existing writer failure latch belongs to shared counters. An atomic latch lets observation report a known failure even while another thread holds the queue. A nonblocking queue observation reports Pending during contention and never waits for filesystem work. The existing finalizer retains its usage-lock deadline and leaves other filesystem operations unbounded.

Complete invocation facts freeze the engine's persistence state when facts are snapshotted. A Pending snapshot remains Pending after a later finalizer reports Failed or Written. Aggregate tallies carry no persistence observation. Legacy count-only facts retain their serialized shape. The shared facade state door attaches counters to call facts across call families without duplicating host logic.

The existing held-writer, stage-failure and inherited-parent queue fixtures exercise the contract. A loopback consumer preserves the valid answer, reported tokens and exact request counts after failure. Only fixed advice reaches complete facts. Host engines, the C status request and three SQL status functions require the remaining adoption slices before the ticket's full outcome is met.

## Progress

- 2026-10-09 started
- 2026-10-09 landed c11c0c1f7; next: Shared persistence observations and the C JSON status doors are landed and combined lint, test and spec pass. Adopt status in the language and SQL surfaces; retain truthful pending, failure and explicit-finish behavior.
- 2026-10-10 landed a52868737c34186559b2520c8b62702c2f59626d; next: Direct typed C engine observation and finish-status methods are landed with safe advice and preserved outputs; focused installed ownership and persistence checks pass. Finish SQL status and adopt explicit methods in each language engine; facts snapshots alone are insufficient. Final platform qualification remains held.
- 2026-10-10 landed 62be191b2a17346501329828f054c3b8d1070ba8; next: Live SQL persistence status is landed across SQLite, DuckDB and PostgreSQL with native aggregation and retained retirement failures. Focused installed checks and review pass. Adopt explicit methods in every language engine next; facts snapshots alone are insufficient. Full installed and platform qualification remain held.
- 2026-10-10 landed 493c81650; next: Direct native usage observation and finalization are landed in Python, Ruby and R, with focused installed checks and fresh review. C and SQL are already adopted. Finish explicit equivalents in the remaining language engines; full installed and platform qualification remain held.
- 2026-10-10 landed bda653d6f; next: Go, C++ and PHP now expose direct native persistence observation and finalization with owned advice and correct thread diagnostics. Focused installed checks and fresh review pass. Finish remaining language engines; full installed and platform qualification remain held.
- 2026-10-10 landed 13ee944a9; next: Ada and COBOL now expose live native persistence status with generated declarations, owned advice and correct diagnostics. Focused installed cases and fresh review pass. Finish the reviewed Dart and Foundation batch; full installed and platform qualification remain held.
- 2026-10-10 landed 5ecfd7390; next: Explicit live usage-status methods are implemented across language engines and SQL, including the reviewed Dart and Foundation batch. Focused installed behavior and generation checks pass; Foundation has real Apple SDK compilation but lacks matching native runtime proof. Full installed and platform qualification remain held.
