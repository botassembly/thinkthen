---
layout: "../../layouts/BenchPage.astro"
title: "Beatles Bench"
tagline: "Beatles Bench is our playground."
slide: "/beatles-bench/img/beatles-bench.png"
source: "https://github.com/botassembly/beatles-bench/blob/48ac3f252c6161bb9518416a41b4c8a2d82a9434/docs/beatles-bench.md"
runsIn: null
---

Beatles Bench asks a small, fast model what it knows about Beatles songs. Every question has a known answer, so you can tell a right answer from a wrong one. The ThinkThen worked examples all run on it.

It is not an official benchmark. Please don't benchmax it. It is a playground. Use it to see how each ThinkThen function answers, where it goes wrong, and what fixes it.

Run every command here from the top folder of the bench.

## Every answer is already known

A script harvested the songs from Wikipedia and Wikidata. It wrote every question from a template, and each song's row fills the slots. No person and no language model chose an answer.

```sh
awk -F '\t' 'FNR > 1 { n[FILENAME]++ } END { print n["data/songs.tsv"], "songs,", n["data/albums.tsv"], "albums" }' data/songs.tsv data/albums.tsv
cat questions/*.jsonl | awk 'END { print NR, "questions" }'
```

```text
306 songs, 20 albums
1501 questions
```

Six of the rows, with the first album and the lead singer:

```sh
awk -F '\t' '$1 ~ /^(Love Me Do|She Loves You|Yesterday|Octopus.s Garden|Here Comes the Sun|Hey Jude)$/ { printf "%-20s %-18s %s\n", $1, $5, $7 }' data/songs.tsv
```

```text
Here Comes the Sun   Abbey Road         Harrison
Hey Jude             Past Masters       McCartney
Love Me Do           Please Please Me   McCartney+Lennon
Octopus's Garden     Abbey Road         Starr
She Loves You        Past Masters       Lennon+McCartney
Yesterday            Help!              McCartney
```

[The data](/beatles-bench/the-data/) follows one row from the table to a question and to Jev's answer.

## Run it for free

Every answer on these pages comes from a saved recording of a real call. A replay needs no key, no network, and no spend.

```sh
git clone https://github.com/botassembly/beatles-bench
cd beatles-bench
./run.sh
```

This block prints no output to check here. [How to run the bench for free](/beatles-bench/run-it-for-free/) shows what it prints.

## What the pages cover

- **Start.** [How to run the bench for free](/beatles-bench/run-it-for-free/) and [The data](/beatles-bench/the-data/).
- **The ten functions.** One page each, from [decide](/beatles-bench/decide/) to [relate](/beatles-bench/relate/). Each shows how Jev answers, where it goes wrong, and whether context helps.
- **Tune your bar.** [audit](/beatles-bench/audit/) finds the bar. [diff](/beatles-bench/diff/) shows what changed between two runs.
- **What Jev knows.** [Jev knows more than search](/beatles-bench/what-jev-knows/), [Jev has blind spots](/beatles-bench/blind-spots/), and [Retrieval-augmented decisions (RAD)](/beatles-bench/rad/).

## Who built it

The author of this bench builds ThinkThen. Weigh the Jev results with that in mind.

The code is MIT. The data and questions carry Wikipedia's CC BY-SA license, because they come from Wikipedia. Wikidata facts are CC0. [data/README.md](https://github.com/botassembly/beatles-bench/blob/48ac3f252c6161bb9518416a41b4c8a2d82a9434/data/README.md) describes each file and its limits.
