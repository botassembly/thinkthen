# What the demos found

Every design finding from the twelve demo pages, strongest argument first. Strength is how hard the demos push: **strong** means a demo could not be written without the change, **medium** means a demo worked but read badly or hid something, **weak** means a demo noticed it.

| Finding | Demos | Section | Proposed change | Strength |
| --- | --- | --- | --- | --- |
| `--replay` is not in the specification. ADR 0005 requires every demo to replay a recording, and no document names the flag, its argument, or what a miss does | 01-12 | A recordings document, and channels.md | Define `--replay DIR`: no network, and a miss is a local failure at exit 5 | strong |
| A result row has no field that separates a judgment from a failure. `assessment.status` carries `accepted`, `unsure`, and `unassessed`, which are all outcomes of a judgment that happened | 12 | result.md | A result carries top-level `status` of `ok` or `error`. An error row carries `error.code` and no `answer` | strong |
| A segment carries unit indices and no line numbers, so `--units paragraphs` gives a shell nothing to cut the file with | 11 | decide.md, `segment` | Every segment carries `start_line` and `end_line` beside `start_unit` and `end_unit`, for both unit kinds | strong |
| Judged columns are needed for JSONL now. `--emit annotated` nests on every pass, so two chained judgments make the next `jq` read `.input.input.sku` | 07 | records.md | Define `--as NAME` for `jsonl`: it implies `--emit input`, appends `NAME` and `NAME_status`, and a record already carrying either field is a record error | strong |
| No demo needed `decide how`. Twelve real jobs asked for yes/no, pick-one, rank, and segment, and none of them wanted a rubric | all | decide.md, `how` | Hold `how` out of the first release. The measurement says rubric judgments reject a fifth to a half of work people accepted, and no job here wanted one | strong |
| The saved question file needs a pinned grammar before `run` can be built | 08 | decide.md, `run` | Markdown: YAML frontmatter for defaults, a level-two heading per question naming it, one fenced `yaml` block per question holding `verb`, `ask`, and `options` or `levels`. Question names are lowercase letters, digits, and underscores | strong |
| A plan cannot prove a key was dropped, because `key_env` has no value when no key will be sent | 09, 10 | channels.md, `--plan` | `key_env` is always a member of the plan and is `null` when no key variable applies | strong |
| Flags that cannot act in the chosen mode are accepted in silence: `--id` under `--emit input`, `--unknown` under `--emit annotated` | 03, 04 | records.md | Each is a usage error at exit 2, with a message naming the mode to pass | strong |
| `--on-error continue` with `--emit input` must be refused, and the refusal must name the mode | 12 | records.md | Keep the draft's recommendation. The demo shows the refusal arriving before any request | strong |
| A per-file loop is outside every limit in records.md. A folder of four hundred notes is four hundred runs, and no flag caps the job | 05 | records.md | No new flag. The stream-verb help says plainly that a per-file loop has no request budget, and shows `find -print0 \| xargs -0 -n 1 -P 4` next to `--jobs` | strong |
| The frontmatter of a question file needs a default pass mark. The draft has a mark per question and one override on the command line, so twelve `if` questions repeat one number twelve times | 08 | decide.md, `run`; ADR 0003 question 7 | The frontmatter carries `min_prob` and a question overrides it. `--min-prob` on the command line still beats both | medium |
| `--url` replaces the URL and keeps the profile's adapter, so a mismatched server is found only when a reply is refused at exit 4 | 10 | backends.md | `--url` without `--adapter` or `--backend` is a usage error at exit 2 | medium |
| `meta.backend` has nothing to hold when `--url` replaced the profile, and `meta` never carries the URL, so two saved results cannot be compared | 10 | result.md | `meta.backend` is `null` when no profile was used, and `meta` carries `url` | medium |
| Exit code 3 is documented as "unsure or unassessed", and `--status` requires `--min-prob`, so `unassessed` cannot arise under `--status` | 01 | channels.md | Code 3 means the accepted answer is unsure. Drop `unassessed` from that row | medium |
| The plan for a stream verb shows one request and hides the run. The framing, the pointer, the identifier, and the request cap decide what the rest of the file sends | 09 | channels.md, `--plan` | The plan carries an `input` object holding `framing`, `on`, `id`, and `max_requests` | medium |
| Exit 6 says at least one record failed and never says how many | 12 | records.md | Under `--on-error continue`, the count of failed records prints on standard error at the end of the run, the way `where` already prints its dropped count | medium |
| `--min-prob` means the symmetric rule for `if` and `where` and the winning-option rule for `which`, so one shell variable holding `0.9` sets two different policies | 01, 02 | result.md, decide.md | Keep one flag name and add `assessment.rule` with the values `symmetric` and `top`, so a saved result says which rule ran | medium |
| An unsure `if` carries no reason, while an unsure `which` carries four named ones | 01 | result.md | `if` carries `reason: "below_min_prob"` when it is unsure, so one `jq` path reads the reason across verbs | medium |
| `answer.pick` is policy-free and sits above `assessment.value`, where a careless `jq` finds it first | 02 | decide.md, `which` | One line in the `which` section and its help: a script reads `assessment.value` and never `answer.pick` | medium |
| `--min-prob` should be optional under `--emit result` and `--emit annotated`. A user exploring a new question has no mark yet, and a saved annotated file can be re-judged at any mark with no model call | 03, 04 | decide.md, `where` | Keep the draft's recommendation: required for `--emit input`, optional otherwise | medium |
| A plan holds the evidence, and channels.md only promises it holds no key | 09 | channels.md, `--plan` | One sentence: the plan carries the evidence and deserves the same care as the request | medium |
| `rank` at exit 7 prints nothing, so a capped run over a large file leaves no number to size the next run with | 06 | records.md | On exit 7, `rank` prints the count of requests it made on standard error | medium |
| CSV does not need a framing of its own once judgments are fields on a record | 07 | records.md | Drop the promised CSV framing and name the `jq -r '@csv'` line instead | medium |
| `--none` stays off by default, `--unknown drop` stays the default for `where`, `--window 0` and `--unknown join` stay the defaults for `segment`, `rank` composes with `where` and keeps yes/no scoring, and `--units` stays at lines and paragraphs | 02, 03, 06, 11 | decide.md | No change. The demos confirm every one of these recommendations | medium |
| `jq -r '.assessment.value'` prints the four characters `null` for an unsure pick, and a `case` that matches on `null` is matching on a spelling accident | 02, 05 | decide.md, `which` | The help shows `jq -r '.assessment.value // "unsure"'` beside the example | weak |
| `--top N` prints N and pays for all N records, and the fact sits in the prose under the option table | 06 | decide.md, `rank` | Move it into the `--top` row of the option table | weak |
| Every user of `run` writes the same checklist `jq`, and every user of `segment` writes the same `sed -n` loop | 08, 11 | decide.md | Put both worked examples in the help for their verbs | weak |
| `--input jsonl --on /body --id /id` is four words in front of every stream question, and nothing safe shortens it | 03 | records.md | No change. The help shows the four as one unit | weak |
| `--status` prints the result, so every shell condition ends in `> /dev/null` | 01 | channels.md | No change. A quiet default would throw away a paid answer | weak |
| `xargs -0 -P` interleaves standard error with nothing tying a line to a file | 05 | none | No change. A script that needs per-file diagnostics writes them itself | weak |

