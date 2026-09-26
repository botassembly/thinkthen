# A mixed-model cache fails every `annotate` record

Status: Open. Filed 2026-09-26 by the queue owner from local experiment 273, report 03, finding 2-5. Placement waits on question 1 in the 0.1 backlog, the default-model pin. The message fix can land in 0.1 on its own. Owner for the message fix: ticket 0159 on `ticket/0159-pin-the-default-model`, ready for review. The cache fix still waits.

## What happens

`annotate` refuses a record whose chunks report different model versions. A cached chunk and a live chunk count as two chunks. So after the vendor moves the `jev-latest` alias, editing one question in a split set fails every record. The unchanged groups come from the cache under the old version, and the edited group comes back live under the new one.

The report simulated it with a replay folder. Unchanged groups came from `fake-1` entries and the edited group from a `fake-2` entry. The run stopped at record 1, exit 4, with `the backend returned different model versions for one record; pin --model and rerun with --record or --cache`.

The spec promises that a split group reuses the chunks whose bytes did not change. After a model move, that promise stops every record. The message blames the backend and suggests `--cache`, though the cache caused the stop. Pinning `--model` changes every digest and asks everything again.

## Checked on main

Verified by reading the code: `crates/thinkthen/src/engine/facade/annotate.rs:243` returns `ModelsDiffer`. The run comes from the report.

## What would fix it

Treat a cached group answered by another model version as a cache miss for that record. Or allow mixed versions and list each group's model in `--details`. Either way, rewrite the message so it does not blame the backend when a cache mixed the versions.

If Ian pins the default model (backlog question 1, option A), the alias stops moving under a default run, and this fault needs an explicit model change to appear. The message fix is still owed.

## Done when

An edited question under a cache holding an older model version either re-asks the record or reports both versions. A replay test covers it.
