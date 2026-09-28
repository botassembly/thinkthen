# Quick Fix: pinned DuckDB setup before Linux installed release smoke

Status: candidate for fresh independent review. Source base `9f54e11a3a3922c095c6082c964d411491e9f408`. The exact changed paths are `sdlc/scripts/release-workflow`, its existing `sdlc/scripts/workflows` checker, this record and the 0128 ticket lessons. This fixes only native Linux release `host-setup` orchestration; it does not establish an actual four-runner rehearsal.

## Failure and correction

In `.github/workflows/release.yml`, Linux `build` jobs use pinned containers for their library packages, while `smoke` runs on fresh native `ubuntu-24.04` x86-64 and ARM64 runners. Both jobs call `sdlc/scripts/release-workflow host-setup`, with `RELEASE_SMOKE=1` only for smoke. The script formerly called `databases/duckdb/tools/setup.sh --fetch` only on Darwin. The native Linux `release-smoke` calls `databases/duckdb/check.sh` against the installed archive; that check needs the pinned stock CLI, Python host, platform marker and genuine older host. Without setup, it returns 77 unless runner state happens to provide them, and the unchanged release verdict treats 77 as failure.

The correction invokes the existing pinned DuckDB setup in the Linux branch only when `RELEASE_SMOKE=1`, after Ruby setup. Linux container build preparation still avoids that download. Darwin continues its prior setup call for both build and smoke. The release graph, draft/publication guards, `release-smoke`, its zero-not-run verdict, and the DuckDB setup pins were not edited. The ARM64 `setup.sh` implementation belongs to ticket 0231 and its candidate `a3ce39bc` is under separate High review; this Quick Fix guarantees its invocation after that candidate lands, not current ARM64 host support.

## Red and green

`sdlc/scripts/workflows --self-test` now invokes the real `host-setup` branch through a temporary fake-command `PATH`. It records the called local setup scripts without running rustup, Cargo fetch, pip, Ruby setup, DuckDB fetch or any global installation. Its four bounded combinations cover x86-64 and ARM64 Linux build versus installed smoke. Before the correction, both smoke cases failed: the checker observed only `libraries/ruby/setup-ruby.sh` where it required that call followed by `databases/duckdb/tools/setup.sh --fetch`; result **19/21**, exit 1. After the correction, all four invocation cases and the existing workflow checks passed **21/21**, exit 0. Build cases prove no extra DuckDB fetch; smoke cases prove one fetch on each native target.

Focused `sdlc/scripts/workflows` passed the real dispatch-only workflow, `sh -n sdlc/scripts/release-workflow`, Python compile and `git diff --check` passed. A separate read-only call to the actual DuckDB setup script with `--target` returned `x86_64-unknown-linux-gnu` on this host; with a temporary `uname` reporting unsupported `Linux:ppc64le`, it exited 77 with `setup: no pinned DuckDB C++ inputs for Linux:ppc64le`. No global `host-setup`, `setup.sh --fetch`, GitHub dispatch, provider call, publication, full matrix or stress run occurred. Existing native package records establish real pinned DuckDB inputs on Linux x86-64 and Apple Silicon; they do not prove this future workflow job ran.

## What the build taught us

The existing workflow checker covered dispatch, pins and outward guards but not the host-setup operation behind a run step. A small invocation witness makes the release-smoke prerequisite observable without fetching on the author machine. The split between Linux container builds and native smoke matters: placing setup on every Linux host-setup would add downloads to stages that do not use the stock host tools. ARM64 orchestration can be proved with a fake host target now, while the separate 0231 source and real ARM64 runner must still prove its pinned setup and installed package.

## Review

Fresh independent review pending. It should check the Linux build/smoke placement, unchanged Darwin setup, unsupported-target and zero-not-run behavior, and the limits of the fake invocation witness.
