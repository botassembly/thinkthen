# 0289 — Core cap plan tally and Rust Polars (T7)

Status: Done. Fresh High code re-review accepted `df77e652681ff4980d034e325ab06f84cf2040ac`. Focused source and SQL compatibility proofs passed. Later host migrations, three older Polars package criteria and release qualification retain their own work.

## Outcome

`EngineBuilder::max_requests_total` and the command's `--max-requests-total`;
the `Engine` plan; the **core `Tally`** (an `Arc`-shared facts sum, wrapped by
Python's `tt.Tally`, taken by F7's `decide_expr`); the Rust Polars door's
probability column (decide and choose), plan, call-level
`threshold`/`true`/`false`.

This ticket supplies the active cap that T1 may only describe as static schema data; the SQL consumers do not finish cap acceptance until this lands.

`Engine::plan` is a public wrapper over 0283's `core/plan_summary.rs`, fed by the same `Batcher::{push,finish}` and prepared chunk boundaries as execution. It returns the accepted `requests`, `records`, `estimated_bytes`, and 0.516/0.908 `estimated_input_tokens` band. The CLI's staged recognize/relate plans retain their marked upper bounds; the typed Engine plan takes ordinary detail questions. No second Rust or CLI estimator, no cache read, no key and no send. Expose the minimal typed summary in `public/plan.rs` and `public/mod.rs` as the reviewed Rust API; update the frozen API inventory from the independent ticket declaration. Keep the command's `--max-requests-total` and the C engine setting inactive to callers until this ticket's atomic reservation is wired before each started transport request.

## Prerequisites and proposed files

Prerequisite: T1, now integrated at main `cac3d07ec`; F3 and F7 consume this result later. The implementation claim is the current codex-2 row in `sdlc/planning/work-plan-2026-09-27.md`: the listed public and core modules, `engine/{mod.rs,facade.rs,schedule.rs,request.rs,usage.rs,call_facts.rs}` and their reservation/attempt helpers, CLI args/constructor/asking, C constructor bridge, feature-gated `crates/thinkthen/src/public/frame.rs` and `frame/` helpers, `crates/thinkthen/tests/polars/`, `libraries/polars/{README.md,check.sh}`, inventory, relevant pages and measured root/C ratchets. There is no `libraries/polars/src` or separate Polars ratchet. `core/plan_summary.rs` is the one T1 summary to reuse. Refresh file caps before extraction.

The retained `SendBudget` in `public/options/budget.rs` is the sole atomic attempted-send count. A reservation takes the calling engine's immutable optional limit; it does not install a process-wide limit. Thus a lower limit refuses once the shared count reaches it, raising a limit allows further attempts up to the raised value, and an unset engine imposes no total while its attempts still increase the count later bounded engines see. Clones retain their snapshot. Engine drop or a new engine does not zero the count; the existing owner-id reset gives a forked child its own count. Count a request only after the usage mark, refund an uncommitted reservation, and reserve again for every retry. A first denied request of a call, an additional ordinary or split request after an earlier attempt, and a retry after a backend status need distinct context. A cache or replay answer reserves none. CLI, C and Rust select the cap before the first transport attempt; an inert accepted setting is forbidden.

## Smallest meaningful proof

Hold simultaneous calls at the loopback backend and prove atomic reservation refuses a later send; pin bodies and exact cap refusal. Feed the same multi-cut independent body fixture through CLI `--plan` and `Engine::plan` and require identical records, planned requests, bytes and token band, with zero accepted requests and no cache/key touch. Check marked staged upper bounds, core Tally against listener, and Rust Polars value/probability from one reply plus score/tag refusal. Measure headroom/ratchets. [Shared proof routes](../records/2026-09-29-sql-frame-redesign-proofs.md) distinguish source from installed qualification.

## Normative Rust public API delta

The following 19 declarations are additive to the accepted 0283 inventory. The no-default-feature inventory excludes the feature-gated Polars trait; its new `column_with`, `plan_series`, `probability_frame`, and `PolarsCallOptions` are checked by the selected feature-on source targets. No existing declaration is retired here.

### Added public declarations

