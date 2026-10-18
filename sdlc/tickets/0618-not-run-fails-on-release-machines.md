# 0618: Count "not run" (exit 77) as failure everywhere

Status: OPEN.

Milestone: 0.2.1

Phase: 1

Depends on: 0615

## Outcome

- Every entrypoint treats exit 77 from a required check as a failure, whatever arguments it was given and on whatever machine it runs. There is no developer-machine exception. The `release` mode argument and the `campaign` switch are deleted.
- One shared helper reads the 0615 manifest. `verdict.sh`, `smoke`, `surfaces` and `coverage` call it, and their own 77 branches are deleted.
- A library gate may exit 77 only on a platform the manifest marks unsupported for that library, and it prints the manifest entry that allowed it. Today Go, Ada and COBOL skip anything but Linux x86-64 (`libraries/go/check.sh:12-14`, `libraries/ada/check.sh:5`, `libraries/cobol/check.sh:5`), and missing zig, uv, maturin or cmake skip silently (`libraries/zig/check.sh:11-13`, `libraries/python/check.sh:16-17`, `libraries/cpp/check.sh:13-17`). Every such silent skip becomes a manifest entry or a failure.
- An unknown test profile or a missing optional input exits nonzero, not 77.

### Proof

- On the M5 through pm job, `sdlc/scripts/smoke` with one binding's toolchain hidden fails and names that binding as not run.
- On the M5 through pm job, the Go gate, whose manifest entry marks macOS unsupported today, exits 77 and prints that entry, and the run that calls it still passes.

## Evidence

A check that lacks its toolchain exits 77, and whether that counts as passing depends on an argument or variable the caller has to remember to pass. `sdlc/scripts/verdict.sh:5` passes a 77 unless its first argument is `release`. `sdlc/scripts/smoke:78` returns success for a 77 unless `$campaign` is set. `sdlc/scripts/surfaces:311` and `sdlc/scripts/coverage:109` each handle it in their own way. A qualification run started without the flag can therefore report a pass when a language was never checked.

## Progress

- 2026-10-18 started
