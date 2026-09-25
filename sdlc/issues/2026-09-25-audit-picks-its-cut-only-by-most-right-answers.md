# audit picks its cut only by most right answers

Status: Open

Filed by the marketing session on 2026-09-25, from the Beatles Bench audit example (`examples/11-audit`, bench ticket 0007). ThinkThen main was at e70bddab.

## What happens

For `decide`, audit's suggested cut is the bar with the most right answers on the tuning part (`specification/audit.md`, "The suggested cut"). The user cannot say which mistake costs more. The count object prints `yes_recall` but no precision and no F1.

## Why it matters

The two mistakes rarely cost the same. A screen that must not miss a case wants every real yes, even at the cost of extra wrong yeses. A list shown to a customer wants few wrong yeses, even at the cost of misses. The best bar differs for each.

The bench example shows the gap. The question is "It appears on the album Abbey Road." over 70 songs, 7 of them on Abbey Road.

| Bar | Caught of 7 | Wrong yes | Right of 70 | Precision | Recall | F1 |
| --- | --- | --- | --- | --- | --- | --- |
| 0.5 | 7 | 14 | 56 | 0.33 | 1.00 | 0.50 |
| 0.75 | 7 | 5 | 65 | 0.58 | 1.00 | 0.74 |
| 0.85 | 5 | 2 | 66 | 0.71 | 0.71 | 0.71 |

audit suggests 0.85 because that bar gets the most right. F1 picks 0.75, which catches all seven. A user who cannot miss one needs 0.75, and audit never names it.

## Asks

1. `audit --optimize accuracy|f1|recall|precision` sets what the suggested cut serves. `accuracy` stays the default.
   - `accuracy`: the bar with the most right answers.
   - `f1`: the bar with the best F1. F1 weighs misses and wrong yeses together.
   - `recall`: the highest bar whose recall reaches `--target`. With a target of 1, that bar misses no real yes. The Abbey Road example gives 0.75.
   - `precision`: the lowest bar whose precision reaches `--target`. With a target of 0.9, nine in ten yeses are right.
   - The highest and lowest rule matters. Recall alone is perfect at a bar of 0, and precision alone is perfect near 1. The target turns each into a useful choice. Ties break as the spec breaks them today.
2. `--target` already sets the agreement a `choose` cut must reach. It takes the recall or precision floor for `decide` too, with a default of 0.9. With no bar that reaches the target, the cut is null, as for `choose`.
3. The count object adds `precision` and `f1` beside `yes_recall`, and `--table` prints them.

The suggested object already names its `objective`, so the output shape holds. Ian can overturn all three.
