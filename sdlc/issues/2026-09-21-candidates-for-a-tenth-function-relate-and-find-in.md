# Candidates for a tenth function: `relate`, and `find` with a second text

Status: Open. Nothing here is for the first release.

Ian asked two questions on 2026-09-21. Is there a map and reduce capability over Jev that ThinkThen lacks? Does the article "Jev + graphical models" (his note of that name) suggest a function? He also asked whether any two-function chain in the use-case slides should be one function. He does not want the verb count to explode. The marketing side, which holds the product shape job, answers here.

## Map exists. One kind of reduce is missing

Every function under `--lines` is a map. `annotate` is a map with a form. A reduce that counts, averages, or groups by a value is the host's job: `jq`, SQL, `dplyr`. ThinkThen should never grow one. A reduce that writes a summary is impossible, because the model writes nothing.

`find` is already a reduce done by the model: many lines go in, one comes out, in one request. The missing reduce has the same form with two inputs: many records on one side, a set of candidates on the other, and one candidate or none comes out per record.

## What the use-case chains showed

Ten chains were built. Eight of them need no new verb. They need the two fixes already filed in `2026-09-21-two-function-flows-lose-the-record-between-stages.md`: print the record with its answer, and let `filter` take a band.

Two chains are the same missing thing. The join use case asked `decide` about every pair. The on-call use case put the alert inside the question string because `find` cannot take a second text. Both want: for each record, pick the matching line from another file, or none.

**Suggestion 1, small: `find --in FILE`.** The records stream on standard input and the candidates sit in the file. One request per record, where the pair approach costs one per pair. A thousand tickets against two hundred incidents is a thousand requests in place of two hundred thousand. In SQL it is `thinkthen_find(question, body, list_of_candidates)`. No new verb.

## What the article showed

The article asks the model many small questions in one request and then does all further work offline with no more requests. Its first experiment filled probability tables. `annotate --details` over a file of conditions does that today. That is a how-to and no verb. ThinkThen should not build an inference engine.

Its second experiment is the useful one. It listed every pair of items, offered three options per pair (an edge one way, an edge the other way, no edge), and sent all 28 pairs in one request. That is step three of `recognize`, run over a list of items in place of the names in a sentence.

**Suggestion 2, one new verb: `relate`.** "Say how the records relate to each other." Records go in with relation rules, and edges come out with a number on each. It reuses the relation rule from `sdlc/planning/recognize-design.md` unchanged: a direction, a kind or `*` at each end, `either` for both ways, "no relation" always offered. `recognize` then becomes "find the names, then `relate` them", and the engine holds one pair-asking path.

What it covers with one verb:

| Job | The rule |
| --- | --- |
| Group alerts into incidents, or find duplicates | `same_as`, either way. The groups are the connected pieces, computed in code |
| Which ticket blocks which | `blocks`, one way |
| Which claim supports or contradicts which | two rules |
| Which requirement a test covers | `covers=test:requirement` |
| The article's graph of what depends on what | `depends_on=*:*` |

In a database the edges are rows, and the engine's own recursive queries walk them.

The cost is the concern. Pairs grow with the square of the records. Short records pack many pairs into one request, as the article did, and the rule's kinds cut the pairs further. `--dry-run` must print the pair count and the request count. `find --in` is the cheap path for a large set, and `relate` is for a set small enough to pair.

## The order

1. The two filed fixes, before the first release if the build team agrees.
2. `find --in`, soon after. It is the join story at an affordable cost.
3. `relate`, only after an experiment measures it on made-up records: accuracy against the pair count, the packing limit, and whether "same as" groups come out stable.

A `group` verb is not proposed. It is `relate` with one rule and a small piece of code.

## A caution for any how-to built on the article

The article multiplies the model's probabilities together, so an error in each one compounds. The second-backend experiment showed that probabilities do not carry between models. A how-to must say both.

## What Ian can overturn

All of it.
