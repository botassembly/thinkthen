---
opens: sdlc/issues/2026-09-28-r-worker-panic-can-copy-payload-text.md sdlc/issues/2026-09-28-binding-panic-hooks-can-print-caught-payloads.md libraries/python/src/lib.rs libraries/python/src/worker.rs libraries/ruby/src/lib.rs libraries/typescript/src/door.rs libraries/r/thinkthen/src/rust/src/calls.rs
---

# 0227: Keep caught language-binding panic payloads out of diagnostics

Status: proposed design for independent High review. Owner: Codex. No runtime change or issue closure. Current-main source pin: `dad31fd8`. The private native pattern in ADR 0098 is accepted as design, but its 0226 code candidate `d0709b69` is still under fresh High review. Refresh this ticket if that review changes the pattern.

## Outcome and retained behavior

A panic caught by the Python, Ruby, TypeScript or R binding must return its existing fixed non-retryable Defect without writing an owned synthetic payload to stderr, stdout or a host error. R also stops copying string payloads into its packed error. Each binding must work on a later call and delegate unrelated host-thread panic diagnostics. Preserve all six kinds, ordinary Python exceptions, Ruby raises and traps, Node promise and abort behavior, R conditions and interrupt handling, caller/worker ownership, detached worker shutdown, native API and package loaders. This adds no public name, fault switch, dependency or host-visible lifetime rule.

## Proposed private boundaries

Use a once-installed delegating Rust hook and a thread-local binding depth only while that binding owns Rust work. An RAII guard restores depth through nested return or unwind. At the existing catch, forget every opaque caught payload **before leaving the marked scope**; its destructor may panic. This intentionally retains one payload allocation per caught panic, potentially including private bytes until process exit. It does not erase the payload. Keep one catch site in each binding where existing source checks demand it. Fixed conversion happens inside the binding scope; no per-call global-hook swapping. A later host `set_hook` or concurrent installer can bypass a wrapper, as ADR 0081 already limits.

- **Python:** `src/lib.rs::caught` serves both the GIL-held `guard` and `src/worker.rs`'s detached, Python-free worker. Replace `.ok()` with explicit opaque-payload disposal and mark both actual threads. The current `resume_unwind` test proves conversion but skips the hook. The GIL-held guard calls user Python methods through `src/input.rs` and `src/arrow/ffi.rs` (`hasattr`, `__arrow_c_array__`, `__arrow_c_stream__`, iteration and extraction). The worker also invokes a foreign Arrow producer's C callbacks (`get_schema`, `get_next`, `get_last_error`, and release functions) through `src/arrow/ffi.rs`, sometimes during `gate::release` at exit. Temporarily leave the marker around **both** interpreter and producer callouts, then restore it, so another extension keeps its own diagnostic. Preserve ordinary `PyErr`, producer release ordering, and `py.check_signals()` on the waiting caller. The complete callback inventory is required before implementation; not all work inside a Python guard or worker is binding-owned. Keep the existing Arrow exit/shutdown gate.
- **Ruby:** `src/lib.rs::guarded` catches the worker call and `Settings::build` on the Ruby thread. Its worker owns plain Rust inputs and calls no Ruby API; `src/ffi.rs::cross` releases the VM lock for waits, resumes Ruby interrupt checks under `rb_sys::protect`, and remains outside the marker. Put `quiet_signals` and guarded worker work in the binding-owned region without changing signal-mask or handoff behavior. A Ruby `with_tick` block runs in the Ruby host layer and remains outside this Rust marker. Do not change `rb_thread_call_without_gvl`, `rb_protect` or native classes.
- **TypeScript:** `src/door.rs::caught` serves engine construction, usage and the per-call answer. Mark the actual call worker, and keep Node's `ThreadsafeFunction` delivery and JavaScript callback outside the scope. Forget the caught payload before forming the fixed Node defect envelope. The public `napi` entry points and abort handle do not change.
- **R:** `thinkthen/src/rust/src/calls.rs::on_worker` is the one catch. Replace `words(&*panic)` and its payload-bearing `the call panicked: boom` with the fixed `the call panicked`, forget the opaque payload inside the worker scope, and retain the later call. R's `Pending` interrupt callback and `R_ToplevelExec` run only on the caller's R main thread, outside the worker marker; do not move or wrap them. Keep the packed kind/retry/escaping contract and detached stop guard.

These private scopes follow [ADR 0081](../planning/adr/0081-duckdb-cpp-api.md) and the accepted ADR 0098 design on the 0226 branch. They do not close the separate 0226 C, SQLite and old DuckDB target-package remainders. The [preflight](../records/0227-panic-preflight.md) names the exact current sources, conflicting holds and smallest proofs.

## Focused proof and dependencies

One selected child per binding installs a prior hook, triggers a string marker and a different panic-on-Drop marker at its real caught boundary, checks fixed non-retryable Defect and both captured streams, makes a later successful call, and observes one unrelated host thread through the prior hook. Python additionally exercises marker suspension through a reentrant host callback. Reuse existing child helpers; no exposed fault flag or duplicate low-level matrices. R's existing `boom` expectation changes to the exact fixed sentence. Source tests prove the hook and disposal; an installed wheel, pinned Ruby gem, Node package and R tarball must separately prove actual host loading and later use, with linkage/lifetime checked per shipped target. Unavailable packages remain open.

Ticket 0212 currently holds Python `engine.rs`/`frame.rs`, TypeScript `door.rs`, Ruby `call.rs` and R `calls.rs` for the native Call/Facts migration. The 0227 builder may start in free Python `lib.rs`/`worker.rs` and Ruby `lib.rs` only after exact claims. Transfer shared files from 0212 after its frozen source or landing; merge its current API instead of coding around it. Python callback suspension needs `input.rs` and `arrow/ffi.rs` and may need `engine.rs`/`frame.rs`; claim them explicitly after a complete callout inventory. `arrow/ffi.rs` is already over the usual 500-line cap, so prefer a coherent private child extraction rather than growing it. No product work starts from this design draft alone.

## Evidence

- Starts from: the two open language-binding panic issues, main `dad31fd8`, accepted ADR 0081 and ADR 0098 design on the reviewed 0226 branch.
- Keeps: six error kinds, fixed non-retryable Defect, ordinary interpreter exceptions and callbacks, cancellation, later calls, worker ownership and package loading.
- Changes: proposes per-binding private diagnostic scopes and opaque payload disposal; R stops copying panic text into its returned error; Python explicitly suspends its marker across host Python callouts.
- Proof: one selected child per binding with string and panic-on-Drop markers, exact defect, captured streams, later success and unrelated prior-hook delivery; Python reentry and actual installed packages have separate checks.
- Defers: runtime claims until 0212 transfers shared paths, fresh High design and code review, actual target packages and any host-hook replacement beyond the accepted cooperative limit.

## What the build taught us

Design preparation only. The 0226 source candidate showed that a fixed returned error and a successful `resume_unwind` test do not establish panic-hook secrecy; payload disposal also matters. Here the new risk is Python reentry while a broad caller guard is marked. Review that boundary before copying a native worker pattern into an interpreter thread.
