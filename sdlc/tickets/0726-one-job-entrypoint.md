# 0726 — One entrypoint for every machine job

Status: OPEN.

Milestone: 0.2.1

Reviews: revision 2ae50ac, accept

## Outcome

- Changes: `sdlc/scripts/job`, `sdlc/pm.json`, `sdlc/scripts/README.md`, `sdlc/scripts/tools.py`.
- `sdlc/scripts/job COMMAND...` is the one way any M5 or Yellow job runs a ThinkThen check. It resolves the manifest's tools through `tools.py`, uses the pinned Python, sets `TMPDIR` to its real path, and then runs the command. Nothing else in a job sets PATH, Python or TMPDIR.
- `ticketsGate` in `sdlc/pm.json` becomes `sdlc/scripts/job sdlc/scripts/lint && sdlc/scripts/job sdlc/scripts/test`, with no hand-written PATH.
- `sdlc/scripts/README.md` tells every developer and brief to launch targeted checks as `pm job run ... -- sdlc/scripts/job CHECK`.

### Proof

- On the M5, `pm job run` with `sdlc/scripts/job python3 -c 'import sys; print(sys.version)'` prints the pinned Python, and with `sh -c 'echo $TMPDIR'` prints a resolved path, from a login environment with no extra PATH.
- One M5 gate through the new `ticketsGate` passes.

## Evidence

The coordinator reported on 2026-10-19 that several targeted M5 launches omitted the pinned Python or the canonical TMPDIR, so jobs spent machine time without product evidence. `ticketsGate` today hand-writes a PATH with five machine-specific folders and a Python one-liner for TMPDIR, which 0.2.1 rule 3 forbids. `sdlc/scripts/resolved-tools` and `tools.py` already resolve the manifest's groups for entry scripts; this puts the same resolution in front of every job.

## Amendment, 2026-10-19

`ticketsGate` keeps the existing gate routing, with lint in the 0619 Linux container and native tests on the M5, as ruled for 0615. `sdlc/scripts/job` wraps each existing gate step; it does not demand native all-language lint, substitute compilers or manifest entries, or add image inputs. The proof's gate run uses that routing.

## Progress

- 2026-10-19 started
