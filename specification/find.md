# `find`

Status: **Settled**. A live run compared `find` with `rank --top 1` on the same units, and ADR 0015 accepted `find` on that evidence. `sdlc/records/0011-the-live-probe.md` holds every number. `find` is slice 11 of `sdlc/planning/plan.md`.

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

## What one run measured

The live run judged twenty made-up documents of 11 to 14 numbered lines, with 12 to 14 options. The cases are few and they are made up. `find` named the answering line on 16 of 16 answerable documents and `rank --top 1` named it on 15 of 16. `find` sent 20 requests and 11,063 input tokens. `rank --top 1` sent 239 requests and 69,143 input tokens.

Those documents run far under the 255 units this page allows, so nothing here says how the pick behaves on a long document. The ticket that builds `find` first repeats the comparison on documents of 100 to 250 lines. This page then states the largest size that held, and the limit drops to that size when the larger documents fail.

## Saying that nothing fits

`--none` puts a `none` option beside the units. When the model picks it, `find` prints nothing on standard output, prints `null` under `--details`, and exits 3. ADR 0009 item 3 and demo 15 asked for that spelling, and it is the one `choose` already uses.

On the same twenty documents, a `none` option answered `none` on 4 of 4 documents with no answering line and on 0 of the 16 answerable documents. It sent 20 requests and 11,183 input tokens. Twenty documents are few and they are made up.

## What it prints

The chosen unit as it arrived, byte for byte, the way [filter.md](filter.md) prints a kept record. Under `--jsonl` that is the whole record. `find` prints one unit and never a list.

`--details` prints the object in [result.md](result.md). The answer kind is an open point below, because no kind in `result.md` carries a pick over units.

## Options

| Option | Meaning | Default |
| --- | --- | --- |
| `--lines` or `--jsonl` | The framing | `--lines` |
| `--none` | Offers a `none` option, so the model can say that nothing fits | Off |
| `--field POINTER` | The part of each record the model sees | The whole record |
| `--details` | Prints the full result object | Off |
| `--input FILE` | Reads the units from a file | Standard input |
| `--dry-run` | Prints the plan and sends nothing | Off |
| Backend options | `--url` and `--model`, in the long help alone. See [backends.md](backends.md) | The two variables and `jev-latest` |

## Exit codes

0 when a unit was chosen, 3 under `--none` when the model says that nothing fits, and 2, 4, 5, and 70 as [channels.md](channels.md) gives them. An empty input exits 0 with no output and no request. Without `--none` the verb never exits 3.

## Examples

```sh
thinkthen find 'This passage explains the login timeout.' --lines < passages.txt
```

```sh
thinkthen find 'This ticket should be worked next.' --jsonl --field /body < queue.jsonl
```

## Open points

- Which answer kind does `find` print under `--details`? `yes_no` carries one probability and `choice` carries one per option, and neither names a unit that arrived on standard input. Recommendation: a kind that names the chosen unit's id and carries a probability per unit.
- Does `find` print more than one unit? Demo 15 wanted the three best lines and found the option free, because one request already answered the whole page. No ADR names such an option, so `find` prints one unit until one does.
