# Accuracy on three public sets, and a speed rerun inside the documented limit

Status: Closed. Ticket 0044 fixed the `cost.jq` defect in `ce96895b`. The transform now reads the id only when `input` is an object. The default-width question stays open for ticket 0077.

Ian gave a standing go-ahead on 2026-09-20 for paid marketing measurements up to one US dollar. `2026-09-20-launch-gaps-found-in-marketing-prep.md` records it. This is the first round under it. A builder ran it as experiment 206 in the workspace, at `experiments/206-thinkthen-accuracy/`. `PREREGISTRATION.md` there fixed the samples, the question wording, the options, and a hash of each labels file before the first paid call. No wording was tuned against a test sample, and every arm that ran is reported. `RESULTS.md` there is the long form. That folder can rot. This page is the record.

Every dataset here is old and public, so the model may have seen it in training. Say so wherever a number is printed. One model answered everything: `jev-1.13.0` behind `jev-latest`.

## The spend

| Figure | Value |
| --- | --- |
| Ledger before and after, read by command | 1,949,118 and 4,669,118 charged tokens |
| Declared across four reservations | 2,720,000 tokens, 11.4 US cents |
| Billed by the responses | 2,537,644 input tokens, 10.7 US cents |
| Left under the one-dollar go-ahead | About 89 US cents |

## M1. `decide` on SMS spam

One thousand messages drawn at random from the UCI SMS Spam Collection, CC BY 4.0. The natural spam rate was kept: 136 spam in 1,000. Arm 1 asked the bare question `The message is spam.` Arm 2 added one `--true` and one `--false` sentence, written once before the run.

| Figure at a 0.5 cut | Arm 1, bare | Arm 2, criteria |
| --- | --- | --- |
| Accuracy | 0.968 | 0.980 |
| Precision | 0.842 | 0.892 |
| Recall | 0.941 | 0.971 |
| F1 | 0.889 | 0.930 |
| True positive, false positive, true negative, false negative | 128, 24, 840, 8 | 132, 16, 848, 4 |
| Brier score | 0.0323 | 0.0201 |
| Billed input tokens and cost | 289,260 and 1.2 US cents | 374,433 and 1.6 US cents |

A message called ham every time would score 0.864. The sweep's best cut was 0.55 for arm 1 and 0.75 for arm 2. Both were picked on the test rows, so both are optimistic.

Calibration, from the repository's own `calibration` transform. The top bucket is honest: arm 1 predicted 0.95 on 97 messages and 0.96 were spam. The bottom is honest too: 570 messages near 0.05, and 1 was spam. The model is cautious in the low middle: in the 0.4 to 0.5 bucket it predicted 0.45 and 0.20 were spam. The buckets from 0.3 to 0.9 hold 4 to 29 rows each, too few to judge.

**Limits.** One sample, one 2011 corpus, one wording per arm. The gap between the arms rests on 8 messages changing side.

## M2. `decide` on BoolQ

Five hundred records from the validation split, CC BY-SA 3.0. Each record brings its own question, so the fixed question was `The answer to the question is yes.` and the evidence came from two pointers, `--field /passage --field /question`.

| Figure at a 0.5 cut | Value |
| --- | --- |
| Accuracy | 0.886 |
| Precision, recall, F1 | 0.918, 0.904, 0.911 |
| True positive, false positive, true negative, false negative | 291, 26, 152, 31 |
| Base rate of yes | 0.644 |
| Brier score | 0.0782 |
| Billed input tokens and cost | 208,104 and 0.9 US cents |

This is reading comprehension and a harder job than the tool is sold for. The calibration runs the other way here: in the 0.1 to 0.4 buckets the true rate of yes was higher than predicted. The documented download address for BoolQ answered 403, so the builder used the Hugging Face mirror, and `SOURCES.md` says so.

**Limits.** One sample of 500. One fixed question that wraps the record's own question.

## M3. `choose` among 77 banking intents

Seven hundred seventy messages from the Banking77 test split, CC BY 4.0, ten per intent. The options were the 77 bare intent names with underscores turned into spaces, given in a question file. No option had a description.

| Figure | Value |
| --- | --- |
| Top-1 accuracy | 0.7805 |
| Unresolved | 1, a tie |
| Mean winning probability when right, and when wrong | 0.925 and 0.734 |
| Tokens per record | 949.8 |
| Billed input tokens and cost | 731,382 and 3.1 US cents |

The errors are near-synonyms: `order physical card` picked as `get physical card`, and `pending transfer` picked as `transfer timing`. The tagging probe showed that a description per label cut false positives by more than half. Option descriptions are the obvious next arm, and it is unchecked. The gap between 0.925 and 0.734 means a threshold would send many of the wrong picks to "not sure".

**Limits.** One arm. Bare names that a bank wrote for its own use.

## M4. The speed rerun inside the documented limit

The same 3,000 lines of Pride and Prejudice and the same question as the first spike, with no cache and no replay, at a width of 3.

