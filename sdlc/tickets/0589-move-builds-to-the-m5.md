# 0589 — move-builds-to-the-m5

Status: COMPLETE.

Milestone: 0.2.1

Reviews: revision 4465afa, accept

## Outcome

ThinkThen's development and release builds, tests, lint and gates run on the M5 through `pm job run`, and `pm ticket land --merge-branch` runs the declared gate there. Linux is checked by GitHub CI on every push and by occasional Yellow runs that no routine or release workflow depends on. The Beelink builds nothing.

## Evidence

- pm asked on 2026-10-15, at Ian's request, extending ruling 24. The C and C++ sample builds with sanitizers load the Beelink to 15 to 18 of its 16 cores while every agent runs there. The M5 has 18 cores and 128 GB. Genome made the same move.
- Starts from: the M5 clone, last synced to 223fb8c on 2026-10-06; `sdlc/scripts/{install,lint,test}`; the Beelink line in `AGENTS.md`; the M5 entry in the agents repo's `machines.json`.
- The M5 runs macOS. The routine suite reads `/proc/pressure/io`, and the 0561 export check uses `nm -D`. Some checks may be Linux-only.
- Ian's ruling of 2026-10-16: the M5 runs every project's development and release builds through `pm job`; Yellow takes only occasional Linux checks; the Beelink coordinates and builds nothing. ThinkThen keeps building on the Beelink, through `pm job run --here`, only until its Mac test failures are fixed. This replaces the Beelink release placement this ticket first kept.
- The 0.2 candidate finishes its final runs on the Beelink under 0543; work on this ticket starts now on main and changes nothing on `ticket/0543-final`.

- Trial at c4368db10, 2026-10-15: the M5 lacked cbindgen 0.29.4, cargo-nextest and Python 3.11+ (Apple's python3 is 3.9); these were installed in user folders. `install` then failed on the missing `jsonschema` Python module. `test`: 1,995 passed, 47 failed, 71 skipped. 40 failures cascade from that module; 4 invalid-UTF-8 filename fixtures fail because the macOS file system refuses those names; 1 test hard-codes a Linux config path; 1 assumes file-listing order; 1 private-name marker matches macOS temp paths. No product defect.

## Decisions

- PM decision, 2026-10-15, at Ian's request through pm: run checklist step 1 now as a trial, in parallel with the 0.2 candidate. Run it on the M5 with `pm job run --here 0589 SHA -- CMD` from the M5's thinkthen clone for `sdlc/scripts/install`, `lint` and `test` at one pushed SHA of `ticket/0543-final`. Change no file, add no `machines.json` entry and switch no landing until 0.2 publishes. Report each failure on macOS with a one-line plan.

## Scope

- Changes: `sdlc/pm.json` (the landing gate), `sdlc/scripts/build-admission*`, `sdlc/scripts/heavy-compile`, `sdlc/scripts/heavy-lock`, `AGENTS.md`, `CLAUDE.md`, `sdlc/scripts/test`, `sdlc/scripts/lint`, `sdlc/scripts/install`, `sdlc/planning/release-process.md`, and the agents repo's `machines.json`.
- Checklist:
  1. Sync the M5 clone and run `sdlc/scripts/install`, `lint` and `test` there through `pm job run` at one pushed SHA. List every check that fails or skips on macOS, with the reason.
  2. Make each Linux-only check skip on macOS with a plain message, or run it in a Linux container on the M5. Name where each such check runs instead: GitHub CI or an occasional Yellow run.
  3. Add thinkthen to the M5's `builds` in `machines.json`.
  4. Shrink build admission (`sdlc/scripts/build-admission.py`, its 624-line test, `heavy-compile` and `heavy-lock`) to the smallest form the M5 needs, or delete it if `pm job` already limits concurrent builds. It grew from a 34-line lock to about 880 lines in three days and deadlocked the rc.14 campaign.
  5. Rewrite the Beelink line in `AGENTS.md` and `CLAUDE.md`: routine builds and tests run on the M5 through `pm job run`; release qualification runs on the M5, with Linux checked by GitHub CI and occasional Yellow runs.
- Keeps: every check in the routine suite still runs somewhere before landing; each Linux-only check runs in GitHub CI or an occasional Yellow run, named in `sdlc/planning/release-process.md`; Mac-specific hosted checks stay on GitHub macOS runners.

## Proof

`pm job run 0589 SHA -- sdlc/scripts/test` and `-- sdlc/scripts/lint` pass on the M5 at the landing SHA, and `sdlc/pm.json` declares the landing gate (`sdlc/scripts/lint` then `sdlc/scripts/test`), so this ticket's own `pm ticket land --merge-branch` runs that gate on the M5, or reuses a passing `pm job run` of it at the same code.

## Defers

Nothing.
