---
layout: "../../layouts/BenchPage.astro"
title: "How to sort records by a question with rank"
tagline: "rank sorts by your criteria."
slide: "/beatles-bench/img/rank.png"
source: "https://github.com/botassembly/beatles-bench/blob/48ac3f252c6161bb9518416a41b4c8a2d82a9434/examples/06-rank/README.md"
runsIn: "examples/06-rank"
---

Use `rank` when you want every record kept and sorted by how likely the answer is yes. It asks one yes or no question of each record and sorts by the probability of yes. It never compares two records with each other.

Here Jev sorts twelve songs by one question: "It is one of the Beatles' biggest hits."

## The files

- `recording/` holds the saved requests and answers, so the command needs no key.
- `rank-cold.jsonl` holds the cases, one song each, with its Wikipedia page views for 2024 as `truth`.

The question sits on the command line, so this example needs no question file. The page views come from `views_2024` in `data/songs.tsv`. They are a loose check. The bench holds no answer key for this question.

Everything in the folder:

- `rank-cold.jsonl`: the cases, one song each, with 2024 page views as `truth`, written by `scripts/generate/examples.py`.
- `lists/rank-cold.jsonl`: the sorted list, as the `rank` command prints it.
- `recording/`, `outputs.jsonl`, `timing.tsv`: the requests, the committed answers, and the wall times.
- `run.sh live OUT | replay [OUT]`: asks the cases through `scripts/run/functions.sh`.

## Run it

Run this in `examples/06-rank`:

```sh
printf '%s\n' Something 'Hey Jude' Blackbird 'Help!' "Octopus's Garden" 'She Loves You' Piggies Yesterday \
    'Penny Lane' 'Her Majesty' "Can't Buy Me Love" 'Good Night' |
  thinkthen rank "The text is the title of a song by the Beatles. It is one of the Beatles' biggest hits." \
    --lines --details --replay recording |
  jq -c '{song: .input, yes: .answer.probability}'
```

```json
{"song":"Hey Jude","yes":0.97}
{"song":"Help!","yes":0.93}
{"song":"She Loves You","yes":0.93}
{"song":"Yesterday","yes":0.9}
{"song":"Can't Buy Me Love","yes":0.84}
{"song":"Penny Lane","yes":0.73}
{"song":"Something","yes":0.57}
{"song":"Blackbird","yes":0.45}
{"song":"Piggies","yes":0.26}
{"song":"Octopus's Garden","yes":0.25}
{"song":"Good Night","yes":0.12}
{"song":"Her Majesty","yes":0.11}
```

- `--lines` takes each line of input as one record.
- `--details` prints each record with its probability. Without it, `rank` prints the songs alone, most likely first.
- `--replay recording` answers from the saved recording. No request leaves the machine.

To ask your own server, leave out `--replay recording` and follow [Ask your own server](/beatles-bench/run-it-for-free/#ask-your-own-server). The answers can then differ by a few points from these.

## Read it

The bench holds no chart data, so no command can mark this order right or wrong. Page views for 2024 give a loose check. This command sets Jev's place for each song beside its place by views:

```sh
jq -sc --slurpfile out outputs.jsonl '
  ($out | map({(.id): .rows[0].answer.probability}) | add) as $yes
  | (sort_by(-.truth) | map(.title)) as $views
  | sort_by(-$yes[.id]) | to_entries[] | .value.title as $t
  | {song: $t, jev_rank: (.key + 1), views_rank: ($views | index($t) + 1), views_2024: .value.truth, yes: $yes[.value.id]}
' rank-cold.jsonl
```

```json
{"song":"Hey Jude","jev_rank":1,"views_rank":1,"views_2024":477821,"yes":0.97}
{"song":"Help!","jev_rank":2,"views_rank":6,"views_2024":114277,"yes":0.93}
{"song":"She Loves You","jev_rank":3,"views_rank":7,"views_2024":103629,"yes":0.93}
{"song":"Yesterday","jev_rank":4,"views_rank":3,"views_2024":239052,"yes":0.9}
{"song":"Can't Buy Me Love","jev_rank":5,"views_rank":9,"views_2024":87105,"yes":0.84}
{"song":"Penny Lane","jev_rank":6,"views_rank":5,"views_2024":158021,"yes":0.73}
{"song":"Something","jev_rank":7,"views_rank":4,"views_2024":232863,"yes":0.57}
{"song":"Blackbird","jev_rank":8,"views_rank":2,"views_2024":470321,"yes":0.45}
{"song":"Piggies","jev_rank":9,"views_rank":11,"views_2024":35806,"yes":0.26}
{"song":"Octopus's Garden","jev_rank":10,"views_rank":8,"views_2024":103246,"yes":0.25}
{"song":"Good Night","jev_rank":11,"views_rank":10,"views_2024":40871,"yes":0.12}
{"song":"Her Majesty","jev_rank":12,"views_rank":12,"views_2024":33769,"yes":0.11}
```

Hey Jude leads both lists. Help!, She Loves You, and Can't Buy Me Love land near the top for Jev and lower by views.

Blackbird shows where the two lists part. It has the second most views and ranks eighth for Jev, at 0.45. The bench holds no sales or chart data to say which list is closer. Octopus's Garden sits tenth for Jev and eighth by views.

## With context

None. A catalog entry gives the singer, the writers, the length, the date, and the album. It says nothing about sales or charts.

## What can go wrong

- **The shared traps.** A replay miss, a missing key, and a cache bound to another server can stop any command. [How to run the bench for free](/beatles-bench/run-it-for-free/#what-can-go-wrong) gives each exit code and its fix.
- **Ties keep their input order.** Help! and She Loves You both sit at 0.93. `rank` keeps the order they came in.
- **The question sets the order.** `rank` sorts by one yes or no question. "Biggest hit" and "most popular today" are two questions and give two orders.

## The slide

The bars show each song's probability. The likeliest hits are bright. The slide draws a bar between Can't Buy Me Love at 0.84 and Penny Lane at 0.73. The five songs over it are white, and the seven under it are dim. The bench holds no answer key for rank. So no row is green or red.

## Related

- [How to keep the records that match with filter](/beatles-bench/filter/): keep only the records over a bar.
- [How to place a text on a scale with score](/beatles-bench/score/): a number for each record in place of an order.
