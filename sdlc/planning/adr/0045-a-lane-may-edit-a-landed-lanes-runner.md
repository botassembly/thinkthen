# ADR 0045: A lane may edit another lane's runner after that lane lands

Date: 2026-09-23. Status: accepted. Generalizes the phase-three permission
recorded in
`sdlc/records/0071-the-two-narrow-exceptions-granted-in-the-review-waves.md`
(item 2). Record 0075 (item 4) and the second review (R2-29) asked for the
general form.

## Context

Lanes do not edit each other's files, so two lanes never collide mid-flight.
In phase 3, new shared conformance cases crashed the DuckDB driver
(`databases/duckdb/tools/conformance.py`). The DuckDB lane had already
landed and pushed.

## Decision

A lane may edit another lane's test runner only after the owning lane's
work has landed. The edit stays inside the branches the new cases need,
implements a shape instead of skipping it wherever the host can spell
it, keeps each remaining skip with a written reason, and keeps the
runner's self-test able to fail on a corrupted expectation. The commit
names the owning lane and the reason.

The phase-three edit followed this rule: `c83add5` extended the driver's
`decide_many` and `annotate` branches, and `2143d7e` added the cases and
the extended self-test.

## Consequences

A runner no longer blocks shared cases after its lane lands. A lane still
in flight keeps its files to itself.

Ian can overturn this by requiring the owning lane to make every such
edit. The cost is a wait on that lane each time the shared cases grow.
