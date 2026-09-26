---
layout: "../../layouts/BenchPage.astro"
title: "Retrieval-augmented decisions (RAD)"
tagline: null
slide: "/beatles-bench/img/rad.png"
source: "https://github.com/botassembly/beatles-bench/blob/48ac3f252c6161bb9518416a41b4c8a2d82a9434/docs/rad.md"
runsIn: null
---

Giving Jev context improves accuracy.

Your own data sits in no model's memory. So put the context in the text, and then ask Jev to decide. We call that retrieval-augmented decisions, or RAD. On the same 196 questions, Jev from memory got 39% right. With the song catalog in the text, it got 95%.

Run every command here from the top folder of the bench. None of them sends a request.

## The context

The catalog lists every song in `data/songs.tsv` as one line under its first album. Each line gives the lead singer, the writers, the length, and the first release date:

```sh
grep -F -e "Hold Me Tight (" -e "Ticket to Ride (" results/runs/2026-09-25-thinkthen-jev-open-book/catalog.txt
```

```text
Hold Me Tight (lead: McCartney; written: Lennon–McCartney; 2:32; released 1963-11-22)
Ticket to Ride (lead: Lennon; written: Lennon–McCartney; 3:09; released 1965-04-09)
```

The open-book run sends "Catalog:", the whole catalog, and then "Text:" with the question's own input. The question stays the same.

## One question, two ways

Which is longer, Hold Me Tight or Ticket to Ride? From memory, Jev picks the wrong one:

```sh
jq -c 'select(.id == "comparison-longer-016")' questions/comparison.jsonl |
  thinkthen choose 'The text names two songs by the Beatles. Which one is longer?' \
    --jsonl --field /input --options /options --details --replay results/runs/2026-09-25-thinkthen-jev/recording |
  jq -c '.input.options as $o | {songs: .input.input, pick: $o[.value], p: (.answer.probabilities | with_entries(.key |= $o[.]))}'
```

```json
{"songs":"Hold Me Tight / Ticket to Ride","pick":"Hold Me Tight","p":{"Hold Me Tight":0.68,"Ticket to Ride":0.32}}
```

With the catalog in the text, it reads the two lengths and picks Ticket to Ride:

```sh
jq -c --rawfile catalog results/runs/2026-09-25-thinkthen-jev-open-book/catalog.txt \
    'select(.id == "comparison-longer-016") | .input = "Catalog:\n" + $catalog + "\nText: " + .input' questions/comparison.jsonl |
  thinkthen choose 'The text names two songs by the Beatles. Which one is longer?' \
    --jsonl --field /input --options /options --details --replay results/runs/2026-09-25-thinkthen-jev-open-book/recording |
  jq -c '.input.options as $o | {songs: (.input.input | split("\nText: ") | last), pick: $o[.value], p: (.answer.probabilities | with_entries(.key |= $o[.]))}'
```

```json
{"songs":"Hold Me Tight / Ticket to Ride","pick":"Ticket to Ride","p":{"Hold Me Tight":0.01,"Ticket to Ride":0.99}}
```

This is the first question in the run's `ids.txt` that memory got wrong and the catalog got right.

## The count

The open-book run asks 196 bench questions, drawn mostly from the questions Jev missed from memory on 2026-09-23. This command compares the answers from memory with the answers from the catalog on those 196:

```sh
python3 scripts/score/open_book.py compare results/runs/2026-09-25-thinkthen-jev results/runs/2026-09-25-thinkthen-jev-open-book | head -9
```

```text
| Topic | n | closed right | open right | misses fixed | hits broken |
| --- | --- | --- | --- | --- | --- |
| first album | 62 | 27 (44%) | 60 (97%) | 33 of 35 | 0 of 27 |
| dates | 56 | 15 (27%) | 55 (98%) | 40 of 41 | 0 of 15 |
| lead singer | 45 | 19 (42%) | 42 (93%) | 24 of 26 | 1 of 19 |
| songwriter | 16 | 7 (44%) | 13 (81%) | 6 of 9 | 0 of 7 |
| song length | 8 | 5 (62%) | 8 (100%) | 3 of 3 | 0 of 5 |
| word traps | 9 | 3 (33%) | 8 (89%) | 5 of 6 | 0 of 3 |
| all | 196 | 76 (39%) | 186 (95%) | 111 of 120 | 1 of 76 |
```

