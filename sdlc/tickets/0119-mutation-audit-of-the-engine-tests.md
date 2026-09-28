---
flow: build
priority: 119
opens: crates/thinkthen/src/engine/deadline_tests.rs crates/thinkthen/src/engine/deadline_tests/schedule.rs sdlc/ratchet.json sdlc/issues/2026-09-24-red-green-scaffold-tests-outlive-their-purpose.md
---

# 0119: Functional audit of the engine tests

Status: amended design proposed for fresh independent review. The 2026-09-24 mutation-campaign design was accepted in [its review](../records/0119-design-review.md), but Ian's later functional-gate ruling supersedes its required whole-scope mutation runs, repeated timing runs, load checks, and whole ladder. No test deletion is authorized by this preparation alone. Owner: Codex, under Ian's later build routing.

## Outcome and authority

Keep the original outcome: remove only engine test code that a stronger routine proof makes redundant, lower `sdlc/ratchet.json` by measured deleted lines, and report exactly what changed and what remains unproved. Preserve distinct parser, secrecy, cancellation, deadline, cache-miss, invalid-input, conflict, wire-byte, digest, replay and stop-accounting regressions. A line count or a mutation score alone cannot make a test redundant. No product code or public API changes belong here.

The later ruling is recorded in [the current work plan](../planning/work-plan-2026-09-27.md): ordinary validation uses small functional, unit and boundary checks; performance, saturation, churn, repeated contention, full-surface and mutation campaigns require a separately named opt-in run. [AGENTS.md](../../AGENTS.md) now requires a test to answer four questions: protected behavior, credible failing regression, why existing tests miss it, and whether it needs a test-only hook. Before deleting a test, name the stronger routine proof for every behavior it uniquely guards. This amendment changes the evidence method, not the accepted cleanup outcome. Ian can overturn the amended method.

## Bounded method proposed for review

1. Choose one unheld engine test family at a time. Read each test and its stronger candidate through the public or real internal boundary. For every proposed deletion, name the exact assertion, the credible product regression that would fail it, the stronger test's assertion and fixture, and any behavior the stronger test does not cover. A missing replacement keeps the test. Test-only exports, flags and hooks do not count as stronger proof.
2. Delete only a small, explicit equivalence group after that comparison. Run the exact proposed replacement tests on unchanged source, remove the proposed duplicate, then run those same replacement tests again. Check that the selection actually ran the expected test count and still covers the behavior. A bounded, targeted deliberate break may help when the equivalence is otherwise unclear, but it is not a whole-scope mutant census or a substitute for reading assertions. Restore any plant before committing. Stop and retain the test if the proof is ambiguous, a focused check fails without the deletion, or a regression loses its only guard.
3. Record each removed test by name, its former assertions, its replacement proof, and the focused commands and results. Record retained tests with distinct behavior, including why a superficially similar boundary is weaker. Measure test-only nonblank lines and test functions before and after from the actual diff; lower the ratchet by the measured source reduction. Do not claim a mutation score or runtime improvement without a separately authorized measurement. A small or zero deletion is an honest result.
4. Obtain fresh read-only code review of the deletion map and source diff. The reviewer checks that each named stronger test would fail on the cited credible regression, that no test-only hook or product source changed, and that the ratchet follows the measured total. Run only focused checks and cheap format, policy and diff checks for this slice. The coordinator names any related-ticket full checkpoint separately.

The first candidate family is `crates/thinkthen/src/engine/deadline_tests.rs` and its `schedule.rs` child, subject to the current lane claim before editing. The [preflight](../records/0119-functional-audit-preflight.md) inventories their distinct boundaries. In particular, `cancellation_is_observed_before_a_spent_deadline` appears to overlap the scheduler's fired-plus-spent stop case, but the low-level test also asserts `Cancel::wait` returns cancellation. It stays until an existing stronger proof for that wait behavior is identified. The exact error-format table, `Duration::MAX` handling, zero-send attempt, held response, retry wait and folder/digest deadline tests each have plausible unique failures and are not deletion candidates merely because they share a deadline topic.

## Scope and dependencies

The first implementation slice may edit only the claimed test family and the shared ratchet after source claims. The other engine and CLI tests, `conformance/`, secrecy sweep, `spec/`, green demos, live test, shell and documentation tests remain untouched in this slice. Existing `cargo-mutants` 27.1.0 pin and the 2026-09-24 review remain historical evidence, not a required tool install. No load gate, repeated mutation baseline, three-run median, arbitrary 500-line target, or whole-ladder run is a stop condition. The earlier report fields for mutation score, Beelink time and whole-suite timing are omitted unless a later opt-in campaign actually measures them.

The related hand-rolled-listener cleanup in [the test-harness issue](../issues/2026-09-25-test-harness-and-review-leftovers.md) may need `conformance/backend` and other test files. It is not silently included in this first deletion slice. It needs a separate exact file claim and preservation proof for its connection-count and arrival-order assertions. Issue item 4's old 50-run high-load demand is historical and opt-in; its functional fix remains landed and the umbrella issue stays open.

## Evidence and routing

The original [scaffold-test issue](../issues/2026-09-24-red-green-scaffold-tests-outlive-their-purpose.md) measured 787 tests and 28,164 test-path lines on `ffb8fb79`. Those numbers are historical, not today's baseline. The accepted 2026-09-24 design's repeated mutation method and review are retained as history. This amendment uses current `origin/main` at `ed99407d` and the [preflight](../records/0119-functional-audit-preflight.md); no mutations, deletions, timings or source executions have run in preparation. A fresh independent design review must accept the amendment before source edits. Fresh Codex review follows implementation under Ian's later route.

The final build record will list deleted and retained tests, exact replacement proofs, measured ratchet and focused results. It will add `## What the build taught us` here before landing and reconcile completed versus deferred work. Missed coverage becomes an issue only when a specific unguarded regression is demonstrated; this ticket does not pre-create an issue from an unmeasured mutant score.
