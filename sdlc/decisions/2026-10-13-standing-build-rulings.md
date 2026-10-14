# Standing build rulings for 0.2

Date: 2026-10-13

The PM made these rulings so the coordinator decides alone in the common cases. A ticket's own text wins where it is more specific. Ian can overturn any of them.

## Size and ratchets

- A source limit may rise to the measured total when the added lines have a named cause, such as a required fix or its regression test, and the total stays at or below main. Record the cause in one line.
- A required new export, call or helper in a library under a fixed limit is offset by a cut in the same library. If no safe cut exists, raise the limit to the measured total and say why.
- A size target is a goal, not a gate. Stop at safe cuts and report the gap. Never cut an assertion, a test or formatting to meet a number.
- Generated lines never count against a handwritten limit.
- The combined Rust total is a gate on the final candidate, not on a lane merge. A reviewed lane that adds required Rust lines merges into `ticket/0543-final` when its other checks pass. The Rust test lane and G2 then cut the combined total to main or below before freeze.

## Measurements

- Run-to-run variance below 0.1% is noise. Do no reproducibility work for it.
- Commit only short summaries of a measurement. Raw data stays as build output outside Git.
- Comparisons at the candidate use counts, not line identities.

## Checks

- Every checkpoint runs one fixed gate list: routine `test`, `lint` and every library build, collecting all failures in each phase before stopping.
- A developer runs its own library's `check.sh`, its lint limit and the routine tests for the files it touched before it hands back.
- Each lane builds into its own Cargo folder, `CARGO_TARGET_DIR=/tmp/thinkthen-target-<lane>`, so lanes never queue on one build directory. The lane deletes it when it lands.
- Every `check.sh` uses one heavy-build lock path. Run every ready developer lane at once; at most four heavy builds run on the Beelink at a time, while machine load stays below the core count.

## Ownership

- One generator developer owns the shared generator files: `generate.py`, `registry.py`, `host_answers.py`, `layouts.py`, `c.py`, `native_request.py`, `operations.py`, `.gitattributes` and `policy.py`. Each language lane owns its own `sdlc/generators/results/templates/<lang>*.py` files and regenerates its own outputs. A lane that needs a shared generator change asks the generator developer.
- Each language lane edits only its own `libraries/<language>` directory and its own limit entry.
- A developer who touches a shared file outside its lane stops and asks the coordinator.

## Questions and rulings

- The coordinator sends every question to the PM with one "From coordinator:" line in the PM's pane, never in Ian's pane or its own.
- The PM types its "From PM:" line as soon as a ruling commits.
- Before asking, the coordinator checks these rulings and the ticket's amendments. It asks only when neither covers the case.

## Records

- A wave gets one short record: what ran, its result and its counts. No raw data, no lists of what the evidence does not prove.
- A review verdict names its exact scope. A review of one repair never stands for a ticket.

## Scope and recovery

- A new ticket joins a running wave only at its next checkpoint.
- After a capacity error or a crash, the coordinator, or the PM on noticing, resumes from the last pushed commit and the open lanes. No lane restarts from scratch while its branch holds pushed work.
