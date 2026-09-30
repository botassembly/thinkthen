# relate's pair planner loses precision on the Beatles Bench, and both-ways edges print a direction

Status: open. Measured by Beatles Bench tickets 0019 and 0018 on 2026-09-30, against main `c22512868`. The bench's `reports/results.md` publishes every figure below.

Owner: ticket 0342 adds the single-answer menu and the precision statement. Ticket 0344 settled the both-ways edge shape: an `either` edge ends with `"either":true` on every surface. The decision run of 2026-09-30 passed the bar, so ticket 0353 makes the menu the recommended form for single-answer relations; this issue closes when 0353 lands. Ian can overturn the split, the shape and the verdict. Blocks 0.1: the default and the shape must be set before the release.

## The problem

Ticket 0167 replaced relate's choice planner with the pair planner, which asks one yes or no question for each pair a rule allows. On the same set of 182 songs, the four Beatles and the 13 core albums, at the 0.5 cut:

| Planner | Edge precision | Edge recall | Edge F1 |
| --- | --- | --- | --- |
| Choice planner, main `02dc0b96` | 0.867 | 0.614 | 0.719 |
| Pair planner, main `c22512868` | 0.420 | 0.693 | 0.523 |

Smaller sets show the same shape:

- On 16 sets where the right album is left out, relate still names an album for every song. Edge precision is 0.296.
- On composer and producer pairs from Wikidata, precision is 0.330 at recall 0.833.
- An audit-tuned cut helps little: held-half F1 is 0.689 on solo songs, 0.373 where the right album is missing, and 0.596 on the Wikidata pairs.

Stated relations read from text do much better. `recognize --relation` reads edges a sentence states at precision 0.973 and recall 0.783.

## What should happen

The queue owner decides whether relate's precision at the default cut is acceptable for 0.1. Options:
- Tune relate's default cut.
- Ask each song's single-answer relations as one choice with a way to say none of these.
- Publish the trade-off on the relate page and point users to `recognize --relation` for relations a text states.

## Recommendation for 0.1

This is a product default. The coordinator sets it, and Ian can overturn it. Drawn from the figures above and experiment 275, section R2.

| Option | Evidence | Cost | Risk |
| --- | --- | --- | --- |
| (a) Tune the default cut (`cli/relate.rs:74`, `unwrap_or(0.5)`) | An audit-tuned cut reached held-half F1 0.689, 0.373 and 0.596. The first row of the table shows the choice planner at F1 0.719 on the same set. The cut never enters the request, and `--details` already prints each pair's probability, so saved runs give every cut with no new spend | Small: one default and the pages | A cut cannot help where the right album is missing: every candidate pair can pass it. The low precision stays |
| (b) Ask each source's single-answer relation as one `choice` that includes a none-of-these option; keep the pair planner for relations that allow many answers | The choice planner scored precision 0.867 against 0.420. In experiment 275 section R2, a pick-one menu per pair kept 27 of 27 relations at 27 of 29 precision with 34 questions, against yes/no pairs at 26 of 27 and 26 of 28 with 56 questions, on 30 stated-relation cases. The none-of-these answer is new and unmeasured. R2 read relations from text, and `relate` asks about names alone | One medium ticket after 0304 slice 4: a way to mark a relation single-answer, a new question shape, new recordings, conformance cases 51 and 52 recorded again, and one paid bench run through `sdlc/scripts/live` under a token cap (ruling 13) | It changes the wire form, so it cannot share slice 4's identical-results proof. If the model rarely picks none, precision on the missing-album sets stays low |
| (c) Keep the pair planner; state the measured precision on the relate page, and point to `recognize --relation` for relations a text states | `recognize --relation` reached precision 0.973 at recall 0.783 | Pages only | 0.1 ships a known weak default |

Recommendation: (b) with (c), in the relate ticket after 0304 slice 4 that also carries the unordered both-ways edge shape (issue priorities, coordinator default 1). Keep the 0.5 cut. The relate page states the measured precision and points to `recognize --relation` in either case. Accept (b) as the default only if the paid bench run beats the pair planner's edge F1 of 0.523 on the 182 songs and its precision of 0.296 on the missing-album sets. Otherwise ship (c) alone for 0.1, and keep this issue open. Option (b) costs more than (a), but (a) cannot fix the missing-album case, and a changed default is cheaper before 0.1 than after.

## The both-ways edge shape

Done in ticket 0344, which chose a flag over the `pair` array below so that every edge keeps one shape.

Moved here on 2026-09-30 from item 3 of `2026-09-25-recognize-and-relate-scale-and-shape.md`. Coordinator default 1 of `../planning/issue-priorities-2026-09-30.md`, which Ian can overturn: change the shape before 0.1, in the same relate ticket, because a breaking change after 0.1 costs every consumer.

