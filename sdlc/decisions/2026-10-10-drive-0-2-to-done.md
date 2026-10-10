# Drive 0.2 to done: delete old code, close tickets, cut process

Ruled by Ian on 2026-10-10.

## Ruling

In Ian's words: "We can definitely empower the coordinator to delete old stuff that we don't need anymore. We need to get rid of any bureaucracy that's slowing us down, and we need to drive the work to completion with high quality."

## What it means

### The coordinator deletes without asking

- Each language's old public API, once its replacement passes the routine installed checks. Put a short old-to-new mapping in that package's README.
- Dead code, superseded compatibility paths and scaffolding unit tests.
- Build output in its own lanes: `target/`, package caches and build folders that a build can regenerate. Delete it whenever a lane passes its cap or a build needs room.
- Its own merged branches and worktrees.

The frozen 0.1 C exports stay, as 0515 records. The coordinator never deletes another lane's active work, the live ledger, recordings the tests replay, or anything outside this repo.

### A migration ticket is done when

1. The language's new typed API works through its installed package against the fake backend, and its routine shared cases pass.
2. Its old public names and handwritten compatibility code are deleted, and the source ceiling drops.
3. `lint` and `test` pass once.
4. A fresh review accepts the code.

Platform qualification does not hold a migration ticket open. Windows, Apple, registry installs and full parity belong to 0530, 0383–0385 and the release suite at the candidate.

### Process that goes

- A fresh review is required for code only. Docs, records, ticket progress and output regenerated from reviewed templates need no separate review. One review may cover several slices of one ticket.
- Write a ticket's record once, at closure. A progress line is one sentence.
- A source-ceiling increase is settled in the code review. It needs no message to the PM.
- Within approved outcomes, decide, say why in the commit, and continue. Ask the PM only about scope, order or a conflict between rulings.

### Quality that stays

- Fresh code review, and the reviewer's power to reject.
- Outside-in behavior tests at the Rust API, the CLI and each installed library.
- Distinct parser, secrecy, cancellation, cache-miss, invalid-input and conflict regressions.
- Publishing waits for Ian's go.

### Candidates and GitHub testing

Ian added: "I'm fine with them cutting candidates, but then also deleting code. The code is all there in Git." And: "GitHub should be allowed to be used for testing Windows and testing other scenarios that we can't test locally, but testing locally should handle 95%+ of our questions."

- Local tests answer most questions. Use GitHub workflows for Windows, Apple and other platforms the Beelink cannot run.
- The coordinator may push candidate tags and dispatch release workflows whenever the work is ready, as long as no run publishes to a registry. This lifts the 2026-10-08 hold on candidates and dispatches. Publishing still needs Ian.
- Deleted code stays recoverable in Git, so deletion needs no extra caution beyond review.

### No ticket waits on Windows

Ian added: "You can close tickets without running them on Windows."

- No ticket stays open only for a Windows run. Close it on its Linux checks and review.
- Windows runs once, at the candidate, on GitHub. A Windows failure there becomes a new bug ticket.
- 0383–0385 close when their Windows packaging is built and reviewed.

## Why

On 2026-10-09 and 2026-10-10 the coordinator landed 149 slices and closed four tickets. Every migration ticket said "full installed parity, platform qualification and compatibility retirement remain held." Platform qualification needs a candidate, and candidates wait for Ian, so no migration ticket could close. Each language kept its old API next to the new one, and the Rust source ceiling rose from 181,586 to 187,880 lines. All three lanes reached the 40 GB cap, and the cap stopped builds while 254 GB of disk sat free. The process asked for permission where the rulings already gave it.

## Options considered

- Keep the rules and add lanes. This leaves tickets unable to close and old code in place.
- Close migration tickets only at the candidate. This defers every deletion to the end, where a coverage gap is found last.
- Close each migration on routine installed checks plus deletion, and move platform work to the candidate. Chosen.

## What it replaces

- The `AGENTS.md` lane rule that kept warm builds at the cap and allowed a full clean only below 50 GB free.
- Migration tickets' "compatibility retirement remains held until installed qualification".
- The PM's 2026-10-09 directions on one record commit per landing, withdrawn earlier on 2026-10-10.
