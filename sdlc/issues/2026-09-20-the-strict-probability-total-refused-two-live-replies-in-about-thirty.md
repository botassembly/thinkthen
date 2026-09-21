# The strict probability total refused two live replies in about thirty

Status: Open

A live run on 2026-09-20 died twice on the same refusal. The run was arm A of `experiments/209-thinkthen-nlp/`: `annotate` with a question set of one `choose` question, 17 options, a description on each. The first attempt was refused at request 19. The resume replayed those and was refused at request 30. Twenty-nine replies were accepted and two were refused. `experiments/209-thinkthen-nlp/NOTES.md` has the log.

The refusal, word for word:

    thinkthen: the reply was refused: the answer to question `q1` has
    probabilities whose total differs from one by more than member count ×
    f64::EPSILON

The check is `crates/thinkthen-core/src/answer.rs:37`. The prospective plan says to keep the strict rule and "collect rounding evidence during an authorized product probe". This is the first evidence.

## What is known

- The refusal moves. Request 19 was refused once and accepted on the next try. The same request bytes got a reply that passed.
- The rule has been in the code since 2026-09-19. The accuracy round on 2026-09-20 sent 770 `choose` requests with 77 options and 1,500 yes-or-no requests, and its notes say "no job was refused, no request failed".
- The tolerance grows with the member count. At 17 members it is about 3.8e-15. At 77 members it is about 1.7e-14. A shorter list gets a tighter bound, and a sum of 17 floats can miss one by a few units in the last place depending on the order of addition.

## What is not known

How far off the two refused totals were. A refused reply is never recorded, and the message does not print the total. The experiment's notes guess at decimal rounding by the service and propose a tolerance of one hundredth. No reply was seen, and that guess is unproven. A miss of a few units in the last place and a miss of one hundredth call for different fixes.

## Why it matters

A user with a list of ten to twenty options loses about one request in fifteen to exit code 4, at random, and a stream stops at the first one. Every surface inherits it. The guard charges a job's whole cap when the job dies, and this run lost two caps to it.

## Recommendation

1. **Print the measured total and the member count in the refusal.** Both are numbers the tool computed, and neither is evidence text. Every refusal then becomes the evidence the plan asked for.
2. **Capture the refused totals before choosing a tolerance.** One small probe under the guard repeats a 17-option request until a refusal shows its total. The cost is a fraction of a cent.
3. **Then set the rule from the numbers.** If the misses are a few units in the last place, a fixed floor under the tolerance ends them and still refuses a total of 0.85. If the service rounds its decimals, the rule needs a different shape, and `specification/result.md` says so in the same commit.

This belongs in the correction pass, ahead of the cache locks. It stops paid runs today.

## What Ian can overturn

The placement in the correction pass. The builder owns the tolerance.
