# 0095: Build the binding members of the Rust contract

Status: built on `ticket/0095-binding-contract-members`. Not landed. Owner: Claude.

## Result

0095 changes design records only. The build changed no code, no dependency, no feature, and no ratchet. It merged `origin/main` at `9f47bd18`, then merged `ticket/0084-freeze-rust-contract` at `0304a6e4`. 0095 needs 0084's files: its acceptance reads the 0084 ticket, and the ADR 0017 amendment that lists its members lives on the 0084 branch. Both branches carried `sdlc/records/2026-09-24-rereview-contract.md`. The 0084 copy is the 0095 copy plus the builder amendment confirmation, so the merge kept the 0084 copy. The build then set the ticket's status line to point here.

## Checks

The acceptance line asks for five things. A checker in the builder's scratch space tested four of them, and `git diff --check` tested the fifth. The same checker built 0084 (`sdlc/records/0084-build-contract.md`).

- 0084 and this ticket agree. Each of the five changes under "Why 0084 changed" appears in the 0084 inventory. 0084 names 0095's interrupt check as the one public callback. The ADR 0017 amendment names every member in the 0095 block, and the block declares every member the ticket names. Pass.
- rustfmt parses the extracted blocks. `rustfmt --edition 2024 --emit stdout` exits 0 on the one `rust` block of 0084 and the one of 0095. Pass.
- Each name has one owner and one shape. No type is declared twice across both blocks, and no member name repeats inside one `impl` target or at the crate root. Pass.
- `wc -m` is under 16,000 for this ticket. Pass.
- `git diff --check origin/main...HEAD` is clean. Pass.

## Planted failures

Each plant ran alone on a scratch copy, and each turned the check red.

| Plant | Check that failed |
| --- | --- |
| Drop the `)` from `Question::kind(&self)` | rustfmt: mismatched closing delimiter |
| Append 4,300 characters to the ticket | character cap |
| Rename `Question::kind` to `rank` | one owner: `Question::rank` declared in 0084 and 0095 |
| Drop `ErrorKind::name` from the ADR 0017 amendment | ADR and ticket agreement |
| Drop `AnnotatedRecord::value_json` from the block | block and ticket agreement |

## Ladder

At `d886fec6`, observed: `install` 0, `lint` 0, `test` 0, `spec` 0. `ratchet.mjs` read `48140/48140`.

## Code review

The code review (`sdlc/records/0084-0095-code-review.md`) accepted 0095. It asked for the 0084 fix to ADR 0017 to reach this branch through a merge, which it did after `d886fec6`. The rerun commands for the checks are in `sdlc/records/0084-build-contract.md`.
