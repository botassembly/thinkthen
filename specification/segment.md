# `segment`

Status: **Settled** for the grammar, the threshold, and the output. **Draft** for `--window`.

Cuts one document into segments at the boundaries a question finds.

```text
thinkthen segment QUESTION [--threshold T] [--units lines|paragraphs] [--window N] [--details] [BACKEND]
```

## What it reads

One text document on standard input, read to its end as UTF-8. `--input FILE` reads a file instead. An empty document is a usage error. `segment` reads no record stream, so `--lines` and `--jsonl` are usage errors.

The tool splits the document into units and asks one yes/no question at each gap between two units. `QUESTION` states what makes a unit the start of a new part. The model never computes an offset, and the tool counts the lines itself.

## What it prints

One JSON object per segment, in document order.

```json
{"start_line":1,"end_line":12,"start_unit":1,"end_unit":3,"text":"Hi, my payouts have been failing.\n\nI tried again this morning.\n\nIt failed the same way."}
```

Line and unit numbers are one-based and inclusive. Line numbers are present for both unit kinds, so `sed -n '1,12p'` can cut the file. The segments never overlap, and together they cover every unit.

`--details` prints the object in [result.md](result.md) for each boundary question, with an `answer.kind` of `yes_no`.

`segment` produces a flat list. It assigns no parent, no heading level, and no role.

## Options

| Option | Meaning | Default |
| --- | --- | --- |
| `--threshold T` | A single cut. A gap is a boundary when the probability reaches the cut. The band form is a usage error | `0.5` |
| `--units lines\|paragraphs` | What counts as one unit. A paragraph ends at a blank line | `lines` |
| `--window N` | Draft. How many units of context travel on each side of the gap. `0` sends the whole document | `0` |
| `--details` | Prints one result object per boundary question | Off |
| `--input FILE` | Reads the document from a file | Standard input |
| `--dry-run` | Prints the plan and sends nothing | Off |
| Backend options | `--profile` and the advanced flags | The selected profile |

## Exit codes

0 when the run finished, and 2, 4, 5, and 70 as [channels.md](channels.md) gives them. A backend failure ends the run at exit 4 and nothing prints, because a partial cover is not a cover.

## Examples

```sh
thinkthen segment 'A new request begins at this line.' --units lines < thread.txt
```

```sh
thinkthen segment 'A new topic begins at this paragraph.' --units paragraphs --threshold 0.9 --window 1 < transcript.txt
```

## Cautions

A single cut never says the model is sure of a join. A gap under the cut is a gap that did not reach the mark. [threshold.md](threshold.md) says more.

A document over the backend's token limit for one request is refused, and the exit code is 4. A window smaller than the document is the answer.

## Open points

- What does `--window N` send? Recommendation: at `0` the whole document travels once and every boundary question rides in that one request. At `N` above zero, each boundary question travels with the `N` units before the gap and the `N` units after it, and questions that share identical evidence still ride in one request.
- What is the default? Recommendation: `0`, which costs the fewest requests. The help says to raise it when a document is too large for one request.
- Do the line numbers of a `--window` run still count from the whole document? Recommendation: yes. The window changes what the model sees and never what the tool reports.
