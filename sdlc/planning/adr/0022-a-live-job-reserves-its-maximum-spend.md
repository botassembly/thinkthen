# ADR 0022: One shared ledger precharges live jobs

- Status: Decided by the agent on 2026-09-20. Ian can overturn this decision
- Date: 2026-09-20

## Decision

`sdlc/scripts/live --max-tokens N JOB [ARG...]` is the paid-call door. It resolves the Git common directory, takes one permanent file lock, validates one append-only ledger, and appends and syncs `charge N` before it starts the job. `N` is a positive canonical decimal no greater than 999,999,999. Exact fit is allowed. The charge remains after wrapper death, job failure, or a signal.

The ledger is private and has `version 1`, one initial limit, one initial charge, then only newline-terminated positive canonical charge rows. Partial, unknown, duplicate, out-of-range, unterminated, or inconsistent data disables live work. `--status` locks and validates the same file and reports the limit, charge, and remainder without rewriting it. A reported append or sync failure has an ambiguous durable result. The operator inspects status or the ledger under the lock before retrying.

`--init` first creates and syncs a permanent initialization marker, then creates the ledger from the two audited values in `sdlc/live-tokens`. The marker prevents recreation from a stale checkpoint if the ledger is lost. Initialization refuses an existing marker or ledger, legacy `state.json`, malformed checkpoint, symlink, or unsafe mode. Every command refuses if legacy state reappears.

After precharge, the launcher releases the lock and replaces itself with `/bin/sh -- JOB ARG...` in the calling checkout. The prebuilt checkout binary leads `PATH`. The job alone receives the validated key. Git helpers receive a fixed keyless environment. Direct execution preserves the job's exit and signal status.

## Boundary

This is a cooperative allowance. It limits reservations made through the script. It does not stop a job that exceeds its declaration or a person who bypasses the script. It has no refund, measured-usage settlement, pending run, recovery protocol, process identity, worktree scan, production fault switch, or wrapper-owned completion output.

The migration retires the old `state.json` under its permanent lock, syncs the authority directory, then runs `--init`. The exact audited values and interruption states live in `sdlc/scripts/README.md`. No paid job runs during migration.
