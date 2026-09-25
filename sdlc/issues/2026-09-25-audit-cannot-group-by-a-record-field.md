# audit cannot group by a record field

Status: Open. Filed on 2026-09-25 by the Beatles Bench owner for Beatles Bench ticket 0011. That ticket moves grading onto `thinkthen audit` only where audit gives the same numbers. This gap keeps one bench measure in Python.

Ian ruled on 2026-09-25 that everything is in 0.1 (`sdlc/planning/one-line-plan-2026-09-25.md`, commit `82336e9a`). Ticket 0131 (`sdlc/tickets/0131-audit-matches-the-bench.md`) settles this issue after ticket 0125 lands.

## What happens

`--by` groups by answer name, question text, or verb. Beatles Bench reports accuracy by category, by kind, by forward fact, and by reversal direction. Those are fields of the question record, such as `category` and `kind`, and no question text carries them alone.

A caller can split the answer file with `jq` and run audit once per group. That works, but each run prints its own suggested cut and bootstrap, and the calls multiply.

## Options

1. `--by POINTER`: group by a JSON pointer into the record, as `--id` already reads one.
2. No change. Callers split the file first.

Recommendation: option 1. Ian can overturn it.