## Draft features no demo needed

Twelve jobs were written before the code. These parts of the draft specification were never reached for.

- `decide how` and `--level`. No job wanted a rubric, and the measurement says the tool is weakest here.
- `decide match` and its four pointer flags. Demo 11 looked at it and chose `segment`, because the hard half of a matching job is building the candidate pairs and the tool does not do that. This is a reason to hold `match`, not to cut it.
- `--invert` on `where`. Every demo that wanted the other side wrote the condition the other way round.
- `--abstain-on NAME` on `which`. Demos 02 and 05 used `--none` and a `case` branch instead, which is code and not policy.
- `--order asc` on `rank`.
- `--from FILE` on `which`. Every demo typed its options, because a short fixed list is what the measurement supports.
- `--input json` and `--input lines`. The jobs had a document or a file of JSON lines.
- `--emit result`. The demos used `input` for filtering and `annotated` for keeping everything.
- `--max-requests`. Demo 05 wanted a budget and it was the wrong shape of budget, across processes rather than within a run.
- `--backend NAME`. There is no configuration file yet, so there is no second profile to name.
- The `THINKTHEN_URL`, `THINKTHEN_ADAPTER`, `THINKTHEN_MODEL`, `THINKTHEN_KEY_ENV`, and `THINKTHEN_BACKEND` variables. Demo 10 used flags, because a demo page has to show what it sends.
- The `chat-logprobs` adapter.
