---
layout: "../../layouts/BenchPage.astro"
title: "How to place a text on a scale with score"
tagline: "score rates using your scale."
slide: "/beatles-bench/img/score.png"
source: "https://github.com/botassembly/beatles-bench/blob/48ac3f252c6161bb9518416a41b4c8a2d82a9434/examples/04-score/README.md"
runsIn: "examples/04-score"
---

Use `score` when you want a number on a scale you name. You give the levels, lowest first. `score` returns the probability of each level and one number: the levels' positions weighted by those probabilities. Level 0 is the first level.

Here Jev guesses the length of eight recordings. The ten levels run from "about 0 minutes" to "about 9 minutes", one per whole minute. Level 3 is "about 3 minutes", so the score reads as minutes.

## The files

- `recording/` holds the saved requests and answers, so the command needs no key.
- `score-cold.jsonl` holds the cases, one song each, with the real length in minutes taken from `data/songs.tsv`.
- `score-context.jsonl` holds the same eight songs with each song's catalog entry.

The question and the levels sit on the command line, so this example needs no question file. The third line of `score-cold.jsonl` holds the case for Octopus's Garden:

```sh
sed -n 3p score-cold.jsonl
```

```text
{"id": "score-cold-03", "function": "score", "args": ["The text is the title of a song by the Beatles. How long is the recording?", "about 0 minutes", "about 1 minute", "about 2 minutes", "about 3 minutes", "about 4 minutes", "about 5 minutes", "about 6 minutes", "about 7 minutes", "about 8 minutes", "about 9 minutes", "--jsonl", "--field", "/input"], "records": [{"id": "score-cold-03", "input": "Octopus's Garden"}], "title": "Octopus's Garden", "truth": 2.85, "fields": [["songs.tsv", "Octopus's Garden", "length_s", "171"]]}
```

`truth` is the real length in minutes. `fields` gives its source: `length_s` in `data/songs.tsv`, 171 seconds. [How to run the bench for free](/beatles-bench/run-it-for-free/) explains every field of a case line.

Everything in the folder:

- `score-cold.jsonl`, `score-context.jsonl`: the cases, one song each, with the real length as `truth`, written by `scripts/generate/examples.py`.
- `recording/`, `outputs.jsonl`, `timing.tsv`: the requests, the committed answers, and the wall times.
- `run.sh live OUT | replay [OUT]`: asks the cases through `scripts/run/functions.sh`.

## Run it

Run this in `examples/04-score`:

```sh
printf '%s\n' 'Her Majesty' Yesterday "Octopus's Garden" Something 'Come Together' 'A Day in the Life' 'Hey Jude' 'Revolution 9' |
  thinkthen score 'The text is the title of a song by the Beatles. How long is the recording?' \
    'about 0 minutes' 'about 1 minute' 'about 2 minutes' 'about 3 minutes' 'about 4 minutes' \
    'about 5 minutes' 'about 6 minutes' 'about 7 minutes' 'about 8 minutes' 'about 9 minutes' \
    --lines --details --replay recording |
  jq -c '{song: .input, level: .answer.level, minutes: .value}'
```

```json
{"song":"Her Majesty","level":"about 0 minutes","minutes":0.67}
{"song":"Yesterday","level":"about 2 minutes","minutes":2.12}
{"song":"Octopus's Garden","level":"about 3 minutes","minutes":3.17}
{"song":"Something","level":"about 3 minutes","minutes":3.81}
{"song":"Come Together","level":"about 4 minutes","minutes":4.34}
{"song":"A Day in the Life","level":"about 5 minutes","minutes":4.868686868687}
{"song":"Hey Jude","level":"about 7 minutes","minutes":7.07}
{"song":"Revolution 9","level":"about 9 minutes","minutes":7.27}
```

- The words after the question are the levels, lowest first.
- `--lines` takes each line of input as one record.
- `--details` prints the whole result, with the likeliest level and the probability of every level. Without it, each line gives the song and its score.
- `--replay recording` answers from the saved recording. No request leaves the machine.

`level` is the likeliest level. `minutes` is the score.

