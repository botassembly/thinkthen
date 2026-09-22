# Architect survey and proposed execution plan

Status: Execution resumed at Ian's direction on 2026-09-22. The accepted sequencing amendment is recorded in `build-queue-2026-09-21.md` and ADR 0017; recommendations that need a separate contract ruling remain proposals. This survey creates no implementation ticket and changes no public behavior. Ian can overturn every recommendation. Accepted ADRs and the specification remain authoritative. This plan serves the ideal state's one-engine, reliable-results, thin-library, and release gaps.

Resumption snapshot: main is clean at `c29e445`, surfaces is clean at `7fdb1fa`, and both match their remote branches with passing code gates. Ticket 0065 remains at `6618694`, outside main, with its original owner. The library team's committed `sdlc/records/2026-09-21-punch-list-report.md` now records Ruby's one-call probability fix and typed recognition/relation results, ownership tests, parser fixtures, and six scalar loops awaiting real-engine bulk entries. The findings below retain the original survey snapshot; do not duplicate fixes already on `surfaces`. The separate website Pages workflow failed at configuration; no website fix belongs to this engine plan.

## Latest execution amendments

Ticket 0065 landed at `ba60f04` after Ian transferred ownership and independent review repaired a directory-sync gap. Tickets 0066/0067 landed the reviewed help corrections. ADR 0036 and ticket 0068 implement the canonical `meta.cached` name while preserving stored-answer semantics and historical readers.

Ian has paused GitHub Actions and selected full coordinator-run local gates for subsequent tickets; the earlier hosted-gate instructions below are historical. Incoming product rulings on main `692ba59` also move structured descriptions ahead of recognize/relate, superseding this survey's earlier after-0.1 exclusion. The current build queue and structured-description issue govern that placement; typed library builders remain with the library team. The many-state packing probe supplies evidence, not permission to change request grouping or make new paid calls. No relation-method decision is implied.

## Snapshot and verdict

The code survey and local gates are pinned to clean main `76b3511`. During review, main advanced to `5c11eb9` through three documentation-only commits: the settled-library-details ledger and two marketing issue closures. The coordinator read that new ledger and observed the latest main hosted gate green. Ticket 0055 is complete; tickets through 0064 are on main. Ticket 0065 is clean and pushed at `6618694`, with its hosted gate green, but is still not an ancestor of main. Its owning agent keeps the landing. The library team keeps `surfaces`; it moved from `a9a967a` to `9ef50a6` during the survey. That change updates its Python fork-proof citation, and its hosted gate is green. No active team worktree was changed.

The core architecture is sound: one package, a mechanically enforced pure core, private engine scheduling and storage, typed failures, scoped workers, immutable recording identities, and independent package checks. It needs completion and selective consolidation, not a replacement framework. Green surface tests currently prove a rehearsal over a stand-in. They do not prove integration with the real engine.

Three SWE-2 researchers surveyed the engine, recognition evidence, and nine surfaces. The coordinator checked consequential findings against source. Two researcher claims were rejected: a universal 100-option backend ceiling contradicts the accepted 255-option find recording; C's last-error storage is per engine, not process-global.

## Architecture to keep

- Keep `crates/thinkthen/src/{core,engine,cli}`. Core owns parsing, validation, question construction, answer interpretation, ranking, recognition assembly, and relation enumeration. Engine owns settings, transport, cache, counters, bounded scheduling, cancellation, and deadlines. CLI owns terminal behavior.
- Keep adapters under `libraries/<language>` and `databases/<database>`. Native adapters may need Rust crates, but no second product core, production stand-in, copied canonicalizer, public engine trait, or per-language scheduler enters the integrated product.
- Expose one concrete blocking engine. Complete the private engine controls before freezing its public API. Every bulk verb uses the same Rust scheduling path. Host code converts arguments, manages host lifetimes, checks host interrupts, and renders results.
- Build one complete immutable Rust judgment snapshot per call, with private fields and read-only accessors. Host adapters derive the established bare answer or requested details from that snapshot without another call. This does not replace the public bare-value forms with new wrapper objects. Explicit conversion to an ordinary host table or object may make a detached copy; it must not write back into the Rust snapshot. Test nested values and buffer lifetimes; do not claim every host container is inherently immutable.
- Reading metadata already returned by a call sends nothing. The existing standalone `details(question, text)` remains a judgment call subject to the normal cache/network rules; changing it requires a separate contract decision. Retain request identity, all required probabilities, partial results, and the six error kinds. Keep CLI record rendering separate from typed library results.

## Findings that shape the order

