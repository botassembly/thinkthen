Status: open. Found by the second release rehearsal, run 36780048676, on 2026-09-30. Owner: ticket 0128 Phase 3b.

Kind: bug

Pay when: before the next rehearsal dispatch.

Keeping it stops the x86-64 Linux `build` job before its container build, so no x86-64 Linux file or managed package is built.

# The managed language tools refuse apt's simulation note

## The problem

The x86-64 Linux `build` job runs `python3 sdlc/scripts/release-language-tools.py managed ... --acquire`. It failed with:

```
release-language-tools: apt did not produce a package acquisition plan
```

`ensure_packages` in `sdlc/scripts/release-language-tools.py`, lines 183 to 185, runs `apt-get -s install` as the runner user and requires the output to start with `Reading package lists`. Run without root, `apt-get -s` first prints `NOTE: This is only a simulation!` and three more note lines. The check refuses that real output. `command()` joins standard output and standard error, so the note leads the text. The self-test fixtures in `sdlc/scripts/release-language-tools-self-test.py` all start with `Reading package lists...`, so they missed it.

## A fix

Drop the `startswith` check. `selected_plan` already refuses a plan with no `Inst` line and refuses a removal. Add the four-line note to one fixture plan, so the self-test fails on the old check.
