---
layout: "../../layouts/BenchPage.astro"
title: "How to fill in a form with annotate"
tagline: "annotate fills in a form."
slide: "/beatles-bench/img/annotate.png"
source: "https://github.com/botassembly/beatles-bench/blob/48ac3f252c6161bb9518416a41b4c8a2d82a9434/examples/08-annotate/README.md"
runsIn: "examples/08-annotate"
---

Use `annotate` when you want several answers about each record at once. You save the questions in one file. Each record comes back as one filled-in form. Each field can carry its own threshold. A field under its threshold comes back `null`, and `null` reads as not sure.

Here Jev fills in a form for two songs: who sings it, which album it first appeared on, and the year it came out.

## The files

- `annotate-cold-card.json` holds the three questions.
- `recording/` holds the saved requests and answers, so the command needs no key.
- `annotate-cold.jsonl` holds the cases, one song each, with the right answer taken from `data/songs.tsv`.

`annotate-cold-card.json` in full:

```sh
cat annotate-cold-card.json
```

```json
{
 "version": 1,
 "questions": {
  "singer": {
   "choose": "The text is the title of a song by the Beatles. Who sings the lead vocal on it?",
   "options": {
    "John": "John Lennon",
    "Paul": "Paul McCartney",
    "George": "George Harrison",
    "Ringo": "Ringo Starr"
   },
   "threshold": 0.8
  },
  "album": {
   "choose": "The text is the title of a song by the Beatles. On which album did it first appear?",
   "options": {
    "Help!": "Help! (1965)",
    "Rubber Soul": "Rubber Soul (1965)",
    "Revolver": "Revolver (1966)",
    "White Album": "The Beatles, the White Album (1968)",
    "Abbey Road": "Abbey Road (1969)"
   },
   "threshold": 0.8
  },
  "year": {
   "choose": "The text is the title of a song by the Beatles. In which year was it first released?",
   "options": [
    "1965",
    "1966",
    "1967",
    "1968",
    "1969"
   ],
   "threshold": 0.8
  }
 }
}
```

- `version` is the file format. It is 1.
- `questions` names each field of the form. Here the fields are `singer`, `album`, and `year`.
- `choose` makes the field a pick of one option. Its value is the question Jev reads.
- `options` lists the choices. In a map, the key is the name that comes back and the value is the longer wording Jev reads. A list gives the names alone.
- `threshold` is the bar. A pick under 0.8 fills the field with `null`.

The second line of `annotate-cold.jsonl` holds the case for Octopus's Garden:

```sh
sed -n 2p annotate-cold.jsonl
```

```text
{"id": "annotate-cold-02", "function": "annotate", "args": ["annotate-cold-card.json", "--jsonl", "--field", "/input"], "records": [{"id": "annotate-cold-02", "input": "Octopus's Garden"}], "title": "Octopus's Garden", "truth": {"singer": "Ringo", "album": "Abbey Road", "year": "1969"}, "fields": [["songs.tsv", "Octopus's Garden", "lead_vocals", "Starr"], ["songs.tsv", "Octopus's Garden", "first_album", "Abbey Road"], ["songs.tsv", "Octopus's Garden", "year", "1969"]]}
```

`truth` is the right form. `fields` names the row and column of `data/songs.tsv` behind each answer. [How to run the bench for free](/beatles-bench/run-it-for-free/) explains every field of a case line.

Everything in the folder:

- `annotate-cold-card.json`, `annotate-context-card.json`: the two question sets.
- `annotate-cold.jsonl`, `annotate-context.jsonl`: the cases, one song each, with `truth`, written by `scripts/generate/examples.py`.
- `recording/`, `outputs.jsonl`, `timing.tsv`: the requests, the committed answers, and the wall times.
- `run.sh live OUT | replay [OUT]`: asks the cases through `scripts/run/functions.sh`.

## Run it

Run this in `examples/08-annotate`:

```sh
printf '%s\n' Blackbird "Octopus's Garden" |
  thinkthen annotate annotate-cold-card.json --lines --details --replay recording |
  jq -c '{song: .input, value, p: (.answers | map_values(.answer.probabilities))}'
```

```json
{"song":"Blackbird","value":{"singer":"Paul","album":"White Album","year":"1968"},"p":{"singer":{"John":0.15,"Paul":0.83,"George":0.02,"Ringo":0.0},"album":{"Help!":0.0,"Rubber Soul":0.0,"Revolver":0.01,"White Album":0.99,"Abbey Road":0.0},"year":{"1965":0.0,"1966":0.01,"1967":0.09,"1968":0.87,"1969":0.03}}}
{"song":"Octopus's Garden","value":{"singer":"Ringo","album":null,"year":null},"p":{"singer":{"John":0.0,"Paul":0.01,"George":0.08,"Ringo":0.91},"album":{"Help!":0.0,"Rubber Soul":0.0,"Revolver":0.36,"White Album":0.6,"Abbey Road":0.04},"year":{"1965":0.0,"1966":0.04,"1967":0.29,"1968":0.6,"1969":0.07}}}
```

- `--lines` takes each line of input as one record.
- `--details` prints the whole result. It holds every probability, and `jq` keeps the parts shown here.
- `--replay recording` answers from the saved recording. No request leaves the machine.

