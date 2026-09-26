---
layout: "../../layouts/BenchPage.astro"
title: "How to keep the records that match with filter"
tagline: "filter keeps what clears your bar."
slide: "/beatles-bench/img/filter.png"
source: "https://github.com/botassembly/beatles-bench/blob/48ac3f252c6161bb9518416a41b4c8a2d82a9434/examples/05-filter/README.md"
runsIn: "examples/05-filter"
---

Use `filter` when you want to keep only the records where the answer is yes. It asks one yes or no question of each record. It keeps the records whose probability of yes reaches the bar, in the order they came.

Here Jev keeps the songs on the album Abbey Road from a list of twelve. Six of the twelve are on it. The bar is 0.7.

## The files

- `recording/` holds the saved requests and answers, so the command needs no key.
- `filter-cold.jsonl` holds the cases, one song each, with the right answer taken from `data/songs.tsv`.
- `filter-context.jsonl` holds the same twelve songs with each song's catalog entry.

The question sits on the command line, so this example needs no question file. The first line of `filter-cold.jsonl` holds the case for Octopus's Garden:

```sh
sed -n 1p filter-cold.jsonl
```

```text
{"id": "filter-cold-01", "function": "filter", "args": ["The text is the title of a song by the Beatles. It appears on the album Abbey Road.", "--threshold", "0.7", "--jsonl", "--field", "/input"], "records": [{"id": "filter-cold-01", "input": "Octopus's Garden"}], "title": "Octopus's Garden", "truth": true, "fields": [["songs.tsv", "Octopus's Garden", "first_album", "Abbey Road"]], "group": "filter-cold"}
```

`truth` is the right answer. `fields` names the row and column of `data/songs.tsv` behind it. [How to run the bench for free](/beatles-bench/run-it-for-free/) explains every field of a case line.

Everything in the folder:

- `filter-cold.jsonl`, `filter-context.jsonl`: the cases, one song each, with `truth` from `first_album`, written by `scripts/generate/examples.py`.
- `lists/`: the kept songs, as the `filter` command prints them.
- `recording/`, `outputs.jsonl`, `timing.tsv`: the requests, the committed answers, and the wall times.
- `run.sh live OUT | replay [OUT]`: asks the cases through `scripts/run/functions.sh`.

## Run it

Run this in `examples/05-filter`:

```sh
printf '%s\n' "Octopus's Garden" 'Yellow Submarine' 'Something' 'Here Comes the Sun' 'Yesterday' 'Come Together' \
    'Hey Jude' 'Penny Lane' 'A Day in the Life' 'Her Majesty' 'Let It Be' "Maxwell's Silver Hammer" |
  thinkthen filter 'The text is the title of a song by the Beatles. It appears on the album Abbey Road.' \
    --threshold 0.7 --lines --replay recording
```

```text
Octopus's Garden
Something
Here Comes the Sun
Come Together
A Day in the Life
Her Majesty
Maxwell's Silver Hammer
```

- `--threshold 0.7` keeps a song when its probability of yes is 0.7 or more.
- `--lines` takes each line of input as one record.
- `--replay recording` answers from the saved recording. No request leaves the machine.
- `--details` prints each kept record with its probability.

