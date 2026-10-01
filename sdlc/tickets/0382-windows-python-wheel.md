# 0382: Windows stage 1: the Python wheel ships for Windows x86-64

Status: ready. It waits for the `release/0.1` cut and for ticket 0380 slice A, which adds the fifth release target. No slice lands on main before the cut (ADR 0116 item 2). Plan: `sdlc/planning/windows.md`, stage 1. Ian ruled on 2026-10-01 to keep all Windows work in 0.2. The coordinator assigns a lane after the cut.

Milestone: 0.2

## Outcome

- The release workflow builds a `win_amd64` wheel. The PyPI job publishes five wheels.
- `pip install thinkthen` on Windows installs the wheel, and it imports and answers from a loopback backend.
- The Python tests that need `fork`, `SIGINT` or `resource` skip on Windows with a reason. The rest pass on `windows-2025`.
- `libraries/python/README.md` names the Windows wheel. Linux and macOS wheels are unchanged.

## Evidence

- Starts from: ticket 0373 (stage 0) did not build the wheel. `libraries/python/build-wheel.sh` runs `maturin` and keeps only names that end in `.so`, so it misses a Windows `.pyd`. The stage 1 report in `sdlc/planning/windows.md` sizes this ticket at 150 to 350 lines and 1 to 2 slices, with low to medium Linux and macOS risk: the wheel script and the wheel count are shared. No experiment preceded this ticket.
- Keeps: every Linux and macOS wheel's name, tags and contents. The package's public API. Each Unix-only test keeps running on Unix.
- Changes: `build-wheel.sh` takes the Windows extension file. The package classifiers name Windows. The Unix-only tests get skips with reasons. `release.yml` builds the wheel on the Windows target, and the PyPI job's count rises to five.
- Proof: a wheel install and smoke on the runner that counts loopback requests. The Python check passes on `windows-2025` with only the named skips. The Linux and macOS Python checks stay green.
- Defers: the main unknown, whether `cibuildwheel` or the present script builds the Windows wheel more simply. The builder picks one and records why. The Polars door follows the wheel in stage 2.
