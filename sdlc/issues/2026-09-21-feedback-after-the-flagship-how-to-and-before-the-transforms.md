# Feedback after the flagship how-to, and before the transforms

Status: Open

Ian asked the marketing side on 2026-09-21 to read ticket 0041's page and give feedback on the builder's next tickets: comparison, sweep, repeated trials, the monitor, the grouped sweep, and the check against human labels. This page authorizes nothing. The builder owns every call in it.

## The flagship page

`demos/16-triage-pipeline/` does the job. Six tickets read as real ones, the question set sends `/body` alone, a null fails closed, and `agreement=6/6` against `reviewed_action` is the right proof. Urgency level 1 going to review is a fair default, and it stays.

Two things would make it serve as the page the README, the talk, and the site lead with. Both fit the limits of ADR 0016, at 85 lines and 687 words today.

1. **Show the result a person would look at.** The visible proof is four counts. A reader never sees which ticket went where, or why. Six lines of id, subject, action, and reason are the picture the slide copies, and they teach the five rules faster than the paragraph does.
2. **Give the idea in two lines before the safe script.** The pipeline is `thinkthen annotate ... | jq -f triage.jq`. The twelve-line script is about staging and cleanup, and it is right to keep. A reader who meets the two lines first sees that the tool judges and the rules decide.

## The transforms

1. **Comparison: a change inside the model's own play is noise.** `2026-09-21-the-same-request-answers-differently-twice-measured.md` has the measurement: one request sent twice moved by up to 0.08, 63 of 100 probabilities differed, and four answers flipped at a 0.5 cut, all within 0.08 of it. A `changes` list that reports every differing value will be long on two honest live runs of one question file. Each entry can carry both probabilities, and the how-to can say which changes to read past. A tolerance argument is the builder's call. The page also says that two runs compare fairly when both are live or both replay one recording.
2. **Repeated trials: a real fixture exists and costs nothing.** `experiments/212-thinkthen-repeat/` holds 100 public SMS messages judged three times with one question: two runs minutes apart, and the 2026-09-20 run. The builder decides whether the text may be committed. The rows serve as an outside check either way.
3. **Sweep: the probabilities have two decimals.** Ticket 0038 found the vendor rounds to hundredths. The 19-cut grid is safe. The page can say that a cut finer than 0.01 means nothing, and that many rows tie.
4. **`transforms/cost/cost.jq:32` while the folder is open.** `$row.input.id` fails on a row whose input is a string, and the accuracy round hit it live. It belongs with this work more than with any later pass.
5. **The monitor.** `reviewed_action` reads well, and one word now serves the page and the transform.
6. **The verdict ticket says "0.1" or "the first release".** Ian ruled the number on 2026-09-20. "Version one" stays in the history and leaves the pages as they are touched.

## For the pass after the transforms

`2026-09-21-triage-of-the-open-issues-by-layer.md` sorts every open issue by the layer that owns it and suggests an order. Six stale issues were closed with their evidence on 2026-09-21. Nothing in it interrupts the transforms. One front-door item is worth knowing now: `README.md` opens with "puts a decider model in the shell", and its section on what the tool is not for says a library comes "after version one". Ian ruled the pitch as semantic commands and ruled the libraries in. The wording ticket in the triage page covers both.

The blocking-engine run is in `experiments/211-thinkthen-blocking-engine/`. Its engine lane matched the bench at 9.666 s and held 32 in flight, and its two binding lanes were running on 2026-09-21. Its report has not landed.

## What Ian can overturn

All of it.
