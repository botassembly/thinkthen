---
flow: build
priority: 38
opens: sdlc/planning sdlc/issues
---

# 0051: Set the one-crate engine boundary

Status: in progress

## Outcome

ADR 0017's required build-team review is complete and its findings are incorporated before any crate moves. Section 8 step 1 becomes an extraction across a private typed boundary rather than a promise to move five command files whole. The amendment preserves streaming command behavior, core purity, retries, diagnostics, package independence, the library's unwind behavior, and the rule that no engine worker survives a call.

## Current facts and decisions

The accepted ADR requires a build-team review before the merge ticket. The review in `sdlc/issues/2026-09-21-build-review-of-adr-0017-before-the-one-crate-move.md` found that the named machinery is not independent today. The schedulers still own command output and failures, the request loop reads command state and credentials, the recorder depends on the cache lock, and the purity gate assumes two crates.

The review also found one boundary that must stay in the command during step 1. The current scheduler detaches a command-owned input reader so a blocked standard-input read cannot hide completed answers or hang a closed-pipe exit. The engine cannot join that reader without changing behavior. Its own request workers can and must remain scoped.

Job 2's DuckDB interrupt proof has already landed in `sdlc/planning/databases/duckdb.md` and experiment 207. It is evidence for the accepted design, not future work.

This ticket makes the smallest corrections needed to keep the accepted design implementable. Ian can overturn any of them.

1. **Extract a private event boundary.** The command keeps argument parsing, environment and credential lookup, input framing, the detached standard-input reader, output, diagnostics, and exit codes. It sends framed input events to a private engine scheduler. The engine receives resolved questions, backend settings, records, and run settings, then returns typed per-row result events through a callback or equivalent bounded bridge. It does not collect a whole run. `cache_lock.rs` moves with recording.
2. **Keep private structured causes under the future six error kinds.** The step-1 engine failure is private. It carries the future kind plus the recording cause, backend cause, and ordered-stop metadata needed to reproduce every current command sentence and count. The command alone maps those fields to `Failure` and an exit code. This ticket fixes no public `thinkthen::Error` type and widens no public surface.
3. **Preserve behavior during the move.** Step 1 changes no command request, retry rule, ordering, width bound, memory bound, recording, output byte, diagnostic, or exit code. The refused-connection change waits for section 8 step 3's fast-stop work.
4. **Replace crate purity with module purity mechanically.** The `core` module keeps module-level `forbid` attributes over the accepted disallowed API tables. The policy check scans the complete `core` source tree for dependency paths to `engine` or `cli` and permits only the former core crate's runtime dependencies: `serde`, `serde_json`, `sha2`, and `thiserror`. Self-tests plant one prohibited standard API, one reverse module reference, and one outer-only dependency such as `ureq` in `core`, then prove that lint rejects each.
5. **Prove the package boundary.** Every CLI-only dependency, including `clap` and `csv-core`, becomes optional and the binary requires the `cli` feature. The gate builds the library with default features off, inspects that resolved graph, packages without a sibling crate, and keeps the core doctests. A release library proves `panic = "unwind"`; the command release path alone opts into abort through its build command or equivalent packaging step. Repository instructions and Rust standards describe the new layout; user behavior pages do not change.
6. **Bound engine thread lifetime.** All engine request workers and internal feeders are scoped and joined before the engine call returns. The command-owned input reader may remain detached during step 1. A test calls the engine inside a process that stays alive and proves no engine worker remains. The public iterator design may replace the private event bridge in section 8 step 2.
7. **Keep the remaining order.** After this amendment, Job 3 lands the shared conformance cases. The merge ticket then implements section 8 step 1. Job 2 is already complete.

## Scope

Append a dated amendment to accepted ADR 0017. Update both active plans to name the corrected extraction, required proofs, completed Job 2, and remaining order. Close the build-review issue as incorporated. Make no Rust, manifest, gate, specification, how-to, or package change.

Excluded: the conformance cases, the crate merge, public engine functions or errors, cancellation, fork repair, fast-stop behavior, cache settings, counters, transform packaging, live calls, and release publication.

## Acceptance

- ADR 0017 says that section 8 step 1 extracts a private engine boundary rather than moving five files whole.
- It assigns arguments, credentials, framing, the detached input reader, output, diagnostics, and exit codes to `cli`; resolved settings, judgments, scheduling, transport, recording, and cache locking to `engine`; and keeps `core` pure and inward-only.
- It requires bounded per-row result events so output order, early closed-pipe exit, width, and flat memory stay unchanged.
- It requires a private engine failure with a future kind, exact structured causes, and ordered-stop metadata sufficient to reproduce current command behavior. It fixes no public error type.
- It freezes current retries and every other command contract during step 1. The refused-connection change remains in section 8 step 3.
- It requires `core` module-level `forbid` attributes, a whole-tree reverse-dependency check, a runtime dependency allowlist of `serde`, `serde_json`, `sha2`, and `thiserror`, and self-tests that plant one banned standard API, one reverse reference, and one outer-only dependency such as `ureq`.
- It requires all CLI-only dependencies to be optional, a no-default-features library build and graph check, package verification without the sibling crate, preserved core doctests, release-library unwind proof, a command-only abort path, and a surviving-process engine-worker lifetime test.
- The two active plans record Job 2 as complete and preserve this remaining order: amendment, Job 3, merge ticket.
- The review issue is closed with a link to this ticket. It is not deleted.
- Documentation checks, all four repository rungs, and `git diff --check` pass.

## Dependencies

Accepted ADR 0017, ticket 0050, the completed Job 2 proof, and the build-team review dated 2026-09-21.

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

Independent design review rejected the first proposal because it would join a deliberately detached command reader, treated completed Job 2 as future work, omitted release panic and optional-dependency proofs, and left purity enforcement too vague. The corrected design keeps the command reader, scopes engine workers, records Job 2, and makes each package and purity proof mechanical. Independent re-review accepted the corrected ticket for implementation.

Implementation review found that the first amendment preserved the API bans and dependency direction but not the old core crate's dependency boundary. The repair adds the exact runtime allowlist and a third planted-failure proof. It also says explicitly that the amendment replaces section 8's stale Job 2 order sentence.