| Figure | Value |
| --- | --- |
| Requests | 3,000, no failures |
| Wall clock | 183.6 seconds |
| Measured rate | 980 requests a minute, against a documented 1,200 |
| Billed input tokens and cost | 870,866 and 3.7 US cents |

The bill matches the first run. Staying inside the limit costs time and no money.

## Published numbers to set beside ours

A survey agent read these from the papers on 2026-09-20. Both less-known papers exist on arXiv, checked by fetch. The table cells were not checked a second time, so check the table in the paper before any number is printed. Every published row is zero-shot unless it says otherwise. Ours are samples, and most published rows use a full split.

| Set | Who | Number | Source |
| --- | --- | --- | --- |
| SMS spam | ThinkThen on Jev, criteria arm | Accuracy 0.980, F1 0.930 | This page |
| SMS spam | GPT-4 | Accuracy 0.973, F1 0.947 | Wang, Pang, and Lin, "Large Language Models Are Zero-Shot Text Classifiers", arXiv 2312.01044, table VI |
| SMS spam | GPT-3.5 | Accuracy 0.873, F1 0.800 | The same table |
| SMS spam | Llama 2 | Accuracy 0.727, F1 0.444 | The same table |
| BoolQ | ThinkThen on Jev | 0.886 | This page |
| BoolQ | PaLM 540B | 0.880 | The PaLM paper, arXiv 2204.02311, table 4 |
| BoolQ | ChatGPT | 0.873 | "Is ChatGPT a General-Purpose NLP Task Solver?", arXiv 2302.06476, table 6 |
| BoolQ | Llama 2 70B | 0.850 | The Llama 2 paper, arXiv 2307.09288, table 20 |
| BoolQ | T5-11B, fine-tuned | 0.910 | The GPT-3 paper, arXiv 2005.14165, table 3.8 |
| BoolQ | Always yes | 0.62 | The BoolQ paper, arXiv 1905.10044 |
| Banking77 | ThinkThen on Jev, bare names | 0.781 | This page |
| Banking77 | The best of 41 open-weight models, Mistral-7B-Instruct-v0.3 | 0.708 | Ganesh, Dozier, and Seals, arXiv 2607.27421 |
| Banking77 | The mean of those 41 models | 0.388 | The same paper |
| Banking77 | Fine-tuned on the full training set | 0.934 to 0.937 | Casanueva and others, 2020, table 3 |

What this shows. With no training, the small model sits beside the largest general models on all three sets, for about a cent per thousand short records. A model fine-tuned on the task still wins on BoolQ and wins clearly on Banking77. No zero-shot GPT-3 number for BoolQ exists in its paper, so never print one.

## The default width passes the documented limit on short records

`--jobs` defaults to 4. From this machine a width of 4 measured 1,267, 1,319, and 1,272 requests a minute on three separate 200-record checks. A width of 3 measured between 972 and 1,017 on five. The vendor documents 1,200. So a new user who never touches `--jobs` runs about 6 to 10 percent over the documented limit on short records. Longer records answer more slowly and stay under it. The service refused nothing, here or at 4,300 a minute in the first spike.

My recommendation stands from the first findings page: no pacer, and a test that replays a 429 in the middle of a wide run. The reference page for `--jobs` should give these measured numbers. Whether the default drops to 3 is the builders' call, and I would leave it at 4 until the vendor refuses a request.

## One defect

`transforms/cost/cost.jq` stops on any `--lines` run. A `--details` row from `--lines` carries a string in `input`, and the transform reads `$row.input.id`. Reproduced by command on 2026-09-20 with one made-up text row: `Cannot index string with string "id"`. The same row with an object for `input` works. The fix reads the id only when `input` is an object. The transform's example should carry a text record. Rows 15 to 17 of the stumble register hold this and two smaller stumbles from the same run.

## What marketing may quote

| Claim | Number | Rule |
| --- | --- | --- |
| Spam detection with no training | 98.0 percent of 1,000 SMS messages, one question and two criteria sentences, 1.6 US cents | Name the dataset and the size. Say the set is public and old |
| The plain-question version | 96.8 percent, 1.2 US cents | The same rule. Print both arms or neither |
| Reading comprehension | 88.6 percent on 500 BoolQ questions for under one US cent. PaLM 540B scored 88.0 zero-shot | Print with the cited baseline and the words "a sample of 500". Check the paper's table first |
| One of 77 intents, bare names | 78 percent top-1. The best of 41 open-weight models scored 70.8 zero-shot, and a fine-tuned model scores about 93 | Print all three numbers together or none. It shows where a trained model still wins. Check the paper's table first |
| Speed inside the vendor's limit | 3,000 lines in about three minutes, at about 980 requests a minute | Quotable now. It replaces the 41-second figure |
| Calibration | "When it said 95 percent, it was right 96 percent of the time, on 97 messages" | Quotable with the count. Never say "calibrated" with no number |

## What Ian can overturn

The quoting rules and the recommendation on the default width.
