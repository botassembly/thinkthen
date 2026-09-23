# ADR 0042: R notices an interrupt at the next tick

Date: 2026-09-23. Status: accepted. Records the ruling that lived in
`sdlc/records/0074-the-r-interrupt-window.md` and `libraries/r/NOTES.md`
("Residual windows, stated"). The second review asked for an ADR (R2-29).

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
- An uncaught re-raised interrupt prints one `Error:` line before
  `Execution halted`. R's own interrupt prints none.

## Consequences

No jump crosses a Rust frame. The latency is bounded by the tick, and
the R suite measures it (0.886 s against a 1 s signal in record 0074).

Ian can overturn this by asking for a shorter tick or a quieter uncaught
interrupt. A shorter tick costs more wake-ups per call. Silencing the
`Error:` line needs a different condition class in the R half.
