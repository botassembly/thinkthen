# The library's timeout has no upper bound

Status: Closed by the quick fix landed as `Land quick fix: pre-0.1 small fixes`. Filed 2026-09-30 from the system grading (`sdlc/planning/grading-2026-09-30/11-settings.md`, item 3), checked against main `f367545a0`. Owner: queue owner. Resolution: A loopback probe on Linux showed no panic, but a timeout of `u64::MAX / 2` seconds or more hung the call; 10^14 seconds still answered. So the Rust setter `timeout` in `crates/thinkthen/src/public/settings.rs` refuses a value past 86,400 seconds with the usage error `a timeout is at most 86400 seconds`. Every surface hands its timeout to that setter. `crates/thinkthen/tests/library/public_timeout.rs` pins the refusal for one nanosecond past the bound, `u64::MAX` seconds and `Duration::MAX`, and runs one loopback call with a 503 retry at 86,400 seconds. `specification/settings.md` and `specification/backends.md` state the bound and the sentence.
Kind: bug
Pay when: before 0.1, if a huge timeout panics. Otherwise close it with the pinning test.

The command refuses a `--timeout` above 86,400 seconds, because "the HTTP client adds it to the clock and a larger number can overflow there" (`crates/thinkthen/src/cli/mod.rs`, `run`). The Rust library's `timeout` setter refuses only zero (`crates/thinkthen/src/public/settings.rs`), and the engine JSON settings check only that `timeout` is a number (`core/settings.rs`, `"timeout" => number().is_some()`) before the C door hands it to the same setter (`libraries/c/src/settings.rs`). Zero is refused everywhere; only the upper bound is missing. The client receives the value through `timeout_global` (`engine/http.rs`). Whether a huge `Duration` panics there is untested and unconfirmed.

## Fix

Add a case that builds an engine with the largest `Duration` and makes one loopback call. If it panics, cap the library and JSON settings at 86,400 seconds with the command's sentence, in the Rust setter `timeout` in `crates/thinkthen/src/public/settings.rs`. Every surface that takes a timeout reaches that setter: the Rust builder, the engine JSON settings behind the C door, Python, TypeScript, Ruby, R, SQLite, DuckDB and PostgreSQL. One cap there covers them all. If it does not, keep the test as the proof.

## Done when

A test shows what the largest timeout does, and no surface can reach a panic through it.
