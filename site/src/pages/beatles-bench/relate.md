---
layout: "../../layouts/BenchPage.astro"
title: "How to link songs to singers and albums with relate"
tagline: "relate links names into a graph."
slide: "/beatles-bench/img/relate.png"
source: "https://github.com/botassembly/beatles-bench/blob/48ac3f252c6161bb9518416a41b4c8a2d82a9434/examples/10-relate/README.md"
runsIn: "examples/10-relate"
---

Use `relate` when you have a set of named things and want the links between them. You give each name a kind and list the relations. `relate` asks which pairs each relation links and returns every edge with a probability. All the names go out together, so each question sees the whole set.

Here Jev links seven songs to four singers and three albums. Who sings each song? On which album did each song first appear?

## The files

- `relate.json` names the two relations.
- `relate-cold.jsonl` holds the fourteen names and the true edges, taken from `data/songs.tsv`.
- `recording/` holds the saved request and answer, so the command needs no key.

`relate.json` in full:

```sh
cat relate.json
```

```json
{
 "version": 1,
 "relate": {
  "relations": [
   {
    "name": "sung_by",
    "source": "song",
    "target": "person",
    "reads": "has its lead vocal sung by"
   },
   {
    "name": "appears_on",
    "source": "song",
    "target": "album",
    "reads": "first appeared on the album"
   }
  ]
 },
 "threshold": 0.01
}
```

- `version` is the file format. It is 1.
- `relations` lists each relation. `name` is the name that comes back. `source` and `target` are the kinds it links. `reads` is how Jev reads the link, as in "Taxman has its lead vocal sung by George Harrison".
- `threshold` is the bar for keeping an edge. At 0.01 it keeps almost every edge, so the output shows the weak ones too.

The names sit in the case file as records:

```sh
jq -c '.records[]' relate-cold.jsonl | head -5
```

```json
{"name":"John Lennon","kind":"person"}
{"name":"Paul McCartney","kind":"person"}
{"name":"George Harrison","kind":"person"}
{"name":"Ringo Starr","kind":"person"}
{"name":"Octopus's Garden","kind":"song"}
```

`truth` in the same case lists the fourteen true edges. `fields` names the row and column of `data/songs.tsv` behind each one: `lead_vocals` for a singer and `first_album` for an album.

Everything in the folder:

- `relate-cold.jsonl`: the one case, with the fourteen names and the true edges, written by `scripts/generate/examples.py`.
- `relate.json`: the two relations and the threshold.
- `recording/`, `outputs.jsonl`, `timing.tsv`: the requests, the committed answer, and the wall time.
- `run.sh live OUT | replay [OUT]`: asks the case through `scripts/run/functions.sh`.

## Run it

Run this in `examples/10-relate`:

```sh
jq -c '.records[]' relate-cold.jsonl |
  thinkthen relate @relate.json --jsonl --replay recording |
  jq -c '[.relation, .source.name, .target.name, .probability]'
```

```json
["sung_by","Octopus's Garden","George Harrison",0.02]
["sung_by","Octopus's Garden","Ringo Starr",0.97]
["sung_by","Something","John Lennon",0.04]
["sung_by","Something","Paul McCartney",0.06]
["sung_by","Something","George Harrison",0.81]
["sung_by","Come Together","John Lennon",0.89]
["sung_by","Here Comes the Sun","John Lennon",0.01]
["sung_by","Here Comes the Sun","Paul McCartney",0.02]
["sung_by","Here Comes the Sun","George Harrison",0.93]
["sung_by","Yesterday","John Lennon",0.01]
["sung_by","Yesterday","Paul McCartney",0.93]
["sung_by","Eleanor Rigby","John Lennon",0.03]
["sung_by","Eleanor Rigby","Paul McCartney",0.81]
["sung_by","Eleanor Rigby","Ringo Starr",0.01]
["sung_by","Taxman","John Lennon",0.03]
["sung_by","Taxman","Paul McCartney",0.11]
["sung_by","Taxman","George Harrison",0.8]
["appears_on","Octopus's Garden","Abbey Road",0.23]
["appears_on","Octopus's Garden","Revolver",0.73]
["appears_on","Something","Abbey Road",1.0]
["appears_on","Come Together","Abbey Road",1.0]
["appears_on","Here Comes the Sun","Abbey Road",1.0]
["appears_on","Yesterday","Abbey Road",0.01]
["appears_on","Yesterday","Help!",0.56]
["appears_on","Yesterday","Revolver",0.3]
["appears_on","Eleanor Rigby","Revolver",0.99]
["appears_on","Taxman","Abbey Road",0.01]
["appears_on","Taxman","Help!",0.02]
["appears_on","Taxman","Revolver",0.96]
```

- `@relate.json` reads the relations from the file.
- `--jsonl` takes each line as one named thing, with its `name` and `kind`.
- `--replay recording` answers from the saved recording. No request leaves the machine.

Each line is one edge: the relation, the song, the singer or album, and the probability.

