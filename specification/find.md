# `find`

Status: **Settled** by ADRs 0015 and 0030. Both accepted live gates passed.

Picks the unit that best answers a question, out of a set the model sees all at once.

```text
thinkthen find QUESTION [--lines|--jsonl] [--field POINTER] [--none] [--details] [BACKEND]
```

## What it reads

From 2 to 255 lines or records on standard input, or 2 to 254 with `--none`. `--input FILE` reads a file instead. Empty input succeeds without output or a request. One unit and either overflow are usage errors before any request. The whole original input may contain at most 16 MiB. [records.md](records.md) gives the framing and pointer rules.

`QUESTION` states what the best unit answers. CSV, TSV, and `--jobs` are absent from this command; the parser refuses them as unexpected arguments.

## One request

`find` sends every unit together in one request, with an id on each, and asks which one best answers the question. It is one request where `filter` and `rank` make one per unit.

The answer is relative. `find` picks the best unit present, and `filter` judges each unit alone against a fixed mark. The units see each other, and a user accepts that by choosing this verb. A job that needs each unit judged on its own merits uses `filter` or `rank`.

## What the live checks measured

The live run judged twenty made-up documents of 11 to 14 numbered lines, with 12 to 14 options. The cases are few and they are made up. `find` named the answering line on 16 of 16 answerable documents and `rank --top 1` named it on 15 of 16. `find` sent 20 requests and 11,063 input tokens. `rank --top 1` sent 239 requests and 69,143 input tokens.

Ticket 0040 then used made-up documents of 100, 175, and 250 units. Both find policies named the trusted unit on 6 of 6 answerable documents, with 2 of 2 at each size. `rank --top 1` also found all 6. `none` found all 3 blank documents and falsely refused 0 of 6 answerable documents. The reach stage also accepted 255 units without `none` and 254 with it. These samples are small and made up; they establish the accepted gate rather than a general accuracy rate.

The comparison made 12 live find requests and reused 6 reach recordings, billing 95,002 input tokens. Rank made 1,050 live requests and billed 322,935. Every correct find winner had probability at least 0.99. No find answer was wrong, so the run supplies no error from which to choose a probability floor.

## Saying that nothing fits

`--none` puts a `none` option beside the units. When the model picks it, `find` prints nothing on standard output, prints `null` under `--details`, and exits 3. ADR 0009 item 3 and demo 15 asked for that spelling, and it is the one `choose` already uses.

On the same twenty documents, a `none` option answered `none` on 4 of 4 documents with no answering line and on 0 of the 16 answerable documents. It sent 20 requests and 11,183 input tokens. Twenty documents are few and they are made up.

## What it prints

The chosen unit follows the shared preservation rules in [records.md](records.md), the way [filter.md](filter.md) prints a kept record. A line loses its input line ending and output adds one line feed, including for CRLF input or a last line with no ending. Under `--jsonl` the whole original record content returns with one output line feed. `find` prints one unit and never a list.

`--details` prints the object in [result.md](result.md). Its question verb and answer kind are `find`. The answer holds the selected generated unit id or `none` and every probability in input order. The value holds the original selected unit or `null`, and the threshold is `null`.

The generated unit id is `u` and the unit's one-based input position, zero-padded to three digits. The first unit is `u001`, and the 255th is `u255`. An answer key for `audit` names units by these ids.

## Options

| Option | Meaning | Default |
| --- | --- | --- |
| `--lines` or `--jsonl` | The framing | `--lines` |
| `--none` | Offers a `none` option, so the model can say that nothing fits | Off |
| `--field POINTER` | The part of each record the model sees | The whole record |
| `--details` | Prints the full result object | Off |
| `--input FILE` | Reads the units from a file | Standard input |
| `--dry-run` | Prints the plan and sends nothing | Off |
| Backend options | `--url` in short and long help, and `--model` in long help. See [backends.md](backends.md) | The two variables and `jev-latest` |

## Exit codes

0 when a unit was chosen, 3 under `--none` when the model says that nothing fits, and 2, 4, 5, and 70 as [channels.md](channels.md) gives them. An empty input exits 0 with no output and no request. Without `--none` the verb never exits 3.

## Examples

```sh
thinkthen find 'This passage explains the login timeout.' --lines < passages.txt
```

```sh
thinkthen find 'This ticket should be worked next.' --jsonl --field /body < queue.jsonl
```

## Selection rule

Equal top probabilities among real units select the first input unit. A strict `none` lead or any top tie involving `none` produces the unresolved result. The command prints one unit and has no threshold or top-count option.
