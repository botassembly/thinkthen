# 0084: Build the frozen Rust contract

Status: built on `ticket/0084-freeze-rust-contract`. Not landed. Owner: Claude.

## Result

0084 is design only. The build changed no code, no dependency, no feature, and no ratchet. It merged `origin/main` at `9f47bd18` into the branch with no conflict, checked the ticket's design acceptance, and set the ticket's status line to point here. The status edit also shortened the ticket's code review line to keep the ticket under its cap.

`Engine`, `EngineBuilder`, and `default_engine` stay provisional. Ticket 0078 has not landed on main, so the reconciliation that 0084's outcome names still waits for it before 0085 or 0086 starts.

## Checks

The acceptance line asks for five things. A checker in the builder's scratch space tested four of them, and `git diff --check` tested the fifth.

- The ticket and the ADR 0017 amendment agree. ADR 0017 says 0084 owns the Rust builders, calls, and values, keeps the `Engine` portion provisional, and gives fork recovery to 0096. The ticket says the same. Pass.
- rustfmt parses the extracted block. `rustfmt --edition 2024 --emit stdout` exits 0 on the one `rust` block. Pass.
- Each public name has one owner and one shape. No type is declared twice, and no member name repeats inside one `impl` target or at the crate root. Pass.
- `wc -m` is 29,983, under 30,000. Pass.
- `git diff --check origin/main...HEAD` is clean. Pass.

Re-review follow-up N3 asked for `Send + Sync` on `CallOptions`. The trait paragraph already reads `Copy + Clone + Default + Send + Sync`, so nothing changed.

## Planted failures

Each plant ran alone on a scratch copy, and each turned the check red.

| Plant | Check that failed |
| --- | --- |
| Drop the `)` from `Engine::usage(&self)` | rustfmt: mismatched closing delimiter |
| Append one line past 30,000 characters | character cap |
| Rename the free `decide_with` to `decide` | one owner: `decide` declared twice at the root |
| Give fork recovery to 0085 in ADR 0017 | ADR and ticket agreement |
| Add a trailing space to the ticket | `git diff --check` exits 2 |

## Ladder

`sdlc/scripts/install`, `lint`, `test`, and `spec` run once at the commit that adds this record. The builder reports that result with the commit.