To ask your own server, leave out `--replay recording` and follow [Ask your own server](/beatles-bench/run-it-for-free/#ask-your-own-server). The answers can then differ by a few points from these.

## Read it

The bench's `run.sh` sent the same requests and saved the answers in `outputs.jsonl`. This command sets each score beside the real length. It marks the likeliest level right when it names the whole minute nearest the real length:

```sh
jq -c --slurpfile out outputs.jsonl '
  ($out | map({(.id): .rows[0]}) | add) as $rows | $rows[.id] as $row
  | ($row.answer.level | capture("(?<n>[0-9]+)").n | tonumber) as $level
  | {song: .title, real: .truth, level: $row.answer.level, score: ($row.value * 100 | round / 100),
     off_by: (($row.value - .truth) | fabs * 100 | round / 100),
     mark: (if $level == (.truth | round) then "right" else "wrong" end)}
' score-cold.jsonl
```

```json
{"song":"Her Majesty","real":0.38,"level":"about 0 minutes","score":0.67,"off_by":0.29,"mark":"right"}
{"song":"Yesterday","real":2.08,"level":"about 2 minutes","score":2.12,"off_by":0.04,"mark":"right"}
{"song":"Octopus's Garden","real":2.85,"level":"about 3 minutes","score":3.17,"off_by":0.32,"mark":"right"}
{"song":"Something","real":3.03,"level":"about 3 minutes","score":3.81,"off_by":0.78,"mark":"right"}
{"song":"Come Together","real":4.32,"level":"about 4 minutes","score":4.34,"off_by":0.02,"mark":"right"}
{"song":"A Day in the Life","real":5.63,"level":"about 5 minutes","score":4.87,"off_by":0.76,"mark":"wrong"}
{"song":"Hey Jude","real":7.13,"level":"about 7 minutes","score":7.07,"off_by":0.06,"mark":"right"}
{"song":"Revolution 9","real":8.37,"level":"about 9 minutes","score":7.27,"off_by":1.1,"mark":"wrong"}
```

From memory, six of eight levels are right. Five scores land within half a minute of the real length, and seven within a minute.

Octopus's Garden is right. It runs 2.85 minutes, and Jev's likeliest level is "about 3 minutes". Its score of 3.17 is off by 0.32.

Revolution 9 is wrong. It runs 8.37 minutes, nearest to "about 8 minutes". Jev's likeliest level is "about 9 minutes", yet its score is 7.27. The level probabilities show why:

```sh
jq -c 'select(.id == "score-cold-08") | .rows[0].answer.probabilities' outputs.jsonl
```

```json
{"about 0 minutes":0.01,"about 1 minute":0.01,"about 2 minutes":0.02,"about 3 minutes":0.03,"about 4 minutes":0.06,"about 5 minutes":0.05,"about 6 minutes":0.05,"about 7 minutes":0.19,"about 8 minutes":0.21,"about 9 minutes":0.37}
```

"About 9 minutes" leads with only 0.37. The rest of the weight sits on shorter levels, and they pull the score down to 7.27.

A Day in the Life misses low. It runs 5.63 minutes, and Jev's likeliest level is "about 5 minutes".

## With context

The context run sends each song's catalog entry before its title. The entry gives the length, such as "2:51" for Octopus's Garden:

```sh
jq -r '.records[0].input' score-context.jsonl | grep -F "Octopus's Garden ("
```

```text
Octopus's Garden (lead: Starr; written: Starkey; 2:51; released 1969-09-26; first album: Abbey Road)
```

```sh
jq -c '.records[]' score-context.jsonl |
  thinkthen score 'The text gives a catalog entry and then names a song by the Beatles. How long is the recording?' \
    'about 0 minutes' 'about 1 minute' 'about 2 minutes' 'about 3 minutes' 'about 4 minutes' \
    'about 5 minutes' 'about 6 minutes' 'about 7 minutes' 'about 8 minutes' 'about 9 minutes' \
    --jsonl --field /input --details --replay recording |
  jq -c '{song: (.input.input | split("\nText: ") | last), minutes: .value}'
```

```json
{"song":"Her Majesty","minutes":0.2}
{"song":"Yesterday","minutes":2.0}
{"song":"Octopus's Garden","minutes":2.99}
{"song":"Something","minutes":3.0}
{"song":"Come Together","minutes":4.01}
{"song":"A Day in the Life","minutes":5.54}
{"song":"Hey Jude","minutes":7.01}
{"song":"Revolution 9","minutes":8.14}
```

- `--jsonl` takes each line as one JSON record.
- `--field /input` names the part of the record Jev reads.

All eight land within half a minute of the real length. The scale has whole minutes only, so the score rounds toward a level. Octopus's Garden runs 2.85 minutes and scores 2.99.

## What can go wrong

- **The shared traps.** A replay miss, a missing key, and a cache bound to another server can stop any command. [How to run the bench for free](/beatles-bench/run-it-for-free/#what-can-go-wrong) gives each exit code and its fix.
- **The score and the level can disagree.** The score is a weighted mean, and the level is the single likeliest one. Read the level when you want a bucket. Read the score when you want a number.
- **The scale limits the answer.** No score can pass the top level. A ten-minute song scores 9 at most here.

## The slide

Each row shows the real length and Jev's score in minutes. Jev's score is white, and the real length is muted. The dashed ring marks the real length. A dot marks the score. The dot is green when the score lands within half a minute of the real length, and red when it lands further off. Five dots are green. Something, A Day in the Life, and Revolution 9 are red.

## Related

- [How to sort records by a question with rank](/beatles-bench/rank/): an order in place of a number.
- [Retrieval-augmented decisions (RAD)](/beatles-bench/rad/): what the catalog entry fixes and what it costs.
