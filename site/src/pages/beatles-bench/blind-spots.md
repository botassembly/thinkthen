---
layout: "../../layouts/BenchPage.astro"
title: "Jev has blind spots"
tagline: null
slide: "/beatles-bench/img/blind-spots.png"
source: "https://github.com/botassembly/beatles-bench/blob/48ac3f252c6161bb9518416a41b4c8a2d82a9434/docs/blind-spots.md"
runsIn: null
---

Where does Jev's memory fail? Three kinds of question pull its score down: songs few people read about, questions with a word trap, and questions that chain two facts. This page shows each share and one question of each kind, replayed from the run of 2026-09-25.

Run every command here from the top folder of the bench. None of them sends a request.

## Obscure songs

The bench splits the songs into quarters by their Wikipedia page views in 2024. Jev knows the famous songs better:

```sh
awk -F '\t' '$1 == "Jev" && ($2 == 1 || $2 == 4) { printf "%-22s %3.0f%% of %d\n", ($2 == 1 ? "least viewed quarter" : "most viewed quarter"), $8 * 100, $6 }' results/tables/popularity.tsv
```

```text
least viewed quarter    51% of 256
most viewed quarter     74% of 259
```

Nothin' Shakin' is a cover from the least viewed quarter. George Harrison sings it. Jev picks John Lennon:

```sh
jq -c 'select(.id == "forward-singer-033")' questions/forward.jsonl |
  thinkthen choose 'The text is the title of a song by the Beatles. Who sings the lead vocal on it?' \
    --jsonl --field /input --options /options --details --replay results/runs/2026-09-25-thinkthen-jev/recording |
  jq -c '{song: .input.input, pick: .input.options[.value], p: .answer.probabilities}'
```

```json
{"song":"Nothin' Shakin'","pick":"John Lennon","p":{"john":0.38,"paul":0.21,"george":0.13,"ringo":0.28}}
```

## Tricky wording

A word trap puts a word from the question into a wrong option. The control asks the same kind of question with no trap:

```sh
awk -F '\t' '$1 == "Jev" && $2 == "lexical-trap" { printf "plain  %3.0f%% of %d\ntrap   %3.0f%% of %d\n", $6 / $4 * 100, $4, $5 / $4 * 100, $4 }' results/tables/controls.tsv
```

```text
plain   67% of 54
trap    48% of 54
```

The album Yellow Submarine first included It's All Too Much. Jev picks the song that shares the album's name:

```sh
jq -c 'select(.id == "lexical-trap-album-to-song-001")' questions/lexical-trap.jsonl |
  thinkthen choose 'The text names an album by the Beatles. Which of these songs did it include before any other album did?' \
    --jsonl --field /input --options /options --details --replay results/runs/2026-09-25-thinkthen-jev/recording |
  jq -c '.input.options as $o | {album: .input.input, pick: $o[.value], p: (.answer.probabilities | with_entries(.key |= $o[.]))}'
```

```json
{"album":"Yellow Submarine","pick":"Yellow Submarine","p":{"Not a Second Time":0.05,"Yellow Submarine":0.58,"Anna (Go to Him)":0.05,"It's All Too Much":0.32}}
```

The song Yellow Submarine first came out on Revolver.

## Two facts in one question

A same-month question asks whether a song came out in the same month as a world event. Jev must know both dates. The bench keeps only the questions where Jev got both single facts right on their own:

```sh
awk -F '\t' '$1 == "Jev" && $2 == "same-month" { print "both facts known:", $5; print "chained right:   ", $5 - $6 }' results/tables/composition.tsv
```

```text
both facts known: 27
chained right:    14
```

Jev knows both facts for 27 questions. It chains them right on 14. The Fool on the Hill and the Apollo 4 launch both came in November 1967. Jev says no:

```sh
jq -c 'select(.id == "multi-hop-same-month-001")' questions/multi-hop.jsonl |
  thinkthen decide 'The text is the title of a song by the Beatles. Was it first released in the same month as this event: Apollo 4?' \
    --jsonl --field /input --details --replay results/runs/2026-09-25-thinkthen-jev/recording |
  jq -c '{song: .input.input, value, yes: .answer.probability}'
```

```json
{"song":"The Fool on the Hill","value":false,"yes":0.39}
```

Ask one fact per question, and chain the answers in code.

## A bar fits one model

Jev leans toward no on yes or no questions. [reports/leaning-no.md](https://github.com/botassembly/beatles-bench/blob/48ac3f252c6161bb9518416a41b4c8a2d82a9434/reports/leaning-no.md) tuned a bar for that lean. A lower bar caught about twice the true yes answers, with no new calls. That bar fits Jev. Another model leans its own way, so tune its bar on its own answers. [How to find your bar with audit](/beatles-bench/audit/) shows how.

## What fixes it

Put the facts in the text. [Retrieval-augmented decisions (RAD)](/beatles-bench/rad/) shows what the song catalog fixes, and what it costs.
