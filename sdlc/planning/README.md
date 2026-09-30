# planning/

[cleanup-2026-09-30.md](cleanup-2026-09-30.md) is the current status authority: Ian's rulings, the order of work, the lanes, and progress.

- `design-study.md`: what thinkthen is, what version one holds, how it fits, and what waits on Ian. A study until he rules.
- `rust-standards.md`: how the code is judged, rule by rule, with the enforcing tool.
- [ownership.md](ownership.md): who owns which folders. Marketing owns `site/`.
- [worktrees.md](worktrees.md): how lanes are claimed and freed.
- [ticket-preparation.md](ticket-preparation.md): how a ticket is prepared and reviewed.
- [issue-priorities-2026-09-30.md](issue-priorities-2026-09-30.md): the ranked open issues, their 0.1 blockers, owners, and the tickets ready now.
- [test-split-2026-09-30.md](test-split-2026-09-30.md): the test inventory for ticket 0335, with times, groups, and the replay smoke per binding.
- [bench-handoff.md](bench-handoff.md): answers for the public Beatles Bench team: the three places for tests, the `status --json` fields, the tags, and how to send a hard case as a recording.
- [mainline-readiness-2026-09-23.md](mainline-readiness-2026-09-23.md): the checked main/worktree snapshot, quality and release assessment, and completion checklist for ticket 0074. Budget: 12,000 characters.
- [prospective-bash-rust-python-plan.md](prospective-bash-rust-python-plan.md): the short completion overview for the command, engine, and libraries. Tickets are written as work begins. Budget: 4,000 characters.
- `flat-verbs-review.md`: a review of Ian's flat-verb redesign of 2026-09-19, with one recommended surface. Ian accepted it, and ADR 0007 records the decision.
- `open-concerns.md`: the contested points across the ADRs and the plan as of 2026-09-19, each with options, a recommendation, and the reason. Ian ruled on it the same day, and ADR 0010 records the rulings.
- `documentation-plan.md`: the full list of how-tos, each with its demo, its slice, and its state. ADR 0011 makes the demo, the how-to, and the test one file.
- [databases/](databases/README.md): the three database extensions Ian ruled in on 2026-09-20, DuckDB, SQLite, and PostgreSQL, each in Rust over the same engine. The eight SQL functions, the rules all three keep, what other people have already published, and one page per database. A fast follow that never holds the launch.
- [libraries/](libraries/README.md): the seven surfaces (the command, Rust, Python, JavaScript, Ruby, R, and C), the goals and anti-goals each one shares, and one page per surface. A study for the rewrite of ADR 0017.
- [polars-plan.md](polars-plan.md): the plan for the native Rust door over the contract. One engine serves the Rust surface, the Python-Polars path, and the pure-Python list crossing; both Python containers stay first-class and all scaling runs in Rust. Experiments 212 through 216 answer the risks.
- [library-equivalence-2026-09-26.md](library-equivalence-2026-09-26.md): which of the command's capabilities each library and database surface matches, with code evidence, the gaps grouped into proposed tickets, the gaps that cannot close and why, and how fast each surface runs. A read-only audit of 2026-09-26. Ian can overturn each recommendation.
- `adr/`: architecture decision records, numbered from 0001. Each says whether Ian can overturn it cheaply.

## Historical

These pages record earlier phases. They no longer set the order of work.

- `plan.md`: the first build order, by slice.
- `build-queue-2026-09-21.md`: the build queue of 2026-09-21.
- `one-line-plan-2026-09-24.md` and `one-line-plan-2026-09-25.md`: one-line plans of those days.
- `work-plan-2026-09-27.md`: the batches before the cleanup phase.
- `remaining-batches-2026-09-28.md`: the batches left on 2026-09-28.
