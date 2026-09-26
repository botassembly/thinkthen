---
layout: "../../layouts/BenchPage.astro"
title: "How to answer yes or no with decide"
tagline: "decide answers yes, no, or not sure."
slide: "/beatles-bench/img/decide.png"
source: "https://github.com/botassembly/beatles-bench/blob/48ac3f252c6161bb9518416a41b4c8a2d82a9434/examples/01-decide/README.md"
runsIn: "examples/01-decide"
---

Use `decide` when you have one yes or no question about a text. It returns the probability of yes. A threshold turns that probability into yes, no, or not sure. The default bar is `--threshold 0.5`.

Here Jev answers one question about six song titles: "It appears on the album Abbey Road." One of the six is on Abbey Road.

## The files

- `recording/` holds the saved requests and answers, so the command needs no key.
- `decide-cold.jsonl` holds the cases, one song each, with the right answer taken from `data/songs.tsv`.
- `decide-context.jsonl` holds the same six songs with each song's catalog entry.

The question sits on the command line, so this example needs no question file. The fifth line of `decide-cold.jsonl` holds the case for A Day in the Life:

```sh
sed -n 5p decide-cold.jsonl
```

```text
{"id": "decide-cold-05", "function": "decide", "args": ["The text is the title of a song by the Beatles. It appears on the album Abbey Road.", "--jsonl", "--field", "/input"], "records": [{"id": "decide-cold-05", "input": "A Day in the Life"}], "title": "A Day in the Life", "truth": false, "fields": [["songs.tsv", "A Day in the Life", "first_album", "Sgt. Pepper's Lonely Hearts Club Band"]]}
```

`truth` is the right answer. `fields` names the row and column of `data/songs.tsv` behind it. [How to run the bench for free](/beatles-bench/run-it-for-free/) explains every field of a case line.

Everything in the folder:

- `decide-love.jsonl`, `decide-cold.jsonl`, `decide-context.jsonl`: the cases, one song each, written by `scripts/generate/examples.py`.
- `recording/`: every request and response. `outputs.jsonl` and `timing.tsv`: the committed answers and wall times.
- `run.sh live OUT | replay [OUT]`: asks the cases through `scripts/run/functions.sh`.

## Run it

Run this in `examples/01-decide`:

```sh
printf '%s\n' 'She Loves You' 'Michelle' 'Yesterday' 'Taxman' 'A Day in the Life' 'Something' |
  thinkthen decide 'The text is the title of a song by the Beatles. It appears on the album Abbey Road.' \
    --lines --details --replay recording |
  jq -c '{song: .input, value, yes: .answer.probability}'
```

```json
{"song":"She Loves You","value":false,"yes":0.06}
{"song":"Michelle","value":false,"yes":0.14}
{"song":"Yesterday","value":false,"yes":0.09}
{"song":"Taxman","value":false,"yes":0.48}
{"song":"A Day in the Life","value":true,"yes":0.94}
{"song":"Something","value":true,"yes":0.81}
```

- `--lines` takes each line of input as one record.
- `--details` prints the whole result, with the probability. Without it, each line gives the song and its yes or no.
- `--replay recording` answers from the saved recording. No request leaves the machine.

`value` is the answer at the default bar of 0.5. `yes` is the probability of yes.

