# 0737: Read SQLite source guard counts on macOS

Status: OPEN.

Milestone: 0.2.1

Phase: 2

## Outcome

- Changes: `databases/sqlite/check.sh`.
- The existing source guards read the same counts on BSD and GNU tools. BSD wc padding does not refuse valid source before the installed SQLite checks run.
- Preserve both expected guard counts, the protected-source scopes, their failure behavior and every downstream check. Change no product code, platform support, timeout, provider or toolchain.
- Reproduce the failure through the existing check on main, then fix the count interpretation in that script. Add no helper, runner or new checking framework.

### Proof

- Through pm job on the M5, the unchanged script reproduces the false refusal, then the repaired existing SQLite routine passes the guards and runs its downstream cases. Verify a genuinely wrong protected-source count still refuses before builds, using an existing fixture or temporary job-local mutation.

## Evidence

M5 job 0724/1792505486688-96923, based on main 969e17b, printed the expected counts of zero catch_unwind and two contained guards but exited one before SQLite portable/settings tests. Its developer traced the string comparison to BSD wc whitespace. This ticket verifies that cause and preserves the guard assertions.
