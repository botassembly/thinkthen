# `decide`

Status: **Settled** for version one, by ADR 0007.

Answers a yes/no question about the evidence and sets the exit code.

```text
thinkthen decide QUESTION|@FILE [--true TEXT] [--false TEXT] [--threshold T|LOW:HIGH] [--quiet] [--details] [RECORD] [BACKEND]
```

## What it reads

One text document on standard input, read to its end as UTF-8. `--input FILE` reads a file instead. `--lines`, `--jsonl`, `--csv`, and `--tsv` turn the input into records, and [records.md](records.md) gives the rules.

`QUESTION` is one argument. It states a fact that is true or false of the evidence. The model reads it as the question. A question that is empty or holds only white space is a usage error. `@FILE` reads the question from a question file instead, and [question-file.md](question-file.md) holds the grammar, the defaults, and the precedence. An empty document is a usage error, because a judgment about nothing is a mistake in the pipeline.

By default a stream of records shares requests, filling each to the smaller of the request-size setting and a backend profile limit. `--max-request-bytes N` sets that size for this command, with `THINKTHEN_MAX_REQUEST_BYTES` next and 96,000 bytes at every address by default; see [settings.md](settings.md). The evidence of a batch is one fixed sentence, and each record appears once, inside its own question. Every run moves a few answers, and batching moves a few more. ADR 0055 records local experiment 275, which asked four yes/no questions over the 306 Beatles songs. It ran each batched form three times on the same bytes and once on each of three shuffled record orders. The batched form stayed within 4 right answers across repeats and orders on every task. It never fell more than 3 right answers below one song a request. On "It appears on the album Abbey Road" it scored 283 to 287 right of 306, where one title a request scored 286. It sent 11,468 input tokens for the 306 titles, where one title a request sent 88,933. Its one measured loss came on "It was released before 1965", against the earlier batch form, which listed every record in the evidence. That form scored 276 to 283 right in the table's own order and 258 to 270 over shuffled orders. The quoted form scored 255 to 259, and one title a request scored 253. `--batch 1` asks one record a request and sends the requests the tool sent before batching.

## What it prints

On one document, `true`, `false`, or `null`. `null` is a not sure answer, and it arises only under a band. `unsure` is the machine name for a not sure answer, in `audit`, `diff`, and the built-in transforms. In record mode each compact JSONL row is `{"input":RECORD,"value":ANSWER}` in input order. `--details` prints the object in [result.md](result.md) instead, with an `answer.kind` of `yes_no`.

## Options

| Option | Meaning | Default |
| --- | --- | --- |
| `--true TEXT` | What a yes means, in the words the model reads. See below | No text |
| `--false TEXT` | What a no means, in the words the model reads. See below | No text |
| `--threshold T\|LOW:HIGH` | The rule in [threshold.md](threshold.md). `decide` is the one verb that takes both forms | `0.5` |
| `--quiet` | On one document, prints nothing on standard output. Record mode refuses it because no record's answer sets the exit code | Off |
| `--details` | Prints the full result object in place of the bare value | Off |
| `--context FILE` | Uses the file's text once as shared evidence in each record batch; see [records.md](records.md) | None |
| `--dry-run` | Prints the plan and sends nothing. See [channels.md](channels.md) | Off |
| Record options | `--input`, `--lines`, `--jsonl`, `--csv`, `--tsv`, `--field`. See [records.md](records.md) | One document |
| Backend options | `--url` in short and long help, and `--model` in long help. See [backends.md](backends.md) | The two variables and `jev-1.13.0` |

## Saying what yes and no mean

`--true` and `--false` carry one sentence each, and either may appear alone. They travel with the question, and [backends.md](backends.md) gives the field they land in. A run that names neither sends the request it always sent, byte for byte.

They are for a question whose two sides are not obvious from the question text. "The writer asks for money back" and "The writer asks for anything else" separate a refund from a complaint. They are not a place for instructions to the model, and they are not a second question.

Both texts have a home in a question file, under `true` and `false`. A file may also give either one as an object, a list, or `null`; a written `null` is a present criterion, and the object or list reaches the model as written. A text that is empty or holds only white space is a usage error.

## Exit codes

| Code | Meaning |
| --- | --- |
| 0 | Single input: the answer is yes. Record mode: the run finished |
| 1 | Single input only: the answer is no |
| 2 | Usage error |
| 3 | Single input only: the answer is not sure |
| 4, 5, 70 | As [channels.md](channels.md) gives them |

In record mode the exit code reports the run, and no record's answer sets it.

## Examples

```sh
thinkthen decide 'The customer explicitly requests a refund.' < message.txt
```

```sh
requests_refund() {
  thinkthen decide 'The customer explicitly requests a refund.' --threshold 0.1:0.9 --quiet
}
if requests_refund < message.txt; then
  route refunds
fi
```

```sh
thinkthen decide 'Does this report a payment failure?' --jsonl --field /body --details < tickets.jsonl
```

```sh
thinkthen decide 'It appears on the album Abbey Road.' --lines --context catalog.txt < songs.txt
```

Ticket 0172's [live record](../sdlc/records/2026-09-27-0172-shared-context-build.md) measured this question with a shared catalog on 306 Beatles titles. Each of three runs sent one request, scored 306 right with no false yeses or misses, and reported 19,634 input and 5,710 output tokens. This result describes that catalog and build.

## Reading the exit code

The help shows a `case` block on the named exit code. It separates a no from a not sure answer and from a failure, and a script that acts on the answer reads all four outcomes.

```sh
thinkthen decide 'The customer explicitly requests a refund.' --threshold 0.1:0.9 --quiet < message.txt && refund_code=0 || refund_code=$?
case $refund_code in
  0) route refunds ;;
  1) route support ;;
  3) route triage ;;
  *) printf 'the judge failed\n' >&2; exit 4 ;;
esac
```

The help shows one piece of advice beside that block. Word the question in the form where yes permits the action. A failure then never permits anything, because every outcome other than 0 leaves the action undone.

## Cautions

`decide` exits 1 on a no and 3 on a not sure answer. Under `set -e` or `set -o pipefail` that ends a script. Put the command in an `if`, a `case`, or a `||` list. [channels.md](channels.md) says more.

Writing a good question matters more than any option. A question works when it names one fact that is visible in the evidence. "Mentions a delivery date" works. "Is a good reply" does not. Measurement of the first System One model showed a narrow question of the first kind catching every planted mismatch while wrongly rejecting 2% to 3% of good work. Outcome questions of the second kind rejected 18% to 46% of work people had accepted.

A claim planted in the evidence moves the answer. A live run judged twenty made-up messages twice, once clean and once with hostile text appended. The cases are few and they are made up. A command aimed at the judge moved the probability of yes by 0.04 or less, in seventeen wordings. A false claim about the case moved it by as much as 0.57, and a third planted claim moved it by 0.02. The tool cannot tell a planted claim from a true one, because both are evidence.

A batch keeps each record in its own question and out of the evidence. A planted claim in one record is therefore not evidence for another record, by ADR 0055 item 5.

Three defenses hold. `--field` keeps the untrusted parts of a record off the wire. A band sends the moved rows to a person: under `0.2:0.8` the twenty hostile rows held their answer on 18 of 20 and left 2 not sure, and no answer flipped to its opposite. An eval with hostile cases measures what is left.
