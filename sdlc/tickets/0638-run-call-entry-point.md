# 0638: Add one blocking run-call C function with one result representation

Status: OPEN.

Milestone: 0.2.1

Phase: 2

Depends on: 0612, 0629, 0635

## Outcome

- One exported C function runs a named call to completion. It takes the engine, the function name, the question, input on the path 0635 chose, options, an optional producer callback for fed input, and a cancellation handle. It returns a status, and through out-parameters an owned result handle or an owned error handle. The handles have accessors and a free function. It wraps the Rust function from 0629 and adds no rules of its own.
- The run-call result handle has one representation, either generated views or owned JSON. The choice and its reasons, with the cost of each option, are recorded in the repo before the core freeze (0688). The C library's own examples and tests use only the chosen one. The other result path and its generated outputs stay until the languages that use them move in phase 3; 0690 deletes them.
- The new function and its handles report failure through the returned error handle. The per-thread error slots and the `thinkthen_error_*(NULL)` and `thinkthen_session_error_*` accessors stay only until the languages move; 0690 deletes them.
- Async hosts run the call on one worker thread or through a completion callback, and cancel through the handle. No host calls wait, push or read directly, so 0690 deletes the low-level session functions once every language has moved.
- The run call invokes the producer callback only on the thread that called it. This is a risk until prototyped: Dart's synchronous callbacks work only on their own isolate's thread, PHP's FFI callbacks are unsafe on threads it does not own, and the JVM needs threads attached.
- The panic guard (`libraries/c/src/failures.rs:318`) covers the new function and its callback re-entry.

### Proof

- On the M5 through pm job, an installed C consumer built against the released header and library calls the new function for all ten functions against the loopback backend: plain and fed input, a failing producer callback, cancellation from another thread while a reply is held, and a deadline.
- Error kind, message, retryable flag and facts read from the error handle match `conformance/cases.json`. The C consumer reads errors on a thread other than the one that failed.
- A small Dart prototype and a small PHP prototype, each feeding input through the producer callback, run one fed call through the new function and show the callback on the calling thread. They are throwaway proofs, not package code.
- The result-representation decision record names the chosen form and the reasons.

## Evidence

The C interface (`libraries/c/include/thinkthen.h`, 46 functions) offers only low-level session steps: `thinkthen_session_new*`, `push`, `try_push`, `wait`, `next_packet`, `try_read` and `finish`. It has no function that runs a whole call, so every C-interface binding writes its own session loop (see 0629).

Errors are kept per thread. `libraries/c/src/failures.rs:198-207` holds them in a `thread_local!`, and callers read them back with `thinkthen_session_error_message()` and similar functions. Hosts with green threads or async runtimes must pin an OS thread to read the right error. Go calls `runtime.LockOSThread()` around native calls (`libraries/go/session.go:132`, `:198` and `:297`). The binding guide makes each worker "copy any immediate failure on the same operating-system thread" (`libraries/BINDING-AUTHOR.md`).

The result side is also split. C emits both fixed layouts (`views_generated.rs` at 10,790 lines, most of the header's 422 structs, plus the generated Zig, Ada and COBOL layouts) and JSON packets, so every new result field changes the interface for several languages.
