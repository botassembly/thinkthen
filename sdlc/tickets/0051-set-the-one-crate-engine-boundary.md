---
flow: build
priority: 38
opens: sdlc/planning sdlc/issues
---

# 0051: Set the one-crate engine boundary

Status: proposed

## Outcome

ADR 0017's required build-team review is complete and its findings are incorporated before any crate moves. Section 8 step 1 becomes an extraction across a typed boundary rather than a promise to move five command files whole. The amendment preserves command behavior, core purity, retries, diagnostics, package independence, and the rule that no engine thread survives a call.

## Current facts and decisions

The accepted ADR requires a build-team review before the merge ticket. The review in `sdlc/issues/2026-09-21-build-review-of-adr-0017-before-the-one-crate-move.md` found that the named machinery is not independent today. The schedulers still own command output and failures, the request loop reads command state and credentials, the recorder depends on the cache lock, the purity gate assumes two crates, and two input-reader threads can outlive a call.

This ticket makes the smallest corrections needed to keep the accepted design implementable. Ian can overturn any of them.

1. **Extract typed engine inputs and results.** The command keeps argument parsing, environment and credential lookup, input framing, output, diagnostics, and exit codes. The engine receives resolved questions, backend settings, records, and run settings. It returns typed judgments, usage, and structured failures. `cache_lock.rs` moves with recording.
2. **Keep structured causes under the six public error kinds.** An engine failure keeps the recording cause, backend cause, and ordered-stop metadata needed to reproduce every current command sentence and count. The command alone maps those fields to `Failure` and an exit code.
3. **Preserve behavior during the move.** Step 1 changes no command request, retry rule, ordering, recording, output byte, diagnostic, or exit code. The later engine step makes ADR 0017 section 3's changed refused-connection rule.
4. **Replace crate purity with module purity mechanically.** The existing ban tables stay intact. The lint rung checks the `core` module for prohibited APIs and checks that `core` has no dependency on `engine` or `cli`. A self-test proves both refusals.
5. **Prove the package boundary.** The command parser becomes optional and the binary requires the `cli` feature. The gate builds the library without default features, packages it without a sibling crate, and keeps the core doctests. The repository instructions and Rust standards describe the new layout; user behavior pages do not change.
6. **Keep input readers in the command.** The engine accepts already framed input or a command-owned producer whose lifetime is joined before return. No detached reader belongs to the engine. A test calls the engine inside a process that stays alive and proves no engine thread remains.
7. **Keep the accepted order.** After this amendment, Job 3 lands the shared conformance cases, Job 2 proves the DuckDB interrupt inside Python, and only then does the merge ticket implement section 8 step 1.

## Scope

Append a dated amendment to accepted ADR 0017. Update both active plans to name the corrected extraction and required proofs. Close the build-review issue as incorporated. Make no Rust, manifest, gate, specification, how-to, or package change.

Excluded: the conformance cases, the DuckDB interrupt experiment, the crate merge, public engine functions, cancellation, fork repair, cache settings, counters, transform packaging, live calls, and release publication.

## Acceptance

- ADR 0017 says that section 8 step 1 extracts an engine boundary rather than moving five files whole.
- It assigns arguments, credentials, framing, output, diagnostics, and exit codes to `cli`; resolved settings, judgments, scheduling, transport, recording, and cache locking to `engine`; and keeps `core` pure and inward-only.
- It requires structured engine causes and ordered-stop metadata sufficient to reproduce current command behavior, while leaving the six public error kinds intact.
- It freezes current retries and every other command contract during step 1.
- It requires mechanical module-purity and dependency-direction checks with self-tests, a no-default-features library build, package verification without the sibling crate, preserved core doctests, and a surviving-process thread-lifetime test.
- The two active plans preserve this order: amendment, Job 3, Job 2, merge ticket.
- The review issue is closed with a link to this ticket. It is not deleted.
- Documentation checks, all four repository rungs, and `git diff --check` pass.

## Dependencies

Accepted ADR 0017, ticket 0050, and the build-team review dated 2026-09-21.

## Complexity

- Contract: 1
- State and timing: 0
- Reach: 2
- Proof: 1
- Cost of error: 2
- Total: 6
- Minimum level floor: none
- Final level: 3
- Reasons: this changes no executable behavior, but it fixes the boundary and proof obligations for the largest remaining code move. A bad boundary would spread command concerns into every library surface.
- Selected model: `gpt-5.6-sol` with high reasoning.

Re-score if implementation begins or a public type is fixed in this ticket.

## Review

Pending independent design review.
