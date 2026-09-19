# What the demos found

Every design finding from the thirteen demo pages, strongest argument first. The pages are written to ADR 0007, so every finding here is a finding against that surface. Strength is how hard the demos push: **strong** means a demo could not be written without the change, **medium** means a demo worked but read badly or hid something, **weak** means a demo noticed it.

| Finding | Demos | Section | Proposed change | Strength |
| --- | --- | --- | --- | --- |
| A per-record input error exits 2, and the exit table says code 2 means a usage error and that nothing was sent. A file whose fortieth record lacks the pointer pays for thirty-nine requests, prints thirty-nine rows, and still exits 2. A `case` branch cannot tell a mistyped flag from a bad record, and the two want different repairs | 03, 07, 12 | Exit codes; Records | Give a run that started and then hit a bad record its own code. Code 6 is reserved and free. Keep code 2 for a failure before any request | strong |
| `--details` fixes the `answer` object for a yes/no answer only. Nothing says what `choose` or `score` puts there, so no page can read the per-option probabilities or check a margin. The dropped `--min-gap` cannot be replaced in a script either | 02, 05, 13 | Output, `--details` | Fix `answer` for every verb. For `choose`, carry the options in the order they were sent with a probability each. For `score`, carry the levels and the number | strong |
| `--raw` prints nothing for an unresolved answer. On one input that is fine, because exit 3 says what happened. Under `--jsonl` there is no exit code per record, and a blank where a label should be either breaks the one-value-per-record promise or is indistinguishable from an empty label | 02, 05 | Output, `choose` | Say what `--raw` prints for `null` under a record flag. An empty line keeps the row count and is the least surprising | strong |
| `report` has no options at all. ADR 0007 says the exact options are Draft until a demo drives them, and demo 13 is that demo | 13 | `report` | One option. `--truth POINTER` names the recorded answer inside each row's `input` and turns the counts into a sweep. The candidate cuts are the distinct probabilities the rows carry, so there is no grid to configure. Single cuts only. A proposal, not a ruling | strong |
| A result object carries no question name, only `question.text`, so `report` has a sentence for a key. A question whose wording is fixed halfway through a file becomes two questions in the report | 13 | Output, `--details`; `report` | Give the result object a stable question name, or make `report` number its questions and print the text beside the number | strong |
| The `annotate` file has no default threshold, and an unknown key anywhere in it is an error, so a user cannot add one. A checklist of twenty `decide` questions repeats the same band twenty times. `annotate` takes no `--threshold` either | 08 | The `annotate` file | One optional top-level `threshold` in the file, which a question overrides | strong |
| Nothing counts what a record run did. The dropped count `filter` used to print is gone, a stopped run says only that it stopped, and a ranked run over a large file leaves no number to size the next run with | 03, 06, 12 | Records | At the end of a record run, print on standard error the records read, the requests made, and, for `filter`, the records dropped | strong |
| `--window N` on `segment` is listed and never defined. The default is unstated and zero has no meaning. Whether a ten-line thread costs one request or nine turns on it | 11 | Commands, `segment` | Define `--window N`: zero sends the whole document in one request and is the default, and N sends N units of context around each boundary question | strong |
| A record run's `--dry-run` plan holds six fields describing one request. The record flag, the pointer, and the threshold decide what the rest of the file sends, and none of them is in the plan. Two saved plans can differ in what they send and read identically | 09 | ADR 0006's plan | The plan carries an `input` object holding the record mode, the pointer, and the threshold, and says which record it came from | medium |
| `annotate --details` gives a record one `meta`. Questions with the same `on` share one request, so a file whose questions have two pointers makes two requests for one record and has one place to report them | 07 | The `annotate` file; Output | `meta.usage` on an `annotate` row is a list, one entry per request, or `meta` names the request count | medium |
| `config show` prints "the effective settings" and no shape is fixed. A settings dump is exactly what people paste into a support thread | 10 | Configuration | Fix the output as one JSON object using the file's own key names, with `profiles` reduced to the selected one and every key `null` where nothing applies | medium |
| `config path --config FILE` has no stated answer. The demo assumes the given file is the path in use | 10 | Configuration | One sentence: `config path` prints the file `--config` or `THINKTHEN_CONFIG` names, and otherwise the default path | medium |
| The rule that `--url` needs `--adapter` lives in ADR 0004, which ADR 0007 partly replaces. A reader of ADR 0007 alone cannot tell whether a URL on its own is allowed | 10 | Configuration | Restate the ad-hoc flag rules in the surface, or name the exact ADR 0004 sections that still hold | medium |
| `segment` is left out of the list of verbs that take `--lines` and `--jsonl`. `--units lines` and `--lines` would then be two words for two ideas on one command line | 11 | Records; `segment` | Say that `segment` reads one document and takes neither record flag | medium |
| An unresolved boundary on `segment` silently joins, and no field on a segment records that anything was close. `segment` takes a single cut only, so there is no third answer to carry | 11 | The threshold; Output | Each segment carries the probability of the boundary that opened it | medium |
| `rank` buffers the whole stream, because an order needs every record. `filter` streams. The two read identically on the command line | 06 | Output, `rank` | One line in the `rank` help | medium |
| No request budget exists anywhere. A per-file loop is many runs and `jobs` bounds only the inside of one run. A ranked run over a million-line file makes a million requests with no lever | 05, 06 | Records; Configuration | No new flag. The help for the record flags says a per-file loop is outside every budget, and shows `find -print0 \| xargs -0 -n 1 -P 4` next to the `jobs` setting | medium |
| Whether `--quiet` and `--details` may appear together is unstated. An option that cannot act in the chosen mode is a usage error, and these two are the obvious candidate pair | 01 | Commands; Everyday options | Rule on the pair. Refusing it reads better than silently printing nothing | medium |
| `decide --jsonl` without `--details` prints `true`, `false`, and `null` with nothing tying a line to a record. Zipping with `paste` is a trap, because a stopped run prints a short file and the zip shifts | 04 | Records | The `--jsonl` help says a record-mode script that needs the record uses `--details` | medium |
| Parallelism left the command line. A user with a rate limit and a large file edits a JSON file to change one number for one run, and nothing on the command line points at the file | 03 | Configuration | No new flag. The record-flag help names the `jobs` key and the configuration path | medium |
| `annotate --dry-run` checks the question file and not the records, so it passes a file that fails on the first record because of a name collision | 07 | The `annotate` file | One line in the help saying what `--dry-run` does and does not check | weak |
| An unresolved answer is `null` everywhere. That is right in JSON and empty in CSV. A spreadsheet cell reading nothing and a listing that says nothing become the same cell | 07 | The `annotate` file | One line in the `annotate` help about the `jq -r '@csv'` step | weak |
| `--details` carries `input`, the whole record including what was never sent. A user with megabyte records pays for it on every row to read one probability | 04 | Output, `--details` | No change. The help names the cost | weak |
| A threshold is measured for one model and nothing on the command line says so. Pointing the same line at a second model is a new measurement with an old number in it | 10, 13 | The threshold | The `--threshold` help says the mark belongs to a model, in the words the backend rules already use about naming an exact version | weak |
| Under the default cut of 0.5 nothing is ever unresolved, so a two-way `if` routes a message the model was unsure about with no sign that it was close | 01 | The threshold | The `decide` help says a three-way gate needs a band, next to the warning about `set -e` | weak |
| A truth pointer names a field that `--field` is what keeps off the wire. A user who forgets `--field` sends the label with the question and poisons the measurement | 13 | `report`; Records | One line in the `report` help | weak |
| A demo cannot show the paid half of `--record DIR --replay DIR`. Every entry exists in a committed recording, so every row replays and the saving is asserted in prose | 12 | Records | No change. The claim belongs to a live check and not to a gate | weak |
| `score` was never reached for. Thirteen jobs asked for yes/no, pick-one, rank, segment, and saved questions, and none of them wanted a level | all | Commands, `score` | No change. ADR 0007 already ships `score` with its measured weakness in its help and lets nothing depend on it. Recorded so a later cut has the evidence | weak |

