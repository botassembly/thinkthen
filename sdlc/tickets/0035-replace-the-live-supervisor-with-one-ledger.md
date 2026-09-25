---
flow: build
priority: 60
opens: sdlc/scripts sdlc/live-tokens .github/workflows sdlc/planning sdlc/ratchet.json
---

# 0035: Replace the live supervisor with one ledger

Status: done

## Outcome

The repository's paid-call door has one shared lock, one append-only ledger, durable precharge, read-only status, one-time initialization, and direct job execution. It has no pending run, recovery protocol, process supervisor, worktree scan, or production fault switch. The product and its requests do not change.

## Current Facts

Ticket 0034 grew the cooperative $20 guard to 996 production-script lines and 1,634 raw Rust proof lines, 1,560 nonblank. The review in `sdlc/issues/closed/2026-09-20-the-live-guard-grew-past-its-job.md` reproduced charged-but-failed writes, abandoned pending runs, misleading missing-setup errors, global worktree refusal, and retained process identities. Its tests require historical commits, run Linux assumptions from the product crate, and contain cleanup and assertion gaps. The current authority is active with no pending run at 476,000,000 allowed and 429,118 charged.

## Scope

- `sdlc/scripts/live --init` creates `.git/thinkthen-live/ledger` under Git's common directory from the two audited values in `sdlc/live-tokens`. It refuses an existing ledger, `.git/thinkthen-live/state.json`, a malformed checkpoint, symlinks, and unsafe modes. It takes no key and runs no job.
- `sdlc/scripts/live --status` locks and validates the ledger, then prints `limit_tokens`, `charged_tokens`, and `remaining_tokens`. Missing setup says to run `--init`. It changes nothing.
- `sdlc/scripts/live --max-tokens N JOB [ARG...]` keeps the current positive canonical bound, relative job resolution, prebuilt-binary check, key validation, caller environment, checkout working directory, and job status. It locks the ledger, refuses an over-limit reservation, appends and syncs one complete charge, unlocks, and replaces itself with the job.
- The ledger has a fixed version and initial limit/charge followed only by canonical positive charge rows. A partial or unknown row fails closed. The job never starts after an append or sync error. A complete row still present is charged; a partial or unknown row disables live work. Durability after a reported sync failure is unknown and requires inspection of `--status` or the documented audit lever before retry. A successfully synced charge remains when the wrapper dies or the job fails.
- Delete the gate, socket, pending and unresolved states, recovery, process identities, signal forwarding, worktree enumeration, migration command, old-launcher checks, production test switches, post-job usage scan, and wrapper-owned completion line.
- Replace the four Rust live suites with a Linux script suite under `sdlc/`, invoked by rung 2 only on Linux. It uses temporary repositories, dummy keys, local jobs, and no network. Remove CI's full-history checkout and the obsolete install prerequisites.
- Rewrite ADR 0022 whole around the retained cooperative guarantee. Rewrite the prospective and main plans around Ian's accepted version-one order. Record raw and nonblank line deletion in the ticket record.
- After landing, the coordinator performs this exact migration with the key unset and the permanent `.git/thinkthen-live/lock` held exclusively: verify the old `state.json` is active, has no pending run, and exactly matches the checkpoint; refuse an existing `ledger` or `state-v1-retired.json`; rename `state.json` to `state-v1-retired.json`; sync `.git/thinkthen-live/`; release the lock; run `live --init`; then require `live --status` to report the same 476,000,000 limit and 429,118 charge. A failure or interruption before the rename leaves only the old authority. One after the rename but before initialization leaves both launchers disabled. `--init` refuses while `state.json` exists, and every new action refuses if it reappears, so old and new authority are never usable together. No paid job is part of migration.

Excluded: refunds, measured usage, hard enforcement against a job that exceeds its declaration, automatic repair, repository transfer, distributed locking, a product status command, or a paid call.

## Acceptance

- Red tests first reproduce missing setup, one-time initialization, status, exact-fit and over-limit charges, two concurrent launchers, malformed and partial rows, storage failure and its ambiguous-charge message, job exit and signal status, relative job arguments and working directory, key absence from argv/files/other children, and direct key delivery to the job alone.
- A job reads its durable charge before it starts. Two reservations of 7 and 3 against 10 both run once; another token is refused. A killed job leaves its charge and no recovery state.
- A shallow clone and a source archive run product tests without historical commits. macOS skips the Linux live suite without running Linux code. No live test sits in the Rust ratchet.
- A migration fixture starts with the current active `state.json` and exact checkpoint. It proves the states before rename, after rename, after initialization, and with a restored legacy state; only the last completed state launches, and it carries exactly 476,000,000 allowed and 429,118 charged.
- The old supervisor, helpers, migration tool, historical fixtures, and worktree warning are gone. No registered worktree is deleted or pruned.
- The four repository rungs pass with the key unset. GitHub's check passes on the pushed main commit. A separate agent accepts the implementation and tests.

## Dependencies

None. Migration waits for the landed green commit.

## Complexity

- Contract score: 1
- State and timing score: 2
- Reach score: 1
- Proof score: 2
- Cost of error score: 2
- Total: 8
- Minimum level floor: level 4, because the change replaces shared durable spend state and migrates live authority that controls a credential-bearing job.
- Final level: 4
- Reasons: the runtime contract is explicit, but partial writes, concurrency, credential isolation, direct execution, and one-time migration require hostile failure proof. The retained behavior cannot be split from the authority conversion without leaving two active meanings of the same budget.
- Selected model: `gpt-5.6-sol` with medium reasoning.

## Review

- Design review: accepted after the reviewer rejected the first draft until the migration paths, interruption states, sync ambiguity, and proof count were exact
- Code review: accepted after the reviewer reproduced two fail-closed defects and accepted their regression-tested repair

## Implementation

Production fell from 996 raw and 869 nonblank lines to 309 raw and 267 nonblank lines. Proof fell from 1,634 raw and 1,560 nonblank Rust lines to 228 raw and 206 nonblank shell lines. The deletions are 687 raw and 602 nonblank production lines and 1,406 raw and 1,354 nonblank proof lines. The Rust ratchet fell from 17,827 to 16,267 nonblank lines.

The first script run failed because missing setup returned the old generic failure instead of the required `--init` instruction. The completed script suite passes with dummy keys, local jobs, and no network. The coordinator still owns landing, the real migration, and GitHub verification.

The first code review reproduced reusable initialization after ledger deletion and acceptance of a final row without a newline. Regression tests failed for both reasons. The remediation syncs a permanent initialization marker before ledger creation, refuses a missing ledger after that point, and requires the ledger bytes to end in a newline.

Commit `6343e7a` landed on `main`, passed the four local rungs with the key and base address unset, and passed GitHub run `35523772125`. The coordinator migrated the active idle authority under its lock. The new status reports a 476,000,000 limit, 429,118 charged, and 475,570,882 remaining. Migration ran no job and made no paid call.
