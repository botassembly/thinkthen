---
layout: "../../layouts/BenchPage.astro"
title: "The data"
tagline: null
slide: null
source: "https://github.com/botassembly/beatles-bench/blob/48ac3f252c6161bb9518416a41b4c8a2d82a9434/docs/the-data.md"
runsIn: null
---

Every right answer in the bench comes from one table, `data/songs.tsv`. It holds 306 released Beatles songs, one row each. A script harvested it from English Wikipedia at pinned revisions, Wikidata, and the Wikimedia Pageviews API for 2024. No person and no language model chose a row or an answer.

Run every command here from the top folder of the bench.

## One row

Here is the row for Octopus's Garden, one column per line:

```sh
awk -F '\t' 'NR == 1 { split($0, head) } $1 == "Octopus\047s Garden" { for (i = 1; i <= NF; i++) print head[i] "=" $i }' data/songs.tsv
```

```text
title=Octopus's Garden
article=Octopus's Garden
year=1969
first_release=Abbey Road
first_album=Abbey Road
songwriters=Starkey
lead_vocals=Starr
with_vocals=
article_lead=Starr
length_s=171
release_date=1969-09-26
date_from=Abbey Road
cover=no
views_2024=103246
catalogue=core 1962-1970
```

The columns the worked examples use:

- `first_album` is the first album or release the list of songs names. A song first out on a single can name Past Masters, the album that collected the singles.
- `lead_vocals` names the lead singers from the list of songs. `article_lead` names them from the song's own article.
- `songwriters` gives the credit. Ringo Starr's credit reads Starkey, his birth name.
- `length_s` is the length in seconds.
- `year` and `release_date` give the first release.
- `views_2024` counts the song article's page views in 2024.

[data/README.md](https://github.com/botassembly/beatles-bench/blob/48ac3f252c6161bb9518416a41b4c8a2d82a9434/data/README.md) describes every file, its source, and its known limits. [data/SOURCES.md](https://github.com/botassembly/beatles-bench/blob/48ac3f252c6161bb9518416a41b4c8a2d82a9434/data/SOURCES.md) states every rule the harvest applies.

## From a row to a question

A script writes each question from a template, and the row fills its slots. Here is one question about Octopus's Garden:

```sh
jq -c 'select(.id == "forward-year-030") | {id, function, question, input, options: (.options | keys), truth, fields}' questions/forward.jsonl
```

```json
{"id":"forward-year-030","function":"choose","question":"The text is the title of a song by the Beatles. In what year was it first released?","input":"Octopus's Garden","options":["1962","1963","1964","1965","1966","1967","1968","1969","1970"],"truth":"1969","fields":[["songs.tsv","Octopus's Garden","year","1969"]]}
```

- `function` names the ThinkThen command that answers it.
- `question` is the text Jev reads. `input` is the text the question is about.
- `options` are the choices.
- `truth` is the right answer. `fields` names the file, row, column, and value it came from.

[questions/README.md](https://github.com/botassembly/beatles-bench/blob/48ac3f252c6161bb9518416a41b4c8a2d82a9434/questions/README.md) lists the 15 categories and how each truth is set.

## How Jev answered it

The fresh run of 2026-09-25 recorded Jev's answer to every question. This command asks the question again from that recording, with no key:

```sh
jq -c 'select(.id == "forward-year-030")' questions/forward.jsonl |
  thinkthen choose 'The text is the title of a song by the Beatles. In what year was it first released?' \
    --jsonl --field /input --options /options --details --replay results/runs/2026-09-25-thinkthen-jev/recording |
  jq -c '{song: .input.input, pick: .value, p: .answer.probabilities}'
```

```json
{"song":"Octopus's Garden","pick":"1967","p":{"1962":0.0,"1963":0.0,"1964":0.0,"1965":0.01,"1966":0.15,"1967":0.46,"1968":0.27,"1969":0.11,"1970":0.0}}
```

- `--jsonl` takes the question line as one record.
- `--field /input` sends only the song title. The rest of the record stays on the machine.
- `--options /options` reads the options from the record.

Jev picks 1967 at 0.46, and it is wrong. The right year, 1969, gets 0.11. Jev gets the singer of Octopus's Garden right on [the choose page](/beatles-bench/choose/). It misses the year here and the album on [the annotate page](/beatles-bench/annotate/).

## The catalog

The context runs send a catalog entry built from the same row. `scripts/run/catalog.py` writes every song as one line under its first album:

```sh
grep -F "Octopus's Garden (" results/runs/2026-09-25-thinkthen-jev-open-book/catalog.txt
```

```text
Octopus's Garden (lead: Starr; written: Starkey; 2:51; released 1969-09-26)
```

The line carries `lead_vocals`, `songwriters`, `length_s` as minutes and seconds, and `release_date`. The album header above it, "Abbey Road (1969-09-26)", gives `first_album`. The worked examples send one song's header and line, and they add "first album:" to the line. [Retrieval-augmented decisions (RAD)](/beatles-bench/rad/) shows one.

## Known limits

- The truth is Wikipedia's at the pinned revisions, and Wikidata's on 2026-09-23.
- A release date is the earliest date in the infobox. Sometimes that is the US date.
- A model trained after this repository is public may have seen the facts and the questions.