To ask your own server, leave out `--replay recording` and follow [Ask your own server](/beatles-bench/run-it-for-free/#ask-your-own-server). The answers can then differ by a few points from these.

## Read it

`outputs.jsonl` holds the same edges as the run above. This command marks each edge at a bar of 0.5. An edge is right when it is true and clears the bar. It is wrong when it is false and clears the bar. It is missed when it is true and falls under the bar. The false edges under the bar are left out:

```sh
jq -c --slurpfile out outputs.jsonl '
  .truth as $truth | $out[0].rows[0].value[]
  | [.relation, .source.name, .target.name] as $e | ($truth | any(. == $e)) as $true
  | select($true or .probability >= 0.5)
  | {relation, song: .source.name, target: .target.name, p: .probability,
     mark: (if $true and .probability >= 0.5 then "right" elif $true then "missed" else "wrong" end)}
' relate-cold.jsonl
```

```json
{"relation":"sung_by","song":"Octopus's Garden","target":"Ringo Starr","p":0.97,"mark":"right"}
{"relation":"sung_by","song":"Something","target":"George Harrison","p":0.81,"mark":"right"}
{"relation":"sung_by","song":"Come Together","target":"John Lennon","p":0.89,"mark":"right"}
{"relation":"sung_by","song":"Here Comes the Sun","target":"George Harrison","p":0.93,"mark":"right"}
{"relation":"sung_by","song":"Yesterday","target":"Paul McCartney","p":0.93,"mark":"right"}
{"relation":"sung_by","song":"Eleanor Rigby","target":"Paul McCartney","p":0.81,"mark":"right"}
{"relation":"sung_by","song":"Taxman","target":"George Harrison","p":0.8,"mark":"right"}
{"relation":"appears_on","song":"Octopus's Garden","target":"Abbey Road","p":0.23,"mark":"missed"}
{"relation":"appears_on","song":"Octopus's Garden","target":"Revolver","p":0.73,"mark":"wrong"}
{"relation":"appears_on","song":"Something","target":"Abbey Road","p":1.0,"mark":"right"}
{"relation":"appears_on","song":"Come Together","target":"Abbey Road","p":1.0,"mark":"right"}
{"relation":"appears_on","song":"Here Comes the Sun","target":"Abbey Road","p":1.0,"mark":"right"}
{"relation":"appears_on","song":"Yesterday","target":"Help!","p":0.56,"mark":"right"}
{"relation":"appears_on","song":"Eleanor Rigby","target":"Revolver","p":0.99,"mark":"right"}
{"relation":"appears_on","song":"Taxman","target":"Revolver","p":0.96,"mark":"right"}
```

All seven singers are right. Octopus's Garden goes to Ringo Starr at 0.97. Six of the seven albums are right. Yesterday clears the bar for Help! at 0.56.

Octopus's Garden gets its singer right and its album wrong. Jev links it to Revolver at 0.73, over the bar. The true album, Abbey Road, gets 0.23 and is missed. The same song goes wrong the same way on [the annotate page](/beatles-bench/annotate/). There Jev puts it on the White Album or Revolver and gives Abbey Road 0.04. From memory, Jev knows who sings Octopus's Garden and not where it first appeared.

The request's size:

```sh
jq -c '{input_tokens, output_tokens}' outputs.jsonl
```

```json
{"input_tokens":3453,"output_tokens":748}
```

## With context

None. Context for `relate` means the whole catalog beside the names. A `relate` request with the whole catalog passes Jev's input limit.

## What can go wrong

- **The shared traps.** A replay miss, a missing key, and a cache bound to another server can stop any command. [How to run the bench for free](/beatles-bench/run-it-for-free/#what-can-go-wrong) gives each exit code and its fix.
- **The set must be complete.** `relate` links only the names you give. Leave out Abbey Road, and no edge can reach it.
- **One request holds the whole set.** A large set makes one large request. It can pass the backend's input limit.
- **An edge can cross the bar from one call to the next.** An earlier run put the Revolver edge for Octopus's Garden over 0.8. The fresh run puts it at 0.73. Set the bar with that in mind.

## The slide

The slide draws only the edges Jev asserts at 0.5 or more. A green edge is one Jev asserts and `data/songs.tsv` agrees with. A red edge is one Jev asserts and the table does not hold. A missing edge is one Jev gave less than 0.5, so the slide leaves it out. The slide draws no dashed lines. Yesterday links to Help! in green. Octopus's Garden links to Revolver in red.

## Related

- [How to find names in a sentence with recognize](/beatles-bench/recognize/): find the names first.
- [How to fill in a form with annotate](/beatles-bench/annotate/): the same facts asked of one song at a time.

## Image credits

The four faces are crops of one photo: United Press International (UPI Telephoto), 7 February 1964, via the Library of Congress (cph.3c11094), https://commons.wikimedia.org/wiki/File:The_Beatles_members_at_New_York_City_in_1964.jpg. The crops are John_Lennon_NY_1964.png, Paul_McCartney_NY_1964.png, George_Harrison_NY_1964.png, and Ringo_Starr_NY_1964.png on Wikimedia Commons, cropped by Zakke and retouched by Indopug and Misterweiss. The photo was published in the US in 1964 with no copyright notice. It is public domain in the US (PD-US-no notice). Other countries may still protect it.
