# The help's first lines and the public words

Status: Open

The marketing side read `thinkthen --help` and each verb's `-h` as a new user on 2026-09-21, at main `907f252`. Ian approved the public vocabulary the same day. This page gives the builder the words for the release-preparation work on help and the manual. It authorizes nothing.

## What worked

All eight verbs took a plain first example under `--dry-run` with no key set. Each printed a plan and exited 0. The plan is readable, and it shows a new user exactly what would leave the machine.

## The first screen

`thinkthen --help` opens with "Put a decider model in the shell". Ian ruled the pitch on 2026-09-20: **semantic commands**. A reader who meets the tool through the announcement sees one phrase there and another here. Suggested opening line:

    Semantic commands for the shell: if, grep, and sort that understand meaning

The command list reads best as one short line each. Two lines today are long and speak to the designer:

- `rank`: "Print the records with the most likely yes first: one yes/no question of each record, a local sort, and no comparison of two records"
- `find`: "Pick the best unit. Every unit leaves together and sees every other unit"

Both cautions are true and they matter. They belong in the long help, under the first line.

Suggested first lines. Each doubles as the slide title and the table cell on the site:

| Verb | Today | Suggested |
| --- | --- | --- |
| `decide` | Answer a yes/no question about the evidence and set the exit code | Answer one yes or no question about a text, and set the exit code |
| `filter` | Keep the records that reach the mark | Keep the records where the answer is yes |
| `rank` | The long line above | Sort records by how likely the answer is yes |
| `choose` | Pick one label from a fixed list and print it | Pick one option from your list |
| `find` | The line above | Pick the one line or record that best answers a question |
| `score` | Place the evidence on named levels and print the number | Place a text on a scale you name |
| `tag` | Return every applicable label as one JSON array | Print every label that fits |
| `annotate` | Ask every question in a saved set and print one annotated JSON object | Fill out a question set for every record |

The list order today is decide, choose, tag, score, filter, rank, find, annotate. Every talk and page teaches by the model's three question types: decide, filter, rank, then choose, find, then score, then tag and annotate. The help in that order teaches the same map.

## Three word choices

| The help says | The public pages say | Note |
| --- | --- | --- |
| evidence | text | "Evidence" is exact inside the specification. A first-time reader has a message or a file, and calls it text |
| label, for a `choose` pick | option | The argument is already named `[OPTIONS]`, and `tag` owns "label". One word for each keeps the two verbs apart |
| unit | line or record | `find` alone says "unit". The short help can say "line or record" and leave "unit" to the specification |

"Unresolved" stays the exact term in the specification and in `--details`. The public pages say "not sure" and give "unresolved" once as the formal name. No rename is asked for.

"The mark" in `filter` is a fourth word for the threshold. "Threshold" serves.

## One layout slip

`tag -h` and `annotate -h` print two example lines above the description. The other six verbs open with the description. One layout for all eight: description, usage, then examples.

## What Ian can overturn

All of it. The builder owns the help text. The vocabulary is in the marketing repository at `products/thinkthen/vocabulary.md`, and Ian approved it on 2026-09-21.
