---
flow: build
priority: 90
opens: sdlc/scripts sdlc/live-tokens sdlc/planning sdlc/issues crates/thinkthen/tests/live_authority.rs sdlc/ratchet.json probes demos
---

# 0034: Share live authority and bound interruption

Status: landed

## Outcome

Every admitted local worktree charges one durable allowance. An interrupted job gets a five-second child-wait grace and then settlement or unresolved state. Recovery retains every charge and never signals a process. Filesystem synchronization can extend wrapper runtime beyond the grace.

## Current Facts

`live` gives each worktree the full allowance and waits indefinitely after interruption. Its 449-line wrapper and 1,158 lines of Rust proof cover one checkout and a filesystem FIFO. The reopened live-budget issue records the evidence.

## Scope

- Apply the rewritten ADR 0022 amendment: shared state/lock, exact identity bindings, a non-authorizing checkpoint, checked recovery/status, and coordinated retirement of every legacy launcher.
- Exec clean `-I -S` helpers and gate. Verify readiness and PID/PGID/SID/boot/start identity before charge; send the key over an anonymous socket only after sync, and require receipt then separate release. The gate inherits no lock descriptor.
- Require a prebuilt binary. Remove automatic builds and explicitly retire the scan's over-reservation warning/exit-1 contract. Preserve validation, key/path bytes, permanent charges, and job/first-signal statuses.
- The coordinator owns audited migration after landing. Other installations or a moved repository require explicit retirement/transfer. Historical checkouts get no live authority. Check runtime prerequisites before charge and update the install rung and live instructions.

Excluded: paid calls, changing the approved allowance, refunds, distributed locking, a general initialization command, automatic process killing during recovery, and a hard cap on a job that exceeds its declared maximum.

## Acceptance

- Red first: use a real temporary Git repository and linked worktree. Against one allowance of 10, reserve 7 in one tree. The other refuses during that run and refuses another 7 afterward; 3 succeeds. Both report 10 charged. Each launches its own binary with intact arguments.
- Run the actual `2c32524` and `a556b97` launchers against the migrated checkpoint; both refuse before build/job. Prove migration refuses dirty/active/locked trees, preserves branch tips/directories, and detects a newly registered historical tree. Reverted checkpoint, separate clone, missing/corrupt authority, mismatched binding, and legacy lock grant nothing.
- SIGKILL/restart at every boundary listed in the amendment, plus injected file/directory sync failures, proves zero pre-release job starts and durable charge before any release. Spawn/socket/setsid/readiness/identity/exec failures retain the stated charge and clean up or remain closed.
- A job and descendant ignore TERM. Signal only the wrapper; prove forwarding, five-second child-wait grace, status 143, unresolved state, and refusal of another launch. Repeated signals never extend grace. Test HUP/INT, cooperative exit, and leader exit with a surviving session member. Recovery clears only after absence.
- Exercise reused PID, changed boot, unreadable identity, corrupt state, and held lock. Assert zero recovery signals and no refunds or double charges. Status/recovery never initialize authority.
- Inspect a blocked helper and ready gate through `/proc/PID/environ`; the marker key is absent. Preserve accepted key/path bytes and exclude key from state, argv, diagnostics, and helper environments. Verify the exact interpreter flags and prerequisite refusals.
- Report production/proof raw and nonblank counts across all languages against 449/426 and 1,158/1,090; make no unmeasured smaller claim. Tests use no paid service and clean up child groups/worktrees. Exact ratchet and four rungs pass with key/base unset.

## Dependencies

ADR 0022's amendment. Ticket 0028 is landed. This ticket blocks the paid measurements in ticket 0015, not its local design work.

## Complexity

- Contract: 2; state and timing: 2; reach: 2; proof: 2; cost of error: 2. Total: 10.
- Minimum floor and final level: 4. Shared paid authority, migration, interruption, credential confinement, and crash recovery require one start-to-recovery proof.
- Selected model: `gpt-5.6-sol`, medium reasoning.
- Split considered: changing only the ledger path leaves the old owner/FIFO state in service. Moving authority and its recovery together avoids a temporary state format and gives one complete migration. No product command changes belong in this ticket.

## Review

- Design review: accepted after the first draft was rejected. The revised design retires registered legacy launchers, execs a keyless gate, verifies complete process identity before charge, defines migration and transfer, explicitly retires the scan contract, covers sudden death and restart, limits the grace claim to child waiting, and measures rather than assumes simplification.
- Code review: rejected three times. The first remediation added cancellation-aware nonblocking delivery, owned-child signaling before verified readiness, stale-boot recovery without process inspection, permanent-lock enforcement, shell-compatible signal statuses, actual `2c32524` wrapper/job detection, primary-tree retirement, and the full fault/state/process proof matrix. The second made failed activation disabled, required consistent wrapper/gate boot identity, and detected an actual relative `2c32524` wrapper with its execed job. The third added a pre-publication activation fence that runtime checks and persistent storage failure cannot remove. The supervisor is split so every production and proof file remains below 500 nonblank lines. The same reviewer then accepted all prior findings as resolved.
- Main CI then failed because the checkout action's default one-commit history omitted `2c32524` and `a556b97`, so required historical-launcher tests could not read their scripts. The pinned checkout action now requests full history with `fetch-depth: 0`; the no-secret gate and historical tests remain unchanged.
- Measured implementation size is 996 raw and 869 nonblank lines: `live` 539/478, `live-state.py` 171/145, and `live-migrate` 286/246. The prior wrapper was 449/426, so shared authority and migration add 547 raw and 443 nonblank lines. Measured proof size is 1,634 raw and 1,560 nonblank lines: `live_authority.rs` 496/476, `live_faults.rs` 504/486, `live_migration.rs` 299/287, `live_recovery.rs` 53/49, and shared support 282/262. The prior proof was 1,158/1,090, so proof adds 476 raw and 470 nonblank lines.

## Migration

Commit `fa3290c` landed on main before migration. The coordinator removed generated `__pycache__` directories that made three otherwise-clean worktrees dirty, then verified that no registered tree was locked, prunable, unreachable, dirty, active, or carrying a legacy lock. Git history and every worktree ledger agreed on a 476,000,000-token limit and the latest 429,118-token charge. No later live run existed.

`live-migrate --retire fa3290c` detached every legacy linked worktree at the landed revision while preserving its branch reference and directory. A second verification passed. One-time activation created authority `5c95a2e0-5bbf-43f1-b31b-b8936992ec97` with the audited limit and charge. `live --status` then reported `active`, the same authority, the same totals, and `pending none`. The key and base address were unset throughout.
