---
layout: "../../layouts/BenchPage.astro"
title: "How to find names in a sentence with recognize"
tagline: "recognize labels things it finds."
slide: "/beatles-bench/img/recognize.png"
source: "https://github.com/botassembly/beatles-bench/blob/48ac3f252c6161bb9518416a41b4c8a2d82a9434/examples/09-recognize/README.md"
runsIn: "examples/09-recognize"
---

Use `recognize` when you want every name in a text, each with a kind from your list. It returns each name with its place in the text and a strength from 0 to 1.

Here Jev finds the people, songs, albums, and places in one sentence written for the talk:

> Ringo Starr wrote Octopus's Garden on a boat off Sardinia, and the band recorded it at Abbey Road Studios for the album Abbey Road.

## The files

- `recording/` holds the saved request and answer, so the command needs no key.
- `recognize-cold.jsonl` holds the one case: the sentence and the four kinds.

The kinds sit on the command line, so this example needs no question file. The sentence was written for the talk, so the bench holds no answer key for it.

Everything in the folder:

- `recognize-cold.jsonl`: the one case, written by `scripts/generate/examples.py`.
- `recording/`, `outputs.jsonl`, `timing.tsv`: the request, the committed answer, and the wall time.
- `run.sh live OUT | replay [OUT]`: asks the case through `scripts/run/functions.sh`.

## Run it

Run this in `examples/09-recognize`:

```sh
printf '%s' "Ringo Starr wrote Octopus's Garden on a boat off Sardinia, and the band recorded it at Abbey Road Studios for the album Abbey Road." |
  thinkthen recognize person song album place --threshold 0.01 --replay recording |
  jq -c '.entities[]'
```

```json
{"name":"Ringo Starr","kind":"person","start":0,"end":11,"strength":1.0}
{"name":"Octopus's Garden","kind":"song","start":18,"end":34,"strength":0.99}
{"name":"Sardinia","kind":"place","start":49,"end":57,"strength":0.98}
{"name":"Abbey Road Studios","kind":"place","start":87,"end":105,"strength":0.9735}
{"name":"Abbey Road","kind":"album","start":120,"end":130,"strength":0.9504}
```

- The words after `recognize` are the kinds.
- `--threshold 0.01` keeps every name with any strength.
- `--replay recording` answers from the saved recording. No request leaves the machine.

`start` and `end` give the name's place in the text, counted in characters from 0. `strength` says how sure Jev is of the name and its kind.

To ask your own server, leave out `--replay recording` and follow [Ask your own server](/beatles-bench/run-it-for-free/#ask-your-own-server). The answers can then differ by a few points from these.

## Read it

With no answer key, read each name against the sentence. This command cuts each name's text out of the sentence at its `start` and `end`:

```sh
s="Ringo Starr wrote Octopus's Garden on a boat off Sardinia, and the band recorded it at Abbey Road Studios for the album Abbey Road."
printf '%s' "$s" |
  thinkthen recognize person song album place --threshold 0.01 --replay recording |
  jq -c --arg s "$s" '.entities[] | {name, text: $s[.start:.end], kind}'
```

```json
{"name":"Ringo Starr","text":"Ringo Starr","kind":"person"}
{"name":"Octopus's Garden","text":"Octopus's Garden","kind":"song"}
{"name":"Sardinia","text":"Sardinia","kind":"place"}
{"name":"Abbey Road Studios","text":"Abbey Road Studios","kind":"place"}
{"name":"Abbey Road","text":"Abbey Road","kind":"album"}
```

Every cut matches its name, so each place in the text is right. Jev finds all five names, and each kind fits. It marks Abbey Road Studios as a place and Abbey Road as an album. The weakest name is the album at 0.9504.

`recognize` asks two questions about each of the 26 pieces of the sentence (24 words and 2 punctuation marks), 52 questions in all, in one request. The first asks whether the piece is part of a name. The second asks its kind. That request is the largest of any example here:

```sh
jq -c '{input_tokens, output_tokens}' outputs.jsonl
```

```json
{"input_tokens":8092,"output_tokens":1918}
```

## With context

None. The text is the input. A name finder reads the sentence it is given, and a catalog entry would add names of its own.

## What can go wrong

- **The shared traps.** A replay miss, a missing key, and a cache bound to another server can stop any command. [How to run the bench for free](/beatles-bench/run-it-for-free/#what-can-go-wrong) gives each exit code and its fix.
- **A trailing newline changes the request.** `printf '%s'` sends the sentence with no newline. `echo` adds one, and the replay then misses.
- **Cost grows with the text.** Two questions per piece make a long text costly. Split a long document into sentences or paragraphs.
- **A low bar keeps weak names.** `--threshold 0.01` shows everything. Use a higher bar to keep only the sure names.

## The slide

Each name is underlined in its kind's color, with its strength beside the kind.

## Related

- [How to link songs to singers and albums with relate](/beatles-bench/relate/): link the names once you have them.
- [How to name every label that fits with tag](/beatles-bench/tag/): labels for a whole text in place of names inside it.
