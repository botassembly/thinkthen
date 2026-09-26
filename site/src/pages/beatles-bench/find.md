---
layout: "../../layouts/BenchPage.astro"
title: "How to pick the one line that answers with find"
tagline: "find picks one from many."
slide: "/beatles-bench/img/find.png"
source: "https://github.com/botassembly/beatles-bench/blob/48ac3f252c6161bb9518416a41b4c8a2d82a9434/examples/07-find/README.md"
runsIn: "examples/07-find"
---

Use `find` when one record out of many answers a question, and the answer depends on the others. `find` sends every line in one request, so each line sees all the others. It returns the one line that best answers the question, and a probability for every line.

Here Jev picks the first of ten songs the Beatles released. The ten came out in 1962, 1963, and 1964.

## The files

- `recording/` holds the saved request and answer, so the command needs no key.
- `find-cold.jsonl` holds the ten songs as one case, with the right line as `truth`. Its `fields` give each song's release date from `data/songs.tsv`.
- `find-context.jsonl` holds the same ten songs with each song's catalog entry.

The question sits on the command line, so this example needs no question file.

Everything in the folder:

- `find-cold.jsonl`, `find-context.jsonl`: one case each, holding all ten songs, with the earliest song as `truth`, written by `scripts/generate/examples.py`.
- `recording/`, `outputs.jsonl`, `timing.tsv`: the requests, the committed answers, and the wall times.
- `run.sh live OUT | replay [OUT]`: asks the cases through `scripts/run/functions.sh`.

## Run it

Run this in `examples/07-find`:

```sh
printf '%s\n' 'She Loves You' 'I Saw Her Standing There' 'From Me to You' 'All My Loving' 'Love Me Do' \
    'I Want to Hold Your Hand' 'Please Please Me' "Can't Buy Me Love" "A Hard Day's Night" 'I Feel Fine' |
  thinkthen find 'These are songs by the Beatles. Which one did they release first?' --details --replay recording |
  jq -c '{pick: .value, p: .answer.probabilities}'
```

```json
{"pick":"Love Me Do","p":{"u001":0.01,"u002":0.01,"u003":0.08,"u004":0.0,"u005":0.63,"u006":0.0,"u007":0.27,"u008":0.0,"u009":0.0,"u010":0.0}}
```

- Each line of input is one choice. `find` reads lines by default.
- `--details` prints the whole result, with a probability for every line. Without it, `find` prints the chosen line alone.
- `--replay recording` answers from the saved recording. No request leaves the machine.

The probabilities name each line by its place in the input. `u001` is the first line.

To ask your own server, leave out `--replay recording` and follow [Ask your own server](/beatles-bench/run-it-for-free/#ask-your-own-server). The answers can then differ by a few points from these.

## Read it

The bench's `run.sh` sent the same requests, cold and with context, and saved the answers in `outputs.jsonl`. This command sets each line's probability beside its release date:

```sh
jq -c --slurpfile out outputs.jsonl '
  ($out | map({(.id): .rows[0].answer.probabilities}) | add) as $p | .id as $id | ($id | sub("cold"; "context")) as $ctx
  | .fields | to_entries[] | ("u" + ("00" + (.key + 1 | tostring))[-3:]) as $line
  | {line: $line, song: .value[1], released: .value[3], cold: $p[$id][$line], context: $p[$ctx][$line]}
' find-cold.jsonl
```

```json
{"line":"u001","song":"She Loves You","released":"1963-08-23","cold":0.01,"context":0.0}
{"line":"u002","song":"I Saw Her Standing There","released":"1963-03-22","cold":0.01,"context":0.0}
{"line":"u003","song":"From Me to You","released":"1963-04-11","cold":0.08,"context":0.0}
{"line":"u004","song":"All My Loving","released":"1963-11-22","cold":0.0,"context":0.0}
{"line":"u005","song":"Love Me Do","released":"1962-10-05","cold":0.63,"context":0.6900000000000001}
{"line":"u006","song":"I Want to Hold Your Hand","released":"1963-11-29","cold":0.0,"context":0.0}
{"line":"u007","song":"Please Please Me","released":"1963-01-11","cold":0.27,"context":0.31}
{"line":"u008","song":"Can't Buy Me Love","released":"1964-03-16","cold":0.0,"context":0.0}
{"line":"u009","song":"A Hard Day's Night","released":"1964-07-10","cold":0.0,"context":0.0}
{"line":"u010","song":"I Feel Fine","released":"1964-11-23","cold":0.0,"context":0.0}
```

From memory, Jev picks Love Me Do at 0.63, and it is right. It came out on 1962-10-05, first of the ten.

The near miss is Please Please Me at 0.27. It came out on 1963-01-11, second of the ten. Jev puts most of its weight on the two earliest songs. Its doubt is which of those two came first.

## With context

The context run gives each of the ten lines as its song's catalog entry and then its title. All ten entries go out together in one request:

```sh
jq -c '.records[]' find-context.jsonl |
  thinkthen find 'Each one gives a catalog entry and then names a song by the Beatles. Which one did they release first?' \
    --jsonl --field /input --details --replay recording |
  jq -c '{pick: .value.id, p: .answer.probabilities}'
```

```json
{"pick":"u05","p":{"u001":0.0,"u002":0.0,"u003":0.0,"u004":0.0,"u005":0.6900000000000001,"u006":0.0,"u007":0.31,"u008":0.0,"u009":0.0,"u010":0.0}}
```

- `--jsonl` takes each line as one JSON record.
- `--field /input` names the part of the record Jev reads. The pick comes back as the whole record, so `.value.id` gives its id.

The pick prints as `u05`, the record's own `id` in the case file. The probability keys `u001` to `u010` are line names, one per place in the input. Both name the fifth line.

With every date in hand, Jev still gives Please Please Me 0.31. The two entries read "released 1962-10-05" and "released 1963-01-11". Both entries sit under the Please Please Me album header in the catalog. The pick stays right and grows surer.

## What can go wrong

- **The shared traps.** A replay miss, a missing key, and a cache bound to another server can stop any command. [How to run the bench for free](/beatles-bench/run-it-for-free/#what-can-go-wrong) gives each exit code and its fix.
- **The line names shift with the input.** `u001` is the first line of this input. Reorder the input, and every name changes.
- **One request holds every line.** A long list makes one long request. It can pass the backend's input limit.
- **A float can print long.** Love Me Do's probability prints as `0.6900000000000001` in the context run. Round it before you show it.

## The slide

Each card shades in grey by its probability, and every value is white. Love Me Do is the pick at 0.63. It is right. It gets a green border and a ✓. Please Please Me at 0.27 takes the next shade.

## Related

- [How to pick one option with choose](/beatles-bench/choose/): pick from options you name, one text at a time.
- [How to sort records by a question with rank](/beatles-bench/rank/): each record asked on its own.
