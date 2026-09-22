# 0069: The R interrupt window

Date: 2026-09-22

Status: landed

## Result

R's Ctrl-C stops a call without a longjmp crossing a Rust frame. The old poll called `R_CheckUserInterrupt` from Rust; its jump unwound R's C stack across the shim's frames, which the language does not define. Every interrupt check now runs inside `R_ToplevelExec`, which catches the jump and answers FALSE; a pending interrupt cancels the engine's token (no new request starts, requests already sent finish), clears the slot, and returns the packed interrupt marker, which the R half raises as R's own interrupt condition. `tryCatch(interrupt = ...)` catches it. The check runs on the main thread only: the pre-check in `call`, the 100 ms tick in the wait, and the R half's checks before and after every `.tt_call`.

The residual window is stated, not hidden: an interrupt is noticed at the next tick, so up to 100 ms after the signal (measured 0.886 s against a 1 s signal); a request already on the wire finishes before the engine stops, and the call returns without waiting for it; an uncaught re-raised interrupt prints one `Error:` line before `Execution halted` where R's own interrupt prints none. No full fix exists in the language: the jump can be caught or avoided, but it cannot be made to cross Rust frames safely. The guard removes the crossing and names the remaining latency.

## Evidence

From `libraries/r/NOTES.md` on branch `surfaces`:

- The throwaway C probe: `R CMD SHLIB guard.c`, then `dyn.load` and `.Call` print `toplevel result: FALSE`, `script continued`, exit 0 — `R_ToplevelExec` catches the interrupt's jump.
- The re-signal checks: a raised interrupt condition is caught as `interrupt`; uncaught it exits 1, matching R's own Ctrl-C behavior.
- The full check with no stub: shim 2 tests, null suite 56 checks, the fast interrupt at 0.886 s with `after: TRUE`, 10 of 10 examples, the conformance slice (67 ok, 3 diverge, 5 skip), 34 recognize checks, 11 ownership checks, the slide.
- The wire interrupt with the stub on 8215: at 2.907 s, the counter frozen at 320 through +1 s, +3 s, +5 s.

Landed at `b11e143` ("Type the R answer columns by kind, guard the interrupt, and check the deadline").