To ask your own server, leave out `--replay recording` and follow [Ask your own server](/beatles-bench/run-it-for-free/#ask-your-own-server). The answers can then differ by a few points from these.

## Read it

`filter` prints only what it keeps, so the dropped songs never show. The bench asked each song through `decide`. `decide` sends the same request as `filter`, and `outputs.jsonl` holds every probability. This command sets each song beside its first album and marks it:

```sh
jq -c --slurpfile out outputs.jsonl '
  ($out | map({(.id): .rows[0]}) | add) as $rows | $rows[.id] as $row
  | {song: .title, first_album: .fields[0][3], truth, kept: $row.value, yes: $row.answer.probability,
     mark: (if $row.value == null then "not sure" elif $row.value == .truth then "right" else "wrong" end)}
' filter-cold.jsonl
```

```json
{"song":"Octopus's Garden","first_album":"Abbey Road","truth":true,"kept":true,"yes":0.75,"mark":"right"}
{"song":"Yellow Submarine","first_album":"Revolver","truth":false,"kept":false,"yes":0.12,"mark":"right"}
{"song":"Something","first_album":"Abbey Road","truth":true,"kept":true,"yes":0.8,"mark":"right"}
{"song":"Here Comes the Sun","first_album":"Abbey Road","truth":true,"kept":true,"yes":0.96,"mark":"right"}
{"song":"Yesterday","first_album":"Help!","truth":false,"kept":false,"yes":0.11,"mark":"right"}
{"song":"Come Together","first_album":"Abbey Road","truth":true,"kept":true,"yes":0.95,"mark":"right"}
{"song":"Hey Jude","first_album":"Past Masters","truth":false,"kept":false,"yes":0.29,"mark":"right"}
{"song":"Penny Lane","first_album":"Magical Mystery Tour","truth":false,"kept":false,"yes":0.43,"mark":"right"}
{"song":"A Day in the Life","first_album":"Sgt. Pepper's Lonely Hearts Club Band","truth":false,"kept":true,"yes":0.93,"mark":"wrong"}
{"song":"Her Majesty","first_album":"Abbey Road","truth":true,"kept":true,"yes":0.9,"mark":"right"}
{"song":"Let It Be","first_album":"Let It Be","truth":false,"kept":false,"yes":0.27,"mark":"right"}
{"song":"Maxwell's Silver Hammer","first_album":"Abbey Road","truth":true,"kept":true,"yes":0.94,"mark":"right"}
```

Eleven of twelve are right. Octopus's Garden is kept at 0.75, just over the bar, and it is right. A bar of 0.8 would drop it.

A Day in the Life is kept at 0.93, and it is wrong. The song first came out on Sgt. Pepper's Lonely Hearts Club Band. Jev makes the same sure mistake on [the decide page](/beatles-bench/decide/) and on [the audit page](/beatles-bench/audit/). No bar drops it without dropping real Abbey Road songs.

## With context

The context run sends each song's catalog entry before its title. The entry names the album above the song's line:

```sh
jq -r '.records[0].input' filter-context.jsonl | head -3
```

```text
Catalog:
Abbey Road (1969-09-26)
Octopus's Garden (lead: Starr; written: Starkey; 2:51; released 1969-09-26; first album: Abbey Road)
```

```sh
jq -c '.records[]' filter-context.jsonl |
  thinkthen filter 'The text gives a catalog entry and then names a song by the Beatles. It appears on the album Abbey Road.' \
    --threshold 0.7 --jsonl --field /input --details --replay recording |
  jq -c '{song: (.input.input | split("\nText: ") | last), yes: .answer.probability}'
```

```json
{"song":"Octopus's Garden","yes":0.96}
{"song":"Something","yes":0.95}
{"song":"Here Comes the Sun","yes":0.96}
{"song":"Come Together","yes":0.95}
{"song":"Her Majesty","yes":0.96}
{"song":"Maxwell's Silver Hammer","yes":0.95}
```

- `--jsonl` takes each line as one JSON record.
- `--field /input` names the part of the record Jev reads.
- `--details` adds the probability to each kept record.

With the entries, filter keeps the six Abbey Road songs and nothing else. Every probability for the twelve songs is in `outputs.jsonl`.

## What can go wrong

- **The shared traps.** A replay miss, a missing key, and a cache bound to another server can stop any command. [How to run the bench for free](/beatles-bench/run-it-for-free/#what-can-go-wrong) gives each exit code and its fix.
- **The dropped records are silent.** `filter` prints only what it keeps. To see why a record was dropped, ask the same question with `decide --details`.
- **The bar decides what you keep.** A bar near a real answer's probability keeps or drops it by a hair. Octopus's Garden clears 0.7 at 0.75. [How to find your bar with audit](/beatles-bench/audit/) shows how to choose one.

## The slide

The left column shows twelve songs with their probability. The songs over the bar of 0.7 are bright, and the rest are dim. The right column shows the seven songs kept. A green ✓ marks each one `data/songs.tsv` puts on Abbey Road (`first_album`). A red ✗ marks A Day in the Life. Octopus's Garden is kept at 0.75.

## Related

- [How to answer yes or no with decide](/beatles-bench/decide/): the question `filter` asks of each record.
- [How to sort records by a question with rank](/beatles-bench/rank/): keep every record and sort them.
- [How to find your bar with audit](/beatles-bench/audit/): choose the bar from graded answers.
