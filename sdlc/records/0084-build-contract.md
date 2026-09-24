# 0084: Build the frozen Rust contract

Status: landed on main through `ticket/0084-freeze-rust-contract`. Owner: Claude.

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

## How to rerun the checks

- Extract a ticket's block: `awk '/^```rust$/{f=1;next} /^```$/{f=0} f' TICKET > block.rs`.
- Parse it: `rustfmt --edition 2024 --emit stdout block.rs`. Exit 0 means it parses.
- One owner: across the 0084 and 0095 blocks, each `pub struct`, `pub enum`, or `pub trait` name appears once, and each `pub fn` name appears once per `impl` target and once at the crate root.
- Cap: `wc -m TICKET`.

## Ladder

At `0304a6e4`, observed: `install` 0, `lint` 0, `test` 0, `spec` 0 (demos 21 green, 0 red). `ratchet.mjs` read `48140/48140`.

## Code review fix

The code review (`sdlc/records/0084-0095-code-review.md`) found one blocking item, B1. The two proposed ADR 0017 amendments sat above Ian's accepted Polars ruling and the dated amendments. They now sit unchanged at the end of the file. The 2026-09-23 amendment gains one sentence naming `EngineBuilder::from_env`. The ticket's status line drops `sdlc/records/` so its code review line can name the review, and the ticket is 29,992 characters.

## Landing

The reviewer confirmed the B1 fix at `ef7269de`. The branch then merged main `c19771a8` at `d9f12f6d`. At `d9f12f6d`, observed: `install` 0, `lint` 0, `test` 0, `spec` 0 (demos 21 green, 0 red). `ratchet.mjs` read `48547/48547`, main's measured total, and this branch leaves `sdlc/ratchet.json` unchanged. The commit that adds this section changes only this record and the ticket's status line, and `lint` ran on it before the merge to main.

Post-0078 reconciliation stays open: 0078 moves `nix` into the library graph and `signal-hook` behind `cli`, so the package-proof sentence and 0086 line 41 change then (code review answer 6).
