# 0034: Share live authority and bound interruption

Branch `ticket/0034-share-live-budget`. Built and migrated 2026-09-20.

## What landed

Every registered local worktree now uses one machine-bound allowance under Git's common directory. A job is charged its declared maximum before it receives the key. The key crosses an anonymous socket into a separately executed keyless gate only after the charge is durable. The wrapper requires a prebuilt binary and no longer scans recordings or builds with the credential present.

Signals reach only an owned child before readiness and the verified session afterward. An interrupted child gets five seconds to exit. A surviving session leaves durable unresolved state. Status and recovery never initialize authority, refund a charge, or signal a process.

The migration tool refuses dirty, locked, unreachable, active, or newly registered legacy trees. It preserves branch references and directories while detaching legacy linked worktrees at the landed revision. A durable activation fence keeps partial activation disabled even when storage synchronization and rollback both fail.

## Red then green

The original wrapper gave every worktree the full allowance and could wait forever after interruption. Early revisions of this ticket also blocked while delivering a large environment, signaled an unverified process group, ran jobs from the caller's directory, misreported signal status, recreated a missing lock, accepted malformed state, missed active historical launchers, cleared a live wrapper when boot identities disagreed, and left usable authority after persistent activation failure.

Thirty-five focused cases now cover shared reservations, process identity, credential confinement, launch failures, partial handoff, signal forwarding, bounded waiting, session survivors, restart recovery, malformed state, filesystem failures, historical launchers, retirement, and one-time activation. The formerly unstable ignored-TERM case passed five consecutive runs in 5.26 to 5.34 seconds. The pre-readiness signal case passed ten consecutive runs after its test gate waited for the actual gate process. Cleanup scans found no surviving wrapper, job, or session.

## Review

The design reviewer rejected the first draft, then accepted the rewrite with coordinated retirement, a keyless gate, complete process identity, fail-closed transfer, and exact sudden-death boundaries.

The code reviewer rejected three revisions. Its findings produced nonblocking delivery, safe group signaling, checkout-owned execution, shell-compatible statuses, separate process bounds, boot-aware recovery, permanent-lock enforcement, fixed malformed-state diagnostics, complete migration process detection, and the durable activation fence. The same reviewer reproduced the repaired failure paths, ran all 35 focused cases, checked the ratchet and file sizes, found no leaked processes, and accepted the result.

## Migration

Commit `fa3290c` landed on main first. Git history and all worktree ledgers agreed on a 476,000,000-token limit and a latest charge of 429,118. No active legacy wrapper, job, lock, dirty tree, or later live run remained. The coordinator retired every legacy linked worktree at `fa3290c`, verified all registered launchers, and activated authority `5c95a2e0-5bbf-43f1-b31b-b8936992ec97` with those totals. Final status was active with no pending run. The key and base address were unset throughout.

## Gates and size

Production measures 996 raw and 869 nonblank lines across `live`, `live-state.py`, and `live-migrate`, up from 449/426. Proof measures 1,634/1,560, up from 1,158/1,090. Every production and proof file remains below 500 nonblank lines. The Rust ceiling rises from 17,357 to 17,827 for the shared-authority process, migration, recovery, and fault proof. The retired FIFO suites and new shared helpers were checked for duplication.

| Rung | Result |
| --- | --- |
| `sdlc/scripts/install` | exit 0 |
| `sdlc/scripts/lint` | exit 0, ratchet `17827/17827` |
| `sdlc/scripts/test` | exit 0 |
| `sdlc/scripts/spec` | exit 0 |

The coordinator ran the ladder with the key and base address unset. No live call ran.
