# The `decide` family

Status: **Settled** for `if`. The other verbs are being drafted from Ian's captures and land here before their tickets.

`decide` judges meaning. Evidence arrives on standard input. The question arrives as arguments. Every verb obeys [channels.md](channels.md), prints the result in [result.md](result.md), and reaches a backend as [backends.md](backends.md) describes.

## `thinkthen decide if CONDITION`

Asks whether a condition holds for the evidence.

```sh
thinkthen decide if 'the customer explicitly requests a refund' < message.txt
thinkthen decide if 'the customer explicitly requests a refund' --min-prob 0.9 --status < message.txt
thinkthen decide if 'the customer explicitly requests a refund' --plan < message.txt
```

- `CONDITION` is one argument. It states a fact that is true or false of the evidence. The decider model reads it as the question. A condition that is empty or holds only white space is a usage error.
- Standard input is read to its end as UTF-8 text and becomes the evidence. Input that is empty or holds only white space is a usage error, because a judgment about nothing is a mistake in the pipeline.
- `--min-prob P` sets the pass mark. Without it the result is `unassessed`.
- `--status` sets the exit code from the assessment and needs `--min-prob`.
- `--plan` prints the request and stops.
- The backend options are `--backend`, `--url`, `--adapter`, `--model`, `--key-env`, `--timeout`, and `--max-retries`.

Writing a good condition matters more than any option. A condition works when it names one fact that is visible in the evidence. "Mentions a delivery date" works. "Is a good reply" does not.

## `thinkthen decide which OPTION...`

Status: **Draft**.

Picks one option from a fixed list.

```text
thinkthen decide which OPTION... [--by QUESTION] [--none] [POLICY] [BACKEND]
thinkthen decide which --from FILE [--by QUESTION] [--none] [POLICY] [BACKEND]
```

```sh
thinkthen decide which bug feature question other < issue.txt
thinkthen decide which billing shipping account --by 'the team that owns this request' < message.txt
thinkthen decide which --from actions.json --by 'the next action that serves the goal' --none < state.json
thinkthen decide which bug feature question other --min-prob 0.8 --min-gap 0.3 < issue.txt
```

| Option | Meaning | Default |
| --- | --- | --- |
| `OPTION...` | The option names, in the order they are sent. Each name is one argument | None. Either the names or `--from` is required |
| `--from FILE` | A JSON object whose keys are option names and whose values are descriptions. A description may be a string or a JSON object | None. Conflicts with positional names |
| `--by QUESTION` | One sentence that says what decides the pick | None. The option names carry the meaning |
| `--none` | Adds an option named `none` that means no other option fits | Off |
| `--min-prob P` | The pass mark on the winning option's probability. `P` is above 0 and at most 1 | None, and the result is `unassessed` |
| `--min-gap G` | The least lead the winner must hold over the runner-up | None |
| `--abstain-on NAME` | Picking `NAME` counts as unsure. Repeatable | None |
| `--plan` | Prints the request and stops | Off |
| Backend options | `--backend`, `--url`, `--adapter`, `--model`, `--key-env`, `--timeout`, `--max-retries` | See backends.md |

Standard input is read to its end as UTF-8 text and becomes the evidence. Empty input is a usage error.

The result prints one JSON document with `answer.kind` of `choice`. It carries the pick and one probability per option. [result.md](result.md) gives the shape.

The answer is unsure when the winner's probability falls under `--min-prob`, when the lead falls under `--min-gap`, when two options tie exactly, or when the winner is an `--abstain-on` option. `assessment.value` is then `null`.

`which` never runs the option it picks. A name is a string that the next program reads.

Exit codes: 0, 2, 4, 5, and 70. `--status` is not offered, because a pick is not a shell condition.

| Limit | What happens past it |
| --- | --- |
| 255 options, counting `--none` | Usage error, exit 2 |
| Duplicate option names | Usage error, exit 2 |
| Evidence over the backend's token limit for one question | The backend refuses and the exit code is 4 |

The tool sends the options in the order the user gave and never reorders them. Measurement of the first decider model showed that option order and added irrelevant options both shift the odds. A short list of options that exclude one another gives the steadiest answer, and a run with a changed list is a different measurement.

**Questions for Ian**

- Should the choice pass mark reuse the name `--min-prob`, or take a separate name such as `--min-top`? Reusing the name keeps one word for one idea, and its meaning differs per verb. A separate name keeps the symmetric yes/no rule alone. Recommendation: reuse `--min-prob` and state its meaning in each verb.
- Should `--none` be on by default? On by default saves a flag in matching work. Off by default keeps the sent list equal to the list the user typed. Recommendation: off.
- Should `--from` accept a plain JSON array of names as well as an object of names and descriptions? An array is friendlier for a generated candidate list. One accepted shape is less to test. Recommendation: accept both, because a candidate list is usually generated.