| Finding | Evidence | Consequence and smallest response |
| --- | --- | --- |
| High, integration gap: question-set rules exist twice | `surfaces:contract/src/lib.rs:1574-1604` ignores root version validation and sorts names. Main's `core/question_set.rs` owns the canonical grammar | Adopt the production parser through the public Rust boundary. Test unknown/duplicate keys, version, empty sets, names, `on` groups, collisions, and file order. Do not repair a second grammar for release |
| High, billing risk on engine swap: details can call again | `surfaces:libraries/ruby/lib/thinkthen.rb:119-126` performs bulk calls then one details call per record | Return the existing probabilities. A real counted listener must observe no extra request when reading details |
| High, design gap: embedding controls are unfinished | Main `engine/http.rs`, both schedulers, and `engine/recorder.rs:395-405`; the recorder installs SIGXFSZ handling process-wide | Before the API freeze, settle width across engines, cooperative stop, whole-call deadlines, warm/busy fork behavior, and signal ownership. PID comparison must happen before any inherited lock. Inspect recorder, counters, pool, and width state together |
| High, evidence gap: stand-in semantics differ | `surfaces:standin/src/lib.rs:442-464,467-521` uses per-unit find and per-question annotate | Replace with real relative find and grouped annotate. Preserve the synthetic evidence labels; never rewrite historical responses to fit new request digests |
| Medium, wording inconsistency: score's nearest name | `surfaces:contract/src/lib.rs:352-363,395-402` defines nearest as the highest-probability level; main `specification/result.md:72` names that `level` | Preserve the existing highest-probability meaning. Probabilities `[0.4,0.35,0.25]` give score `0.85` but highest-probability index 0. Correct the product summary that calls this the numerically closest level. Keep CLI `answer.level`; reconcile names without changing the calculation or adding a second metric |
| Medium, unreachable recognition rules | Harvest `tools/pipeline.py:238-257` calls neither `resolve_overlaps` nor `trim_possessive`; the functions live at 410-465 | Preserve recorded behavior first. Test an enabled policy through real assembly before calling it supported. Do not copy the helper's exhaustive subset search into an unbounded production path |
| Medium, remaining surface cleanup | SQLite `usage('reset')` still clears process counters and its shim cache; native Polars wrappers loop scalar choose/score/tag calls; C header has no usable cancellation/deadline options | Remove the reset behavior, route bulk verbs through Rust bulk entry points, and settle C options before its ABI freezes |
| Performance risk, not a measured regression | Main `engine/usage.rs:105-118,175-218` serializes durable counter writes for each event | Measure cache-hit and loopback throughput with persistence before changing it. Do not batch away crash/accounting guarantees based on a guess |

`surfaces:` paths above refer to the library team's branch. Its stand-in limitations are integration blockers, not claims that released libraries are broken. Nothing is released.

## Recognition and relation decisions to prepare

The connector whitelist is gone. Ampersand joins only when its own detection answer says it belongs. Trailing possessive trimming is a separate helper, and the recorded assembly does not enable it. The harvest's forty expected outputs capture what happened; ten are labeled divergences. They prove faithful replay, not forty correct answers, Unicode safety, or cross-batch correctness.

Use the exact harvested prompt words and measured min-times-mean strength formula as the baseline. Keep every member word in the formula. No margin term, connector exclusion, vocabulary list, or new prompt wording slips into the port. The reported end-to-end relation F1 is about 0.47; relation features remain preview.

Ian now requests configurable special rules. Prepare typed options in the Rust builder and question file for supported boundary/possessive policies and overlap selection. The measured assembly baseline keeps repairs off. Specify probability-weighted selection with deterministic leftmost/longest ties versus pure leftmost-longest before implementing either override. Identify the real source of competing candidates first; do not expose a setting that cannot change an answer. Record the chosen policy in detailed metadata. Prompt changes and deterministic assembly changes need separate proof. Do not offer arbitrary executable callbacks or revive the deleted list.

Proposed method amendment requiring Ian's confirmation: use independent yes/no questions per legal relation instead of the pick-one method recorded in `relate-design.md:41-46`. Ian's forwarded plans contain both forms, so the experiment verdict alone cannot settle authority. The record-pair comparison recovered 22/24 true relations versus 11/24 with pick-one and used fewer tokens; it permits several true edges. Its cost is a changed request form and missing fresh in-text evidence. Keep request-shape implementation blocked until this amendment is confirmed. The forty in-text cases still use pick-one; preserve them as historical fixtures and separately validate any replacement. A paid refresh needs its own authorization and the live guard.

Use byte ranges internally in Rust and explicit boundary conversion for CLI code points and host indexing. Pin accent, emoji, combining mark, repeated-name, empty-result, cross-batch, and boundary round trips. Batch questions without cutting the source text's context or assembling names per batch. Validate exact locally countable profile limits, including complete encoded request size. Do not infer token limits from bytes or adopt the harvest's unsupported universal 100-option ceiling; `probes/find-0040/README.md:19` records acceptance of 255 options. The 255-record relation limit is a product guard, not a backend option limit.

