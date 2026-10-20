# 0749: Delete test tooling that guards no product behavior

Status: OPEN.

Milestone: 0.2.1

Phase: 2

Depends on: 0740, 0746

## Outcome

The test machinery shrinks, as rule 5 of [the 0.2.1 ruling](../decisions/2026-10-18-0-2-1-rebuilds-the-basis.md) asks. Every check that stays guards product behavior or a release artifact.

- `sdlc/scripts/build-admission.py`, `sdlc/scripts/build-admission-test.py` and every call to them are deleted. pm job admission is the only build lock, as [sdlc/README.md](../README.md) says. Callers today: `sdlc/scripts/heavy-compile`, `smoke`, `release-workflow`, `install_check_tools.py`, `registry-readmes.py`, the two GitHub workflows, two conformance tests, the DuckDB source hosts, and the Dart, PHP, Swift and Zig installed fixtures.
- `sdlc/live-test` is deleted. The `live` checks run on the real checkout, and no fixture copies `sdlc/scripts/live`.
- Each self-test that tests another test or release script, and guards no product behavior, is deleted. Candidates: `tools-self-test.py`, `hosted-workflow-self-test.py`, `release-archive-self-test.py`, `release-registry-self-test.py`, `release-language-tools-self-test.py`, `release-managed-pair-self-test.py`, `release-python-self-test.py`, `release-windows-command-self-test.py`, `release-windows-c-self-test.py`, `release-process-cleanup-test.py`, `registry-readmes-self-test.py`, `install-check-self-test.py`, `install-check-postgresql-self-test.py`, `demos-self-test`, `fetch-self-test` and `feature-inventory-test.py`, all under `sdlc/scripts/`. A candidate that guards product behavior stays, and the ticket's record says why. The installer tests stay, since the installers ship.
- `sdlc/scripts/README.md` and the gate drop the deleted entries.

This ticket changes the test setup, so it lands alone, never in the freeze batch, under rule 4 of [the standard proof commands](../decisions/2026-10-20-standard-proof-commands.md).

### Proof

- The configured gate passes through `pm ticket land 0749`.
- On the M5: `pm job run --machine m5 0749 SHA -- sdlc/scripts/job sdlc/scripts/test` and `pm job run --machine m5 0749 SHA -- sdlc/scripts/job sdlc/scripts/surfaces` pass.
- `git grep -n 'build-admission\|live-test'` finds nothing outside `sdlc/records`, `sdlc/tickets` and `sdlc/issues`.

## Evidence

Ian's process assessment of 2026-10-20 counted about 32,000 lines of test and release tooling against 225,000 lines of product, with 24 scripts that test other test scripts. The scripts above total about 10,000 lines. `build-admission.py` is a second build lock beside pm job admission. `sdlc/live-test` copies `sdlc/scripts/live` into its fixture, which the 0.2.1 ruling forbids. `tools-self-test.py` is also changed by 0740 and 0746, so this ticket follows them.

## Progress
