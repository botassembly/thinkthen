# 0660: Build Python wheels per platform and simplify the Python gate

Status: OPEN.

Milestone: 0.2.1

Phase: 3

Depends on: 0615, 0618, 0619, 0688

## Outcome

- The release builds abi3 wheels for each supported platform from one script. The gate installs the wheel for the host.
- The gate has one install path and one test path. Coverage staging is deleted, since coverage does not gate. Parity staging moves into the shared harness. Inline Python blocks move into tests.
- Python conformance goes through a packet dumper over the public API following the 0642 contract. tests/native_case.py, tests/native_frame_case.py and the Python fixture drivers are replaced by it.
- Native-layer tests such as `test_native_results.py` move onto the installed public API or are deleted.

### Proof

- On the M5 through pm job, natively and in the pinned arm64 container, the host's wheel installs into a fresh environment and passes the tests and the dump comparison.
- The other platforms' wheels are proven on the candidate in 0687.

## Evidence

- libraries/python/check.sh is 111 lines and switches on six environment variables: THINKTHEN_PARITY_PREPARE, THINKTHEN_PARITY_PREPARED, THINKTHEN_COVERAGE_STAGE, THINKTHEN_COVERAGE_PHASE, THINKTHEN_FULL_COMPILER and THINKTHEN_ARTIFACT.
- It exits 77 without uv or maturin (:16-17).
- It builds the routine selection list in an inline Python block (:85-93).
- Windows wheels have their own script, check-windows.py. Windows fixes keep recurring: 46c9ac453, 16ea372f0, cd869e34c.
- pyproject.toml claims macOS, Linux and Windows (:15-19), but routine gates prove only the build host.
