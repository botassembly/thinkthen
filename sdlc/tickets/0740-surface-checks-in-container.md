# 0740: Run one surface's check in the Linux container with its own tools

Status: OPEN.

Milestone: 0.2.1

Phase: 3

Depends on: 0738

Reviews: revision e0dfd1a, accept

## Outcome

- Changes: `sdlc/scripts/linux-check`, `sdlc/linux/Dockerfile`, `sdlc/linux/install.py`, `sdlc/scripts/tools.py`, `sdlc/tools.json`, `sdlc/scripts/README.md`, `conformance/parity.py`, `sdlc/scripts/smoke`, `sdlc/scripts/surfaces`, `sdlc/scripts/tools-self-test.py`, `sdlc/scripts/hosted-workflow-self-test.py`, `sdlc/tools/python.json`.
- `linux-check surface PATH` runs that surface's existing `check.sh` (for example `libraries/r` or `databases/duckdb`) in the pinned image, with the same caps, labels and mounts as the other checks.
- The image installs a language's or database's tools when its `sdlc/tools/NAME.json` declares Linux tools, each in its own layer from 0738. Adding a language's tools edits only that language's file, never a shared list, so phase 3 tickets add tools in parallel without touching the same lines.
- A surface whose declared tools are missing from the image fails, never skips.
- `test-full-cases` and `test-stress` read the same per-surface selection, so the checkpoint covers every surface whose tools the image holds.

### Proof

- On the M5 through pm job: `sdlc/scripts/linux-check surface libraries/c` passes in the arm64 image, and on Gator through pm job the same command passes in the x86_64 image.
- Declaring a tool for a scratch surface in its own `sdlc/tools/NAME.json` (not landed) rebuilds only that layer and the surface check sees the tool; removing it makes the surface check fail with the missing tool named.

## Evidence

The core cut moved each language's and database's toolchain into its own phase 3 ticket, which proves it in the container. About 35 phase 3 tickets prove Linux "at the 0619 container checkpoint", but `linux-check` accepts only `lint`, `test`, `test-full-cases`, `package`, `test-stress` and `prepare`, so no command runs one surface's check. Tool selection is one `linux.tool_groups` list in `sdlc/tools.json`, so every language ticket would edit the same lines and none could run in parallel. Found by the phase 3 readiness review, 2026-10-20. It lands before any language ticket's container proof.

## Progress

- 2026-10-20 Standing ruling7 covers shared parity, smoke and surface selection, canonical Python tool ownership and their existing fixtures; applicable pure and mechanical Gator proofs passed, while real image and C surface proofs remain before acceptance.
- 2026-10-20 started
- 2026-10-20 next: setup: original Gator prepare failed on pkg-config discovery, fixed by cee0c65d8; fresh review accepted e0dfd1a after actual M5 C, Gator C and scratch fragment proofs passed, and the configured landing gate is next.
