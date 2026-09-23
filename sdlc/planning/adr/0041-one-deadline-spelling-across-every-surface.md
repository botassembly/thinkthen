# ADR 0041: One deadline spelling across every surface

Date: 2026-09-23. Status: accepted. Supersedes the draft recorded in
`sdlc/records/surfaces-notes/NOTES-rulings-wave.md` (fourth review, records
group: the ruling lived in a notes file). Second-agent review: the external
review's own probes confirmed each surface's spelling (third review, items
confirmed; fourth review, finding 21's reconciliation), and the independent
verifier's table on main re-ran the hostile-deadline probes.

## Context

Nine surfaces spell "how long may this call run" in nine hosts. Before this
ruling, four behaviors coexisted: some surfaces refused every negative
number, some treated any negative as "no deadline", one treated a computed
budget of −1 as none by accident, and one surface had no deadline door at
all. A computed budget (`end − now`) can land on −1 by chance, silently
disabling a deadline the caller meant to spend.

## Decision

Exactly one spelling of "no deadline" exists across every surface: the
sentinel −1 — the C header's `THINKTHEN_NO_DEADLINE`. Every other negative
value is refused with the usage kind. Zero is a spent deadline: legal,
and the call returns the deadline kind having sent nothing. The contract
owns the conversion and the sentinel (`deadline_from_seconds` /
`deadline_from_millis`, `NO_DEADLINE = -1.0`), and its tests pin all of
this.

A computed budget clamps to zero (`Math.max(0, end - now)` in the Node
spelling): a deadline that already passed is zero, not none. An
unrepresentable budget (one the clock cannot name) is treated as no
deadline, documented and tested — never a host panic.

## Why −1 and not a separate explicit-none argument

The C ABI is flat scalars; a second boolean argument would touch every
signature the product deck draws, and −1 is the shipped, tested spelling
the probes confirmed. The accidental-−1 hazard is answered by the clamp
rule, which is the semantically correct statement anyway.

## Consequences

Python and Node adopt "−1 means none" through the contract's converter;
Node stops coercing `true`, `"5"`, and `[]` into numbers. DuckDB's ruled
deadline door names the same sentinel. Documentation on every surface
states the rule once, in the same words.

Ian can overturn this by ruling a different sentinel or an explicit-none
argument; the cost is touching every surface's door and every drawn
signature that carries a deadline.

## Amendment, 2026-09-23: only a number crosses on Python too

The Consequences named Node alone. Python's bool is an int, so
`deadline=True` ran as a one-second deadline and `deadline=False` as a
spent one, while Node refused `true`. The fifth review found the split.
Python now refuses a bool, Python's or NumPy's, or any non-number with
the usage kind, in the same sentence on every door. A NumPy int or float
is still a number and still crosses. Ian can overturn this by ruling
that a bool is a number here; the cost is one extraction type in the
Python shim.