## `thinkthen decide how PROPERTY`

Status: **Draft**.

Rates the evidence on named levels that run from lowest to highest.

```text
thinkthen decide how PROPERTY --level TEXT --level TEXT [--level TEXT...] [POLICY] [BACKEND]
```

```sh
thinkthen decide how urgency --level 'no deadline is stated' --level 'a deadline is stated' < ticket.txt
thinkthen decide how 'handoff completeness' \
  --level 'missing essentials' --level 'usable with follow-up' --level 'ready for review' \
  < handoff.txt
thinkthen decide how urgency --level low --level medium --level high --plan < ticket.txt
```

| Option | Meaning | Default |
| --- | --- | --- |
| `PROPERTY` | One argument naming the property being rated | Required |
| `--level TEXT` | One level. Repeat it once per level, lowest first | Required, at least two |
| `--min-prob P` | The pass mark on the winning level's probability. `P` is above 0 and at most 1 | None, and the result is `unassessed` |
| `--plan` | Prints the request and stops | Off |
| Backend options | The same set as `if` | See backends.md |

Standard input is read to its end as UTF-8 text and becomes the evidence. Empty input is a usage error.

The result prints one JSON document with `answer.kind` of `rating`. It carries the level names, one probability per level, and a weighted value from 0 to 1. [result.md](result.md) gives the shape.

The answer is unsure when the winning level's probability falls under `--min-prob`. `assessment.value` is then `null`. The weighted value still prints, because it is what the backend said.

Exit codes: 0, 2, 4, 5, and 70. `--status` is not offered.

| Limit | What happens past it |
| --- | --- |
| Fewer than two levels | Usage error, exit 2 |
| More than 255 levels | Usage error, exit 2 |
| Evidence over the backend's token limit for one question | The backend refuses and the exit code is 4 |

Rating is the weakest thing a decider model does. Measurement of the first decider model showed rubric judgments rejecting 18% to 46% of work that people had accepted. The help text for `how` says so. A gate that has to hold belongs in `if` or `which`, and a weighted value from `how` belongs in a review queue that a person reads. The weighted value is a position on the named levels. It is not a probability that the property holds.

**Questions for Ian**

- Should `how` ship in the first release at all? Shipping it with the warning in its help lets people explore a rubric. Holding it until a pass mark can be measured against labeled cases keeps the weakest primitive out of scripts. Recommendation: ship it with the warning, because holding it does not stop anyone from writing the same rubric by hand.
- Should the weighted value be the position from 0 to 1, or the raw level index from 0 to the count minus one? The 0 to 1 form compares across rubrics of different lengths. The index form matches the levels a person typed. Recommendation: 0 to 1, with the level count in the result so the index is one multiplication away.

## `thinkthen decide where CONDITION`

Status: **Draft**.

Keeps the records whose evidence satisfies a condition.

```text
thinkthen decide where CONDITION --min-prob P [RECORD] [--unknown drop|keep|error] [--invert] [BACKEND]
```

```sh
thinkthen decide where 'describes a reproducible bug' --input jsonl --on /body --min-prob 0.9 < issues.jsonl
thinkthen decide where 'mentions an unresolved action' --input lines --min-prob 0.9 < notes.txt
thinkthen decide where 'reports a payment failure' --input jsonl --on /body --id /id \
  --min-prob 0.9 --unknown error --emit annotated < tickets.jsonl
```

| Option | Meaning | Default |
| --- | --- | --- |
| `CONDITION` | One argument stating a fact that is true or false of each record | Required |
| `--min-prob P` | The symmetric pass mark from [result.md](result.md) | Required. A filter with no pass mark has nothing to filter on |
| `--unknown drop\|keep\|error` | What an unsure record does. `drop` leaves it out and counts it on standard error. `keep` selects it. `error` ends the run at that record | `drop` |
| `--invert` | Swaps accepted yes and accepted no. An unsure record stays unsure | Off |
| Record options | `--input`, `--on`, `--id`, `--emit`, `--jobs`, `--max-requests`, `--max-record-bytes`, `--on-error`. [records.md](records.md) defines them | See records.md |
| `--plan` | Prints the request for the first record and stops | Off |
| Backend options | The same set as `if` | See backends.md |

Standard input carries the records. `--input text` is a usage error for `where`, because one document is not a stream.

