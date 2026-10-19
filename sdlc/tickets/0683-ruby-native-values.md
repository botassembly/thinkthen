# 0683: Return finished Ruby values from Rust and use the run call

Status: OPEN.

Milestone: 0.2.1

Phase: 3

Depends on: 0629, 0688, 0689, 0727, 0728

## Outcome

- Changes: `libraries/ruby/src/`, `libraries/ruby/lib/thinkthen/session.rb`, `libraries/ruby/lib/thinkthen/results_generated.rb`, `libraries/ruby/sig/thinkthen/`, `libraries/ruby/Cargo.toml`, `libraries/ruby/Cargo.lock`, `libraries/ruby/tests/test_owned_session.rb`, `build-tools/results/templates/ruby.py`, `build-tools/results/templates/ruby_rbs.py`, `build-tools/results/registry/ruby.py`, `build-tools/results/legacy/ruby.txt`, `conformance/library-surface/ruby.json`.
- The extension calls `Engine::call` (the Rust run function from 0629) and builds Ruby objects by mapping the 0689 value tree. No JSON crosses from Rust into Ruby, and `serde_json` leaves the extension's dependencies.
- `Results::Completed#value` and `#probability` in `session.rb` read the answer and probability from the tree. Their projection over `CALL_RESULTS` is deleted.
- The session loop in `Client::Operation` (`result`, `feed`, `close_producer`) is deleted, with its nil-producer check and its private closed, reader and producer error text. The call runs without the global VM lock and cancels through the run function's handle.
- The generated Ruby result files keep only declarations, or are deleted where the tree makes them unused. The Ruby template and registry entries change to match, and `conformance/library-surface.json` names the Ruby declarations that remain.
- `libraries/ruby/lib/thinkthen/session.rb`, `build-tools/results/templates/ruby.py` and `build-tools/results/templates/ruby_rbs.py` leave this language's exception file `build-tools/results/legacy/ruby.txt`.
- The public Ruby API keeps its method names and result readers. The phase 3 dumper for Ruby is 0712.

### Proof

- On the M5: `pm job run --machine m5 0683 SHA --memory 8 -- sh -c 'cargo build --locked --offline -p conformance-backend && sh libraries/ruby/check.sh'`. The routine check builds and installs the gem on Ruby 4.0 and runs `tests/test_owned_session.rb`, `tests/test_examples.rb` and the shared routine cases in `tests/native_cases.py` through the named calls. A held-cancel case returns `cancelled` promptly.
- `python3 build-tools/results/answer_rules.py` passes with the three Ruby entries gone from `LEGACY_FILES`. On main, removing those entries alone fails on `session.rb`.
- `python3 conformance/library-surface.py --library ruby` passes.
- Ruby 3.4 runs at the release suite (`THINKTHEN_TEST_PROFILE=full`), not in this proof.

## Evidence

- After 0727 lands, this language's answer-rule exceptions live in `build-tools/results/legacy/ruby.txt` and its surface entry in `conformance/library-surface/ruby.json`. This ticket edits only those two files, so it does not collide with other languages' call tickets.
- **JSON between Rust and Ruby.** Ruby binds directly to the crate through magnus 0.8.0 (`libraries/ruby/Cargo.toml`), yet results cross as JSON text: `src/ffi/native_result.rs` parses packets with `serde_json` in `packet` and converts them by walking the generated schema graph in `convert` and `field`.
- **Answers shaped in Ruby.** `Results::Completed#value`, `#probability` and `item_probabilities` in `lib/thinkthen/session.rb` read `value_projection`, `probability_mode`, `resolution` and `pick_path` from `CALL_RESULTS` in `lib/thinkthen/results_generated.rb` and pair spans in Ruby. 0612 now puts each call's answer, probability, pick and resolution in the final packet.
- **Session loop in Ruby.** `Client::Operation` drives `RequestSession` through `wait`, `poll`, `push` and `finish` (`src/ffi/request.rs`, `src/ffi/session_wait/ffi.rs`). 0629's `Engine::call` owns a named call from start to settled result, with the producer advanced on the caller.
- **Generated glue.** `src/ffi/results_generated.rs` (about 1,100 lines), `lib/thinkthen/results_generated.rb` (about 1,040) and `sig/thinkthen/results_generated.rbs` come from `build-tools/results/templates/ruby.py` and `ruby_rbs.py` through the `ruby` groups in `build-tools/results/registry.py`. `check.sh` runs `generate.py --target ruby --check`.
- **Answer-rule exceptions.** `build-tools/results/answer_rules.py` lists `session.rb`, `templates/ruby.py` and `templates/ruby_rbs.py` in `LEGACY_FILES`; each phase 3 call ticket removes its own entries. Other language call tickets edit the same set, so these tickets touch `answer_rules.py` and `registry.py` one at a time.
- `crates/thinkthen` is unchanged, so `specification/rust-api-0.2.md` does not change.
