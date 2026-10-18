# 0683: Return finished Ruby values from Rust and use the run call

Status: OPEN.

Milestone: 0.2.1

Phase: 3

Depends on: 0629, 0688, 0689

## Outcome

- The extension calls the Rust run function from 0629 and builds Ruby objects by mapping the 0689 value tree. No JSON crosses from Rust into Ruby.
- The projection in `session.rb:21-64` and the session loop in `session.rb:100-296`, with its nil-producer check and its private closed, reader and producer error text, are deleted. The call runs without the global VM lock and cancels through the handle.
- Its files leave the 0637 answer-rule check's list of exceptions.

### Proof

On the M5 through pm job, the installed-gem tests (`tests/test_package.rb`, after `gem install --local`) run all ten named calls against a recorded backend. Their packet dump matches the 0642 oracle.

## Evidence

- **JSON between Rust and Ruby.** Ruby binds directly to the crate through magnus (`libraries/ruby/Cargo.toml:20-22`), yet results still cross into Ruby as JSON (`Cargo.toml:23`).
- **Answers shaped in Ruby.** `lib/thinkthen/session.rb:21-64` works out values and probabilities and pairs spans in Ruby, duplicating PHP and Swift.
- **Hand-written glue.** About 950 lines: `session.rb`, `src/ffi/request.rs`, `src/native_result.rs`, `src/ffi.rs`.
