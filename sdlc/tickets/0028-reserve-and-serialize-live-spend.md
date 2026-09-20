---
flow: build
priority: 86
opens: sdlc/scripts/live sdlc/scripts/README.md sdlc/live-tokens crates/thinkthen/tests/live_script.rs probes sdlc/issues sdlc/planning/plan.md
---

# 0028: Precharge and serialize live spend

Status: ready

## Outcome

The live-call door charges each job's approved maximum before paid work starts, and concurrent or interrupted jobs cannot reopen or lose that authority.

## Current Facts

The script starts any job while `spent_tokens` is one below `limit_tokens`, however large the job may be. It reads and replaces one ledger with no lock. Its later file scan can miss a paid repeat whose immutable recording stays unchanged, a deleted or backdated file, and a call that wrote no usage file. The issue and ADR 0022 record the observed undercount and race.

## Scope

- Require `--max-tokens N` before the job path. Accept canonical decimal values from 1 through 999,999,999. Reject the old invocation before build or job execution.
- Validate one canonical positive limit and one canonical nonnegative spend no greater than the limit, both at most 999,999,999. Reject duplicate or unknown data rows.
- Hold `sdlc/live-tokens.lock` from ledger validation through child completion. Write its safe owner and state record without printing its contents.
- Refuse a reservation above the remainder. Atomically add an accepted reservation to the ledger before starting the job, and never refund it automatically.
- Forward HUP, INT, and TERM to the job shell, wait for it to stop, preserve the signal status, and then clean up. Leave uncertain precharge or uncatchable-stop state fail-closed for the documented manual recovery.
- Keep the newer-JSON count as informational output only. Warn and fail a successful job when measured use exceeds its reservation. Preserve a failed job's status.
- Update the script page and living invocation examples. Make no network call in a test.

Excluded: exact vendor-token prediction, interrupting a request before its response, automatic reservation refunds, changing the approved total, counting output tokens, a public binary budget flag, and treating recording files as authoritative accounting.

## Acceptance

- With a fake build command and local job, one token remaining refuses a two-token reservation before either runs. A one-token reservation precharges the ledger and runs. A job measured below its reservation leaves the full reservation charged.
- A repeated or absent usage file returns no budget. A job that deletes or backdates its output also leaves the full reservation charged. These cases prove that the scan cannot reopen authority.
- A failed job retains its nonzero status and full charge. A successful job measured above its reservation keeps the charge, exits 1, and names only the reservation and measured count.
- A deterministic two-process test holds the first job after precharge. The second invocation refuses before build and job execution. After release, the first removes its lock and temporary files.
- A deterministic TERM test sends the signal to the wrapper alone. The wrapper forwards it, waits for a child that records receipt, keeps the full charge, removes the lock, and exits 143.
- A prebuilt stale lock in `charged` state refuses. Its fixture proves the documented recovery keeps the recorded charge. An uncertain or lower ledger stays fail-closed.
- Missing, zero, leading-zero, signed, decimal-point, over-999999999, and extremely long reservations run no build or job. Ledger fixtures cover missing and duplicate fields, unknown data, zero or oversized limits, leading zeroes, spend above limit, and the exact numeric edges.
- The old `live JOB` form refuses before build or job execution. The script header, script README, and living probe instructions use the new form. Historical records remain unchanged.
- A missing job, blank key, held lock, invalid ledger, build failure, and precharge write failure run no job and preserve a safe ledger or fail-closed lock as the ADR specifies. The full ladder passes with the real key and base address unset.

## Dependencies

ADR 0022, decided with this ticket. Ticket 0027 is landed.

## Complexity

- Contract score: 2
- State and timing score: 2
- Reach score: 1
- Proof score: 2
- Cost of error score: 2
- Total: 9
- Minimum level floor: level 4
- Final level: 4
- Reasons: the invocation and recovery contract change; correctness depends on shared durable state, concurrent processes, interruption, and partial filesystem failure; proof needs fake paid-work boundaries and exact accounting; a wrong check can authorize irreversible external spend or lose its record.
- Selected model: `gpt-5.6-sol` with medium reasoning

This ticket is irreducible at level 4. The precharge, serialization, and recovery state form one guarantee: no paid job begins without durable authority that another process and a later recovery can see.

## Review

- Design review: accepted after one rewrite. The reviewer found that refunding from the recording scan recreated the undercount, signal and accounting failures could reopen the door, numeric bounds were undefined, stale-lock recovery lacked evidence, the old invocation was untested, and shared paid state sets a level 4 floor. It accepted the permanent conservative precharge, fail-closed recovery, signal lifecycle, numeric bounds, compatibility proof, and level 4 Sol Medium route.
- Code review: pending
