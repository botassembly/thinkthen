# 0688: Name the core-freeze commit

Status: OPEN.

Milestone: 0.2.1

Phase: 2

Depends on: 0612, 0626, 0627, 0628, 0629, 0635, 0636, 0637, 0638, 0642, 0643, 0689, 0691, 0692, 0705, 0707, 0708, 0709, 0719, 0720, 0721, 0723, 0724, 0725, 0731, 0732, 0741, 0744

## Outcome

- Changes: `sdlc/decisions/`, `sdlc/planning/milestones.md`.
- One reviewed main commit is named as the 0.2.1 core freeze. It fixes `libraries/c/include/thinkthen.h`, the final packet schema, `Engine::call`, the value tree for the direct Rust bridges, the settings parser and the packet oracle `conformance/fixtures/packets.json`.
- A new dated decision record in `sdlc/decisions/` names the commit hash, with the rule that any later change to the header, the packet schema, the run function or the value tree needs a ruling, and every language then moves in one batch. The 0.2.1 section of `sdlc/planning/milestones.md` links it.
- Findings after the freeze go to the next release unless they block shipping.
- Every phase 3 ticket depends on this ticket.

### Proof

- The decision record names the commit, and that commit passes the configured landing gate and native Rust/C smoke and package on the M5, each through pm job with one log.
- Exact native smoke command: `pm job run --machine m5 0688 SHA --disk 20 --memory 16 -- sdlc/scripts/job python3 sdlc/scripts/tools.py rust,c -- sh sdlc/scripts/smoke libraries/rust libraries/c`.
- Exact native package command: `pm job run --machine m5 0688 SHA --disk 20 --memory 16 -- sdlc/scripts/job python3 sdlc/scripts/tools.py rust -- sh sdlc/scripts/package`.
- The full 0691 checkpoint run on both containers continues alongside language work and gates 0687, per [the freeze decision](../decisions/2026-10-20-freeze-names-the-contract.md).
- `pm next --milestone 0.2.1` shows no phase 3 ticket started before this ticket closed.

## Evidence

- Rule 1 of `sdlc/decisions/2026-10-18-0-2-1-rebuilds-the-basis.md` says a named commit fixes the C interface and the final result packet, and no language ticket starts before it. In 0.2, the binding design changed four times and the header changed 51 times during binding work, because no commit was ever named as the freeze. Rule 7 closes scope at the same point.
- `sdlc/decisions/2026-10-19-no-version-numbers-before-1-0.md` lands its removals before the freeze; 0706 to 0709 carry them, and 0705 removes the fixture time limits that fail gates on a loaded M5.

## Amendment, 2026-10-20 (freeze names the contract)

Ian approved [the freeze decision](../decisions/2026-10-20-freeze-names-the-contract.md) after [the five-delays analysis](../records/2026-10-20-five-delays-root-cause.md). The freeze commit is the first main commit after 0744 lands whose configured gate and native smoke and package pass; the contract tickets already landed. 0743 and 0745 leave this ticket's dependencies, land alongside language lanes and gate 0687. The three-platform checkpoint run moves to 0687; a finding there that changes the header, packet schema, `Engine::call` or value tree needs a ruling.

## Progress

- 2026-10-20 started
- 2026-10-20 next: The 52a arm64 package checkpoint failed the shared-engine worker-cleanup assertion in M5 receipt 1792518200923-4409, now owned by fresh developer 0743; native Rust/C smoke passed in 1792518176184-85386, independent native package and Yellow checks continue, and the freeze decision remains unwritten until the repaired checkpoint qualifies.
- 2026-10-20 next: product: Gator 0691/1792522536190-1088521 stopped on the shipped TLS advisory now owned by 0744, while Yellow 0691/1792518201049-861514 passed package and lint but hit nine routine deadlines with underlying causes unresolved; native smoke and package passes remain retained, 0743 has fresh acceptance, 0745 owns the pipe-fixture race, and no freeze decision is written before repaired source qualifies.
- 2026-10-20 next: setup: PM cleanup removed raw logs for Gator 0744 receipts 1792527683967-1234165, 1792528070953-1250064 and 1792529452380-1255765, while their recorded diagnostics and four preserved M5 receipts remain; native smoke and package qualification now proceeds on published main 4e6bb853d before the freeze batch.