Close the usability review explicitly: tuning guidance; actual strength components without fictitious margin/connector fields; policy metadata; meaningful overlap override; settled number and endpoint names; threshold help naming the gated quantity; Unicode round trips; pair-count refusal; no unmeasured price claim; and visible failure/send counts without corrupting bare stdout. An exact recognition relation count is unavailable before detecting entities; dry-run must distinguish a bound from a known count.

## The library team's punch list

Ian can forward this list. It requests no edits to main and no new stand-in features merely to imitate the coming engine.

1. Isolate duplicated parser/validation and scheduling code for removal at engine integration. Inventory every shim call that re-asks for details or loops a scalar engine call. Remove Ruby's second details pass when results carry its data.
2. Adopt immutable result access, with tests for nested mutation and lifetime safety. Return typed data without scraping serialized answer JSON. Keep one index conversion per host and test selected records and relation endpoints.
3. Keep score's bare answer numeric and its detailed level as the highest-probability level. Do not reinterpret it as rounding the score. Keep rank's existing index/probability pairs; find additionally owes the full candidate distribution and an explicit none outcome. Do not change CLI record output to match library tuple syntax.
4. Remove SQLite `thinkthen_usage('reset')`. Keep the temporary shim cache a named divergence with a removal test. A new engine does not reset process-wide counters; callers can take differences between snapshots.
5. Document each database's authority: question-file access, chosen backend, credential source, query execution, connection lifetime, and cancellation channel. Prove SQLite is not registered deterministic. Verify DuckDB's LOAD-time signal handling coexists with its host. A configured PostgreSQL credential must either reach the engine safely or be refused, never silently ignored.
6. C must cover cancellation, deadlines, partial completion, null/length inputs, allocation/free ownership, and concurrent callers. Keep the interface small; the current missing options cannot be papered over by header promises.
7. Supply clean installed-artifact checks on Linux and macOS, a self-contained R package, supported database/ABI floors, and one tested examples file per surface. Label absent checks as absent. Node child-process creation is not proof of inheriting a warm native Rust pool.

## Twelve tracks and dependencies

These are work streams, not twelve simultaneous writers. Run one accepted engine ticket at a time. The library team continues its separately owned branch. Parallelize independent research only.

| Track | Next outcome | Dependency |
| --- | --- | --- |
| 1. Command contract | Recheck the 40-item issue against the current binary; fix only surviving wording/behavior | Existing 0056-0058; 0065 landing where files overlap |
| 2. Settings and profiles | One typed engine setting/validation path, preserving landed profile/config behavior | Reconciled spec; no speculative configuration redesign |
| 3. Cache and counts | Let 0065 land; verify migration/refusal behavior; rename replayed to cached; measure persistence cost | Owning agent's completed landing |
| 4. Recognize and relate | Reconcile method, measure optional packing, port pure code, add engine/CLI callers and shared cases | Settled policies, profile limits, final request evidence |
| 5. Rust engine and API | Private width, cancel/deadline, retries, fork and host-signal safety; later, the public API | Private controls depend on settings/cache invariants from 2-3. Public API depends on controls, track 4, and settled result shapes |
| 6. C | Small owned-result ABI with options and failure/partial-result rules | Track 5 |
| 7. Python, Polars, pandas | One bulk crossing; native lifetime/interrupt/offset proof | Track 5 and the library team's branch |
| 8. TypeScript | Real-engine async adapter, abort, result and indexing proof | Track 5 and the library team's branch |
| 9. Ruby and R | No duplicate calls; host-lock/interrupt/fork and result proof | Track 5 and the library team's branch |
| 10. Databases | Real-engine integration, safe registration, query/cancel/credential and dependency workaround proof | Track 5 and the library team's branch |
| 11. Install and packages | Local artifacts, Homebrew/download rehearsal, clean-machine checks | Relevant interfaces stable; names/tap and publication remain Ian's |
| 12. QA and release | Shared-case union, three testing seats, installed cross-surface pass | Runs throughout; final pass depends on all shipping surfaces |

Proposed queue amendment on acceptance: move private engine controls ahead of the public Rust API and recognition integration. This deliberately revises A6-A8's current order; it does not silently replace the build queue. Update that queue and the affected ADR sequence in the accepted planning change before dispatch.

