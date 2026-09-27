# A mixed-model cache fails every `annotate` record

Status: Open. Filed 2026-09-26 by the queue owner from local experiment 273, report 03, finding 2-5. Question 1 in the 0.1 backlog, the default-model pin, is answered. The message half is done: ticket 0159 landed on 2026-09-26, and the message now says a cache or recording folder may hold answers from the other version and names `--no-cache` and `cache prune`. The cache fix still waits and has no owner.

## What happens

`annotate` refuses a record whose chunks report different model versions. A cached chunk and a live chunk count as two chunks. So after the vendor moves the `jev-latest` alias, editing one question in a split set fails every record. The unchanged groups come from the cache under the old version, and the edited group comes back live under the new one.

The report simulated it with a replay folder. Unchanged groups came from `fake-1` entries and the edited group from a `fake-2` entry. The run stopped at record 1, exit 4, with `the backend returned different model versions for one record; pin --model and rerun with --record or --cache`.

The spec promises that a split group reuses the chunks whose bytes did not change. After a model move, that promise stops every record. The message blames the backend and suggests `--cache`, though the cache caused the stop. Pinning `--model` changes every digest and asks everything again.

## Checked on main

Verified by reading the code: `crates/thinkthen/src/engine/facade/annotate.rs:243` returns `ModelsDiffer`. The run comes from the report.

## What would fix it

Treat a cached group answered by another model version as a cache miss for that record. Or allow mixed versions and list each group's model in `--details`. Either way, rewrite the message so it does not blame the backend when a cache mixed the versions.

Ticket 0159 pinned the default model (backlog question 1, option A). The alias no longer moves under a default run, and this fault needs an explicit model change to appear. Ticket 0159 also landed the message fix.

## Done when

An edited question under a cache holding an older model version either re-asks the record or reports both versions. A replay test covers it.
