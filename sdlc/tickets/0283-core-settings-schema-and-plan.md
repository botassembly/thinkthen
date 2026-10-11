# 0283 — Core settings schema and plan (T1)

Status: COMPLETE.

Opened as: 2026-10-11. Fresh High code review accepted `9f6ae289676eabcf608af747449070a788c6513c` after the deadline and staged-plan corrections. The queue owner integrates the reviewed source with its focused checks and records below. Remaining SQL, frame and release criteria keep their own tickets.

## Outcome

Build `thinkthen.settings/1` and its parser once in the Rust core
(`crates/thinkthen/src/core` boundary respected: the parser is pure), beside
one core engine-settings schema shared with the C door's
`thinkthen_engine_new_with` (ADR 0037), reserving `max_requests_total` until 0289 activates it. Add canonical numeric `deadline_ms` in Rust, retaining existing Rust seconds/milliseconds methods while their separate-workspace callers migrate through 0284–0291. ADR 0105 makes the every-surface result final at 0291; Rust keeps `Duration` internally. Replace the asking verbs' `--dry-run` with
`--plan` (first request body plus a whole-input count line; `check --plan`;
`cache prune --dry-run` stays), moving in the same commit: the `spec/` pages,
the `specification/` pages and fixtures, the `specification/settings.md`
Dry-run and Deadline rows and the new schema row, the gated demo blocks (03,
06, 14, 15, 16, 21, 45), the probe self-tests
(`probes/probability-total-0038`, `probes/find-0040`), and the remaining prose
(`demos/27-test-with-no-network/README.md`, `demos/FINDINGS.md`, `AGENTS.md`).

Current C constructor enters libraries/c/src/ffi.rs and parses in libraries/c/src/settings.rs; the Rust builder is crates/thinkthen/src/public/settings.rs. The parser belongs in pure core and cannot read files, environment, clock or sockets. The C engine schema refuses max_requests_total until T7 supplies active reservation and final exposure; no public configure path accepts an inert value. Existing CLI preview branches include asking, batched, find, annotate, recognize, relate and check; cache prune --dry-run is a deliberate exception. At the 500 nonblank-line cap, cli/args.rs measures 473 (27 headroom), public/settings.rs 470 (30), and cli/args/command.rs 430 (70); cli/asking.rs is 419 (81) and core/question_file.rs 392 (108). Reuse modules or one cohesive private extraction rather than consuming unrelated headroom. Libraries/c/src/settings.rs is 143 nonblank lines. Remeasure all touched files and shared ratchets before review.

The changed executable blocks include demos 03, 06, 14, 15, 16, 21 and 45, probes probability-total-0038 and find-0040, specification/settings.md Dry-run/Deadline/schema rows, demos/27 and FINDINGS, plus AGENTS.md. The old spec/decide.md assertion rejecting --plan must change with the flag. The full V/I corpus belongs in one core fixture, not copied to every host.

**One preview owner.** Add `crates/thinkthen/src/core/plan_summary.rs`, exported privately through `core/mod.rs`, as the pure full-input accumulator for `records`, `requests`, `estimated_bytes` (sum of exact prepared request body lengths), and the `estimated_input_tokens` band at ADR 0105's 0.516/0.908 tokens per byte. Pin integer rounding and overflow refusal in an independent fixture. Retain the first prepared body for CLI disclosure and mark `recognize`/`relate` request counts as upper bounds where later answers determine work. CLI adapters feed validated ordered records to the summary. For ordinary record streams the summary drives `core/batch.rs::Batcher::{push,finish}` over the **whole** input, including content cuts, 4,096-member and configured batch limits; staged adapters supply their already prepared chunks and explicit upper-bound counts. Neither summary nor adapter invents a second cut/split algorithm. Non-batch paths already use `engine/prepared_request.rs::PreparedRequests::with_profile` through `engine/facade.rs::split` for request-byte, question and profile limits. If core's inward dependency rule requires that pure splitter in core, move it once and leave the engine forwarding to it. Planned counts exclude cache answers, refusal splits and retries.

Adapt all CLI entries to the same summary: `cli/asking/plan.rs` now prints one record; `cli/asking/batched.rs::planned` stops at the first closed batch; `cli/find.rs` and `cli/annotate/plan.rs` have separate first-set/group paths; `cli/recognize/dry_run.rs` stops at one record and knows staged name/pair bounds; `cli/relate/dry_run.rs` reports prepared relation chunks; `cli/check.rs` has four fixed probes. Preserve existing framing, disclosure and first-body shapes while adding whole-input totals. `core/plan_document.rs` may serialize the summary but does not calculate another estimate. Contract pages must name the upper-bound mark and measured token-band rates.

## Prerequisites and proposed files

Prerequisite: first build. Proposed exact future claim: new `crates/thinkthen/src/core/plan_summary.rs`; `crates/thinkthen/src/core/{mod.rs,batch.rs,plan_document.rs,question_file.rs,question_file/resolve.rs,threshold.rs}`; `crates/thinkthen/src/engine/{prepared_request.rs,facade.rs}` only if sharing its pure splitter needs an extraction; `crates/thinkthen/src/public/{settings.rs,options.rs}`; `libraries/c/src/{settings.rs,ffi.rs}`; `crates/thinkthen/src/cli/{args.rs,args/command.rs,asking.rs,asking/batched.rs,asking/plan.rs,find.rs,annotate.rs,annotate/plan.rs,recognize.rs,recognize/dry_run.rs,relate.rs,relate/dry_run.rs,check.rs,failure.rs}`; `crates/thinkthen/tests/settings_cases.rs`, focused CLI tests, and spec/, specification/, demos and probes named above. Current nonblank counts/cap: `core/batch.rs` 458/500, `engine/facade.rs` 468/500, `engine/prepared_request.rs` 177/500, `core/plan_document.rs` 127/500. Measure again and extract only cohesive pure code if headroom requires it. No source file is claimed by this preparation draft.

