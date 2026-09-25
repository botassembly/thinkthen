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

The four scores pick three different bars on this data:

| Score | Best value | Bar | Formula |
| --- | --- | --- | --- |
| accuracy | 94% | 0.85 | right answers / all answers |
| precision | 100% | 0.95 | right yeses / all yeses |
| recall | 100% | 0.75 | real yeses caught / all real yeses |
| F1 | 74% | 0.73 | 2 × precision × recall / (precision + recall) |

## Asks

Ian's design of 2026-09-25.

1. `audit --optimize accuracy|precision|recall|f1` sets the score the suggested cut maximizes. `accuracy` stays the default.
2. Each score's top is usually shared by a run of bars. Tie rules make each pick useful:
   - recall takes the highest bar with the top recall. A bar of 0 also reaches 100%, but it says yes to everything.
   - precision takes the lowest bar with the top precision.
   - accuracy and F1 keep the spec's rule today: nearest to 0.5, then the smaller bar.
3. The count object adds `precision` and `f1` beside `yes_recall`, and `--table` prints all four scores at the suggested cut.
4. An optional `--all` prints the suggested cut for each of the four scores in one run. The talk's audit slide draws those four lines on one table.

The suggested object already names its `objective`, so the output shape holds. Ian can overturn all three.
