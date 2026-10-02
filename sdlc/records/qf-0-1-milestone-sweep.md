# Quick Fix qf-0-1-milestone-sweep: close the 0.1 items the clean rehearsals proved

Status: built in lane claude-3; fresh review returned five findings, all answered. Records only. Ian can overturn any close.

## Why

`release/0.1` was cut from `4e880cdf6`, tagged `rc/0.1.0-rc.1`. Three release rehearsals then passed every job through `draft` on all four targets:

- Run 36945370940 on main `94d0500c0`, version 0.0.1.
- Run 36998358908 on main `f65faea4e`, version 0.1.0.
- Run 37010060315 on `release/0.1` at `4e880cdf6`, version 0.1.0.

Release QA round 5 on `checkpoint/surfaces/2026-10-02-1` (`4e880cdf6`) was clean. milestones.md still listed fifteen tickets and two issues as open 0.1 items. Most of them waited only on proof from the release runners.

## Change

- Tickets 0222, 0224, 0226, 0227, 0231, 0268, 0269, 0270, 0271, 0272, 0273 and 0299 read `landed`. Each status names the runs, the smoke job IDs and the installed cases that close it. Each loses its `Milestone:` line, as the tickets README asks of a closed ticket.
- Ticket 0268 gains `## What the build taught us`. Ticket 0222 marks its post-landing audit resolved by ticket 0240.
- Tickets 0149 and 0157 stay open. Each status names the one proof left: the settings corpus, request-size and retry-count cases on native Intel macOS and macOS 15.
- The README key, backend and overhead issue moves to `closed/`. Commit `248bc6aa4` added its overhead sentence, and `release/0.1` carries it.
- Ticket 0128's Phase 3b log gains runs 36903959951 to 36937157758 and the three clean runs. Phase 3b is done. Phase 4 steps 2 to 4 are done.
- The release and install issue marks Phase 3b and the package proofs done. It names the two docs team asks that still change `release.yml`.
- milestones.md marks exit criteria 1 and 2 and path steps 2 to 4 done, refreshes the 0.1 and 0.2 blockers, and lists what is left.

## Checks

- Each run's job list: `gh run view RUN --json jobs`. Every job through `draft` succeeded. The release-only jobs were skipped.
- The four smoke job logs of run 37010060315 (110863135525, 110863135686, 110863135603, 110863135461), the x86 Linux build log (110847453213) and the `draft` log (110874926335). Each smoke job printed `surfaces: pass` for every installed file, `keeps its own panic hook` for every library ticket 0374 checks and the token cap lines. No surface printed "not run".
- `sdlc/scripts/tickets`, `sdlc/scripts/pages`, `git diff --check` and `sdlc/scripts/lint` with a non-matching private-name list. They pass.
- The private-name guard over this diff's added lines finds nothing.

## Lessons

- The rehearsal's installed modes run a selected subset of each SQL check. DuckDB, SQLite and PostgreSQL do not rerun the shared settings corpus or the request-size case there. Ticket 0231's per-target proofs ran on macOS 26, with Intel under Rosetta. So tickets 0149 and 0157 stay open. A future close that leans on a rehearsal should first list which cases the installed mode runs.
- ADR 0105 removed `thinkthen_warm`. Ticket 0149's warm parity half no longer applies, and nothing had said so in the ticket.
- Main's full `sdlc/scripts/lint` stops at the private-name check on `sdlc/pm.json:21` and `sdlc/records/qf-pm-adoption.md`. Those lines predate this change. The queue owner should fix them.

## Deferred gap

- The `nuget` job reads `secrets.NUGET_API_KEY`, which will not exist, and the `pub` job uses Google Cloud, which Ian ruled out. The docs team's asks of 2026-10-01 move both to trusted publishing. Owner: the queue owner. A change to `release.yml` restarts ticket 0128's Phase 4 checklist at step 2.
- Registry setup and Ian's go remain with the docs team and Ian.
