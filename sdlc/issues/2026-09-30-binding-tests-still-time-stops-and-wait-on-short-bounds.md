Status: open. Found while building ticket 0352. Owner: none yet.

Kind: debt

Pay when: a binding's surface check fails once under load, or before 0.1.

Debt: 029

Severity: low

Keeping it lets a loaded machine fail a binding's surface check that proves nothing about the binding.

# Binding tests still time stops and wait on short bounds

## The problem

Ticket 0352 took wall-clock proofs out of the Rust test rung and the Python, DuckDB and SQLite surface checks. The other binding checks still hold the same kinds of proof. Some assert that a stop lands within 100 ms. Some wait two to five seconds for an event that a loaded machine can delay. The explorer for 0352 found them in these files, among others:

- Ruby: `libraries/ruby/tests/test_interrupt_single.rb`, `test_interrupt_batch.rb`, `test_flood.rb`, `test_tick_thread.rb`, `test_fork.rb`.
- TypeScript: `libraries/typescript/tests/abort.test.mjs`, `throttle.test.mjs`, `settings.test.mjs`, `settings_cases.test.mjs`.
- Go: `libraries/go/thinkthen_test.go`.
- R: `libraries/r/tests/interrupt.R` and `recognize.R`.
- JVM, Zig, Swift and C#: each one's test `backend.py` helper, which reads the clock.

## A fix

Apply 0352's rules to each binding. A routine check proves order: the cancel arrives while the loopback backend still holds the reply. A millisecond promise runs only under `THINKTHEN_TEST_PROFILE=stress`. A positive wait gets a guard of 30 to 60 s. The backend's `wait` line now waits up to 30 s, so a binding helper that assumed 5 s should say so.
