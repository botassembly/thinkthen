# The library's timeout has no upper bound

Status: open. Filed 2026-09-30 from the system grading (`sdlc/planning/grading-2026-09-30/11-settings.md`, item 3), checked against main `f367545a0`. Owner: queue owner.
Kind: bug
Pay when: before 0.1, if a huge timeout panics. Otherwise close it with the pinning test.

The command refuses a `--timeout` above 86,400 seconds, because "the HTTP client adds it to the clock and a larger number can overflow there" (`crates/thinkthen/src/cli/mod.rs`, `run`). The Rust library's `timeout` setter refuses only zero (`crates/thinkthen/src/public/settings.rs`), and the engine JSON settings check only that `timeout` is a number (`core/settings.rs`, `"timeout" => number().is_some()`) before the C door hands it to the same setter (`libraries/c/src/settings.rs`). Zero is refused everywhere; only the upper bound is missing. The client receives the value through `timeout_global` (`engine/http.rs`). Whether a huge `Duration` panics there is untested and unconfirmed.

## Fix

Add a case that builds an engine with the largest `Duration` and makes one loopback call. If it panics, cap the library and JSON settings at 86,400 seconds with the command's sentence, on every surface that takes a timeout: the Rust builder, the engine JSON settings behind the C door, and the SQLite, R and Ruby settings paths (for example `databases/sqlite/src/settings.rs`). If it does not, keep the test as the proof.

## Done when

A test shows what the largest timeout does, and no surface can reach a panic through it.
