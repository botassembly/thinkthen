# Ruby surface notes

The tag's notes stay on tag `surfaces-wave7-frozen-2026-09-24b` as history. These notes cover the port onto the public API (ticket 0112). The rulings live in ADR 0047's Ruby section.

## The crossing

- `ffi.rs` reads every input into owned Rust values on the Ruby thread, then `lib.rs` starts one worker thread per call. The worker blocks asynchronous signals and owns an engine clone, the inputs, and a clone of the call's own token.
- The worker hands its answer back through a mutex and a condition variable. Dropping the worker's end with no answer closes the handoff, and the Ruby thread raises `DefectError`.
- The Ruby thread waits in 50 ms slices through `rb_thread_call_without_gvl`. The unblock function only wakes the wait. After each slice it runs `rb_thread_check_ints` under `rb_protect`, then reads the handoff, the caller's token, and the call's own token.
- Any stop fires the call's own token and returns. The engine then sends no new request. Conversion back to Ruby runs under `rb_protect` too.
- `lib/thinkthen.rb` turns an `Interrupt` into `CancelledError`. Ruby sets the cause.

## The watchdog

- A row exists only while a thread with a tick has a call in flight. The row holds the tick, the call's own token, and the tick's error. Rust holds no Ruby object, so the collector cannot take a tick from under a call. `check.sh` fails when `src` names `Opaque`, `BoxValue`, `rb_gc_register`, or a `Value` field in a wrapped struct.
- A raising tick stores its error and fires the token. The call's `ensure` removes the row and raises the error.

## Engine settings

- `Engine.new` starts from `EngineBuilder::from_env()` and applies each given keyword. `cache: false` maps to `no_cache`.
- The binding refuses a list longer than `max_requests` before any send. The engine's lazy batch sends up to the limit first and refuses only then, as documented. `relate` is not capped by the limit.
- A cache folder belongs to the first backend address that wrote it. The tests pass `cache: false` for any engine on another arm.

## Deviations from the ticket

- Decision 9 names `Question::choose_labels` and `tag_labels`. The keyword builders write the question file's JSON and call `Question::from_json` for every verb. Both routes reach the same engine rules, and one route keeps keywords and files on one digest.
- The ticket's conformance case numbers come from the tag's file. Main's `conformance/cases.json` has 55 cases. The accent-and-emoji offsets case is `41-offsets-past-an-accent-and-an-emoji`, and the counters case is `40-decide-counters`.
- Five cases do not run, each with its reason printed: two `none: true` finds, the two-group annotate that reads parts of a record, the injected defect, and the question-file loader.
- `test_errors.rb` keeps only the set boundaries. The conformance runner's error paths hold every other fault kind.
- The pinned-Ruby guard and the deny run with its `file://` plant live in `check.sh`. `lint` reaches Ruby only through `surfaces --registry`, and this port changes no ladder script.

## 2026-09-28: named single-question files

The older five-skip bullet above records the port's historical source. Current shared conformance retained two skips before this change; selected case 30 now passes through `ThinkThen.question(file:)`, leaving only the internal-injection case 25. The native edge reads at most 1 MiB plus one byte, returns the existing `QuestionValue`, and maps file failures to non-retryable Local. The case-01 file call checks the independently captured digest and one send; missing, blank, oversized and invalid-UTF-8 files raise before any send. Keyword questions and set files retain their separate meanings.
