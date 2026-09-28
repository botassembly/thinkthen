# The probability-total refusal prints float noise

Status: Closed on 2026-09-22. Merged into 2026-09-22-command-wording-and-help-fixes-for-0-1.md.

When a backend's distribution does not sum to one, the refusal prints the tolerance with fifteen digits. The page that rules the tolerance says one hundredth.

## Reproduction

A local stand-in answers with probabilities summing to 0.5. Commands from a scratch folder, 2026-09-21:

    $ thinkthen choose 'Which team?' billing shipping --max-retries 0 --input msg.txt; echo $?
    thinkthen: the reply was refused: the answer to question `q1` has probability total 0.5, member count 2, and tolerance 0.010000000000000444; the total differs from one by more than the tolerance
    4

## Expected

`specification/backends.md` rules the tolerance as `0.01 + member count × f64::EPSILON`. The epsilon term is a binary-parsing guard, and the page sells the rule as one hundredth. `0.01`, or one more digit, carries the whole fact for a reader.

## How bad it is for a user

Minor. The refusal is correct and the exit code is right. The tail digits are noise a stranger does not need.

Found by experiment 218, wave 1, areas 3 and 7.
