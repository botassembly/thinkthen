# Go-ahead for the build team, 2026-09-21

Written by the product side for Ian to give the build team after ticket 0053. It answers `sdlc/planning/build-team-response-to-handoff-2026-09-21.md`. Ian's word on the day: "now is the time to make those changes." If Ian's own message differs from this page, his message wins.

## Approved

1. The revised order: request identity (done), partial-question failures, the mechanical crate move, backend profiles, the record that comes back with its answer, engine settings, the default cache. Then `recognize` and `relate`, then the C door freezes.
2. `meta.requests` as a list. Landed in 0053.
3. Partial success with the failed marker and `meta.failed_questions`.
4. `{"input","value"}` rows by default in record mode, with `annotate` keeping its enrichment of objects.
5. One threshold per question, with a warning when the backend profile differs.

## Changed since the response was written

| Topic | The ruling | Page |
| --- | --- | --- |
| The number on a relation | `probability`. Approved as recommended | `sdlc/issues/closed/2026-09-21-one-rule-for-every-number-the-tool-prints.md` |
| The number on a recognized name | **Settled: `strength`.** The comparison showed no plain model probability gates as well, so the computed number stays under a name that claims nothing about chance. `--details` prints its parts | The same page |
| The vendor's `confidence` | Passes through under `--details` only, under the vendor's name. No function gates on it and no bare output prints it | The same page |
| A relation's two ends | `source` and `target` on every surface, in the command's JSON, and in the question file. The command-line rule stays `--relation NAME=FROM:TO` | Both design pages, last ruling section |
| `recognize` options | No depth option. A relation rule turns relations on. Default kinds are `person`, `organization`, `place` | `sdlc/planning/recognize-design.md`, rulings table |
| `relate` input | Records only. Names a user already has are records with a kind field | `sdlc/planning/relate-design.md`, ruling section |
| Polars and pandas | No engine work. Lists and columns ride the same bulk call | `sdlc/issues/2026-09-21-the-polars-shape-as-the-deck-shows-it.md` |

## Five additions for the tickets that are coming

From `sdlc/issues/closed/2026-09-21-the-product-sides-reply-to-the-build-teams-response.md`:

- A. A run with a failed question changes the exit code. The response already plans this for the next ticket.
- B. The failed marker needs a ruled form where a value has one type: a library's single call, a bulk result, and a SQL function. Settle it in the conformance cases before the C door freezes.
- C. One text in still prints one bare answer. 0053 kept this. Keep a test on it.
- D. `recognize` and `relate` follow the same record rule as the other functions.
- E. The profile-mismatch warning also goes in `meta`, so a program can see it.

## What to read when `recognize` and `relate` come up

`experiments/225-recognize-harvest-package/` first: the exact words, the rules with tests, and forty recorded cases in the ruled shape. Its keys still say `from` and `to`, and the rename to `source` and `target` is mechanical. The library team is building both functions on the `surfaces` branch against the stand-in now, and its conformance cases will be ready to share.

## Not now

Widening `rank` to sort by a scale or by one option, `find --in`, `filter` with piles, the spend ledger, and a command that measures a question. All are recorded as backlog.

## What Ian can overturn

All of it.
