# 0660: Build Python wheels per platform and simplify the Python gate

Status: OPEN.

Milestone: 0.2.1

Phase: 3

Depends on: 0615, 0618, 0619, 0688

## Outcome

- Changes: `libraries/python/check.sh`, `libraries/python/check-windows.py`, `libraries/python/build-wheel.sh`, `libraries/python/README.md`, `libraries/python/tests/`, `.github/workflows/windows.yml`, `conformance/parity.py`, `sdlc/scripts/coverage`, `sdlc/scripts/coverage_installed_initial.py`. It changes no Rust, no Python package source and no generator output.
- Every platform's wheel comes from `build-wheel.sh`, which `release-pack python` already runs per target. `check-windows.py` is deleted, and the Windows workflow installs the target's wheel and runs the same installed check as the other platforms.
- The gate has one install path and one test path: it builds or takes one wheel, installs it into a fresh environment, and runs the tests and the dump comparison. Coverage staging (`THINKTHEN_COVERAGE_STAGE`, `THINKTHEN_COVERAGE_PHASE`) is deleted from `check.sh`, and the Python collections leave `coverage_installed_initial.py` and the consumer-name map in `sdlc/scripts/coverage`, since coverage does not gate. Parity staging (`THINKTHEN_PARITY_PREPARE`, `THINKTHEN_PARITY_PREPARED`) moves into `conformance/parity.py`. The inline Python blocks in `check.sh` move into tests.
- Python conformance goes through a packet dumper over the public API that follows the 0642 contract, for the plain, pandas and Polars consumers. `tests/native_case.py` and `tests/native_frame_case.py` are replaced by it.
- Native-layer tests such as `tests/test_native_results.py` move onto the installed public API or are deleted.
- The package release number in `pyproject.toml` stays; it is not a format version.

### Proof

- On the M5 natively: `pm job run --machine m5 0660 SHA --memory 8 -- sh libraries/python/check.sh` builds the host wheel, installs it into a fresh environment, passes the tests, and the dump matches the 0642 oracle for every shared case. Before the change no Python dump exists, and the check still runs `tests/native_case.py` through `conformance/children/native_fixture.py`.
- `git grep -n 'THINKTHEN_COVERAGE_\|THINKTHEN_PARITY_PREPARE' libraries/python` finds nothing; before the change it finds the branches in `check.sh`.
- The pinned Linux arm64 and x86_64 wheels run at the 0619 container checkpoint. The Windows and Intel macOS wheels are proven on the candidate in 0687.

## Evidence

- `libraries/python/check.sh` switches on six environment variables: `THINKTHEN_PARITY_PREPARE`, `THINKTHEN_PARITY_PREPARED`, `THINKTHEN_COVERAGE_STAGE`, `THINKTHEN_COVERAGE_PHASE`, `THINKTHEN_FULL_COMPILER` and `THINKTHEN_ARTIFACT`. It builds the routine selection list and runs the native fixture consumers in inline Python blocks.
- `sdlc/scripts/coverage_installed_initial.py` drives the Python coverage stage through those variables, and `sdlc/scripts/coverage` maps Python to four consumer names. `conformance/parity.py` sets the parity variables.
- The release already builds one wheel per target: `.github/workflows/release.yml` runs `release-pack TARGET ... python` on each of five targets, `part_python` in `sdlc/scripts/release-pack` calls `build-wheel.sh`, and the `wheels` job checks five wheels. Only Windows differs: `.github/workflows/windows.yml` runs its own `check-windows.py`. Windows fixes keep recurring: 46c9ac453, 16ea372f0, cd869e34c.
- `pyproject.toml` claims macOS, Linux and Windows, but routine gates prove only the build host.
- Python's dumper replaces its fixture consumer here; 0662 then proves its run-call switch against the same dump.
