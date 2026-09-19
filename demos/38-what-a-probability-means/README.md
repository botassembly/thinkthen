# How to see whether a probability means what it says

Status: green

Verbs: `decide`

A probability of 0.8 claims that about eight cases in ten like this one are really yes. Use this page to test that claim against labels, to see where the model puts its answers, and to learn which part of the scale you have no evidence about. Nothing here calls a model.

## Input

`../../recipes/rows/runs/run-a.jsonl` holds forty judged cases. Each row carries `answer.probability`, the stored probability of yes, and `input.label`, the answer a person gave. The recipe is `../../recipes/calibration/calibration.jq`. It sets ten bands a tenth wide beside the share of cases in each band that were truly yes.

## Read the table

```bash
set -euo pipefail
rows=../../recipes/rows/runs/run-a.jsonl

jq -n -f ../../recipes/calibration/calibration.jq "$rows" \
  | jq -c '{rows, unlabeled}' \
  | mustmatch '{"rows":40,"unlabeled":["C-12"]}'

jq -n -f ../../recipes/calibration/calibration.jq "$rows" \
  | jq -c '.bands[] | select(.rows > 0)' \
  | mustmatch '{"band":"0-0.1","rows":17,"unresolved":0,"labeled":17,"truly_yes":0,"share_truly_yes":0,"mean_probability":0.0171}
{"band":"0.1-0.2","rows":1,"unresolved":0,"labeled":1,"truly_yes":0,"share_truly_yes":0,"mean_probability":0.1}
{"band":"0.3-0.4","rows":1,"unresolved":1,"labeled":1,"truly_yes":0,"share_truly_yes":0,"mean_probability":0.34}
{"band":"0.5-0.6","rows":2,"unresolved":2,"labeled":1,"truly_yes":0,"share_truly_yes":0,"mean_probability":0.545}
{"band":"0.7-0.8","rows":1,"unresolved":1,"labeled":1,"truly_yes":1,"share_truly_yes":1,"mean_probability":0.79}
{"band":"0.8-0.9","rows":6,"unresolved":0,"labeled":6,"truly_yes":6,"share_truly_yes":1,"mean_probability":0.84}
{"band":"0.9-1","rows":12,"unresolved":0,"labeled":12,"truly_yes":12,"share_truly_yes":1,"mean_probability":0.9842}'
```

Read `mean_probability` beside `share_truly_yes` on each line. The band below 0.1 holds seventeen cases at a mean of 0.017 and none of them was truly yes. The band above 0.9 holds twelve at a mean of 0.984 and every one of them was. At the two ends the number means what it says.

The middle is another story. The band from 0.5 to 0.6 holds two rows, one of them labeled, and that one was a no. A claim of "a bit more likely than not" is not tested by one case, and the table says so by printing the count next to the rate.

## Find out where you have no evidence

```bash
set -euo pipefail
rows=../../recipes/rows/runs/run-a.jsonl

jq -n -f ../../recipes/calibration/calibration.jq "$rows" \
  | jq -c '[.bands[] | select(.rows == 0) | .band]' \
  | mustmatch '["0.2-0.3","0.4-0.5","0.6-0.7"]'

jq -n -f ../../recipes/calibration/calibration.jq "$rows" \
  | jq -c '.bands[] | select(.band == "0.2-0.3")' \
  | mustmatch '{"band":"0.2-0.3","rows":0,"unresolved":0,"labeled":0,"truly_yes":0,"share_truly_yes":null,"mean_probability":null}'
```

An empty band yields null and never zero. Zero would read as "none of them was yes", which is a finding. Null reads as "nobody asked", which is the truth.

Thirty-five of the forty rows sit below 0.2 or above 0.8. This model does not hedge much, so most of the scale is empty on a run this size, and a threshold anywhere between 0.2 and 0.8 would move almost nothing. That fact is worth more than any single rate in the table.

## See which rows the run refused

The `unresolved` column counts the rows the run's own band left with no answer. They keep their probabilities, so they stay in their bands and are visible where they happened.

```bash
set -euo pipefail
rows=../../recipes/rows/runs/run-a.jsonl

jq -n -f ../../recipes/calibration/calibration.jq "$rows" \
  | jq -c '[.bands[] | select(.unresolved > 0) | {band, rows, unresolved}]' \
  | mustmatch '[{"band":"0.3-0.4","rows":1,"unresolved":1},{"band":"0.5-0.6","rows":2,"unresolved":2},{"band":"0.7-0.8","rows":1,"unresolved":1}]'
```

Every refused row sits between 0.3 and 0.8, which is the band the run applied. A refusal is not a wrong answer and it is not a no, and it is counted apart here as it is in every other recipe.

## The same line, kept as a file

```bash
set -euo pipefail

sh ../../recipes/calibration/example.sh \
  | jq -c '.bands[] | select(.band == "0.9-1") | {rows, truly_yes, mean_probability}' \
  | mustmatch '{"rows":12,"truly_yes":12,"mean_probability":0.9842}'
```

## What can go wrong

- **`jq` is missing.** The `install` rung names it.
- **A recipe stops with exit 5.** `jq` exits 5 for a line it cannot parse and for an error the recipe raises.
- **Reading a band that holds one case.** A share of 0 or 1 over a single row is noise. Read `labeled` first, and treat anything under a few dozen rows a band as a hint.
- **Forty cases across ten bands.** Three bands are empty and three hold one row. A calibration table earns trust at hundreds of cases, not tens.
- **Calling a model well calibrated from the ends alone.** Answers near 0 and 1 are the easy cases. The middle is where a probability has to earn its keep, and this run has almost nothing there.
- **A confident answer about evidence that was absent.** Calibration measures the judge on the cases you showed it. A question whose answer lies outside the message will be answered near 1 and be wrong, and no band in this table can see it.
- **Treating unresolved rows as missing.** They carry probabilities and belong in their bands. Dropping them would make a run look better calibrated than it is, because the refused rows are the uncertain ones.

## Related how-tos

- [How to check the judge against human labels](../25-check-the-judge/)
- [How to pick a threshold from labeled cases](../13-pick-a-threshold/)
- [How to compare two runs](../24-compare-two-runs/)
