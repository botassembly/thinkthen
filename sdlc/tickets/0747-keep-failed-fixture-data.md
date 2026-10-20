# 0747 — Keep a failed test's fixture data after the run

Status: OPEN.

Milestone: 0.3

## Outcome

- A routine or container test that fails leaves its fixture data where the pm job log names it, until `pm sweep` removes it. A passing test still cleans up.
- Changes: the test `temp_home` cleanup, `sdlc/scripts/linux-check`'s tmpfs mounts for `target/tmp` and `target/library-only/tmp`, and `sdlc/scripts/README.md`.

### Proof

- A planted failing fixture run through `pm job run --machine m5 0747 SHA --memory 16 -- sdlc/scripts/linux-check test` leaves its data in the job folder, and the log names the path; the same run passing leaves none.

## Evidence

- 2026-10-20, found by 0605: local `Scratch` names data on panic, but `temp_home` cleans TMPDIR on exit and `linux-check` mounts the temp folders as tmpfs under `--rm`, so failed fixture data is lost. pm job logs are retained. Structural risk 9 in the shared skills covers keeping failure evidence.
- Ordered after 0.2.1: rule 4 of `sdlc/decisions/2026-10-20-standard-proof-commands.md` keeps the test setup still during language work, and the retained logs have sufficed so far.

## Progress