`--emit input` is the default and prints the selected records unchanged. `--emit result` and `--emit annotated` print one row per input record, selected or not, each carrying a `selected` field. `--unknown keep` and `--invert` change which records `--emit input` prints. They never change what `--emit result` prints.

Exit codes: 0, 2, 4, 5, 6, 7, 8, and 70. [records.md](records.md) fixes 6, 7, and 8.

| Limit | What happens past it |
| --- | --- |
| `--max-record-bytes` | The record fails. Exit 5 under `--on-error stop`, an error row under `continue` |
| `--max-requests` | The run stops at that record and exits 7. Printed output is a prefix |
| A record over the backend's token limit | The backend refuses. Exit 4 under `stop`, an error row under `continue` |

A record that `where` leaves out failed to reach the pass mark. It is not a record the model called false. `--emit annotated` into a file keeps both groups for review, and no second model call is needed to change the pass mark later.

**Questions for Ian**

- Should `--unknown` default to `drop` or to `error`? `drop` reads like `grep` and prints a count of dropped records on standard error. `error` refuses to let a script lose an unsure record without saying so. Recommendation: `drop`, with the count always printed.
- Should `--min-prob` really be required? Requiring it makes every `where` command state its policy. Making it optional lets a user explore with `--emit annotated` before choosing a mark. Recommendation: require it for `--emit input`, and allow its absence when `--emit` is `result` or `annotated`, because nothing is being selected then.

## `thinkthen decide rank CRITERION`

Status: **Draft**.

Orders records by how well each one fits a criterion.

```text
thinkthen decide rank CRITERION [--top N] [--order desc|asc] [RECORD] [BACKEND]
```

```sh
thinkthen decide rank 'helps diagnose the login timeout' --input jsonl --on /body --top 5 < passages.jsonl
thinkthen decide where 'describes a reproducible bug' --input jsonl --on /body --min-prob 0.9 < issues.jsonl |
  thinkthen decide rank 'affects many users' --input jsonl --on /body --top 10
```

| Option | Meaning | Default |
| --- | --- | --- |
| `CRITERION` | One argument stating what makes a record fit. The tool asks it of each record as a yes/no question | Required |
| `--top N` | Prints only the first `N` records of the order | All records |
| `--order desc\|asc` | `desc` puts the best fit first | `desc` |
| Record options | `--input`, `--on`, `--id`, `--emit`, `--jobs`, `--max-requests`, `--max-record-bytes`, `--on-error` | See records.md |
| `--plan` | Prints the request for the first record and stops | Off |
| Backend options | The same set as `if` | See backends.md |

The method is fixed and printed in the help. The tool asks one yes/no question of each record, sorts the records by the yes probability, and breaks exact ties by input order. It never compares two records in one question, and it never runs a tournament.

`--emit input` is the default. `--emit result` and `--emit annotated` print the same order with the probability on each row.

`rank` takes no pass mark, so every row is `unassessed`. `rank` orders and never selects. A user who wants a floor runs `where` first, as the second example shows.

`rank` holds every record until the input ends, because a final order needs the whole set. `--top 5` limits what prints. It does not reduce the number of requests. An endless stream has to be cut into windows upstream.

Exit codes: 0, 2, 4, 5, 6, 7, and 70. Code 8 cannot arise, because `rank` declares no requirement about unsure records.

| Limit | What happens past it |
| --- | --- |
| `--max-record-bytes` | The record fails. Exit 5 under `stop`, an error row under `continue` |
| `--max-requests` | The run stops and exits 7. Nothing prints, because a partial order is not an order |
| Records held in memory | Bounded only by the input. A very large file belongs in windows |

An error row under `--on-error continue` has no probability. It sorts last in either direction and keeps its input order among the other error rows.

**Questions for Ian**

- Should `rank` gain its own floor option instead of composing with `where`? A floor inside `rank` saves one process. Composition keeps one idea per verb and makes the cut visible in the pipeline. Recommendation: compose, and show the pipeline in the help.
- Should `rank` be able to score with `how` levels instead of a yes/no probability? Levels would let a user name what "best" means. Rating is the weakest primitive by measurement. Recommendation: no, keep the yes/no probability.

## `thinkthen decide match RELATION`

Status: **Draft**.

Judges whether the two sides of each supplied pair stand in a named relation.

```text
thinkthen decide match RELATION [--left POINTER] [--right POINTER] [--left-id POINTER] [--right-id POINTER] [POLICY] [RECORD] [BACKEND]
```

