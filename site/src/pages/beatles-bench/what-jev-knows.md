---
layout: "../../layouts/BenchPage.astro"
title: "Jev knows more than search"
tagline: null
slide: "/beatles-bench/img/what-jev-knows.png"
source: "https://github.com/botassembly/beatles-bench/blob/48ac3f252c6161bb9518416a41b4c8a2d82a9434/docs/what-jev-knows.md"
runsIn: null
---

How much does Jev know from memory? The bench asks 1,313 questions about the Beatles alone and scores four ways of answering them. Search that matches text or meaning sits near a guess. Jev knows about two answers in three. A big chat model knows nearly all of them, and it takes far longer.

Run every command here from the top folder of the bench. None of them sends a request.

## The ladder

```sh
awk -F '\t' '$2 == "beatles-only" && ($1 == "chance" || $1 == "Embeddings" || $1 == "Jev" || $1 == "GLM-5.3 Flash") { printf "%3.0f%% of %d  %s\n", $5 * 100, $3, $1 }' results/tables/accuracy.tsv | sort -n
```

```text
 31% of 1313  chance
 38% of 1313  Embeddings
 68% of 1313  Jev
 96% of 1313  GLM-5.3 Flash
```

- **chance** is a random guess among the options.
- **Embeddings** is vector search. It picks the option closest in meaning to the question. It knows no facts. [reports/baselines.md](https://github.com/botassembly/beatles-bench/blob/48ac3f252c6161bb9518416a41b4c8a2d82a9434/reports/baselines.md) shows three other kinds of search that did no better.
- **Jev** answers from memory through ThinkThen, with no context in the text.
- **GLM-5.3 Flash** is a large chat model from Z.ai. It answers from memory, with thinking off and no search tool.

## Time and money

```sh
awk -F '\t' '$1 == "Jev" || $1 == "GLM-5.3 Flash" { printf "%-14s %5.2f s  $%.3f per 1,000 answers\n", $1, $10, $8 }' results/tables/cost.tsv
```

```text
GLM-5.3 Flash   8.24 s  $0.056 per 1,000 answers
Jev             0.32 s  $0.015 per 1,000 answers
```

The time is the median time for one answer. Jev answers in about a third of a second. The big chat model takes about 8 seconds.

## One answer from memory

Jev knows the famous facts. Here it names the writer of Here Comes the Sun, replayed from the run of 2026-09-25:

```sh
jq -c 'select(.id == "forward-songwriter-026")' questions/forward.jsonl |
  thinkthen choose 'The text is the title of a song by the Beatles. Who is credited with writing it?' \
    --jsonl --field /input --options /options --details --replay results/runs/2026-09-25-thinkthen-jev/recording |
  jq -c '{song: .input.input, pick: .value, p: .answer.probabilities}'
```

```json
{"song":"Here Comes the Sun","pick":"harrison","p":{"lennon-mccartney":0.01,"harrison":0.99,"starr":0.0,"other":0.0}}
```

George Harrison wrote it, and Jev gives him 0.99.

## What it means

Jev is a classifier that carries world knowledge. It knows the broad facts. It misses fine detail, such as a year or a first album. [Jev has blind spots](/beatles-bench/blind-spots/) shows where. [Retrieval-augmented decisions (RAD)](/beatles-bench/rad/) shows what happens when the facts go in the text.

[reports/results.md](https://github.com/botassembly/beatles-bench/blob/48ac3f252c6161bb9518416a41b4c8a2d82a9434/reports/results.md) has every category and the full tables.
