# Quick Fix qf-ticket-evidence: tickets from 0120 on name their evidence in five parts

Status: landed. It carries out workspace decision `2026-09-24-experiments-reduce-risk.md` in the dotfiles repo. That decision asks each product ticket to name its evidence, retained behavior, deliberate changes, current proof, and deferred gaps. A fresh read-only Opus review is in `sdlc/records/qf-ticket-evidence-review.md`.

## Result

- `sdlc/README.md` states the rule under the folder table and cites the decision. A ticket numbered 0120 or higher carries a `## Evidence` section with five list items: `Starts from`, `Keeps`, `Changes`, `Proof`, and `Defers`. Earlier tickets are exempt by number. Every numbered ticket here counts as a product ticket.
- `sdlc/tickets/README.md` points to that paragraph in one sentence.
- `sdlc/scripts/tickets` checks every `sdlc/tickets/NNNN-*.md` at or above 0120. It strips fenced blocks, and a fence closes only on the marker that opened it. It removes `**` from each line, finds the first `## Evidence` heading, and needs each label as a list item with text after its colon. A bold label passes. A label outside the section fails.
- `sdlc/scripts/lint` runs `tickets --self-test` and then `tickets` right after the catalog check. `sdlc/scripts/README.md` lists the script.
- `CLAUDE.md` is unchanged at 4,988 of 5,000 characters.

## Planted bugs

The self-test builds one fresh tree per case and pins the whole failure list. It holds thirteen cases: a good ticket, an exempt 0119, a missing section, a section inside a backtick fence, a section inside a tilde fence, a section inside a backtick fence that a `~~~` line does not close, a missing `Defers` item, an empty `Proof` item, an empty bold `Keeps` item, a `Keeps` item under the wrong heading, and three passing items: `**Proof**:`, text that opens in bold, and text that opens in italics.

Each plant below ran by hand in the worktree and was then removed.

| Plant | Command | Observed | Exit |
|---|---|---|---|
| A real `sdlc/tickets/0120-planted.md` with no Evidence section | `tickets` | `0120-planted.md: ticket 0120 and later needs a `## Evidence` section` | 1 |
| The same ticket with four of five items, `Defers` missing | `tickets` | `0120-planted.md: the Evidence section lacks a `- Defers:` item with text` | 1 |
| `FIRST = 10000`, which exempts every ticket (first pass) | `tickets --self-test` | `2/7 cases hold` | 1 |
| The item pattern drops its "text after the colon" clause (first pass) | `tickets --self-test` | `6/7 cases hold`, the `0123-empty.md` case fails | 1 |
| The check reads raw lines and skips the fence strip (first pass) | `tickets --self-test` | `6/7 cases hold`, the `0121-fenced.md` case fails | 1 |
| The item pattern takes any character after the colon, the first-pass code | `tickets --self-test` | `9/10 cases hold`, the `0125-bold-empty.md` case fails | 1 |
| The fence pattern knows backticks only, the first-pass code | `tickets --self-test` | `9/10 cases hold`, the `0126-tilde.md` case fails | 1 |
| The second-pass item pattern, which needs a first character other than `*` | `tickets --self-test` | `11/13 cases hold`, the `0128-bold-text.md` and `0129-italic-text.md` cases fail | 1 |
| Any fence marker closes any fence, the second-pass code | `tickets --self-test` | `12/13 cases hold`, the `0130-mixed-fence.md` case fails | 1 |
| All plants removed | `tickets --self-test`, `tickets` | `13/13 cases hold`, `tickets: 0 evidence failures from ticket 0120 on` | 0 |

## Checks

`sdlc/scripts/lint` exited 0. It printed `tickets self-test: 13/13 cases hold`, `tickets: 0 evidence failures from ticket 0120 on`, and `ratchet: crates + conformance 50378/50378`.

`sdlc/scripts/live` did not run.