## Smallest meaningful proof

Run V1–V11 and I1–I10 from the [independent corpus](../records/2026-09-29-sql-frame-redesign-corpus.md) through one pure core parser test; add one C engine-schema conversion edge. Use a multi-cut input whose **independently recorded wire bodies** establish full-input records, planned requests and exact summed body bytes; assert the token band's fixed-rate rounding and unchanged first body. Include recognize/relate marked upper bounds and an invalid later record that a first-record preview would miss. A loopback listener must accept **zero** requests for invalid settings and every `--plan` route; pin exit codes/refusal sentences. Keep `cache prune --dry-run`. Run changed executable pages/demo blocks/probe self-tests with a matched binary, measure ratchets and focused format/policy/pages/tickets/diff. No broad gate or provider run belongs to preparation. [Shared proof routes](../records/2026-09-29-sql-frame-redesign-proofs.md) separate installed and release qualification.

## Normative Rust public API delta

This additive reviewed contract adds only the named declarations. The rest of the 0278 inventory, including both existing Rust deadline methods, remains authoritative. The C ABI and result envelopes are unchanged. `Settings` is a pure parsed call value: hosts validate verb applicability and conflicts before converting arguments, and the builder bridge checks the same closed engine schema before C map conversion. There is no accepted `max_requests_total` setter until 0289 installs admission.

### Added public declarations

```text
For::Choose
For::Decide
For::Find
For::Score
For::Tag
SettingsError::BadBatch
SettingsError::BadNone
SettingsError::BlankContext
SettingsError::DeadlineNotWhole
SettingsError::DeadlineOutOfRange
SettingsError::Json(String)
SettingsError::NotAnObject
SettingsError::Question(String)
SettingsError::RepeatedField(String)
SettingsError::TwoMemberKeys(String, String)
SettingsError::TwoMembers
SettingsError::UnknownKey(String)
SettingsError::WrongVerb(String)
const fn Settings::batch_max(&self) -> bool
const fn Settings::batch_records(&self) -> Option<usize>
const fn Settings::deadline_ms(&self) -> Option<i64>
const fn Settings::none(&self) -> Option<bool>
enum For
enum SettingsError
fn CallOptions::deadline_ms(self, i64) -> Result<CallOptions<'a>, Error>
fn EngineBuilder::validate_settings_json(&str) -> Result<(), Error>
fn Settings::check(&self, For) -> Result<(), SettingsError>
fn Settings::conflicts(&self, &[&str], bool) -> Result<(), SettingsError>
fn Settings::context(&self) -> Option<&str>
fn Settings::parse(&str) -> Result<Settings, SettingsError>
fn Settings::question_json(&self, For, &str) -> Result<String, SettingsError>
impl Default for Settings
impl Display for SettingsError
impl Error for SettingsError
struct Settings
```

## Evidence

- Starts from: Main `869710193`, accepted ADR 0105, experiment 2038 HANDOFF and saved spikes, and the [preparation](../records/2026-09-29-sql-frame-redesign-preparation.md).
- Keeps: Existing successful values, owned facts, NULL/not-sure, cache identity, six error kinds, offline replay and privacy except the accepted changes.
- Changes: Pure settings parser, validated engine schema and one full-input plan summary reused by all CLI adapters; 0289 activates the cap.
- Proof: Core V/I table, multi-cut exact bodies/bytes/token band, later-record refusal, staged upper bounds, zero loopback accepts and changed executable pages.
- Defers: Unrelated package/release qualification, provider work, marketing site, token cap and per-record cache; named prerequisites remain.

## What the build taught us

The accepted preview needs a second output line for whole-input totals while retaining the first prepared request's body. Existing first-record-only tests were wrong under this contract: a malformed later JSON or CSV record now refuses before any disclosure. The stronger regressions pin those two distinct parser boundaries, and the former first-only JSON test was retired. A packed two-record request followed by a singleton uses different wire shapes; counting records does not establish body bytes. The real batcher and group planner supply the bodies while one core summary owns the arithmetic. C's handwritten duplicate scan could be deleted once the shared pure schema validated before map conversion. The parser must become reachable by separate SQL/frame crates, and the numeric Rust deadline name reaches the C door's internal call and frozen API inventory; the accepted claim includes the public module, C door and inventory delta. Cross-crate visibility was invisible to the original CLI-only route: C must use the public builder bridge, while later hosts need the re-exported pure call value. `max_requests_total` is reserved until 0289 activates it, so this ticket never accepts an inert cap. High code review of `674a82041` found that SQLite, PostgreSQL, DuckDB, Python, R, Ruby and TypeScript still call the existing Rust deadline methods from separate workspaces; removing them here broke intermediate source compilation. They remain temporary source support with exact fractional-second and clearing semantics while the host tickets migrate callers. Ticket 0291 removes them after a whole-tree caller scan, before the final clean-break API freeze. The same review found recognize omitted its possible name-stage requests from the whole-input summary, and the pure settings parser accepted out-of-range integral deadlines. The first correction included possible name requests, but High recheck of `9d4d4f1e` showed that using the prepared step-one request count still underbounded profile-split step two. Each found name asks at most one kind and one edge question, each stage-two request holds at least one question, and names cannot outnumber pieces. The final bound uses pieces without kinds or twice pieces with kinds, independent of stage-one grouping. The relation bound remains safe because each possible pair under each rule contributes one question and each request holds at least one. The pure settings parser also refuses `-2` and max+1 at the shared boundary. This does not create a public 0.1 compatibility promise or host aliases.
