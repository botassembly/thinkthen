Status: Closed by the quick fix landed as `Land quick fix: release builds clear host setup, apt, offline crates, and library tests`. Found by the second release rehearsal, run 36780048676, on 2026-09-30. Owner: ticket 0128 Phase 3b. Resolution: `ensure_packages` in `sdlc/scripts/release-language-tools.py` no longer requires the plan to start with `Reading package lists`. `selected_plan` reads only `Inst` lines and skips the rest. It still refuses a plan with no `Inst` line, a malformed or duplicate `Inst` line, or a removal. The good acquisition fixture in `sdlc/scripts/release-language-tools-self-test.py` now starts with apt's four-line simulation note, and it failed on the old check.

Kind: bug

Pay when: before the next rehearsal dispatch.

Keeping it stops the x86-64 Linux `build` job before its container build, so no x86-64 Linux file or managed package is built.

# The managed language tools refuse apt's simulation note

## The problem

The x86-64 Linux `build` job runs `python3 sdlc/scripts/release-language-tools.py managed ... --acquire`. It failed with:

```
release-language-tools: apt did not produce a package acquisition plan
```

`ensure_packages` in `sdlc/scripts/release-language-tools.py`, lines 183 to 185, runs `apt-get -s install` as the runner user and requires the output to start with `Reading package lists`. Run without root, `apt-get -s` first prints `NOTE: This is only a simulation!` and three more note lines. The check refuses that real output. The job log shows only the refusal, not apt's output. Running `apt-get -s install` without root on Ubuntu reproduces the note on standard output ahead of `Reading package lists`. The self-test fixtures in `sdlc/scripts/release-language-tools-self-test.py` all start with `Reading package lists...`, so they missed it.

## A fix

Drop the `startswith` check. `selected_plan` already refuses a plan with no `Inst` line and refuses a removal. Add the four-line note to one fixture plan, so the self-test fails on the old check.
