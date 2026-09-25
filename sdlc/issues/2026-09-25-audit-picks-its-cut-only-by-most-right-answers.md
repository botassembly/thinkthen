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

1. `audit --objective accuracy|f1` chooses what the suggested cut maximizes. `accuracy` stays the default.
2. `--min-recall R` suggests the highest bar that keeps recall at or above R. `--min-precision P` suggests the lowest bar that keeps precision at or above P. Precision or recall alone has no useful maximum: recall peaks at a bar of 0, and precision peaks near 1. Each needs a floor on the other side.
3. The count object adds `precision` and `f1` beside `yes_recall`, and `--table` prints them.

The suggested object already names its `objective`, so the output shape holds. Ian can overturn all three.
