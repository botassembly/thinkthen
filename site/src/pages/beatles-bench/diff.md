---
layout: "../../layouts/BenchPage.astro"
title: "How to see what changed with diff"
tagline: "diff shows what changed."
slide: "/beatles-bench/img/diff.png"
source: "https://github.com/botassembly/beatles-bench/blob/48ac3f252c6161bb9518416a41b4c8a2d82a9434/examples/12-diff/README.md"
runsIn: "examples/12-diff"
---

Use `diff` when you have two runs of the same records and want to know what changed. It lists the saved answers that changed between the two runs. With an answer key, it marks each change. It ends with a summary. It sends no request and needs no API key.

[`11-audit`](/beatles-bench/audit/) asked 70 Beatles songs one question, "Is this song on the album Abbey Road?", in two runs. The cold run sends the title alone, so Jev answers from memory. The context run sends each song's catalog entry before the title. Which answers change when the facts are in the text, and does each change fix a mistake or make one?

This page reads every answer at the band `--threshold 0.2:0.8`. A probability of 0.8 or more reads yes. A probability under 0.2 reads no. Anything between reads not sure. At that band, 48 of 70 answers change. Right answers go from 20 to 68. None goes wrong.

## The files

- `../11-audit/rows.jsonl` holds the cold answers, one line per song.
- `../11-audit/rows-context.jsonl` holds the answers with the catalog entry.
- `../11-audit/key.jsonl` holds the key, taken from `first_album` in `data/songs.tsv`.

[How to find your bar with audit](/beatles-bench/audit/) shows a line of each. diff needs no recording. It reads saved answers and sends no request.

Everything in the folder:

- `diff-band.jsonl`: each change at the band `0.2:0.8`, then the summary, as `diff` prints them. `diff.jsonl`: the same at the single bar 0.5.
- `diff.sh [IN [OUT]]`: writes `OUT/a.jsonl` and `OUT/b.jsonl` from `IN/rows.jsonl` and `IN/rows-context.jsonl`, then `OUT/diff.jsonl` and `OUT/diff-band.jsonl` through `scripts/score/diff_guard.sh --no-digest`. `IN` defaults to `11-audit` and `OUT` to this folder. Only the two diff files are committed.
- `run.sh [IN [OUT]]`: runs `diff.sh` from `IN` into `OUT`, by default from `11-audit` into `replay/`. It needs no mode, no API key, and no recording.

## Run it

Run this in `examples/12-diff`. The two runs send different text, so the first step sets each line's input to the song's title. That gives both runs and the key one shared id:

```sh
title='.input = {id: (.input.input | split("\nText: ") | last)}'
jq -c "$title" ../11-audit/rows.jsonl > a.jsonl
jq -c "$title" ../11-audit/rows-context.jsonl > b.jsonl
thinkthen diff a.jsonl b.jsonl --threshold 0.2:0.8 --key ../11-audit/key.jsonl |
  jq -c '.summary // empty | {records, changed, right_a, right_b, gained, lost}'
```

```json
{"records":70,"changed":48,"right_a":20,"right_b":68,"gained":3,"lost":0}
```

- `--threshold 0.2:0.8` reads each answer as yes, no, or not sure in both runs.
- `--key` marks each change against the right answer.
- `--table` prints the same report for a person. Without it, diff prints one JSON object per change and then a summary. `jq` picks the summary fields.

Leave out `--key`, and diff still lists every change. `diff.sh` writes the JSON form to `diff-band.jsonl`. `run.sh` writes it to `replay/`.

## Read it

diff marks each change with an effect. `gained` means wrong in the cold run and right with context. `resolved` means not sure in the cold run and right with context. `lost` means right before and wrong after.

```sh
jq -sc 'map(select(.id)) | group_by(.effect) | map({effect: .[0].effect, songs: length})' diff-band.jsonl
```

```json
[{"effect":"gained","songs":3},{"effect":"resolved","songs":45}]
```

Three answers go from wrong to right. Forty-five go from not sure to right. None breaks. These are the three Jev was sure of from memory and got wrong:

```sh
jq -c 'select(.effect == "gained") | {song: .id, cold: .probability[0], context: .probability[1], key}' diff-band.jsonl
```

```json
{"song":"A Day in the Life","cold":0.97,"context":0.12,"key":"no"}
{"song":"The Long and Winding Road","cold":0.92,"context":0.18,"key":"no"}
{"song":"Lovely Rita","cold":0.81,"context":0.02,"key":"no"}
```

A Day in the Life first came out on Sgt. Pepper's Lonely Hearts Club Band, and so did Lovely Rita. The Long and Winding Road came out on Let It Be. No bar on the cold run could fix A Day in the Life without losing real Abbey Road songs. The catalog entry fixes it.

## With context

The context run is the second side of this comparison. [`11-audit`](/beatles-bench/audit/) shows its answers. With the catalog entry, every answer sits far from the middle. [`11-audit`](/beatles-bench/audit/#with-context) gives the gap between the yes and no answers. [`01-decide`](/beatles-bench/decide/) shows the same effect on six of these songs.

## What can go wrong

- **The shared traps.** A replay miss, a missing key, and a cache bound to another server can stop any command. [How to run the bench for free](/beatles-bench/run-it-for-free/#what-can-go-wrong) gives each exit code and its fix.
- **The ids must pair.** diff pairs answers by record id. The two runs here have different ids (`audit-01` and `audit-context-01`), so the first step sets each id to the song's title. Without that step nothing pairs, and diff still exits 0.
- **Different questions pair without a word.** diff does not check that both runs asked the same question. `diff.sh` passes `--no-digest` to its guard because these two runs word the question differently by design.
- **No key, no verdict.** Leave out `--key`, and diff lists the changes without saying which fixed a mistake.
- **A single bar hides not sure.** At `--threshold 0.5` every answer reads yes or no, so fewer answers change:

```sh
jq -c '.summary // empty | {records, changed, right_a, right_b, gained, lost}' diff.jsonl
```

```json
{"records":70,"changed":20,"right_a":50,"right_b":70,"gained":20,"lost":0}
```

At 0.5, 20 answers change, and every one is a fix. The band shows more. It also counts the 45 answers Jev was not sure of from memory.

## The slide

The slide shows 12 of the 70 songs, by title. The answer column gives the truth from `data/songs.tsv` (`first_album`). The next two columns give the probability of yes with no context and with the catalog entry. Every value is white, or muted on a dim row. A mark beside each value carries the colour.

The slide reads every answer at the band `0.2:0.8`, as this page does. Its label at the top right says "THRESHOLD 0.2:0.8". A red ✗ marks an answer Jev is sure of and gets wrong. An amber ? marks an answer Jev is not sure of. A green ✓ marks an answer Jev is sure of and gets right.

The bright rows with an arrow are the nine of these 12 that diff lists at the band. Two go from wrong to right. They are A Day in the Life and The Long and Winding Road. Seven go from not sure to right. They are Blackbird, Get Back, Glass Onion, In My Life, Something, Taxman, and Ticket to Ride. The dim rows kept the same answer in both runs.

## Related

- [How to find your bar with audit](/beatles-bench/audit/): the two runs and the key.
- [Retrieval-augmented decisions (RAD)](/beatles-bench/rad/): what the catalog entry fixes and what it costs.
