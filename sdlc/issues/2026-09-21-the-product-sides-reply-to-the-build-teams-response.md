# The product side's reply to the build team's response

Status: Closed on 2026-09-22. The reply was delivered, and its additions landed through tickets 0053, 0054, 0059, and 0060.

Written 2026-09-21 for Ian, who rules. The response is `sdlc/planning/build-team-response-to-handoff-2026-09-21.md`. The product side agrees with the revised order and with all five recommendations. It asks for five additions, and none of them blocks the first two tickets or the crate move.

| # | Recommendation | Reply |
| --- | --- | --- |
| Order | Two small result shapes, then the move, then the larger features | Agreed. It is better than the order the handoff gave. The cache is the quality wave's one blocker, and it must still land before any release |
| 1 | `meta.requests` as a list | Agreed. The caller's team asked for a way to join a judgment to its recording, and a list does that |
| 2 | A failed marker and `meta.failed_questions` | Agreed, with additions A and B |
| 3 | `{"input","value"}` rows by default in record mode | Agreed, with additions C and D |
| 4 | `probability` on a relation, `confidence` on a recognized name | Agreed. `products/thinkthen/vocabulary.md` in the marketing repo already says this |
| 5 | One threshold and a warning on a profile mismatch | Agreed, with addition E |

## The five additions

**A. A run with a failed question must not look clean to a script.** The marker is in the JSON, and a script reads the exit code. The failure ticket rules one of two things: a distinct non-zero exit code, or exit 0 with a fixed line on standard error. The product side prefers the exit code, because QA found that a quiet retry is invisible and this is the same fault.

**B. The marker needs a ruled form where a value has one type.** A Python `decide` returns `True`, `False`, or `None`. A SQL `thinkthen_decide` returns a Boolean or `NULL`. Neither can hold an object. The product side's suggestion: a library raises for a single call and returns a typed failed value inside a bulk result, and a database function raises a query error by default, because a `NULL` would send a failed row to the "not sure" pile. `filter` and `rank` need the same ruling for a record whose question failed. This is ruled before the C door freezes, inside the conformance cases the response lists.

**C. One text in, one bare answer out, unchanged.** `thinkthen decide '...' < ticket.txt` still prints `true`. The response implies this. The ticket should pin it with a test, because it is the first command every new user runs and the first slide of the deck.

**D. `recognize` and `relate` follow the same record rule.** `recognize-design.md` says a record comes back with the object attached under `recognize`. That changes to the response's rule: enrichment for an object record under the key `recognize`, and `{"input","value"}` for a text line. One rule for every function that returns an answer per record. The key name `input` is fine, and it avoids the open "evidence" against "text" ruling.

**E. The mismatch warning must reach a program.** A warning on standard error never reaches a Python caller or a SQL query. `meta` carries the same fact, a field that names the profile the threshold was tuned for and the profile that ran. A question file that names no profile warns nobody.

## What the product side does after Ian rules

It rewrites the record-mode outputs on the deck slides and the public examples page, changes the `recognize` design page per addition D, and updates the library team's page. The recorded requests do not change, only the printed rows.

## What Ian can overturn

All of it. The additions are suggestions to the build team's tickets.
