---
layout: "../../layouts/BenchPage.astro"
title: "How to name every label that fits with tag"
tagline: "tag applies labels to data."
slide: "/beatles-bench/img/tag.png"
source: "https://github.com/botassembly/beatles-bench/blob/48ac3f252c6161bb9518416a41b4c8a2d82a9434/examples/03-tag/README.md"
runsIn: "examples/03-tag"
---

Use `tag` when a text can take any number of labels from a list you give. It asks one yes or no question per label. It returns every label over the bar and a probability for each label. The default bar is `--threshold 0.5`.

Here Jev tags six songs with five labels: love song, sad, psychedelic, about a place, and about the sea.

## The files

- `recording/` holds the saved requests and answers, so the command needs no key.
- `tag-cold.jsonl` holds the cases, one song each.

The question and the labels sit on the command line, so this example needs no question file. The bench holds no answer key for these labels, so each case has `"truth": null`.

Everything in the folder:

- `tag-cold.jsonl`: the cases, one song each, written by `scripts/generate/examples.py`.
- `recording/`, `outputs.jsonl`, `timing.tsv`: the requests, the committed answers, and the wall times.
- `run.sh live OUT | replay [OUT]`: asks the cases through `scripts/run/functions.sh`.

## Run it

Run this in `examples/03-tag`:

```sh
printf '%s\n' Michelle Yesterday 'Eleanor Rigby' 'Penny Lane' "Octopus's Garden" 'Lucy in the Sky with Diamonds' |
  thinkthen tag 'The text is the title of a song by the Beatles. Which of these describe it?' \
    'love song' sad psychedelic 'about a place' 'about the sea' --lines --details --replay recording |
  jq -c '{song: .input, tags: .value, p: .answer.probabilities}'
```

```json
{"song":"Michelle","tags":["love song"],"p":{"love song":0.86,"sad":0.35,"psychedelic":0.18,"about a place":0.06,"about the sea":0.07}}
{"song":"Yesterday","tags":["love song","sad"],"p":{"love song":0.72,"sad":0.78,"psychedelic":0.11,"about a place":0.03,"about the sea":0.04}}
{"song":"Eleanor Rigby","tags":["sad"],"p":{"love song":0.17,"sad":0.93,"psychedelic":0.27,"about a place":0.09,"about the sea":0.04}}
{"song":"Penny Lane","tags":["psychedelic","about a place"],"p":{"love song":0.29,"sad":0.3,"psychedelic":0.56,"about a place":0.95,"about the sea":0.03}}
{"song":"Octopus's Garden","tags":["psychedelic","about a place","about the sea"],"p":{"love song":0.42,"sad":0.12,"psychedelic":0.68,"about a place":0.86,"about the sea":0.9}}
{"song":"Lucy in the Sky with Diamonds","tags":["psychedelic"],"p":{"love song":0.28,"sad":0.28,"psychedelic":0.96,"about a place":0.16,"about the sea":0.04}}
```

- The words after the question are the labels.
- `--lines` takes each line of input as one record.
- `--details` prints the whole result, with every probability. Without it, each line gives the song and its labels.
- `--replay recording` answers from the saved recording. No request leaves the machine.
- `--threshold 0.7` would keep only the labels at 0.7 or more.

To ask your own server, leave out `--replay recording` and follow [Ask your own server](/beatles-bench/run-it-for-free/#ask-your-own-server). The answers can then differ by a few points from these.

## Read it

These labels are matters of taste, so no command can mark them right or wrong. Read each label as its own yes or no. The labels near the bar are the ones to check by hand. This command lists every label from 0.3 to 0.7:

```sh
jq -c '.rows[0] | {song: .input.input, close: (.answer.probabilities | with_entries(select(.value >= 0.3 and .value <= 0.7)))}' outputs.jsonl
```

```json
{"song":"Michelle","close":{"sad":0.35}}
{"song":"Yesterday","close":{}}
{"song":"Eleanor Rigby","close":{}}
{"song":"Penny Lane","close":{"sad":0.3,"psychedelic":0.56}}
{"song":"Octopus's Garden","close":{"love song":0.42,"psychedelic":0.68}}
{"song":"Lucy in the Sky with Diamonds","close":{}}
```

Octopus's Garden takes three labels: about the sea at 0.9, about a place at 0.86, and psychedelic at 0.68. The song is about a garden under the sea, so the first two fit. Psychedelic is a close call. Love song sits at 0.42, under the bar. Each label is a separate question, so one song can clear several bars at once. Yesterday takes both love song at 0.72 and sad at 0.78.

## With context

None. No catalog entry says whether a song is sad, psychedelic, or about the sea. The entry gives the singer, the writers, the length, the date, and the album.

## What can go wrong

- **The shared traps.** A replay miss, a missing key, and a cache bound to another server can stop any command. [How to run the bench for free](/beatles-bench/run-it-for-free/#what-can-go-wrong) gives each exit code and its fix.
- **No labels is an answer.** A song under every bar comes back with an empty list. That is not a failure.
- **Labels that overlap.** "About a place" and "about the sea" can both fit one song. `tag` asks each on its own. Use `choose` when only one label may win.

## The slide

The slide shows these five labels for the six songs, at the default bar of 0.5. A chip at or over the bar is bright, with a white outline. A chip under it is muted. The bench holds no answer key for tag. So no chip is green or red.

## Related

- [How to pick one option with choose](/beatles-bench/choose/): exactly one answer from the list.
- [How to answer yes or no with decide](/beatles-bench/decide/): one label is one `decide` question.
