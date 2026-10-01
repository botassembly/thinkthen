# A cancelled C scalar call can return success

Status: Closed on 2026-09-27 by ticket 0166 (`sdlc/tickets/0166-a-cancelled-call-never-succeeds.md`), landed. A fired token now wins over every result on every public route, and a worker sees the token before each attempt and retry. Found while finishing the Zig package in experiment 273.

## Failure

A caller starts `thinkthen_decide_opts` with a live cancel token. A loopback server acknowledges the HTTP request and holds the response. Another caller thread fires that same token twice. After a further 250 milliseconds, the server releases a valid answer. The call returns `THINKTHEN_OK`, writes YES and probability 0.9, and never reports cancellation.

The same token immediately refuses a subsequent call with `THINKTHEN_ECANCELLED` and leaves its output sentinel untouched. That control verifies that the token fired and was passed correctly. The backend counts exactly one request in each trial. It sees no request for the subsequent pre-cancelled call.

The parent reproduced this three times through Python ctypes calling the C ABI directly. The Zig wrapper is absent from that reproduction. All three cancelled calls returned code 0 instead of code 5. An uncancelled control returned code 0. All engine and token owners remained alive until their caller threads joined.

## Contract

`libraries/c/include/thinkthen.h` lines 18–20 say the token is checked on every tick and cancellation ends the call within a tick. The `thinkthen_cancel` declaration promises that calls carrying a fired token return `THINKTHEN_ECANCELLED` with no results. `libraries/c/DESIGN.md` section 1 also says already-sent requests finish and calls carrying the token return cancellation.

The delayed completion and successful result need separate treatment. A design that drains accepted requests can legitimately wait for that drain. It still must honor the documented final cancellation result. This issue does not propose interrupting transport unsafely or abandoning live workers.

## Provenance and reproduction

- Source: landed main `e0a6150a0325d92b93ac8f4e504ffbfc3792fba9`.
- Host: Ubuntu 24.04.3, Linux x86_64, glibc 2.39.
- Native release library SHA-256: `08527edfe9f80034c129072423bb18c898108b9002fe2262ed640ce1fe3feffc`.
- Parent reproduction: local experiment 273's `verification-stage2/cancel_c_api.py`.
- Evidence: `verification-stage2/cancel-c-api-attempt-2.log`, exit 1. It records arrival, cancellation return, reply release and call return in monotonic order for every trial.
- Earlier Zig failures: `stage2/logs/concurrent-run-1.log` and `concurrent-run-2.log`.

The parent ran the reproduction under the ThinkThen heavy lock with a clean process environment, an isolated cache, a numeric loopback destination and the experiment canary. No paid inference or external backend was used. The first parent attempt timed out waiting for the lock and ran no test. Its empty log supplies no evidence.

## Source pointers for the owner

At the pinned source, `engine/mod.rs::Cancel::poll_between_sends` skips its check while an HTTP send is active. `engine/workers.rs::on_worker` joins and returns the worker result. A successful send returns immediately from `engine/http.rs::Client::post_observed`. `public/options.rs::Stop::run` returns the successful result after `finish`, which handles saved panics without a final token check. These paths explain where to investigate. The experiment has not chosen the product fix.

## Acceptance

Retain a request-arrival barrier test through the public C interface. Fire the token while the reply is held, then release the reply and require the documented cancellation code with unchanged outputs. Prove a fresh token on the same engine still works and no new request starts after the cancellation boundary. Check the corresponding scalar JSON and bulk paths. Reconcile the header's tick promise with the accepted-request drain contract. Keep ownership, accounting and no-resend behavior intact.
