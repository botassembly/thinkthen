# Is there an eleventh function? A sweep of the three primitives

Written 2026-09-21 by the product side at Ian's request, before the marketing starts. The answer is no. The sweep found two widenings of functions we have and one command that is no function. All three are backlog candidates. Nothing here is authorized.

## The sweep

Jev has three primitives: yes or no, pick one, place on a scale. An input has three shapes: one text, many records, pairs of records.

| | Yes or no | Pick one | Place on a scale |
| --- | --- | --- | --- |
| One text | `decide`, and `tag` asks one per label | `choose`, and `recognize` asks one per word | `score` |
| Many records | `filter` keeps, `rank` sorts | `find` picks the record | **gap 1** |
| Pairs of records | `find --in`, in the backlog | `relate` | none needed |
| All at once | `annotate` | `annotate` | `annotate` |

## What each idea turned out to be

| Idea | What it is | Verdict |
| --- | --- | --- |
| Sort records by a scale, or by how likely one option is | **Gap 1.** `rank` sorts by a yes-or-no probability only | Widen `rank` to take any question file. A scale question sorts by the expected level. A pick-one question sorts by one named option's probability. No new name |
| Send records to piles by a band or by the option picked | `filter` with a band and a choice of pile | Already in the backlog. No new name |
| Group records that say the same thing | `relate --either same_as`, then connected groups in plain code | A how-to |
| Remove near-duplicates | The same, keeping one per group | A how-to |
| Compare two texts and say which is better | `find` over two lines | Covered |
| Check that a text supports a claim | `choose` with supported, contradicted, not_stated. The `choose` help already teaches it | Covered |
| Pull a value out of a text | `recognize` with the user's own kind, such as `invoice_number` | Covered. Worth one example in the manual |
| Show which sentence drove a yes | `decide`, then `find` over the text's sentences | A how-to |
| Pick the key sentences of a long text | `rank --top` over the sentences | A how-to |
| Guard a step in a script or an agent | `decide` and its exit code | Covered. It is the first slide |
| Know whether a question is good enough to trust | **Gap 2.** Nothing measures a question against records a person already labeled | A command, below |

## Gap 2: a command that measures a question

Every buyer asks one thing the ten functions cannot answer: "How do I know it is right on my data?" The vendor states that the model is calibrated. We cannot confirm that, and a threshold tuned on one backend did not carry to another. A user needs a way to find out on their own records.

The shape: the user gives records that carry a person's answer in one field. The command asks the question of each record, and prints how often the tool agreed, how many fell in "not sure", and what the numbers become at a few other thresholds. It asks through the same engine and the same cache, so a second run is free.

It is a command like `status`. It is no eleventh function, because it judges the question and never a text. The libraries do not need it for the first release. A user can do it today with `decide --jsonl --details` and `jq`, and the first step is a how-to that does exactly that. The how-to tells us whether the command earns its place.

## Are the primitives mixed as well as they can be?

`annotate` is the mix: all three kinds ride in one request, and one request is the unit of cost. `recognize` chains pick-one three times. The how-to candidates chain one function's answer into the next. The one mix we leave on the table is gap 1.

## Can any function be split further?

No. `recognize` was the one bundle, and `relate` came out of it. Its kind step is `choose`. `tag` is several `decide` questions in one request, and `filter` and `rank` are `decide` over records. Each of those earns its name by what it prints, and none hides a second job.

## What Ian can overturn

All of it. The cheap ones: widening `rank` before 0.1, and building the measuring command before 0.1.