```sh
thinkthen decide match 'the same product, including model and size' --input jsonl --min-prob 0.95 < pairs.jsonl
thinkthen decide match 'both records describe the same incident' --input jsonl \
  --left /a/text --right /b/text --left-id /a/id --right-id /b/id --min-prob 0.95 < pairs.jsonl
```

| Option | Meaning | Default |
| --- | --- | --- |
| `RELATION` | One argument naming the relation. The tool asks it as a yes/no question about the pair | Required |
| `--left POINTER` | The JSON Pointer to the left side's evidence | `/left` |
| `--right POINTER` | The JSON Pointer to the right side's evidence | `/right` |
| `--left-id POINTER` | The JSON Pointer to the left side's identifier | None |
| `--right-id POINTER` | The JSON Pointer to the right side's identifier | None |
| `--min-prob P` | The symmetric pass mark | None, and every row is `unassessed` |
| Record options | `--input jsonl`, `--emit`, `--jobs`, `--max-requests`, `--max-record-bytes`, `--on-error` | See records.md |
| `--plan` | Prints the request for the first pair and stops | Off |
| Backend options | The same set as `if` | See backends.md |

Standard input carries one JSON object per line, and each object holds both sides. `--input jsonl` is the only accepted framing. A pointer that hits nothing is an error for that row.

Both pointers are the disclosure boundary. Only the two pointed values reach the backend.

`--emit result` is the default and prints one result row per pair, carrying the relation, both identifiers, and the probability. `--emit input` prints the pairs that were accepted as yes. `--emit annotated` prints every pair with its result.

`match` judges one pair at a time. It never assigns, never enforces one-to-one, never clusters, and never closes a relation over three records. A pair that is accepted as yes is a proposed edge that a later program may use.

The tool never builds pairs. A cross product of two files would hide a very large cost inside a short command line, so the candidate pairs are made upstream and arrive on standard input.

Exit codes: 0, 2, 4, 5, 6, 7, 8, and 70.

| Limit | What happens past it |
| --- | --- |
| `--max-record-bytes`, measured on both sides together | The row fails. Exit 5 under `stop`, an error row under `continue` |
| `--max-requests` | The run stops and exits 7 |
| A pair over the backend's token limit | The backend refuses. Exit 4 under `stop`, an error row under `continue` |

**Questions for Ian**

- Should `match` exist, or should a pair be judged by `where` with `--on` over a pair record? Keeping `match` names both sides, carries both identifiers into the result, and makes the two-sided disclosure boundary explicit. Dropping it removes a verb and one grammar. Recommendation: keep it, because a one-pointer verb cannot carry two identifiers.
- Should the default pointers be `/left` and `/right`? Those defaults make the common case a one-flag command. A required pointer makes every command state its own shape. Recommendation: keep the defaults.

## `thinkthen decide segment`

Status: **Draft**.

Finds where one document breaks into parts.

```text
thinkthen decide segment --boundary CONDITION [--units lines|paragraphs] [--window N] [--min-prob P] [--unknown join|split|error] [BACKEND]
```

```sh
thinkthen decide segment --boundary 'a new request begins here' --units lines < thread.txt
thinkthen decide segment --boundary 'a new topic begins here' --units paragraphs --window 1 --min-prob 0.9 < transcript.txt
```

| Option | Meaning | Default |
| --- | --- | --- |
| `--boundary CONDITION` | One argument stating what makes a unit the start of a new part | Required |
| `--units lines\|paragraphs` | What counts as one unit. A paragraph ends at a blank line | `lines` |
| `--window N` | How many units of context travel on each side of the candidate boundary. `0` sends the whole document | `0` |
| `--min-prob P` | The symmetric pass mark on each boundary question | None, and every boundary is `unassessed` |
| `--unknown join\|split\|error` | What an unsure boundary does. `join` keeps the two units together. `split` breaks between them. `error` ends the run | `join` |
| `--max-requests N` | The cap on requests for the document | None |
| `--plan` | Prints the requests and stops | Off |
| Backend options | The same set as `if` | See backends.md |

Standard input is read to its end as UTF-8 text and split into units by the tool. Empty input is a usage error.

The tool counts the units and asks one yes/no question per candidate boundary. The model never computes an offset. Boundary questions that share identical evidence travel in one request, up to the backend's token limit for a request, so `--window 0` sends many boundaries at once.

The result prints one JSON document holding an ordered list of segments. Each segment carries a one-based inclusive `start_unit` and `end_unit`, and each boundary carries its probability and its assessment. The segments never overlap, and together they cover every unit.

`segment` produces a flat list. It assigns no parent, no heading level, and no role.