To ask your own server, leave out `--replay recording` and follow [Ask your own server](/beatles-bench/run-it-for-free/#ask-your-own-server). The answers can then differ by a few points from these.

## Read it

The bench's `run.sh` sent the same requests and saved the answers in `outputs.jsonl`. This command sets each answer beside the song's first album and marks it:

```sh
jq -c --slurpfile out outputs.jsonl '
  ($out | map({(.id): .rows[0]}) | add) as $rows | $rows[.id] as $row
  | {song: .title, first_album: .fields[0][3], truth, jev: $row.value, yes: $row.answer.probability,
     mark: (if $row.value == null then "not sure" elif $row.value == .truth then "right" else "wrong" end)}
' decide-cold.jsonl
```

```json
{"song":"She Loves You","first_album":"Past Masters","truth":false,"jev":false,"yes":0.06,"mark":"right"}
{"song":"Michelle","first_album":"Rubber Soul","truth":false,"jev":false,"yes":0.14,"mark":"right"}
{"song":"Yesterday","first_album":"Help!","truth":false,"jev":false,"yes":0.09,"mark":"right"}
{"song":"Taxman","first_album":"Revolver","truth":false,"jev":false,"yes":0.48,"mark":"right"}
{"song":"A Day in the Life","first_album":"Sgt. Pepper's Lonely Hearts Club Band","truth":false,"jev":true,"yes":0.94,"mark":"wrong"}
{"song":"Something","first_album":"Abbey Road","truth":true,"jev":true,"yes":0.81,"mark":"right"}
```

Five of six are right. Something is on Abbey Road, and Jev says yes at 0.81.

A Day in the Life is wrong, and Jev is sure of it at 0.94. The song first came out on Sgt. Pepper's Lonely Hearts Club Band. No bar fixes a miss this sure. A bar high enough to drop 0.94 would also drop Something at 0.81.

Taxman is right by a hair. It sits at 0.48, just under the bar of 0.5. A band such as `--threshold 0.3:0.7` would mark it not sure.

## With context

The context run sends each song's catalog entry before its title:

```sh
jq -r '.records[0].input' decide-context.jsonl | head -3
```

```text
Catalog:
Singles
She Loves You (lead: Lennon, McCartney; written: Lennon–McCartney; 2:21; released 1963-08-23; first album: Past Masters)
```

The question opens "The text gives a catalog entry and then names a song by the Beatles."

```sh
jq -c '.records[]' decide-context.jsonl |
  thinkthen decide 'The text gives a catalog entry and then names a song by the Beatles. It appears on the album Abbey Road.' \
    --jsonl --field /input --details --replay recording |
  jq -c '{song: (.input.input | split("\nText: ") | last), value, yes: .answer.probability}'
```

```json
{"song":"She Loves You","value":false,"yes":0.04}
{"song":"Michelle","value":false,"yes":0.04}
{"song":"Yesterday","value":false,"yes":0.04}
{"song":"Taxman","value":false,"yes":0.03}
{"song":"A Day in the Life","value":false,"yes":0.04}
{"song":"Something","value":true,"yes":0.95}
```

- `--jsonl` takes each line as one JSON record.
- `--field /input` names the part of the record Jev reads.

With the entry, every answer is right. A Day in the Life drops from 0.94 to 0.04. Every probability sits near 0 or 1.

## What can go wrong

- **The shared traps.** A replay miss, a missing key, and a cache bound to another server can stop any command. [How to run the bench for free](/beatles-bench/run-it-for-free/#what-can-go-wrong) gives each exit code and its fix.
- **The exit code carries a single answer.** On one text, `decide` exits 0 for yes, 1 for no, and 3 for not sure. Under `set -e`, a no ends the script. Put the command in an `if`.
- **A sure miss.** A high probability does not make an answer right. A Day in the Life shows it.

## The slide

The slide asks a different question of four songs: "It is a love song." The bench holds no answer key for it. A single bar says yes or no. A band such as `--threshold 0.3:0.7` says not sure in its middle:

```sh
printf '%s\n' 'She Loves You' 'Michelle' 'Yesterday' 'Taxman' |
  thinkthen decide 'The text is the title of a song by the Beatles. It is a love song.' \
    --lines --details --threshold 0.3:0.7 --replay recording |
  jq -c '{song: .input, value, yes: .answer.probability}'
```

```json
{"song":"She Loves You","value":true,"yes":0.96}
{"song":"Michelle","value":true,"yes":0.73}
{"song":"Yesterday","value":null,"yes":0.56}
{"song":"Taxman","value":false,"yes":0.05}
```

Yesterday lands in the band at 0.56. Its answer is `null` and reads as not sure. The slide draws these four answers under three settings: `--threshold 0.5`, `--threshold 0.3:0.7`, and `--threshold 0.1:0.9`. Taxman reads no under all three. Yesterday reads yes under the single bar and not sure under both bands. Michelle turns not sure only under the widest band. She Loves You reads yes under all three.

## Related

- [How to keep the records that match with filter](/beatles-bench/filter/): the same question over a list, keeping the yeses.
- [How to find your bar with audit](/beatles-bench/audit/): the same question over 70 songs, and the bar that makes the fewest mistakes.
- [Retrieval-augmented decisions (RAD)](/beatles-bench/rad/): what the catalog entry fixes and what it costs.
