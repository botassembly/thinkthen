# The same request answers differently twice, measured

Status: Open

Item 5 of `2026-09-21-where-a-user-could-lose-trust-a-first-list.md` asked whether one request sent twice gets one answer. It does not. `experiments/212-thinkthen-repeat/` holds the probe, and Ian authorized probes on 2026-09-21. One capped job sent 100 SMS messages twice, minutes apart, with one bare question, no cache, and no retries. Fifty messages were picked because they sat near 0.5 the day before, and fifty were taken in sample order. Every answer came from `jev-1.13.0`.

| Compared | Probabilities that differ, of 100 | Largest difference | Flips at a 0.5 cut |
| --- | --- | --- | --- |
| Two runs minutes apart | 63 | 0.08 | 4 |
| Today and 2026-09-20 | 68 | 0.08 | 6 |

Most differences were 0.01. The fifty clear messages moved by 0.03 at most. The fifty borderline ones moved by up to 0.08. Every flip sat within 0.08 of the cut, and a band of 0.4:0.6 would have sent all four to a person in both runs. The job reported 59,646 input tokens under a 75,000 reservation.

## What follows

1. **The pages say it.** A probability is a reading with about 0.08 of play on a hard case, inside one model version. `specification/threshold.md` and the how-to on picking a threshold state the number and its test. No page says the same input always gives the same answer.
2. **The band is the answer, and the number sizes it.** A band narrower than about 0.1 on each side of a cut does not hold a flip out. The three-way gate page can say why its band is as wide as it is.
3. **`--cache` and `--replay` are what make a run repeatable.** That is a second reason to teach them, beside the bill. An eval that compares two question files reruns both live or replays both. One live and one replayed is not a fair pair.
4. **The comparison transform needs a floor.** A change of 0.08 between two runs is noise. `transforms/` reports every changed value today. The comparison how-to says which changes to ignore, or the transform takes a tolerance.
5. **Repeated trials earn their place.** The builder's plan already averages saved probabilities across trials of one case. This is the measurement behind it.

Limits: one question, one public set, one hundred messages, one day, half of them picked for being borderline.

## What Ian can overturn

All of it. Point 4 is the only one that may change code.
