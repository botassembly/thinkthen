# planning/

Read status, order and lanes through `pm daily`, `pm next` and `pm lanes`. [milestones.md](milestones.md) defines release outcomes; [preserved planning rulings](../decisions/2026-10-08-preserve-planning-rulings.md) names the lasting decisions from removed status and handoff files.

- `design-study.md`: what thinkthen is, what version one holds, how it fits, and what waits on Ian. A study until he rules.
- `rust-standards.md`: how the code is judged, rule by rule, with the enforcing tool.
- [ownership.md](ownership.md): who owns what. The queue owner owns the whole repo.
- [worktrees.md](worktrees.md): how lanes are claimed and freed.
- [ticket-preparation.md](ticket-preparation.md): how a ticket is prepared and reviewed.
- [release-process.md](release-process.md): the coordinator's release process, from checkpoints to the release. The one home of that process.
- [milestones.md](milestones.md): the 0.1, 0.2 and later milestones and their exit criteria.
- [windows.md](windows.md): the Windows stages and the verdict for each surface.
- `quality-plan.md`, `review-lessons.md`, `ten-use-cases.md`, `interface-audit.md`, `how-to-portfolio-study.md`, `sdk-design-study.md`, `related-projects.md`, `recognize-design.md`, `relate-design.md`: earlier studies and designs that ADRs and tickets cite.
- [Preserved bench rulings](../decisions/2026-10-08-preserve-planning-rulings.md#bench-evidence-and-fixtures): test ownership, recording provenance and historical measurement sources.
- `flat-verbs-review.md`: a review of Ian's flat-verb redesign of 2026-09-19, with one recommended surface. Ian accepted it, and ADR 0007 records the decision.
- `open-concerns.md`: the contested points across the ADRs and the plan as of 2026-09-19, each with options, a recommendation, and the reason. Ian ruled on it the same day, and ADR 0010 records the rulings.
- `documentation-plan.md`: the how-to form, the capability map, the numbers that left and the rules. The one list of how-tos, with each page's state, is `demos/README.md`. ADR 0011 makes the demo, the how-to, and the test one file.
- [databases/](databases/README.md): the three database extensions Ian ruled in on 2026-09-20, DuckDB, SQLite, and PostgreSQL, each in Rust over the same engine. The eight SQL functions, the rules all three keep, what other people have already published, and one page per database. A fast follow that never holds the launch.
- [libraries/](libraries/README.md): the seven surfaces (the command, Rust, Python, JavaScript, Ruby, R, and C), the goals and anti-goals each one shares, and one page per surface. A study for the rewrite of ADR 0017.
- [polars-plan.md](polars-plan.md): the plan for the native Rust door over the contract. One engine serves the Rust surface, the Python-Polars path, and the pure-Python list crossing; both Python containers stay first-class and all scaling runs in Rust. Experiments 212 through 216 answer the risks.
- [0.3/](0.3/README.md): the product plans for ThinkThen 0.3: the design files, ten PRDs with their worked questions, and the decision list for Ian.
- `adr/`: architecture decision records, numbered from 0001. Each says whether Ian can overturn it cheaply.