`value` is the filled-in form. `p` gives the probability of every option in each field.

To ask your own server, leave out `--replay recording` and follow [Ask your own server](/beatles-bench/run-it-for-free/#ask-your-own-server). The answers can then differ by a few points from these.

## Read it

The bench's `run.sh` sent the same requests and saved the answers in `outputs.jsonl`. This command sets each field beside its right answer and marks it:

```sh
jq -c --slurpfile out outputs.jsonl '
  ($out | map({(.id): .rows[0]}) | add) as $rows | $rows[.id] as $row | .title as $song | .truth as $truth
  | $truth | keys_unsorted[] as $f | $row.answers[$f].answer as $a
  | {song: $song, field: $f, truth: $truth[$f], top: $a.pick, p_top: $a.probabilities[$a.pick],
     p_truth: $a.probabilities[$truth[$f]],
     mark: (if $row.value[$f] == null then "not sure" elif $row.value[$f] == $truth[$f] then "right" else "wrong" end)}
' annotate-cold.jsonl
```

```json
{"song":"Blackbird","field":"singer","truth":"Paul","top":"Paul","p_top":0.83,"p_truth":0.83,"mark":"right"}
{"song":"Blackbird","field":"album","truth":"White Album","top":"White Album","p_top":0.99,"p_truth":0.99,"mark":"right"}
{"song":"Blackbird","field":"year","truth":"1968","top":"1968","p_top":0.87,"p_truth":0.87,"mark":"right"}
{"song":"Octopus's Garden","field":"singer","truth":"Ringo","top":"Ringo","p_top":0.91,"p_truth":0.91,"mark":"right"}
{"song":"Octopus's Garden","field":"album","truth":"Abbey Road","top":"White Album","p_top":0.6,"p_truth":0.04,"mark":"not sure"}
{"song":"Octopus's Garden","field":"year","truth":"1969","top":"1968","p_top":0.6,"p_truth":0.07,"mark":"not sure"}
```

`top` is Jev's likeliest option. `p_top` is its probability, and `p_truth` is the probability of the right answer.

Blackbird fills all three fields, and all three are right. Octopus's Garden gets its singer right: Ringo at 0.91. The album and the year come back not sure.

Here is why. From memory, Jev puts Octopus's Garden on the White Album at 0.6 or on Revolver at 0.36. It gives the right album, Abbey Road, only 0.04. The year follows the same guess: 1968 at 0.6, and the right year, 1969, at 0.07. Neither guess clears the bar of 0.8, so the form leaves both fields empty. The bar turns two wrong answers into two honest blanks.

## With context

The context run sends each song's catalog entry before its title. Jev then reads the answer from the text:

```sh
jq -r '.records[0].input' annotate-context.jsonl
```

```text
Catalog:
The Beatles (White Album) (1968-11-22)
Blackbird (lead: McCartney; written: Lennon–McCartney; 2:18; released 1968-11-22; first album: The Beatles (White Album))
Text: Blackbird
Catalog:
Abbey Road (1969-09-26)
Octopus's Garden (lead: Starr; written: Starkey; 2:51; released 1969-09-26; first album: Abbey Road)
Text: Octopus's Garden
```

`annotate-context-card.json` asks the same three questions. Each opens "The text gives a catalog entry and then names a song by the Beatles."

```sh
jq -c '.records[]' annotate-context.jsonl |
  thinkthen annotate annotate-context-card.json --jsonl --field /input --details --replay recording |
  jq -c '{song: (.input.input | split("\nText: ") | last), value, album: .answers.album.answer.probabilities}'
```

```json
{"song":"Blackbird","value":{"singer":"Paul","album":"White Album","year":"1968"},"album":{"Help!":0.0,"Rubber Soul":0.0,"Revolver":0.0,"White Album":1.0,"Abbey Road":0.0}}
{"song":"Octopus's Garden","value":{"singer":"Ringo","album":"Abbey Road","year":"1969"},"album":{"Help!":0.0,"Rubber Soul":0.0,"Revolver":0.0,"White Album":0.0,"Abbey Road":1.0}}
```

- `--jsonl` takes each line as one JSON record.
- `--field /input` names the part of the record Jev reads.

With the entry, all six fields fill, and all six are right. Octopus's Garden goes to Abbey Road at 1.0.

## What can go wrong

- **The shared traps.** A replay miss, a missing key, and a cache bound to another server can stop any command. [How to run the bench for free](/beatles-bench/run-it-for-free/#what-can-go-wrong) gives each exit code and its fix.
- **A blank is not a wrong answer.** A `null` field means Jev was not sure enough. Lower the threshold, and the field fills with its top pick. Here that would fill two wrong answers.

## The slide

Blackbird fills every field at the bar of 0.8. Each value is white with a green ✓. Octopus's Garden fills the singer at 0.91 and leaves the album and the year as not sure, in amber. Its album was Abbey Road, and its year was 1969.

## Related

- [How to pick one option with choose](/beatles-bench/choose/): each field here is one `choose` question.
- [How to link songs to singers and albums with relate](/beatles-bench/relate/): the same facts as links between names.
- [Retrieval-augmented decisions (RAD)](/beatles-bench/rad/): what the catalog entry fixes and what it costs.
