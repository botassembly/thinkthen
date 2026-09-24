# A measurement tier for ThinkThen: audit and diff

Status: Open

Ian asked on 2026-09-23, after the second eleventh-function sweep and the plain-English explanation: file audit and diff as one product-enhancement option to be judged. Both are utility capabilities, not judgment functions. Neither calls Jev. Neither touches the ten.

## The shared case

Every buyer asks one thing the ten functions cannot answer: "how do I know it is right on my data?" The threshold how-to (ticket 0087) and Beatles Bench answer it today, but as a hand-built experiment. The proof loop — run a question on labeled records, see the agreement, change the wording, see whether it improved — keeps being rebuilt because no command owns it. A measurement tier would own it, the way `status` owns settings and `cache` owns answers. Local math over recordings and answer keys. No request, no key, no spend.

## Option A: `thinkthen audit`

Grade a run against an answer key.

- **Input:** recorded answers (a recording or cache folder, or a fresh run's results) plus an answer key: the same records with the correct answers, from a person or a held-out label file.
- **Output:** agreement overall and per question (a question set grades each question separately); the disagreement table both directions (said yes when the key said no, and the reverse), the costly direction first by the page's own harm ordering; and a probability readout — when the tool said 90 percent, how often it was right — so a threshold can be picked from evidence instead of a guess.
- **Never calls Jev.** Same tier as `cache` and `status`.
- **Ancestry:** the threshold how-to's `jq` sweep over a replay is a small audit; the "check the judge" how-to is one by hand; Beatles Bench (22 runs, answer key, scored output) is one at full size. The pattern is proven three times; this names it.
- **Release value:** closes the loop the marketing story opens. "Measure your question before you trust it" becomes one command, and the site's trust page links to something runnable.

## Option B: `thinkthen diff`

Show what a wording change actually changed.

- **Input:** two recordings (or two result folders) of the same records under two wordings of a question.
- **Output:** which records changed answers, which way they moved, the probability shift per record, and the totals (X of N changed, Y flipped yes-to-no). Paired with audit on both sides, it answers "is the new wording better, and exactly where did it differ?"
- **Never calls Jev.**
- **Ancestry:** Beatles Bench R22 replayed R21's requests through `annotate` — a hand-built diff of two shapes. Same lesson, named.

## Judged together

Audit and diff are two halves of one loop: audit says which question is better; diff says what changed. If only one is built, audit carries more of the buyer's question alone. If both are built, tuning a question file becomes: replay under wording one, replay under wording two, diff, audit, commit the winner — all free, all offline, all reproducible from committed recordings.

## Suggested shape, if ruled in

- Subcommands beside `status` and `cache`: `thinkthen audit <results> <key>`, `thinkthen diff <a> <b>`.
- Output is data (JSONL for pipelines, a readable table for the person), consistent with how value commands print.
- No new public words beyond the two names; the vocabulary page gains one row each if ruled in.
- Tests: the gate ladder runs both against committed fixtures with known answers, and the Beatles recordings are the first real corpus.

## What Ian can overturn

All of it: both, one, or neither; the names; the placement as subcommands; and the timing (before or after 0.1 — this issue does not assume either).

## Ruled 2026-09-23 (product owner, marketing side)

Both, after 0.1. Beatles Bench already does both by hand: its `score/` computes agreement with intervals, calibration, the accuracy-by-coverage curve, and a cut tuned on a held-out half, and its paired test is a diff between two runs. The leaning-no report (`botassembly/beatles-bench`, `reports/leaning-no.md`) is ten diffs and one audit. That evidence settles the need.

Two open issues fold into this one and close with it:

- `2026-09-23-show-what-changes-when-the-cut-moves.md` becomes `diff` across two cuts on one run. The cache makes it free.
- Its added accuracy-by-coverage report becomes part of `audit`, which also suggests a cut tuned on labeled cases and checked on a held-out part. The Beatles tests found that a tuned cut beat every wording change.

`audit` reports, per question: agreement with a 95% interval, both disagreement directions, the yes/no AUC, calibration, accuracy at each coverage level, and a suggested cut. `diff` takes two runs over the same records, from two wordings or two cuts, and lists what flipped and which way. The Beatles recordings are the first corpus, and Beatles Bench switches its decide and choose scoring to `thinkthen audit` once it ships. Ian can overturn the ruling.

## Prototype first (Ian, 2026-09-23)

Both commands are prototyped in Beatles Bench under `tools/`, in plain Python, reading ThinkThen's own JSONL output and recordings plus an answer key. The prototype defines the behavior. Its fixtures, including a reproduction of the leaning-no report's held-out numbers, become the golden tests the Rust `thinkthen audit` and `thinkthen diff` must match in the release after 0.1.