Exit codes: 0, 2, 4, 5, 7, 8, and 70. Code 6 cannot arise, because one document is judged whole and `--on-error` does not apply. A backend failure ends the run at exit 4, and nothing prints.

| Limit | What happens past it |
| --- | --- |
| `--max-requests` | The run stops and exits 7. Nothing prints, because a partial cover is not a cover |
| A document over the backend's token limit for one question | The backend refuses and the exit code is 4. A window smaller than the document is the answer |
| `--unknown error` and an unsure boundary | Exit 8. Nothing prints |

**Questions for Ian**

- Should `--units` offer sentences? Sentences match how people read a transcript. A sentence splitter needs language rules and a dependency, and its mistakes would be blamed on the model. Recommendation: lines and paragraphs only, and a user who wants sentences splits them upstream and uses `lines`.
- Should `--window` default to `0` or to `1`? `0` sends the whole document once and costs the fewest requests. `1` sends a small window per boundary and suits a long document. Recommendation: `0`, and the help says to raise it when a document is too large for one request.

## `thinkthen decide run FILE`

Status: **Draft**.

Asks several saved questions about the same evidence.

```text
thinkthen decide run FILE [RECORD] [--min-prob P] [BACKEND]
```

```sh
thinkthen decide run triage.md < issue.txt
thinkthen decide run triage.md --input jsonl --on /body --id /id < issues.jsonl
thinkthen decide run triage.md --plan < issue.txt
```

| Option | Meaning | Default |
| --- | --- | --- |
| `FILE` | The saved question file | Required |
| `--min-prob P` | Overrides the pass mark of every question in the file | The mark each question carries, if any |
| Record options | `--input`, `--on`, `--id`, `--emit`, `--jobs`, `--max-requests`, `--max-record-bytes`, `--on-error` | See records.md |
| `--plan` | Prints the compiled request as JSON and stops | Off |
| Backend options | The same set as `if` | See backends.md |

Standard input carries the evidence. `--input text` reads one document, and the stream framings read one evidence per record.

The file needs four things per question and nothing more: a name, a verb of `if`, `which`, or `how`, the question text, and the options or levels the verb requires. A question may carry its own pass mark.

All the questions for one evidence travel in one request, because the backend answers several named questions over one evidence. A question never sees another question's answer. Work that depends on an earlier answer is a second command.

The result prints one JSON document per evidence. It holds an `answers` object keyed by question name, and each entry carries the same `question`, `answer`, and `assessment` layers that a single judgment prints.

Exit codes: 0, 2, 4, 5, 6, 7, and 70.

| Limit | What happens past it |
| --- | --- |
| The questions and evidence over the backend's token limit for one request | The backend refuses and the exit code is 4. Fewer questions per file is the answer |
| `--max-requests` | The run stops and exits 7 |

**Questions for Ian**

- What is the saved question file? Markdown with frontmatter is readable by the person who owns the questions, and frontmatter holds the settings while a heading holds each question. JSON is what every capture proposed and needs no new grammar to test. Recommendation: Markdown, with `--plan` printing the JSON request so the machine form is always one command away. Cost: this repository specifies and tests a small Markdown grammar.
- Where does a pass mark live? The file keeps the mark beside the question it was measured for. A flag keeps policy out of the file. Recommendation: the file holds a mark per question, and `--min-prob` on the command line overrides every one of them.

## Dropped from the captures

Status: **Draft**. The captures proposed these, and this draft leaves them out.

| Proposal | Why it is out |
| --- | --- |
| `--context FILE` for extra evidence | The settled `if` section has no such option, and one verb carrying it alone would split the grammar. A user concatenates the context into standard input |
| `--evidence lines\|sentences\|paragraphs` on `if` and `where` | It asks the backend for spans this specification has not defined. It waits for its own document |
| `--files0-from` and `--emit paths0` | Reading files named in a stream turns a judging tool into a reading tool. `find` and a loop do it today |
| `match --against FILE` and `--all-pairs` | A cross product hides a very large cost inside a short command line. Pairs arrive on standard input |
| `--audit-fd` and `--audit-file` | The five channels are settled. An audit channel needs its own document |
| `--scale LEVEL...` taking many values | A repeated `--level` keeps the argument boundaries clear next to a positional property |
| `--true-at` and `--false-at` | The symmetric pass mark is settled, and an asymmetric pair is named there as a later option |
| `--policy FILE` | Policy in a file duplicates the saved question file before either exists |
| `validate`, `eval`, `test`, `reassess`, `record`, `replay` | Out of scope for this draft by instruction. They belong to other documents |
