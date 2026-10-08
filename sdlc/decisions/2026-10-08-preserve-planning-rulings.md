# Preserve planning rulings

Ian directed the repository to use the workspace instructions and shared team skills on 2026-10-08. The PM message `2026-10-08-pm-delete-status-files-and-adopt-the-shared-team-skills.md` authorizes removing historical status and handoff files. Ian can overturn these rulings.

## Sources and precedence

The deleted work plan, cleanup plan and four handoffs remain historical evidence at [the retained source commit](https://github.com/botassembly/thinkthen/tree/eafcd3f26/sdlc/planning). Their queues, lane assignments, completion counts, one-time paid jobs and harness assignments carry no current authority. Current status comes from pm. Existing specifications and accepted ADR amendments govern behavior. The workspace instructions govern roles and review.

## Cache, types and release rulings

The 2026-09-29 cleanup rulings 2–6 accept per-question caching, neighbor effects from batching, backend question kinds as the cache unit, taken-at timestamps and simple recordings. [ADR 0111](../planning/adr/0111-question-cache-and-one-batching-path.md) preserves those decisions and their amendments. Ruling 7 gives Rust ownership of output types and the generated result schema under [ADR 0112](../planning/adr/0112-rust-owns-the-result-schema.md). Later typed SDK contracts amend the earlier thin JSON port scope; use [the result contract](../../specification/result.md) and [the type contract](../../specification/types.md).

Ruling 8 removes pedantry and wording-only checks. Keep checks that protect behavior, secrecy, spend and boundaries. Preserve distinct parser, secrecy, cancellation, cache-miss, invalid-input and conflict regressions until a stronger replacement lands. Routine checks use focused functional cases; load, timing, churn and repeated contention remain explicit opt-in work. The shared outside-in and proof-spiral skills govern implementation and review.

The 2026-09-30 rulings accept batching quality differences and retain the per-record alternative. The first public release required every surface and binding and every registry (rulings 10 and 15). [The release process](../planning/release-process.md) and [ADR 0116](../planning/adr/0116-release-branches-cut-at-the-release-candidate.md) carry release steps; [ownership.md](../planning/ownership.md) carries the later whole-repository ownership ruling. Pandas stays optional (ruling 12). Ruling 14 removes default pacing; callers configure rates per backend under [settings.md](../../specification/settings.md). Paid validation uses the live runner, its ledger and token admission; an old single-job approval does not authorize a repeat. New dependencies follow the Rust standards.

## Approved API choices

The 2026-09-27 work plan approves the reviewed outcomes in tickets 0201 and 0208–0210: staged platform proof, guarded cache maintenance, native frame failure representations and an optional effective-key snapshot that refuses key-bearing URLs before output or send. The 2026-09-28 approval covers tickets 0212, 0214, 0216, 0222–0224, 0228–0230: synchronous caller-owned iterators, prompt cancellation with final accounting, request-aligned details, recoverable SQL rows, graded ranking, ordered SQL find, unbound empty-cache protection, read-only unused-entry reporting, C value/facts replies and shared dynamic-question details. Their tickets, accepted ADRs and specifications retain exact contracts; these approvals do not waive remaining evidence. [ADR 0095](../planning/adr/0095-rank-graded-score-questions.md) preserves the graded-rank choice.

The accepted SQL/dataframe redesign remains under ADRs 0105 and 0107 and its existing tickets. It supersedes the earlier redesign hold and does not revive superseded aggregate campaigns. Default rationale and binding guidance remain in their owning contracts. [ADR 0113](../planning/adr/0113-every-surface-adds-to-the-usage-totals.md) keeps durable count-only product usage separate from the developer live ledger.

The 2026-09-21 handoffs defer to the existing [recognize](../../specification/recognize.md), [relate](../../specification/relate.md), [record](../../specification/records.md), [result](../../specification/result.md), [recording](../../specification/recording.md) and [SDK boundary](../../specification/sdk-boundary.md) contracts. They retain ten judging functions, explicit caller inputs, record-with-answer behavior, distinct null/failure outcomes, named numeric measures and one native engine. The tool judges and never acts. Later caller-wording and size-admission rulings supersede the old fixed-method and kind-count assumptions. Historical accuracy and cost observations establish only the measured corpus and revision.

## Bench evidence and fixtures

The removed bench handoff's test split remains in [test-split-2026-09-30.md](../planning/test-split-2026-09-30.md): the repository checks mechanics offline, the public benchmark measures accuracy/cost/speed, and release QA qualifies releases. Loopback error-path tests remain in the repository. Count-only status fields follow [ADR 0114](../planning/adr/0114-named-backends-each-name-their-key-variable.md); tag conventions follow [worktrees.md](../planning/worktrees.md).

Hard cases use public input and key files beside a sorted `thinkthen.jsonl`, never a committed SQLite store, headers or credentials. The recording contract governs conversion and replay; updated cache identity rules govern recorded address/model inputs. Benchmark fixture placement defaults to `specification/fixtures/<function>/bench-<case>/`, with source provenance in its README. [The relate decision run](../issues/closed/2026-09-30-relate-pair-planner-loses-precision-on-the-beatles-bench.md) retains its measured result and the unrun alternate-provider cell.

## Build capacity

Inspect load, memory and storage before substantial work. Independent builds may overlap in isolated outputs. Reduce compiler jobs or delay a new heavy step when measurements show pressure. Keep lane output exclusion and narrow toolchain/cache mutation locks. Stop only owned confirmed-abandoned processes by PID. Never kill a process by name. Preserve warm build folders and remove only ticket-owned scratch at landing. The repository instructions retain storage and platform boundaries.

## Retained 0.2 scope and order

Every SDK must expose the same ten functions with admitted inputs, complete typed results, errors, cache/record/replay, image decide/choose/score, named questions and local MCP. Each engine resolves one endpoint, key and provider API type; proxy business routing stays outside the SDK. [milestones.md](../planning/milestones.md) retains the outcomes and [the SDK boundary](../../specification/sdk-boundary.md) retains the contract. Existing installed-package evidence stays in its owning records and is reused while its inputs hold.

Confirmed correctness fixes precede new controls. Cache admission settles before snippet controls and additive proposed-span details. Usage persistence and bounded DuckDB file admission settle native contracts before family adoption. Independent prose/help corrections can proceed without colliding source ownership. Adopt settled shared contracts once per binding family using the shared suite and installed-package checks. The recognize kind-count change follows confirmed bugs; actual encoded-byte admission and complete menus remain required. Current accepted scope and order follow [milestones.md](../planning/milestones.md). Caller-given-span kind-only execution is dropped. An optional step-one batch control ships only if it stays small. Existing representation defaults and limits remain defined by configuration and code.

Ian held release management on 2026-10-08. No candidate tag, manual GitHub workflow dispatch, release branch advancement or publication proceeds without his permission. Main targets 0.2; release/0.1 stays frozen. Public install copy remains 0.1.2 until 0.2 ships. No new paid run follows from this cleanup. Final platform qualification, candidate QA and publication retain their existing approvals and owners.

The configured `planning/team-0-2-2026-10-04.md` lane table remains untouched until the pm layout migration. Read lanes through pm. Its duplicate status prose is retained only because the migration owns that removal.