## Settled by ADR 0007

The twelve earlier pages raised these, and the ADR answered them. They are listed once and not repeated above.

- `--replay DIR` is in the specification, with a miss as a local failure at exit 5.
- The result has one shape with a bare value in front of it, and `--status`, `unassessed`, and the symmetric `--min-prob` are gone.
- Every segment carries `start_line` and `end_line` beside the unit indices.
- Judged columns are flat top-level fields on the record, so a chain of judgments never nests. `--as`, `--emit`, and `--id` are gone with the problem.
- `decide how` is gone. `rank` orders by the probability of yes and takes no rubric.
- The saved question file is JSON, so the proposed Markdown grammar is dead.
- `key_env` is always present in the plan and `null` when no key applies. `meta` carries `url` and `meta.profile` is `null` for an ad-hoc backend.
- Exit code 3 means unresolved and nothing else.
- Options that cannot act in the chosen mode are usage errors at exit 2.
- `--plan` is `--dry-run`, `--backend` is `--profile`, and the five backend environment variables are gone.
- A threshold is one option with two forms, and boundaries are inclusive.
- `--none`, `--invert`, `--abstain-on`, `--from`, `--order`, `--on-error`, and `--max-requests` are all gone.

## Draft features no demo needed

Thirteen jobs were written before the code. These parts of ADR 0007 were never reached for.

- `score`, and the two-to-ten levels it takes. No job wanted a level, and the measurement says the tool is weakest here.
- A `score` question inside an `annotate` file, and the `levels` key that carries it.
- `--lines`. Every job had a document or a file of JSON lines.
- `--units paragraphs`. Demo 11 has no blank line in its fixture, so the unit numbering and the line numbering agree on every segment and the page proves nothing about the case the line numbers exist for.
- `--key-env`. The one demo with a second backend gave it a profile whose `key_env` is `null`, because the point of the page is that no key crosses hosts.
- `--` as the end of option parsing. No demo had a question or an option label that begins with a dash.
- `THINKTHEN_CONFIG` and `THINKTHEN_PROFILE`. Demo 10 used flags, because a demo page has to show what it selects.
- The `jobs`, `timeout_seconds`, and `max_retries` keys in the configuration file. Demo 10 carries them in its fixture and never exercises one.
- A file profile named `jev` replacing the built-in one.
- `options` as a plain list of labels in an `annotate` file. Demo 08 used the map form, because a label deserves a description.
- Exit codes 5 and 70. Demo 12 names 5 in a `case` branch and no page produces either.
- The `chat-logprobs` adapter. Demo 10 names its own adapter on the command line and in a profile.
