# relate's output must read on its own, and the real job needs two sets

Status: Open, for the build team, after the preview. Filed 2026-09-22 by the product side from Ian's review of the relate examples. Ian can overturn.

## The problem

An edge prints as `{"name":"same_as","source":1,"target":3,"probability":1.0}`. The reader counts lines to learn what 1 and 3 are. For a rule that reads both ways, `source` and `target` carry no meaning. Ian's words: "relate source and target should be more obvious too. im worried that function isnt obvious or useful for real needs."

## Asks

1. **The edge carries the records.** With `--lines` the row carries both texts. With `--jsonl` and an id field named on the command, the row carries both ids, as the SQLite surface already does. `--details` keeps the numbers.
2. **A both-ways rule prints a pair, not a direction.** No `source` and `target` on an `--either` edge. A one-way rule keeps the direction and may print the sentence the rule implies, using the rule's verb from the rule table.
3. **Two sets.** The common real job is across two sets: which invoice does this payment settle, which open incident does this ticket belong to, which job does this résumé fit. `relate` takes one set today. A form that takes two inputs and returns the matches covers that job. The join how-to does it today with `decide` over pairs built by `jq`.

## Until then

The product side sells `relate` as a preview only, on one example: five bug reports, two duplicate pairs, the texts printed beside the pair. The cause-and-effect framing leaves the relate slide. The graph slide stays as the picture of what edges become.
