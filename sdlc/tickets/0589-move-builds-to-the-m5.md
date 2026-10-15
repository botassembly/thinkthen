# 0589 — move-builds-to-the-m5

Status: OPEN.

Milestone: 0.2.1

## Outcome

ThinkThen's routine builds and tests for a ticket run on the M5 through `pm job run`, and `pm ticket land --merge-branch` runs the declared gate there, while Linux release qualification stays on the Beelink at each candidate.

## Evidence

- pm asked on 2026-10-15, at Ian's request, extending ruling 24. The C and C++ sample builds with sanitizers load the Beelink to 15 to 18 of its 16 cores while every agent runs there. The M5 has 18 cores and 128 GB. Genome made the same move.
- Starts from: the M5 clone, last synced to 223fb8c on 2026-10-06; `sdlc/scripts/{install,lint,test}`; the Beelink line in `AGENTS.md`; the M5 entry in the agents repo's `machines.json`.
- The M5 runs macOS. The routine suite reads `/proc/pressure/io`, and the 0561 export check uses `nm -D`. Some checks may be Linux-only.
- Starts after 0.2 publishes. The 0.2 candidate qualifies on the Beelink under 0543 and must not change machines mid-run.

## Scope

- Changes: `AGENTS.md`, `CLAUDE.md`, `sdlc/scripts/test`, `sdlc/scripts/lint`, `sdlc/scripts/install`, `sdlc/planning/release-process.md`, and the agents repo's `machines.json`.
- Checklist:
  1. Sync the M5 clone and run `sdlc/scripts/install`, `lint` and `test` there through `pm job run` at one pushed SHA. List every check that fails or skips on macOS, with the reason.
  2. Make each Linux-only check skip on macOS with a plain message, or run it in a Linux container on the M5. Keep each such check in the Beelink candidate run.
  3. Add thinkthen to the M5's `builds` in `machines.json`.
  4. Rewrite the Beelink line in `AGENTS.md` and `CLAUDE.md`: routine builds and tests run on the M5 through `pm job run`; Linux qualification runs on the Beelink at each candidate.
- Keeps: every check in the routine suite still runs somewhere before landing; the candidate still runs the full Linux release suite on the Beelink; Mac-specific hosted checks stay on GitHub macOS runners.

## Proof

`pm job run 0589 SHA -- sdlc/scripts/test` and `-- sdlc/scripts/lint` pass on the M5 at the landing SHA, and one `pm ticket land --merge-branch` on another ticket runs its gate on the M5.

## Defers

Moving the release suite off the Beelink. That waits on a Linux builder decision.
