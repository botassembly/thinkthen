# ADR 0042: R notices an interrupt at the next tick

Date: 2026-09-23. Status: accepted. Records the ruling that lived in
`sdlc/records/0074-the-r-interrupt-window.md` and `libraries/r/NOTES.md`
at tag `surfaces-wave7-final` ("Residual windows, stated"). The second review asked for an ADR (R2-29).

## Context

R's Ctrl-C arrives as a longjmp. A jump across Rust frames is undefined
behavior, and no spelling in the language makes it safe.

## Decision

The R surface checks for an interrupt only inside `R_ToplevelExec`, on
the main thread, at three points: before a call, at the 100 ms tick of
the wait, and before and after every `.tt_call`. A pending interrupt
cancels the engine's token and returns a marker. The R half raises the
marker as R's own interrupt condition.

The surface accepts three residual windows and states them in its
documentation:

- An interrupt is noticed at the next tick, up to 100 ms after the signal.
- A request already on the wire finishes on the backend. The call returns
  without waiting for it.
- An interrupt during a call prints one blank line to standard error
  that R's own interrupt does not print. The amendment below says why.

## Consequences

No jump crosses a Rust frame. The latency is bounded by the tick, and
the R suite measures it (0.886 s against a 1 s signal in record 0074).

Ian can overturn this by asking for a shorter tick or a quieter
interrupt. A shorter tick costs more wake-ups per call. Silencing the
blank line needs an interrupt check other than `R_CheckUserInterrupt`.

## Amendment, 2026-09-23: the blank line, not an `Error:` line

The seventh review's verifier ran an uncaught Ctrl-C during a call and
during plain R code, with the full output kept. Neither printed an
`Error:` line, before or after R7-13. The third window above said one
did. That was wrong.

The real difference is one blank line on standard error. Plain R prints
one blank line and `Execution halted`; a call prints two blank lines and
`Execution halted`, with the same exit status 1. A caught interrupt shows
the source: plain R prints nothing before the handler runs, and a call
prints one blank line. The guarded check runs `R_CheckUserInterrupt`
under `R_ToplevelExec`, where no R handler stands, and R's own delivery
prints that line before the jump the guard catches. The line comes from
R, so the R half cannot suppress it. Removing it would mean reading R's
pending-interrupt flag directly instead of calling R's check. That
changes the mechanism four reviews have checked, for one blank line. Decision: keep it and record it. Ian can overturn this.

Since R7-13 an uncaught interrupt meets a user's `options(error = ...)`
hook as plain R does: the hook runs once and the script goes on.

## Amendment, 2026-09-24: ported to main, and the R surface ticket owns it

Ticket 0099 ported this ADR to main at its number from tag `surfaces-wave7-final` (`f6a7faea`). The ruling above stands as accepted. The R surface ticket (queue item 3 in `sdlc/planning/one-line-plan-2026-09-24.md`) owns it going forward. That ticket rebinds R to the public API and re-proves the tick and the three residual windows against the real engine. Ian can overturn this owner by moving the ruling to another ticket.

## Amendment, 2026-09-25: re-proved on the public API

Ticket 0108 rebound R to the public API, and the ruling holds on the real engine. `libraries/r/tests/interrupt.R` proves the three check points and the three windows on the 0092 held arm. Each test signals only once the backend's count shows the request on the wire.

- Signal to `CAUGHT` for a single held call over ten runs on main: minimum 0.037 s, median 0.078 s, maximum 0.096 s. A 200-text batch at throttle 8 measured 0.030 s and 0.056 s. The test bound is 0.5 s: the 100 ms tick plus the child's slack under load.
- After release, a cancelled batch sends nothing more. Its count stays at the count read at the signal.
- The blank line stands. An uncaught held call prints one more blank line than plain R, and a caught one prints one blank line where plain R prints none.
- The port retired `ACTIVE`, `tt_cancel_active`, and `.tt_cleanup`. A drop guard on the wait cancels the call's token on every way out, and each call runs on its own detachable worker. The fourth cleanup piece, the hook held aside in `.tt_call`, stays for R7-13.
