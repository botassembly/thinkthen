# `annotate`

Status: **Settled** for the file grammar, the output, and several pointers on `on`. ADR 0010 accepted the pointers and struck structured question values from version one; ADR 0039 amends that exclusion. ADR 0048 amends the requests for batches. ADR 0104 adds the narrow missing-pointer continuation and detailed-name exemption.

Asks a saved question set about each record and adds one field per question.

```text
thinkthen annotate FILE [--lines|--jsonl|--csv|--tsv] [--field POINTER] [--batch max|N] [--max-request-bytes N] [--details] [--dry-run] [--on-error continue] [BACKEND]
```

## What it reads

`FILE` is the saved question set. It holds several named questions, and each entry has the shape of a question file. `@FILE` names the same file, as a question file does on the other verbs. `annotate` reads one document on standard input by default, and it reads records under `--lines`, `--jsonl`, `--csv`, or `--tsv`. `--input FILE` reads the evidence from a file. [records.md](records.md) gives the framing and the pointer rules.

With no record framing flag and no `--field`, `annotate` tries to parse the whole document as JSON. Valid JSON becomes a JSON value, including an object, array, string, number, boolean, or `null`; input that fails JSON syntax stays text. Other JSON validation errors are refused. A JSON object can gain named answer fields in the output. To choose record framing explicitly, use `--lines` for separate text lines or `--jsonl` for separate JSON values; CSV and TSV have their own flags. `--lines` makes each nonblank line a separate record, so it cannot preserve a multi-line text document as one record. There is no flag that forces a JSON-looking whole document to be treated as text.

## The question set

```json
{
  "version": 1,
  "threshold": "0.1:0.9",
  "profile": "jev",
  "questions": {
    "open": {"decide": "Is this still open?", "true": "The report names a failure that is still happening.", "false": "Anything else.", "threshold": "0.1:0.9", "on": "/body"},
    "kind": {"choose": "Which kind of request is this?", "options": {"bug": "Reports broken behavior.", "feature": "Asks for new behavior.", "other": "Neither fits."}, "threshold": 0.8},
    "impact": {"score": "How much disruption does this report?", "levels": ["None.", "Work continues with a workaround.", "Work is blocked."]}
  }
}
```

Each entry has the shape of one question file, and [question-file.md](question-file.md) holds that grammar once. A question has exactly one of `decide`, `choose`, `tag`, or `score`, and its value is the question text, as a string, an object, or a list. A `decide` question takes `true` and `false`, the two values that say what each side means. `options` and `labels` are lists or ordered maps from label to description. `levels` is a list, lowest first, or an ordered map from each name to its description. `threshold` follows the command-line rule for its verb, so a `choose` question takes a single cut alone. A question name is not empty and uses only lowercase letters, digits, and underscores. An unknown key anywhere in the file is an error. A set without the required `questions` object says that the wrapper is missing before it reports any unknown top-level key.

The top-level `threshold` applies to every `decide` question that names none. The optional top-level `profile` names the backend profile used to calibrate the set's thresholds. `version`, `threshold`, `profile`, `batch`, and `questions` are the only top-level keys. The optional top-level `batch` is `"max"` or a positive whole number. It selects the record-stream batch size after the flag and environment tiers and stays outside `questions_sha256`. A nested question cannot carry `batch`. A nested question cannot carry another profile. The set holds no backend address, model, output path, or format. An exact check beyond equality is a `jq` field on the record, by ADR 0008 item 6.

`on` is a JSON Pointer inside the value that the record selected: the whole record, or what `--field` selected. A string is text, even when it holds JSON, and `on` never parses it. A text record or a selected string has no members, and a question that reads `on` in one is refused at exit 2 with a sentence naming the question. It can never reach outside that evidence. Under `--lines`, a set with such a question is refused at exit 2 before any input is read. A table asks different questions of different columns, and no question should see a column it does not need. The selection follows the `state` rule of [records.md](records.md): an object or a list travels as that JSON value, and several pointers send one ordered object.

### Several pointers on `on`

Settled by ADR 0008 item 2, accepted in ADR 0010. `on` takes one pointer or several. Several pointers build an evidence object, each member keyed by the last part of its pointer, and two members that would share one key are an error in the file. A correctness check names `/input`, `/gold`, and `/output`. A grounding check beside it names `/context` and `/output` and never sees the gold answer.

```json
{"grounded": {"decide": "Is every claim in the output supported by the context?", "on": ["/context", "/output"]}}
```

## What it prints

One JSON object per record. In record mode an object record gains one top-level field per question, so a chain of judgments stays flat. A line, JSON scalar, or JSON array keeps its parsed record under `input` and its named answer object under `value`. CSV and TSV rows are objects and stay flat. On one document, an object still gains the answers and every other JSON shape or text document still yields the named answers alone.

A not sure answer is `null`. A failed question is a failure marker and never `null`. When the same reply contains a usable answer, the good answer and failed marker both print and the completed run exits 6. A reply with no usable answer ends the run at exit 4.

