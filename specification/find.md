# `find`

Status: **Draft**, from Proposed ADR 0009. `find` is planned after `rank` and is not part of version one until the owner accepts that ADR.

Picks the unit that best answers a question, out of a set the model sees all at once.

```text
thinkthen find QUESTION [--lines|--jsonl] [--field POINTER] [--top N] [--details] [BACKEND]
```

## What it reads

Up to 255 lines or records on standard input. `--input FILE` reads a file instead. More than 255 units is a usage error before any request. [records.md](records.md) gives the framing and the pointer rules.

`QUESTION` states what the best unit answers.

## One request

`find` sends every unit together in one request, with an id on each, and asks which one best answers the question. It is one request where `filter` and `rank` make one per unit.

The answer is relative. `find` picks the best unit present, and `filter` judges each unit alone against a fixed mark. The units see each other, and a user accepts that by choosing this verb. A job that needs each unit judged on its own merits uses `filter` or `rank`.

## What it prints

The chosen unit as it arrived, byte for byte. `--top N` prints the `N` most likely units, best first. `--details` prints the object in [result.md](result.md), with the probability of every unit under `answer`.

## Options

| Option | Meaning | Default |
| --- | --- | --- |
| `--top N` | Prints the `N` most likely units, best first | 1 |
| `--lines` or `--jsonl` | The framing | `--lines` |
| `--field POINTER` | The part of each record the model sees | The whole record |
| `--details` | Prints the full result object | Off |
| `--input FILE` | Reads the units from a file | Standard input |
| `--dry-run` | Prints the plan and sends nothing | Off |
| Backend options | `--profile` and the advanced flags | The selected profile |

## Exit codes

0 when a unit was chosen, and 2, 4, 5, and 70 as [channels.md](channels.md) gives them. An empty input exits 0 with no output and no request.

## Examples

```sh
thinkthen find 'This passage explains the login timeout.' --lines < passages.txt
```

```sh
thinkthen find 'This ticket should be worked next.' --jsonl --field /body --top 3 < queue.jsonl
```

## Open point

- How does `find` say that nothing fits? Three shapes are open: a second yes/no question asked of the winner, a `none` option among the units, and a cut on the backend's confidence. Each costs something different, and a measurement settles it. Until then `find` always names a winner.
