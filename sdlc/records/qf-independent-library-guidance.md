# Quick Fix: clarify library key and cache guidance

Status: built, awaiting final checks and review. Owner: Codex. Branch: `qf/independent-library-guidance`.

## Evidence

Local experiment 284, findings 79 and 100. The shared library error advised `EngineBuilder::api_key` even on Python and SQL surfaces. The bare `Engine::builder()` starts from library defaults and never reads the configuration file's `cache: false`; `Engine::from_env()` does read it. The work plan keeps these separate from ticket 0148.

## Retained behavior

The command keeps its own missing-key sentence and exit 4. Libraries keep `ErrorKind::Usage`. `Engine::builder()` keeps its explicit settings contract, and `Engine::from_env()` keeps reading the configuration file. No constructor or cache behavior changes.

## Change

The shared library error now says `no key is set; set THINKTHEN_API_KEY`, a remedy every surface can use. The root and Rust library READMEs state that a bare builder ignores the configuration file's cache switch and name `no_cache()` for a caller who wants no answer cache.

## Proof

`test_missing_key_names_a_remedy_python_can_use_before_any_send` calls the real Python API with no key and a non-loopback address. It pins the complete `UsageError` sentence and checks that a loopback listener received zero requests. Before the message edit it failed on `or call EngineBuilder::api_key`.

The test protects the public error kind and usable remedy. Restoring the Rust-only phrase fails it. Existing Python tests do not pin this missing-key sentence; existing command tests pin the separate exit-4 path. The test uses no test-only export, flag, or hook. Documentation of the deliberately distinct constructors needs no wording test.

The Rust size ratchet remains 72,630 of 72,630 nonblank lines. The production edit replaces one line with one line. The Python source and test ratchet rises from 2,378 to 2,388 for the ten nonblank lines of the real API regression. I checked `public/error.rs`, `test_inputs.py`, and the existing Python secrecy tests for a missing-key message check to reuse; none had one.

## Checks

Pending final targeted test and coordinator gate.

## Deferred

Ticket 0148 owns engine settings unification. This Quick Fix changes no cache setting behavior.