```json
{"id":"T-91","body":"Payouts have failed for 3 days.","open":true,"kind":"bug","impact":1.6}
```

```json
{"input":"Payouts have failed for 3 days.","value":{"open":true,"kind":"bug","impact":1.6}}
```

`--details` prints `input`, `value`, `answers`, and `meta`, as [result.md](result.md) gives them.

### Read a mixed record stream

Use `--details` when a consumer needs one reliable carrier for every record shape. This example reads JSONL records and keeps the original input beside a named result for every question:

```sh
thinkthen annotate triage.json --jsonl --details < issues.jsonl | jq -c '{input, results: (.answers | to_entries | map({name: .key} + (if .value | has("failure") then {status: "failed", failure: .value.failure} else {status: "answered", value: .value.value} end)))}'
```

Choose `--lines`, `--csv`, or `--tsv` instead when that is the input's actual framing. The command emits one normalized row per completed input record. `input` preserves an original object, array, scalar, or `null`; `results` is a list so question names stay data even when they spell `input`, `value`, or `meta`. An answered value can itself be `null` (not sure), a boolean, number, string, or list. A failed question has `status: "failed"` and a `failure` object instead of a value. On CSV and TSV input, every original cell in `input` is a string, including text that looks like a number or `null` ([records.md](records.md)). A run that stops before completing a record does not emit its row.

The bare output cannot identify an arbitrary object's original fields just by looking for `input` and `value`: those can be the object's own keys. With `--details`, the command itself establishes the carrier, and original fields remain under `input`. [result.md](result.md) describes the detailed answer entries and their digests.

## Options

| Option | Meaning | Default |
| --- | --- | --- |
| `--lines`, `--jsonl`, `--csv`, or `--tsv` | Reads a record stream in place of one document | One document |
| `--field POINTER` | The part of each record the questions see. An `on` pointer works inside it | The whole record |
| `--details` | Prints the full result object per record | Off |
| `--on-error continue` | With `--jsonl --details --batch 1`, prints one error row for a missing question-set `on` pointer and judges later records | Off; stop at the first failed record |
| `--input FILE` | Reads the evidence from a file | Standard input |
| `--dry-run` | Checks the file, prints the plan, and sends nothing. See below | Off |
| `--profile FILE` | Applies explicit local backend limits and names the running calibration profile. See [backends.md](backends.md) | None |
| Backend options | `--url` in short and long help, and `--model` in long help. See [backends.md](backends.md) | The two variables and `jev-1.13.0` |

`annotate` takes no `--threshold`, no `--quiet`, and no `--raw`. A question carries its own threshold.

## Exit codes

0 when the run finished with every question answered, 6 when it finished after one or more logical questions failed, and 2, 4, 5, and 70 as [channels.md](channels.md) gives them. A question name that the record already holds is an input error for that record in bare mode, at exit 2 before any request for it. With `--details`, the original member remains under `input` while the answer uses its name under `value` and `answers`; the name no longer collides, including in dry run. An unreadable or invalid file is exit 5. A later whole-run failure keeps its own code and stop boundary.

`--on-error continue` requires explicit `--jsonl --details --batch 1`, refuses `--dry-run`, and is not an option on another verb. Missing any required flag or typing another policy is exit 2 before input is read. Only a missing question-set `on` pointer is recoverable. It emits one `thinkthen.record-error/1` row in input order, with one-based `at` and `failure:{"kind":"usage","cause":"missing_pointer","pointer":"/body"}`. It sends no request for that record. A completed run with any such row exits 7 and prints `thinkthen: 1 record skipped` or the plural count once on standard error; exit 7 outranks a completed exit-6 partial answer, whose failed marker stays in its row. Malformed/oversized/invalid-UTF-8 input, backend/group/replay/model/cancellation and output failures still stop with their existing code and boundary. A skip count, if any, prints before the terminal diagnostic. The error row has no input, answer, metadata, token count or backend body; route it by `schema` instead of feeding it to a judgment consumer. [result.md](result.md) fixes its full shape.

## Examples

The first and third examples use the one-document rule on a JSON object. The second selects JSONL records explicitly.

```sh
thinkthen annotate triage.json < issue.json
```

```sh
thinkthen annotate triage.json --jsonl < issues.jsonl | jq -c 'select(.kind == "bug")'
```

```sh
thinkthen annotate triage.json --dry-run < issue.json
```

## `--dry-run`

`--dry-run` validates the file and sends nothing. It needs no key. An empty document is a usage error. An empty line or JSONL stream succeeds and prints nothing, unless a question reads `on` under `--lines`. An empty CSV or TSV input fails because its required header is missing. With evidence it prints the first request a live run sends, which is the first chunk under a profile, and an `on` object that names the normalized pointers for every question. `request_count` counts the requests the first record makes, and `group_requests` counts each `on` group's requests in group order.