Engine order after 0065: surviving command corrections; shared settings/result decisions and cached metadata; structured question descriptions and schema under ticket 0069 and ADR 0039; private engine controls; recognition policy and packing decision; pure recognition/relation code plus callers; public Rust API; C; separately reviewed surface integration; release checks. Pure-core work can be subdivided without weakening acceptance. Packing is optional: retain the baseline if the candidate fails its quality/token criteria or lacks authorized measurement. No fivefold saving claim survives without evidence.

The new `2026-09-21-the-settled-details-across-all-languages.md` ledger on main records Ian's delegation and approval. Preserve those settlements rather than treating them as unreviewed proposals: optional Series-only Polars, non-deterministic SQLite functions, rejection of built questions plus competing members, spent-deadline behavior, host-rendered rows, the settled result fields and relation columns, and host cancellation channels. Test the SQLite shim cache's removal at the engine swap and DuckDB signal coexistence without reopening the chosen channel. Preserve score's implemented highest-probability meaning as above. Rank pairs already follow ADR 0017's shared-case amendment; find pairs alone do not satisfy its full-distribution promise.

Keep ten functions, simultaneous 0.1 across surfaces, the 100 MB cache, count reporting without a spending ceiling, and the agreed installers. Keep spreadsheets, serve mode, Windows, and an eleventh function outside this tranche. Structured descriptions are now authorized before recognition; typed library builders remain separately owned. Do not silently repair an unrelated issue or change an accepted safety control to pass a gate.

## SWE-2 workflow and proof

Use `swe2-researcher` for inventories and reproducer preparation, `swe2-implementer` for bounded accepted implementation/remediation, and separate `sol-reviewer` sessions for design and code review. Resume only the code reviewer for remediation review. Preserve the complexity rubric and its floors. Record Ian's requested SWE-2 implementation route in each ticket; do not relabel high-risk work as easy. Escalate ambiguous semantics, irreducible security/concurrency work, or repeated missed invariants to `sol-implementer`. The coordinator owns scope and architecture. Profile selection is not billing evidence; `/session-stats` and `/usage` show actual usage.

Once Ian authorizes execution, freeze the accepted queue and create each small ticket only when its stage begins. Give the worker an exact worktree, allowed files, invalid cases, focused checks, and a small scope. Require an observed red test, independent accepted design, focused implementation, independent code review, parent-run final gates, and exact pushed hosted evidence before landing. Do not make paid provider calls or handle credentials in workers.

Use one shared conformance file with captured and synthetic evidence distinguished. Add properties absent from rehearsal: input validation/order, relative find probabilities, grouped annotate request counts, no second send for details, mixed-success preservation, no-send refusals, concurrent independent engines, busy-backend cancellation, actual wire retries, warm/busy forks, cache failures, and immutable ownership. Keep the three QA seats: membership matrix, specification cross-reader, and a stranger with only the installed program and help. Do not seed the stranger with known findings.

## Observed verification and limits

Resumption verification: an independent Sol reviewer accepted the queue/ADR sequencing amendment and the preserved ownership and authority boundaries. The coordinator reran `install`, `lint`, `test`, and `spec` on `c29e445` plus this planning change with the provider key empty and Cargo offline mode: all passed, including 545 Rust tests, doctests, replay checks, and nineteen green how-tos. `git diff --check` passed. No product source changed and no paid call ran. Surface runtimes and website deployment were not rerun.

Coordinator ran `sdlc/scripts/install && sdlc/scripts/lint && sdlc/scripts/test && sdlc/scripts/spec` in the separate survey worktree at `76b3511`, with the real provider key empty and Cargo offline mode set. Exit 0: policy, package/no-default-features checks, ratchet 32,713/32,713, dependency audit, formatting, clippy, docs, 545 Rust tests, doctests, transform/probe/live-launcher fixture tests, 27 specification checks, 7 transform-page checks, and 19 green how-tos. Packaging warned that documentation/homepage/repository metadata is missing. The install script itself can fetch dependencies/advisories; this is not a provider call.

Coordinator ran harvest `python3 rules/tests.py`: 25 checks, zero failures. The forty-case full replay and all surface runtime suites were not rerun. No fresh model quality, persistence-performance, macOS, or installed real-engine surface claim is made. Hosted checks were observed green for main `76b3511` and later documentation-only `5c11eb9`, 0065 `6618694`, and surfaces `9ef50a6`. Product/marketing outputs and final publication remain outside this survey.

An independent Sol reviewer rejected the first plan for unclear score semantics, bare-answer versus details ownership, relation-method authority, and conflicting control/API dependencies. The revised plan preserves modal score semantics and standalone details calls, marks the relation amendment for confirmation, and states the queue amendment explicitly. Re-review accepted the planning document only. The final snapshot update incorporates the newly landed settlement ledger. No implementation, merge, paid call, publication, or registry claim was performed.
