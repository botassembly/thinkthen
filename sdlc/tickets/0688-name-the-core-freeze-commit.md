# 0688: Name the core-freeze commit

Status: OPEN.

Milestone: 0.2.1

Phase: 2

Depends on: 0612, 0626, 0627, 0628, 0629, 0635, 0636, 0637, 0638, 0642, 0643, 0689, 0691, 0692, 0705, 0706, 0707, 0708, 0709

## Outcome

- Changes: `sdlc/decisions/`, `sdlc/planning/milestones.md`.
- One reviewed main commit is named as the 0.2.1 core freeze. It fixes `libraries/c/include/thinkthen.h`, the final packet schema, `Engine::call`, the value tree for the direct Rust bridges, the settings parser and the packet oracle `conformance/fixtures/packets.json`.
- A new dated decision record in `sdlc/decisions/` names the commit hash, with the rule that any later change to the header, the packet schema, the run function or the value tree needs a ruling, and every language then moves in one batch. The 0.2.1 section of `sdlc/planning/milestones.md` links it.
- Findings after the freeze go to the next release unless they block shipping.
- Every phase 3 ticket depends on this ticket.

### Proof

- The decision record names the commit, and that commit passes the 0691 checkpoint run: the release suite natively on the M5, in the arm64 container on the M5 and in the x86_64 container on Yellow, each through pm job with one log.
- `pm next --milestone 0.2.1` shows no phase 3 ticket started before this ticket closed.

## Evidence

- Rule 1 of `sdlc/decisions/2026-10-18-0-2-1-rebuilds-the-basis.md` says a named commit fixes the C interface and the final result packet, and no language ticket starts before it. In 0.2, the binding design changed four times and the header changed 51 times during binding work, because no commit was ever named as the freeze. Rule 7 closes scope at the same point.
- `sdlc/decisions/2026-10-19-no-version-numbers-before-1-0.md` lands its removals before the freeze; 0706 to 0709 carry them, and 0705 removes the fixture time limits that fail gates on a loaded M5.