```text
const fn PlanEstimate::estimated_bytes(&self) -> usize
const fn PlanEstimate::estimated_input_tokens(&self) -> (usize, usize)
const fn PlanEstimate::records(&self) -> usize
const fn PlanEstimate::requests(&self) -> usize
const fn PlanEstimate::upper_bound(&self) -> bool
fn Engine::plan<Q, I>(&self, &Q, I) -> Result<PlanEstimate, Error> where Q: DetailQuestion + ?Sized, I: IntoIterator, I::Item: Evidence
fn Engine::plan_with<Q, I>(&self, &Q, I, CallOptions<'_>) -> Result<PlanEstimate, Error> where Q: DetailQuestion + ?Sized, I: IntoIterator, I::Item: Evidence
fn EngineBuilder::max_requests_total(self, Option<u64>) -> EngineBuilder
fn PlanEstimate::first_body(&self) -> Option<&[u8]>
fn Tally::facts(&self) -> Facts
fn Tally::new() -> Tally
fn Tally::run<T>(&self, impl FnOnce() -> Result<Call<T>, Error>) -> Result<Call<T>, Error>
fn Tally::start(&self) -> TallyStart<'_>
fn TallyStart::finish(self, &Facts) -> Result<(), Error>
impl Default for Tally
SendBudgetDenial::BeforeAdditionalSend
struct PlanEstimate
struct Tally
struct TallyStart<'a>
```

## Evidence

- Starts from: Main `869710193`, accepted ADR 0105/0107, experiment 2038 HANDOFF and saved spikes, and the [preparation](../records/2026-09-29-sql-frame-redesign-preparation.md).
- Keeps: Existing successful values, owned facts, NULL/not-sure, cache identity, six error kinds, offline replay and privacy except the accepted changes.
- Changes: Atomic process cap, Engine wrapper over 0283's plan summary, core Tally, Rust Polars eager probability/options.
- Proof: Held concurrent listener pins cap/no-send; CLI and Engine return the same multi-cut summary; tally and eager Polars facts match the observed bodies.
- Defers: Unrelated package/release qualification, provider work, marketing site, token cap and per-record cache; named prerequisites remain.

## What the build taught us

The Rust Polars source is feature-gated in `crates/thinkthen/src/public/frame.rs` and `frame/`; there is no `libraries/polars/src` or separate Polars ratchet. `public/options/budget.rs` already owns the reservation count, while `engine/request.rs` carries the selected limit into sends. The limit must be selected for each reservation, since a lower, raised or unset engine can share the process count. The existing `public/bulk.rs` selector is reused by the public plan and `public/results.rs` registers the new Tally module; neither requires a second estimator or facts store. The core planner cannot own a clock, so the Tally belongs at the public boundary while remaining the one type later Python and expression doors will wrap.

The selected Polars proof found that keeping null input positions requires filtering them before a call and restoring them after the typed result; rejecting every nullable column would preserve the old restriction rather than the accepted frame outcome. Source-only feature tests establish this Rust behavior, not an installed package or release artifact. A later call denied by the cap needs its own failure context, distinct from a first send and a status retry. The actual boundary spans `engine/send_budget.rs`, `engine/mod.rs` and `call_facts.rs`, `engine/error.rs`, `public/error.rs`, `cli/failure/convert.rs` and `public/native_batch.rs`, with the public enum variant added to this ticket's inventory. The cohesive private reservation helper keeps `engine/mod.rs` below 500 nonblank lines. An explicit call budget must reserve alongside the process budget, since replacing it would let an unset engine hide attempts from later bounded engines.

The existing Polars package issue retains its three older criteria: the full shared `27-decide-many` case, mid-column deadline, and 20-text one-request assertion. The selected source `polars_door` proof covers a separate chunked case-27 witness and current eager behavior; it is not a whole package-gate receipt.

Fresh High review found two boundaries the preparation omitted. `Engine::plan` now applies the same ordinary `DetailQuestion` kind guard as `details_many_with` before batching; rank and both find variants return Usage without a disclosed body or listener send. SQLite and PostgreSQL now map `BeforeAdditionalSend` to their established spent-total messages as they already do first-send and retry denials. Their exact compatibility matches are included here; host redesigns 0284 and 0285 still own their broader contracts. Source-matched, bounded host receipts use batch one, cap one and two distinct rows: each listener accepts exactly one request, and each host reports its spent-total refusal. The old Polars package criteria remain open.
