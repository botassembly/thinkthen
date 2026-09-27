# Quick Fix: clarify library key and cache guidance

Status: built, reviewed and verified; ready to land. Owner: Codex. Branch: `qf/independent-library-guidance`.

## Evidence

Local experiment 284, findings 79 and 100. The shared library error advised `EngineBuilder::api_key` even on Python and SQL surfaces. The bare `Engine::builder()` starts from library defaults and never reads the configuration file's `cache: false`; `Engine::from_env()` does read it. The work plan keeps these separate from ticket 0148.

## Retained behavior

The command keeps its own missing-key sentence and exit 4. Libraries keep `ErrorKind::Usage`. `Engine::builder()` keeps its explicit settings contract, and `Engine::from_env()` keeps reading the configuration file. No constructor or cache behavior changes.

## Change

The shared library error now says `no key is set; configure an API key for the engine (THINKTHEN_API_KEY)`. Environment-built engines take the named variable, and a bare Rust builder takes `api_key()`. The root and Rust library READMEs state that a bare builder ignores the configuration file's cache switch and name `no_cache()` for a caller who wants no answer cache.

## Proof

`test_missing_key_names_a_remedy_for_a_library_call` calls the real Python API with no key and a local address that the engine's exact loopback rule does not exempt. It pins the complete `UsageError` sentence. Before the message edit it failed on `or call EngineBuilder::api_key`. After rebuilding the Python extension it passed. The test does not measure sends; the existing backend and Python secrecy tests cover the no-key send rule at their own listener boundaries.

The test protects the public error kind and usable remedy. Restoring the Rust-only phrase fails it. Existing Python tests do not pin this missing-key sentence; existing command tests pin the separate exit-4 path. The test uses no test-only export, flag, or hook. Documentation of the deliberately distinct constructors needs no wording test.

The production edit replaces one line with one line. The Python source and test ratchet rises from 2,378 to 2,386 for eight nonblank lines of the real API regression. I checked `public/error.rs`, `test_inputs.py`, and the existing Python secrecy tests for a missing-key message check to reuse; none had one.

## Checks

Fresh read-only Codex code review accepted exact QF code commit `089f3738ab7ecd1ca871a8d96d2e2e91b86d2a21`. The merge of 0146 into QF commit `bf8d3ae81ae918070f354c668634a1c4e1224a08` had no overlap with QF files. With `THINKTHEN_API_KEY` unset, `maturin develop --locked --offline --features probe` rebuilt the Python extension under the shared heavy lock. `pytest -q tests/test_inputs.py` passed: 5 tests. Before the error edit, the new assertion failed on the Rust-only `EngineBuilder::api_key` advice; after the edit it passed.

The full five-rung gate ran on `bf8d3ae81ae918070f354c668634a1c4e1224a08` with `THINKTHEN_HEAVY_LOCK=/run/user/1000/thinkthen-heavy.lock`. `install`, `lint`, `test`, `spec`, and `surfaces` each exited 0. Lint took an outer `flock -o`; the other heavy rungs used their own lock. The test child alone had `RLIMIT_NOFILE=4096`. Logs and machine-readable results are under ignored `target/codex-logs/library-guidance-*`. The separate loopback fixture lifetime issue remains open; its 0202 design and runtime claim accompany this landing, but this gate does not prove that issue fixed.

After the five rungs, current main and the accepted 0202 design and runtime claim merged as metadata. They changed no source, binding, specification or demo file covered by the full gate. The merged Rust ratchet reads 73,890/73,890, Python Rust 4,726/4,726 and Python source and tests 2,386/2,386. `git diff --check` passed. No API key or paid call ran.

## Deferred

Ticket 0148 owns engine settings unification. This Quick Fix changes no cache setting behavior.
