# `segment`

Status: **Settled** for version one, by ADR 0007. **Draft** for the single-request form, from Proposed ADR 0009.

Cuts one document into segments at the boundaries a question finds.

```text
thinkthen segment QUESTION [--threshold T] [--units lines|paragraphs] [--details] [BACKEND]
```

## What it reads

One text document on standard input, read to its end as UTF-8. `--input FILE` reads a file instead. An empty document is a usage error. `segment` reads no record stream, so `--lines` and `--jsonl` are usage errors.

The tool splits the document into units and gives every unit an id. `QUESTION` states what makes a unit the start of a new part. The model never computes an offset, and the tool counts the lines itself.

## One request

Draft, from Proposed ADR 0009. The whole document travels once, with the unit ids marked in it, and one yes/no question per gap rides in that same request. This is the vendor's own measured recipe. The evidence is billed once, and the questions of one request are answered independently.

The tool appends the two unit ids to the user's question, so each question names the gap it asks about. `--dry-run` shows the exact text of every question, and a user who dislikes the wording reads it before paying for it.

A document too large for one request is refused before any request goes out, at exit 2. The message names the token limit in [records.md](records.md). Splitting the document upstream is the answer.

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
| `--details` | Prints one result object per boundary question | Off |
| `--input FILE` | Reads the document from a file | Standard input |
| `--dry-run` | Prints the plan, with every boundary question as it would be sent | Off |
| Backend options | `--profile` and the advanced flags | The selected profile |

## Exit codes

0 when the run finished, and 2, 4, 5, and 70 as [channels.md](channels.md) gives them. A backend failure ends the run at exit 4 and nothing prints, because a partial cover is not a cover.

## Examples

```sh
thinkthen segment 'A new request begins at this line.' --units lines < thread.txt
```

```sh
thinkthen segment 'A new topic begins at this paragraph.' --units paragraphs --threshold 0.9 < transcript.txt
```

```sh
thinkthen segment 'A new request begins at this line.' --dry-run < thread.txt | jq '.request.questions'
```

## Cautions

A single cut never says the model is sure of a join. A gap under the cut is a gap that did not reach the mark. [threshold.md](threshold.md) says more.

Every unit sees every other unit, because the whole document rides in one request. A job that must judge each unit alone uses `filter` over `--lines`.