An `--either` edge still prints `source` and `target`, normalized to input order (`specification/relate.md`). A reader cannot tell a both-ways pair from a one-way edge without the rule. Ian, reviewing the relate examples on 2026-09-22: "relate source and target should be more obvious too."

The fix: give `--either` edges an unordered shape, for example `{"relation":"duplicates","pair":[{…},{…}],"probability":…}`, on the command and every surface in one change. Keep the one-way shape.

## The decision run

The Beatles Bench team asked for three conditions on 2026-09-30. The bench harness is ready. It needs the thinkthen commit that adds the single-answer menu with "none of these".

1. Report the none-of-these rate beside precision: how often relate answers "none of these" when the right album is missing from the set.
2. Compare at an audit-tuned cut as well as at 0.5. The bench reports that tuning moved the pair planner's F1 on solo songs only from 0.689 to 0.696. The menu should win at both cuts.
3. Run on Liquid d1 as well as on the default backend.

The acceptance bar above holds on the default backend at the 0.5 cut. The other two cells and the none-of-these rate go in the relate ticket's record and on the relate page.

### Result, 2026-09-30: the menu passes

thinkthen ran the decision run itself on main `8bb32e59a`, which holds 0342 and 0344, so the bench need not run it. It used the bench's own case files, harness (`scripts/run/ask_suite.py`) and scorer (`scripts/score/relate_audit.py`, `scripts/score/score_suite.py`) at bench commit `34128b0f`. The only change to the cases was `"single": true` on `appears_on`, the song-to-album relation, in a scratch copy of `relate-suite.json`. `sung_by` stayed on the pair planner.

- Backend and model: the default address `https://api.typesafe.ai/v1` with `jev-1.13.0`, the model the baseline run's backend reported.
- Cases: `relate-songs` (182 songs), the 16 `solo` sets and the 16 `wrong-album-only` sets, 33 calls in all, four at a time. The plan's upper bound was 293,161 input tokens; duets and links were left out to stay under 300,000, since neither enters the bar and links has no song-to-album relation.
- Spend: one `sdlc/scripts/live` job under a 325,000-token cap. 36 requests, 1,220 questions, 151,401 input and 49,389 output tokens, about $0.006.
- Recording: made with `--record` into a scratch folder and converted with `thinkthen cache convert` (1,220 answers). A replay with no key gave byte-identical outputs. It holds only public bench names and stays local, uncommitted.

At the 0.5 cut, beside the pair planner's figures from the table above:

| Set | Measure | Pair planner, `c22512868` | Menu, `8bb32e59a` | Bar |
| --- | --- | --- | --- | --- |
| 182 songs | edge F1 | 0.523 | 0.635 (0.587 to 0.689) | above 0.523: passes |
| 182 songs | edge precision | 0.420 | 0.749 | |
| 182 songs | edge recall | 0.693 | 0.551 | |
| 16 missing-album sets | edge precision | 0.296 | 0.541 (0.384 to 0.690) | above 0.296: passes |
| 16 missing-album sets | edge recall | | 0.526 | |
| 16 solo sets | edge F1 | | 0.737 | |

The none-of-these rate: on the missing-album sets, the menu answered none for 16 of 31 songs (0.516). It named a wrong album at or above 0.5 for 11 songs and below 0.5 for 4. Where the right album was listed, it answered none for 7 of 182 songs (0.038) and 2 of 31 solo songs (0.065).

The tuned cut: `thinkthen audit` on a seeded half chose 0.47 on both the solo and the missing-album sets. The 182-song case is one case, so it has no halves and no tuned cut. At 0.47 the held half scored:

| Set | Held-half F1, pair planner | Held-half F1, menu | Menu precision, recall |
| --- | --- | --- | --- |
| solo | 0.689 | 0.680 | 0.708, 0.654 |
| missing album | 0.373 | 0.526 | 0.500, 0.556 |

Verdict: the menu beats both bars on the default backend at the 0.5 cut, so option (b) is accepted with (c), and ticket 0353 carries the default. The menu trades recall for precision: song-set recall fell from 0.693 to 0.551. At the tuned cut the menu wins on the missing-album sets and ties the pair planner on solo sets, 0.680 against 0.689 on eight held cases, so the bench's second condition holds on one of two sets.

The Liquid d1 cell was not run. The instruction for this run allowed one paid run, and the bar needs only the default backend. It stays open in ticket 0353's Defers.

## Evidence

Beatles Bench `results/runs/2026-09-30-relate-jev` and `results/runs/2026-09-30-recognize-jev`, with their recordings, and `reports/results.md`, "The function suite".
