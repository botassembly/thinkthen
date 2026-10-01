# Packing many rows into one request, measured

Status: Closed on 2026-09-25 as superseded by `2026-09-25-packing-and-batching-what-they-buy-what-they-cost-and-the-setting.md`, which carries its numbers and the open design question. Earlier status: Open for the design question. The numbers are in. The live-launcher defect is closed by ticket 0101 (`sdlc/records/0101-live-refuses-non-shell-jobs.md`).

Public projects that put Jev inside PostgreSQL and DuckDB pack many rows into one request. That is where their speed comes from. `closed/2026-09-20-database-extensions-ruled-in-as-a-fast-follow.md` raised the question, and this page answers it with our own run. A builder ran it as local experiment 208, under Ian's standing go-ahead of 2026-09-20. The arms, the layout, and the label hashes were fixed in `PREREGISTRATION.md` before the first paid call, and every arm that ran is reported. That folder can rot. This page is the record.

Both datasets are old and public, so the model may have seen them. That flatters every arm, the baselines included. One model answered: `jev-1.13.0`.

## The layout, copied from the largest public project

One request holds one `state` object. The question sits once under `condition`, and every row sits in a list under `rows`. The request then asks one yes or no question per row, named `r0`, `r1`, and onward, each worded as "Does the record `rows[i]` satisfy the condition stated in `condition`?" An answer belongs to a row by its position alone.

## The spend

| Figure | Value |
| --- | --- |
| Ledger before and after, read by command | 4,669,118 and 5,970,118 charged tokens |
| Declared | 1,301,000 tokens, 5.5 US cents |
| Billed by the responses | 645,840 input tokens, 2.7 US cents |
| Billed so far under the one-dollar go-ahead | About 13.4 US cents |

## SMS spam, the same 1,000 messages as the accuracy round

The baseline is one row per request, already paid for: accuracy 0.968, recall 0.941, 289,260 tokens, 979 requests.

| Rows per request | Requests | Billed tokens | Accuracy | Precision | Recall | F1 | Answers that flipped against the baseline |
| --- | --- | --- | --- | --- | --- | --- | --- |
| 1, the baseline | 979 | 289,260 | 0.968 | 0.842 | 0.941 | 0.889 | |
| 5 | 200 | 113,996 | 0.949 | 0.735 | 0.978 | 0.839 | 47 |
| 10 | 100 | 86,396 | 0.968 | 0.821 | 0.978 | 0.893 | 32 |
| 20 | 50 | 73,096 | 0.941 | 0.745 | 0.860 | 0.799 | 59 |
| 40 | 25 | 66,446 | 0.882 | 0.590 | 0.434 | 0.500 | 128 |

At 40 rows the recall fell from 0.94 to 0.43. The damage follows position: the first fifth of the rows in a request scored 0.99 and the later fifths scored 0.80 to 0.87. At 20 rows the last fifth scored 0.89. No answer came back missing or misnamed in any arm. The 5-row arm scoring under the 10-row arm is most likely noise.

## BoolQ, the same 500 records

The baseline is 0.886 with 208,104 tokens. Five rows per request scored 0.884 with 116,462 tokens and 19 flips. Ten rows scored 0.890 with 102,512 tokens and 20 flips. These rows are five times longer than an SMS and the cliff did not come sooner. The count of rows breaks packing. The length does not.

## Neighbors change a row's answer

The 1,000 messages ran a second time at 10 rows per request, dealt into different groups by a seeded shuffle. Thirty-four answers crossed the 0.5 cut between the two groupings. The mean move in probability was 0.047 and the largest was 0.66. Only 200 of the 1,000 probabilities were identical. The accuracy barely moved, 0.964 against 0.968.

So the totals hold at 10 rows and the single answers do not. About 3 rows in 100 get a different answer depending on which rows shared the request.

## What this means

- **Ten rows is the most anyone should pack.** Twenty is at the edge, and 40 destroys recall. One public DuckDB extension defaults to 40.
- **Packing to ten buys about 3.3 times fewer tokens and ten times fewer requests** on short rows. The saving is the fixed overhead of about 256 tokens per request, shared across the rows.
- **Packing breaks three things this tool promises.** A row's answer stops being a fact about that row and its question alone. A recording keyed by the whole request replays only when the same rows land in the same group in the same order. `specification/annotate.md` says records never share a request, and a question never sees another record.
- **My recommendation: one record per request stays the rule and the default on every surface.** If packing is ever offered, it is an engine option for an ADR, off by default, capped at ten, named so that nobody mistakes it for the exact form, and documented with these numbers. It would serve `filter` and `rank` over short lines, where a total matters more than any one row. It never applies to `decide`, `choose`, `annotate`, or anything that feeds a gate.
- **The comparison somebody will make in public now has an honest answer.** The packed tools are faster per thousand rows. At their default widths they are also measurably less accurate, and at any width a row's answer depends on its neighbors.

## A defect in the live launcher

`sdlc/scripts/live` runs the job through `/bin/sh`. The builder's first job was a Python file. It died at once, and the launcher kept the whole reservation of 560,000 tokens, about 2.4 US cents, because the ledger has no refund. A shell wrapper fixed it. Two cheap guards would stop the next one: the launcher refuses a job whose first line is not a shell line, and `sdlc/scripts/README.md` says in its first paragraph that a job is a shell script. The no-refund rule is right and should stay.

Ticket 0101 closed both guards. The launcher refuses a job whose first line is not `#!/bin/sh` before it charges, and `sdlc/scripts/README.md` says a job is a shell script.

## Limits

Two datasets, one question each, one model, one day. One layout, copied from one project. The SMS set has 136 positives, so one message moves recall by about 0.007. The stability arm ran once at one width.

## What Ian can overturn

The recommendation. Whether packing is ever offered is a design decision for an ADR.