One record makes at least one request per distinct `on`, and a profile can split a group into several. The plan prints one request: the first that the first record's first `on` set sends, taking the questions in file order. `request_count` and `group_requests` count the rest. The plan's `input` object names the framing and, under `on`, the pointers of every question, so a reviewer sees what each check would see and not only the check that the plan printed.

```json
{"framing":"jsonl","on":{"correct":["/input","/gold","/output"],"grounded":["/context","/output"]}}
```

`--dry-run` reads the question set and the first record. A name that collides with a field on record two is invisible to it. [How-to 16](../demos/16-triage-pipeline/) keeps the collision caution beside its table-input pipeline.

## Requests

One record makes one logical request group for each distinct `on`. Every question with the same evidence rides in that group. An explicit backend profile splits a group into the fewest contiguous requests that satisfy its exact request-byte and expanded-question limits. Every chunk repeats the same evidence, and the model sees no answer from another chunk. Without a profile, or when the group fits, the historical single request stays byte for byte unchanged. The project measured forty clear yes-or-no questions over 120 cases. Packed requests changed no answer and used 20.8 times fewer billed input tokens than separate requests (`sdlc/issues/closed/2026-09-20-live-probe-findings-packing-tagging-status-and-cost.md`). A second measurement packed one decision, one choice, and one score. The values stayed the same, and billed input fell from 915 tokens across three requests to 371 in one request (`probes/annotate-0015/mixed-summary.json`). Neither measurement found a lower question-count limit.

In record mode, consecutive records of one `on` group share a request until content, count, exact byte or profile limit, a real 50 ms input pause, input end, or the 4,096-member cap closes it. A profile may split a group into contiguous question slices; compatible slices batch independently. A structured JSON question text keeps its historical singleton body. `--batch 1` keeps every record's historical request bytes. One document ignores ambient and file batch settings and refuses a typed `--batch`. A question never sees another question's answer. Work that depends on an earlier answer is a second command.

`--details` reports `meta.usage` as the checked sum over the record's requests only when every reply reports usage. `meta.requests` lists every chunk's recording digest in question-set group and chunk order. `meta.requests_sent` sums actual sends, including retries. In a detailed row, `meta.batches` aligns with `meta.requests` when any contributing request held several records or came from a split. Each entry names the one-based `on` group, exact request digest, setting, record count and position, close reason, whole-request usage when known, attempts and a true split marker when applicable. Ordinary singleton entries are included to keep the arrays aligned. The field is absent when every chunk is an ordinary singleton. `meta.cached` is true only when every chunk replayed. Each answer also carries the digest of the chunk that produced it. Every chunk must report the same model. [result.md](result.md) gives the shape.

A failed bare value is `{"failed":{"kind":"backend","cause":CAUSE}}`. In detailed output its entry carries `question`, `failure`, and `request`, and carries no `value`, `answer`, or `threshold`. `meta.failed_questions` counts failed logical questions. It is always present, including zero. `null` means not sure and never means failed.

Questions sharing one `on` selection ride in one logical group. A reply with at
least one usable answer prints that answer beside a failed marker for each
unusable answer and makes a completed run exit 6. A reply with no usable
answer in any group stops the run at exit 4. It publishes none of that record's
answers, even when a sibling `on` group answered successfully. A failed request
covering several records cannot identify which member caused it;
`--on-error continue` does not recover that whole batch. Separating questions
into different `on` groups spends more requests and limits which questions
share a partial reply or a changed request. It does not preserve sibling-group
answers when one group wholly fails. Editing a question in a packed group
changes that group's request identity, so a rerun may send the whole group
again. Keep the exact group and batch setting when comparing saved runs or
estimating request cost.

## Cautions

A profile enforces only limits stated in bytes, expanded questions, or options. A backend limit stated only in tokens remains unenforceable without a tokenizer or a measured byte ceiling.

Each exact encoded chunk has its own cache key. A group that fits remains one historical chunk, so adding or changing one question asks that whole group again for every record. A split group reuses only chunks whose exact bytes and positions remain unchanged; changing one question can also move later chunk boundaries and their keys. An answer near its threshold can move when neighboring questions change. In six deliberately borderline cases, one answer moved from `false` to not sure when neighboring questions joined it. The largest probability shift was 0.04. Keep the group fixed while comparing runs and retain `--details` probabilities. Use a narrower, distinct `on` group when the record permits it and the questions need separate stability.

## An eval is `annotate` and a saved run

Settled by ADR 0008 item 1. A case is one flat JSON object with any field names. A definition is a question set. A run is the file of `--details` rows. `jq` reads the run and counts it. No eval engine enters this tool.

```sh
thinkthen annotate checks.json --jsonl --record runs/v1 --replay runs/v1 < cases.jsonl > run-v1.jsonl
jq -s '[.[] | select(.value.correct)] | length' run-v1.jsonl
```

ADR 0010 holds the `report` command out of version one and puts the `jq` transforms first. `specification/roadmap.md` records the five outcomes and the test between them.

A missing field and a failed request stop the run, so a completed run holds a judgment for every case. A rerun on the same recording folder answers the finished cases from disk.
