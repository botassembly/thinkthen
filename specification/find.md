# `find`

Status: **Draft**. ADR 0010 keeps `find` out of version one. Nothing is built from this page until a live run compares `find` with `rank --top 1` on the same units. If `find` picks as well, it enters. If it picks worse, it moves to the roadmap.

Picks the unit that best answers a question, out of a set the model sees all at once.

```text
thinkthen find QUESTION [--lines|--jsonl] [--field POINTER] [--details] [BACKEND]
```

## What it reads

Up to 255 lines or records on standard input. `--input FILE` reads a file instead. More than 255 units is a usage error before any request. [records.md](records.md) gives the framing and the pointer rules.

`QUESTION` states what the best unit answers.

## One request

`find` sends every unit together in one request, with an id on each, and asks which one best answers the question. It is one request where `filter` and `rank` make one per unit.

The answer is relative. `find` picks the best unit present, and `filter` judges each unit alone against a fixed mark. The units see each other, and a user accepts that by choosing this verb. A job that needs each unit judged on its own merits uses `filter` or `rank`.

## What it prints

The chosen unit as it arrived, byte for byte, the way [filter.md](filter.md) prints a kept record. Under `--jsonl` that is the whole record. `find` prints one unit and never a list.

`--details` prints the object in [result.md](result.md). The answer kind is an open point below, because no kind in `result.md` carries a pick over units.

## Options

| Option | Meaning | Default |
| --- | --- | --- |
| `--lines` or `--jsonl` | The framing | `--lines` |
| `--field POINTER` | The part of each record the model sees | The whole record |
| `--details` | Prints the full result object | Off |
| `--input FILE` | Reads the units from a file | Standard input |
| `--dry-run` | Prints the plan and sends nothing | Off |
| Backend options | `--profile` and the advanced flags | The selected profile |

## Exit codes

0 when a unit was chosen, and 2, 4, 5, and 70 as [channels.md](channels.md) gives them. An empty input exits 0 with no output and no request. Whether `find` ever exits 3 rides on the first open point below.

## Examples

```sh
thinkthen find 'This passage explains the login timeout.' --lines < passages.txt
```

```sh
thinkthen find 'This ticket should be worked next.' --jsonl --field /body < queue.jsonl
```

## Open points

- How does `find` say that nothing fits? Three shapes are open: a second yes/no question asked of the winner, a `none` option among the units, and a cut on the backend's confidence. Each costs something different, and a measurement settles it. ADR 0009 item 3 names demo 15's argument for the `none` option, which spells the outcome the way `choose` does: `null`, nothing on standard output, and exit 3.
- Which answer kind does `find` print under `--details`? `yes_no` carries one probability and `choice` carries one per option, and neither names a unit that arrived on standard input. Recommendation: a kind that names the chosen unit's id and carries a probability per unit.
- Does `find` print more than one unit? Demo 15 wanted the three best lines and found the option free, because one request already answered the whole page. No ADR names such an option, so `find` prints one unit until one does.
