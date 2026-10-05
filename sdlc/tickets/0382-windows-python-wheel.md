# 0382: Windows stage 1: the Python wheel ships for Windows x86-64

Status: in progress. The Windows Python wheel and core runner candidate passes focused Linux Python, installed-wheel and workflow checks. Fresh High review, landing and the combined native Windows run remain. Native dispatch is pending approval.

Milestone: 0.2

## Outcome

- The release workflow builds a `win_amd64` wheel. The wheel count rises to five in both places it lives: `verify-family`'s wheel count and the PyPI job's own count.
- `pip install thinkthen` on Windows installs the wheel, and it imports and answers from a loopback backend.
- The Python tests that need `fork`, `SIGINT` or `resource` skip on Windows with a reason. The rest pass on `windows-2025`.
- `libraries/python/README.md` names the Windows wheel. Linux and macOS wheels are unchanged.

## Evidence

- Starts from: ticket 0373 (stage 0) did not build the wheel. `libraries/python/build-wheel.sh` runs `maturin` and keeps only names that end in `.so`, so it misses a Windows `.pyd`. The stage 1 report in `sdlc/planning/windows.md` sizes this ticket at 150 to 350 lines and 1 to 2 slices, with low to medium Linux and macOS risk: the wheel script and the wheel count are shared. No experiment preceded this ticket.
- Keeps: every Linux and macOS wheel's name, tags and contents. The package's public API. Each Unix-only test keeps running on Unix.
- Changes: `build-wheel.sh` takes the Windows extension file. The package classifiers name Windows. The Unix-only tests get skips with reasons. `release.yml` builds the wheel on the Windows target, and both wheel counts in `release.yml` rise to five.
- Proof: a wheel install and smoke on the runner that counts loopback requests. The Python check passes on `windows-2025` with only the named skips. The Linux and macOS Python checks stay green.
- Defers: the main unknown, whether `cibuildwheel` or the present script builds the Windows wheel more simply. The builder picks one and records why. The Polars door follows the wheel in stage 2.

## Build decisions

Keep maturin and the existing shell build route. It supports the abi3 `win_amd64` wheel once the content check admits `.pyd`. Windows needs `VirtualQuery` in the existing raw-memory leaf because the prior non-Linux path links Unix `mincore`; inspect committed readable regions before Arrow reads them. Use the existing pinned windows-sys 0.61.2 with only its memory feature. Keep Unix memory inspection unchanged. Port protected-page fixtures and token/release checks; skip only Unix fork, signal, resource and /proc fixtures with reasons. Run the release wheel's installed import, counted loopback answer and no-send refusals, then the full applicable probe suite and pandas 2 safety checks. Root owns the one High review, full landing gates and the combined hosted/native approval request.