"Closed" means from memory. "Open" means with the catalog. The catalog fixes 111 misses and breaks 1 right answer. The sample holds mostly misses, so the share from memory sits far below Jev's share on the whole bench. [reports/open-book.md](https://github.com/botassembly/beatles-bench/blob/48ac3f252c6161bb9518416a41b4c8a2d82a9434/reports/open-book.md) has the full report.

## What it costs

Jev costs $42 per billion input tokens, and output is free. The whole catalog is large, and each open-book question carries it. This command adds up the input tokens of each run and prices them at 0.042 dollars per million:

```sh
for run in 2026-09-25-thinkthen-jev 2026-09-25-thinkthen-jev-open-book; do
  jq -sc --arg run "$run" '(map(.input_tokens) | add) as $t
    | {run: $run, answers: length, input_tokens: $t, tokens_per_answer: ($t / length | round),
       dollars_per_1000_answers: ($t / length * 0.042 | round / 1000)}' "results/runs/$run/answers.jsonl"
done
```

```json
{"run":"2026-09-25-thinkthen-jev","answers":1501,"input_tokens":534901,"tokens_per_answer":356,"dollars_per_1000_answers":0.015}
{"run":"2026-09-25-thinkthen-jev-open-book","answers":196,"input_tokens":2394007,"tokens_per_answer":12214,"dollars_per_1000_answers":0.513}
```

From memory, 1,000 answers cost about 0.015 dollars. With the whole catalog, they cost about 0.513 dollars.

- **The cache.** A repeated request comes from the answer cache and costs nothing. Changing the threshold asks nothing new. [thinkthen.dev/trust](/trust/) says more.
- **The caps.** A database or a library can cap the requests it sends. [thinkthen.dev/trust](/trust/) says more.

## Send only what you need

The whole catalog is the simplest context and the most costly. Two ways cut the cost:

- **Send only the entry you need.** The worked examples send one song's entry, tens of tokens. That works when you know which song the question is about.
- **Let Jev pick the sections.** [reports/rad.md](https://github.com/botassembly/beatles-bench/blob/48ac3f252c6161bb9518416a41b4c8a2d82a9434/reports/rad.md) has Jev choose which parts of the catalog to read, then answer from those parts alone.

Seven worked examples ask the same cases twice, from memory and with one song's entry. [`docs/context.jq`](https://github.com/botassembly/beatles-bench/blob/48ac3f252c6161bb9518416a41b4c8a2d82a9434/docs/context.jq) marks each answer against its case's `truth` and adds up the input tokens of each side:

```sh
for ex in 01-decide 02-choose 04-score 05-filter 07-find 08-annotate 11-audit; do
  cat examples/$ex/*-cold.jsonl examples/$ex/*-context.jsonl |
    jq -sc --arg ex "$ex" --slurpfile out "examples/$ex/outputs.jsonl" -f docs/context.jq
done
```

```json
{"example":"01-decide","cold":{"right":5,"of":6,"input_tokens":1729},"context":{"right":6,"of":6,"input_tokens":2101}}
{"example":"02-choose","cold":{"right":4,"of":5,"input_tokens":1632},"context":{"right":5,"of":5,"input_tokens":1929}}
{"example":"04-score","cold":{"right":6,"of":8,"input_tokens":3063},"context":{"right":8,"of":8,"input_tokens":3567}}
{"example":"05-filter","cold":{"right":11,"of":12,"input_tokens":3469},"context":{"right":12,"of":12,"input_tokens":4214}}
{"example":"07-find","cold":{"right":1,"of":1,"input_tokens":537},"context":{"right":1,"of":1,"input_tokens":1179}}
{"example":"08-annotate","cold":{"right":4,"of":6,"input_tokens":1158},"context":{"right":6,"of":6,"input_tokens":1303}}
{"example":"11-audit","cold":{"right":50,"of":70,"input_tokens":19543},"context":{"right":70,"of":70,"input_tokens":24871}}
```

With one entry, every answer is right in every example. In the audit example, 70 questions went from 19,543 input tokens to 24,871.

## What this page leaves out

- **A whole article.** [examples/context-article/](https://github.com/botassembly/beatles-bench/blob/48ac3f252c6161bb9518416a41b4c8a2d82a9434/examples/context-article/README.md) hands Jev a whole Wikipedia article. Its runs keep no recording, because a recording would store Wikipedia text. It does not replay, and this page draws no number from it.
- **relate with the catalog.** The whole catalog beside a set of names passes Jev's input limit. [The relate page](/beatles-bench/relate/) says more.
