# Build the rest of 0.2 in waves and test once

Date: 2026-10-12. Ian can overturn this ruling.

Ian asked how to pause testing, make the remaining changes quickly and in parallel across the languages, and then test safely, so that the team spends less time waiting on test runs.

## Why

Most of the waiting in 0.2 has been repeated test runs. Each ticket ran the routine suite, installed checks in many languages, and sometimes the full campaign. Later tickets then changed the same code, so the earlier runs proved a state that never shipped.

The remaining tickets fit a different pattern:

- 0561 and 0565 mostly delete code.
- 0562 changes the generators, so it changes every language's generated files at once.
- 0563 changes each language library, and the languages do not touch each other.
- 0564 deletes and moves tests.

Each ticket after the first depends on the shape the earlier ones leave. Testing each ticket fully would prove intermediate states. The corrective batch for 0549 to 0558 already worked this way: the team built one combined branch, gated it once, reviewed it once and landed it.

The aim is maximum throughput of fixes with one full proof at the end. Fast checks guard each step, and checkpoints keep a late bug traceable.

## Ruling

### The waves

1. **Baseline.** Finish 0560 and record the coverage baseline on main. It is the "before" picture. It shows which code is dead and which tests add nothing.
2. **Core.** One developer does 0561 and 0565 together, because both delete from the C library and the Rust API. Another developer does the Rust-only part of 0564 at the same time, because it touches test files only.
3. **Generators.** One developer does 0562 and regenerates every language in one commit.
4. **Languages.** 0563 fans out to one developer per language group, each in its own worktree. The coordinator picks the groups from 0563's list. Library work in 0564 joins the matching language group.
5. **Candidate.** Combine everything, review it once, and run the full campaign once.

A wave starts only after the previous wave reaches its checkpoint. Languages built before the core and generators settle would be rebuilt.

### Checks during the waves

- Developers run only fast checks while they work: build, lint with the source limits, and the routine checks of the parts they touch.
- Nobody runs the full campaign, the full installed parity run or the stress checks before the candidate.
- Each wave ends at a checkpoint on the combined branch. The checkpoint runs the routine suite once, plus a build of every language library. A build is cheap and catches a deleted function that a language still calls.
- Ticket Proof lines run on the candidate SHA. No ticket closes before the candidate passes. They all close together on that SHA.

### Where the work lives

- All waves build on one integration branch, `ticket/0543-final`. Main receives the whole batch once, from the accepted candidate SHA.
- Each developer works on a short branch off the integration branch and commits small, one change per commit, and pushes after each fix.
- The coordinator merges developer branches into the integration branch.

## Risks and how we avoid them

| Risk | How we avoid it |
|---|---|
| A bug found at the end could come from any of six tickets, so it is hard to trace. | Checkpoints after each wave limit where a bug can hide. Small commits allow `git bisect`. Each language group runs its own routine checks before merging. |
| Developers build on code that is still changing and must redo work. | Strict wave order. The core and generators settle before any language work starts. |
| Two developers edit the same file and collide. | Each language developer edits only its own `libraries/` directory and its own source-limit file. Only the generator developer edits generators and generated files. The coordinator edits shared files: lint, `sdlc/scripts`, CI workflows, the core limit and the top-level docs. A language developer who needs a core or generator change asks the coordinator, who batches those requests into one change. |
| A deletion removes something a language still uses. The 0561 boundary review already found one: the Zig library calls `thinkthen_question_author`. | Search every caller before deleting, using the coverage baseline. Build every language library at the end of the core wave. |
| Deleting tests loses coverage that mattered. | 0564 works from the baseline. The candidate's coverage run must not fall below the baseline, apart from lines whose code was deleted. Write the outside-in test before deleting the inside one. |
| The machine runs out of capacity. Heavy builds wait behind one shared lock, so many developers can end up waiting on each other. | Keep at most four developers on heavy builds at once. Language checks are light, so the fan-out wave can run more developers when their checks stay light. |
| One giant diff is too big to review well. | Split the fresh review by area: core, generators, and each language group. All reviewers read the same SHA. |
| The full campaign finds several bugs at the end. | Fix forward on the integration branch. A fix confined to one language reruns that language's full tier. A fix to the core or the generators reruns the full campaign. The SHA that lands on main is the SHA that passed. |
| Skipping per-ticket runs hides a slow failure that piles up. | The routine suite still runs at every checkpoint, and every developer still runs fast checks. Only the expensive runs wait for the candidate. |

## What this replaces

This replaces the order in the fourth 0543 amendment and the per-ticket full-campaign proofs in 0560 to 0565. The other 0543 rulings still apply. One revision is tested, accepted and landed. The candidate runs the full campaign, the GitHub Apple and Windows runs and a coverage run on that revision. Publishing still needs Ian's go.
