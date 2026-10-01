# The result

Status: **Settled** for the bare value, the object, the five answer kinds, the full distribution with `confidence`, and request identity. ADR 0010 accepted the original kinds, ADR 0030 accepted `find`, and the 2026-09-21 amendment to ADR 0017 accepted `meta.requests`. ADR 0048 amends `meta` for batches.

One internal result model feeds both views. The view never changes the request or the answer. Every probability and token count in an example here is illustrative.

The C JSON door returns `{"value":VALUE,"facts":FACTS}` for every successful asking call. `VALUE` keeps the bare shape below, or the detailed object when `details:true` is requested. The four judgment verbs also accept a `records` array; their `VALUE` is an ordered array of bare judgments or full detailed record objects. Each detailed record keeps its whole original `input`, request digest, batch receipt and applicable warnings and context digest. A failed C call returns `NULL`; `thinkthen_error_facts_json` then exposes final started-call facts. The direct `{"usage":true}` response remains the engine's counters rather than a call result. Each engine counts its own calls and its clones', and a host that keeps several engines adds them; the durable usage totals give the process view.

Direct C JSON calls on any of the ten verbs may set `"attempts":true` to add an outer `attempts` array on success, including `[]` after no live send. The default two-member object and failed-call NULL/facts route stay unchanged. This opt-in third member requires an updated reader; the older Objective-C outer reader rejects it. It does not enter `facts` or imply support in typed wrappers.

The call's `facts` object may add `estimated_cost_usd` when its engine has both caller prices and every started attempt supplied complete, exactly summed reported usage. It is a fixed six-decimal string, including `"0.000000"` for priced work with no live sends; without prices it is absent and the old JSON shape remains. The same member appears in copied started-failure facts when complete. This estimate does not change `value`, `meta`, the count-only month or the C ABI. A matching priced producer intentionally requires a matching strict reader: older Ada, Objective-C and COBOL validators reject the extra key, while older tolerant readers may drop it. Installed packages remain at their own source version until rebuilt and proved.

The optional Rust Polars eager door returns `Call<Series>` or `Call<DataFrame>` for every completed column or frame call. Its value keeps the typed Polars shape and its facts count that invocation after all of its batches finish. A started failure returns an error with final facts; a refusal before work starts has no invented account. The lazy `decide_expr`, `choose_expr`, `score_expr` and `tag_expr` family returns a Polars `Expr`, with nullable positions preserved. Decide and choose may return a `Struct{value, probability}`; score and tag refuse probability. Each evaluated morsel is a separate call, whose facts go to a caller-owned `Tally` when supplied. The existing observer supplies eager row details when requested.

A caller-owned `Tally` sums the facts of the calls it records. Its `estimated_cost_usd` adds each call's rounded six-decimal estimate, so it can differ from one rounding of the same work by up to n/2 micro-dollars for n calls. The member is absent when any recorded call lacked an estimate and when the tally recorded no call. Its `model` ignores calls that got no reply, as the command's `model` does; a replied call without a model or with a different model clears it.

## The bare value

| Command | Default standard output |
| --- | --- |
| `decide` | `true`, `false`, or `null` |
| `choose` | a JSON string, or `null`. `--raw` prints the bare label, as [choose.md](choose.md) describes |
| `tag` | a JSON array of every label that reaches the cut, including `[]` |
| `score` | a JSON number |
| `recognize` | an object with `entities` and optional beta `relations` |
| `relate` | one compact name-and-kind edge per line, or no lines when no edge reaches the cut |
| `filter` | each kept line or JSONL record as it arrived; each kept table row as compact JSON, in input order |
| `rank` | each line or JSONL record as it arrived and each table row as compact JSON, ordered by probability of yes or by the weighted value of a saved `score` question |
| `annotate` | one JSON object per record |
| `find` | the selected line or JSONL record as it arrived; no output when `--none` wins or ties |

The table describes one-document output. In record mode, default `decide`, `choose`, `tag`, and `score` rows are `{"input":RECORD,"value":ANSWER}`. The record is parsed: a line is a JSON string, JSONL keeps its value, and CSV or TSV becomes an object of string cells. `choose --raw` keeps its plain-text record view. `filter`, `rank`, and `find` keep returning records.

Every result is compact and sits on one line, so one answer is also one record for `jq`, `grep`, and `wc -l`.

## `--details`

`--details` prints a result object in place of the bare value. For an individual question, check `answer.kind` before reading its fields; `answer.probability` exists only for `yes_no`.

| Command | `answer.kind` | Fields under `answer` | Read `value` as |
| --- | --- | --- | --- |
| `decide`, `filter`, plain `rank` | `yes_no` | `probability` (of yes) | Boolean or `null` for `decide`; `true` on a kept `filter` row; `null` for `rank`, which orders by probability |
| `choose` | `choice` | `pick`, `probabilities`, optional `confidence` | Selected label or `null` |
| `tag` | `tag` | `probabilities` | Labels that reach the cut, possibly `[]` |
| `score`, graded `rank` | `score` | `level`, `probabilities`, optional `confidence` | Weighted numeric position |
| `find` | `find` | `pick`, `probabilities`, optional `confidence` | Selected original unit or `null` |

`pick` and `level` name a leading option even when `value` is `null` or numeric; use `value` for the actionable judgment. The probabilities describe answers, not confidence in the final judgment. `confidence` appears only when the backend reports it. Aggregate `annotate`, `recognize`, and `relate` details have their own [shapes below](#a-detailed-result-keeps-everything) and do not have one outer `answer.kind` from this table.

For example, one detailed `decide` result is:

```json decide
{"schema":"thinkthen.result/1","value":true,"question":{"verb":"decide","text":"Does this ask for a refund?"},"answer":{"kind":"yes_no","probability":0.92},"threshold":0.5,"meta":{"tool":"thinkthen 0.4.0","question_sha256":"982f...88","url":"https://api.typesafe.ai/v1/systemone","model":"jev-1.13.0","usage":{"input_tokens":312,"output_tokens":48},"requests_sent":1,"cached":false,"requests":["6b1f...c4"],"failed_questions":0}}
```

- `value` is the bare judgment. For `filter --details`, it is the cut's boolean; `filter` keeps the record when that boolean is true.
- `question` names the question kind and the text the model received. `filter` and ordinary `rank` ask a `decide` question, so their `question.verb` is `decide`. `rank` with a saved `score` question has `question.verb: score`.
- `answer` is everything the backend said, in thinkthen's own words. No vendor field name appears in it.
- `threshold` is a number for a single cut, the string `"LOW:HIGH"` for a band, and `null` when none applies. `decide` never prints `null` here, because a rule always exists and the default is the cut of one half. [threshold.md](threshold.md) gives the rule.
- `meta` carries the run. `usage` may be absent when the backend reports none. `profile_warning` appears only for a calibration mismatch. No surface prints `batch`: ADR 0111 section 7 removed it from the command's rows, and slice 3 removed it from the libraries, because a row's request depends on which neighbours missed. On `decide`, `filter`, `rank`, `choose`, `tag` and `score` over records, `batch_setting` names the run's resolved batch setting (ticket 0349). `batch_warning` follows when a file's tuned setting differs from the run. `context_sha256` appears only when a context was supplied. The other fields are always present.
- On `decide`, `choose`, `filter`, `rank`, `score` and `tag`, a printed `--details` row may include `meta.attempts` for its current live sends. Each event has a one-based call or command ordinal, the prepared request SHA-256, transport and bounded body-read `wall_ms`, `outcome` (`ok`, `status`, `transport`), optional HTTP `status`, backend-reported `server_ms`, and a screened `request_id`. Retries and split children have separate ordinals; a shared packed send keeps its ordinal on each represented row. A refused split parent appears with either child, while cache and replay add no current event. Completion order may differ from ordinal order. `wall_ms` is not whole-call time or calibrated network time; `server_ms` is not proven model-only time. Terminal failures without a printed row have no CLI attempt display in this slice. `find`, `recognize`, `relate` and `annotate` have separate CLI writers and no `meta.attempts` here.

## A detailed result keeps everything

`relate --details` is an aggregate Option A result. `value` holds accepted edges. `question` holds the resolved fields, ordered relation rules, threshold, and optional saved calibration profile. `answer.questions` keeps each yes/no relation pair, including its endpoints, probability, accepted marker or failure, and question key. `meta.failed_questions` counts failed entries and is always present. [relate.md](relate.md) fixes the exact ordered schema.

`recognize --details` keeps the bare object under `value` and the resolved recognition shape under `question`. `answer.pieces` lists each piece's offsets and its five tag probabilities. `answer.names` lists each found name's span as step 1 found it, its kind probabilities, and its edge option probabilities, or null when it had no edge question. `answer.pairs` lists each relation pair's probability. `meta.requests` lists the question keys of step 1, then step 2, then the relation questions. Name `strength` is P(kind) times P(span), computed from these inputs, and is not itself a probability.

Settled by ADR 0009 item 2, accepted in ADR 0010. `answer` carries the probability of every option or every level, and it carries the backend's own `confidence` when the backend reports one. A saved run can then be swept at another rule with no second request.

`confidence` is present only when the backend sends it. No cut is taken on it. [backends.md](backends.md) says why. The vendor sends no `confidence` on a yes/no answer, so a `yes_no` answer carries none. `sdlc/planning/interface-audit.md` found the page promising one, and this page no longer does.

## Five answer kinds

**`yes_no`**, from `decide`, `filter`, and ordinary `rank`. It carries `probability`, the probability of yes, and nothing else.

**`choice`**, from `choose`.

```json choose
{"schema":"thinkthen.result/1","value":"bug","question":{"verb":"choose","text":"Which kind of request is this?","options":["bug","feature","other"]},"answer":{"kind":"choice","pick":"bug","probabilities":{"bug":0.94,"feature":0.04,"other":0.02},"confidence":0.91},"threshold":0.8,"meta":{"tool":"thinkthen 0.4.0","question_sha256":"5d5f...25","url":"https://api.typesafe.ai/v1/systemone","model":"jev-1.13.0","usage":{"input_tokens":312,"output_tokens":48},"requests_sent":1,"cached":false,"requests":["6b1f...c4"],"failed_questions":0}}
```

`answer.pick` is the first option with the highest probability, before any threshold. `answer.probabilities` holds one entry per option sent, in the order the options were sent. `value` is the sole leading label when it clears a supplied cut, or the sole leader when no cut was supplied.

`value` is `null` when the answer is not sure, and `answer.pick` still names the option that led. A script reads `value` and never `pick`. A person reading a not sure row learns from `pick` what the model was leaning toward.

**`tag`**, from `tag`.

```json tag
{"schema":"thinkthen.result/1","value":["billing"],"question":{"verb":"tag","text":"Which topics?","labels":["billing","urgent"]},"answer":{"kind":"tag","probabilities":{"billing":0.91,"urgent":0.22}},"threshold":0.5,"meta":{"tool":"thinkthen 0.4.0","question_sha256":"00b0...df","url":"https://api.typesafe.ai/v1/systemone","model":"jev-1.13.0","usage":{"input_tokens":208,"output_tokens":32},"requests_sent":1,"cached":false,"requests":["6b1f...c4"],"failed_questions":0}}
```

`answer.probabilities` holds one entry per label in the order sent. `value` keeps every label whose probability reaches the one cut, in that order. An empty array is a complete successful answer.

**`score`**, from `score` and `rank` with a saved `score` question.

```json score
{"schema":"thinkthen.result/1","value":1.6,"question":{"verb":"score","text":"How much disruption does this report?","levels":["None.","Work continues with a workaround.","Work is blocked."]},"answer":{"kind":"score","level":"Work is blocked.","probabilities":{"None.":0.05,"Work continues with a workaround.":0.30,"Work is blocked.":0.65}},"threshold":null,"meta":{"tool":"thinkthen 0.4.0","question_sha256":"005c...f5","url":"https://api.typesafe.ai/v1/systemone","model":"jev-1.13.0","usage":{"input_tokens":208,"output_tokens":32},"requests_sent":1,"cached":false,"requests":["6b1f...c4"],"failed_questions":0}}
```

`answer.level` is the first level with the highest probability, so an exact top tie names the lowest tied level. `answer.probabilities` holds one entry per level, in the order the levels were given, lowest first. `value` is the weighted position on those levels, and [score.md](score.md) gives the arithmetic. A 0.5, 0, 0.5 split across three levels gives `value: 1` and names the lowest level in `answer.level`.

**`find`**, from `find`.

```json find
{"schema":"thinkthen.result/1","value":"Refunds take five days.","question":{"verb":"find","text":"When does a refund arrive?","none":true},"answer":{"kind":"find","pick":"u002","probabilities":{"u001":0.01,"u002":0.98,"none":0.01}},"threshold":null,"meta":{"tool":"thinkthen 0.4.0","question_sha256":"1f2a...9c","url":"https://api.typesafe.ai/v1/systemone","model":"jev-1.13.0","usage":{"input_tokens":312,"output_tokens":48},"requests_sent":1,"cached":false,"requests":["6b1f...c4"],"failed_questions":0}}
```

`answer.pick` names the first wire choice at the highest probability. `answer.probabilities` follows unit order and puts `none` last. `value` is the selected original unit. A strict `none` lead or any top tie involving `none` makes `value` null. A tie among real units selects the first input unit.

The canonical `find` question is compact JSON with keys in this order: `{"verb":"find","text":TEXT,"none":BOOL}`. Generated unit ids, evidence, and unit count are absent. For `Which unit answers?` without `--none`, the canonical bytes are `{"verb":"find","text":"Which unit answers?","none":false}` and their SHA-256 is `01456d0e17c98c801c2ad9b2a9b56e47aeb33ff0eacde8d44bd6f55e4d0ab9ef`. Question text or the `none` policy changes the digest; changing only the units does not.

## Ties by command

For `choose`, `score`, `find`, ordinary `rank`, and `recognize` step-2 options, a tie means equal reported probabilities. Graded `rank` compares the computed weighted values instead. `recognize` step 1 instead compares accumulated scores of valid tag paths. Those paths can tie even when individual tag probabilities differ. Its duplicate-name rule compares printed strengths. The bare `value` and a detailed answer field can differ because they serve different purposes.

| Command | Result at a tie | Existing proof |
| --- | --- | --- |
| `choose` | `value` is `null`; on one document the command exits 3. `answer.pick` still names the first tied option in caller order | `crates/thinkthen/src/core/answer_tests.rs::the_leader_is_the_first_of_a_tie_and_a_tie_is_still_unresolved`; `crates/thinkthen/tests/backend/choosing.rs::a_winner_under_the_cut_and_an_exact_tie_are_both_unresolved` |
| `score` | `value` is the probability-weighted position, not a selected level. `answer.level` names the first tied level, which is the lowest tied level | `crates/thinkthen/src/core/answer_tests.rs::a_score_is_the_weighted_position_on_the_levels_it_was_given` proves the value; `crates/thinkthen/src/core/answer.rs::Distribution::leader` defines the detailed level |
| `tag` | Each label that reaches the cut is included in caller order; labels never compete for one winning place | `crates/thinkthen/src/core/answer_tests.rs::tag_selects_each_label_at_or_above_one_shared_cut_and_empty_succeeds` |
| `find` | The first tied real unit in input order wins. A top tie involving `none` gives `value: null` and exits 3 when `--none` was supplied | `crates/thinkthen/src/core/find.rs::real_ties_take_the_first_and_any_none_tie_is_unresolved`; `crates/thinkthen/tests/backend/find.rs::a_none_tie_is_unresolved_but_details_keep_the_first_wire_leader` |
| `rank` | Equal yes probabilities or equal weighted score values keep input order, including at a `--top` boundary | `crates/thinkthen/src/core/order.rs::the_highest_probability_comes_first_and_an_exact_tie_keeps_input_order`; `crates/thinkthen/tests/backend/keeping/graded_rank.rs::graded_rank_orders_weighted_positions_and_keeps_the_earlier_top_tie` |
| `recognize` | Step-1 tag ties take the earlier tag in its fixed order. Step-2 kind and edge-option ties take the first option asked; a duplicate span and kind at equal strength keeps the first found name | `crates/thinkthen/src/core/recognize/bilou.rs::best_of`, `crates/thinkthen/src/core/recognize.rs::Odds::leader` and `settle` define these paths; no existing test isolates the step-2 tie |

`decide` and `filter` read one yes probability against a rule, while `relate` reads each edge probability against one cut. They do not pick a winner among competing options. An `annotate` question follows its own verb's row above.

## `meta`

ADR 0036 names the stored-answer field `cached`, replacing `replayed` without changing its meaning. New results emit only `cached`. The cost and trials readers still accept historical rows with `replayed`; a present `cached` field takes precedence even when false. Explicit read-only replay also reports `cached: true`.

ADR 0032 adds `meta.profile_warning` only when a saved calibration name and the explicitly selected run profile differ. Its value is `{"tuned_for":NAME,"running":NAME}`. The command prints the same mismatch once on standard error at the first successful logical result. `filter` still warns when it rejects every result and prints no records. A failure on the first logical record warns nobody, even when a later parallel worker completed. A missing name on either side and equal names add no field. The run profile itself stays out of metadata because the field records a warning, not backend selection.

Library and SQL details forms carry the same optional warning in `meta`. The public Rust `Details::profile_warning()` returns the saved and running names in that order. Bare values have no warning channel; a caller that needs the comparison asks for details.

| Field | Holds |
| --- | --- |
| `tool` | The name and version of the binary that made the row |
| `question_sha256` | The digest of the exact question the row answers, as [question-file.md](question-file.md) fixes it. The same question typed and read from a file gives one digest, and any override gives another |
| `url` | The URL that answered |
| `model` | The model that answered, as the backend reported it |
| `usage` | The token counts the backend reported. Absent when the backend reports none. A batched row carries an even share of each of the batch's counts, and the earliest records of the batch carry the remainder, by ADR 0048 item 9 |
| `requests_sent` | The HTTP attempts that produced this result. A replay or cache hit reports zero. Each retry of a retried status adds one. When a too-large batch halves, the refused whole request counts in the first half. A batched row carries its share of its request's attempts by the same rule, by ADR 0048 item 9. On the seven record commands, a row whose question joined another row's send in flight counts no attempt for it, so the rows' counts add up to the sends |
| `cached` | `true` when the answer came entirely from stored exchanges — a recording or a cache — rather than a live backend |
| `requests` | The question keys of the answers that produced the result, in answer order, by ADR 0111 section 7 and [recording.md](recording.md#the-question-store). Retries add nothing |
| `failed_questions` | The number of failed logical questions in this result. Always present, including zero |
| `profile_warning` | The saved calibration profile and selected run profile when both exist and differ. Absent otherwise |
| `batch` | No longer written. Rows saved before ADR 0111 may hold it, with the run's `setting` beside the batch's `records`, `position`, `closed`, `usage` and `requests_sent`. `audit` and `diff` still read its `setting` when a row has no `batch_setting` |
| `batch_setting` | The run's resolved batch setting: a whole number of at least 1, or `"max"`. A structured JSON question runs one record a request and names 1. Present on each detailed record row of `decide`, `filter`, `rank`, `choose`, `tag` and `score`, from the command and from every library call over records. Absent on one document. `audit` and `diff` read it, by ADR 0085 |
| `batch_warning` | The file's tuned batch setting and the running one when they differ, as `{"tuned_for":1,"running":"max"}`. Absent otherwise. A file with a threshold and no batch key was tuned at 1, without changing the run's setting |
| `context_sha256` | The SHA-256 of the exact `--context` file bytes. Absent without a context |

## The run facts line

On an asking command, `--facts` prints one compact `thinkthen.run/1` JSON object as the last standard-error line. Without the flag, a finished run stays silent there. The line follows a stop diagnostic and any usage-counter warning, and precedes a stopping signal's re-raise. `records` counts finished input records, including filtered rows and rows dropped by `rank --top`; a one-document success and one finished `find` or `relate` set count one, while a plan counts zero and sends nothing. `requests_sent`, `retries`, and `cache_answers` come from this process's counters. `seconds` is elapsed wall time in seconds, rounded to three decimals. `input_tokens` and `output_tokens` appear only if at least one live reply arrived, every live reply reported usage, and their exact sum remained valid. `model` appears only if at least one reply arrived and all live or stored replies named the same model. With both caller prices configured, `estimated_cost_usd` is a six-decimal string only when every started attempt supplied both token counts and the exact sum remained valid. Priced no-send, cache-only and replay-only work reports `"0.000000"`; an unpriced run omits this member. The estimate uses caller-selected prices, not a provider bill or admission limit.

A failed run adds `stopped` with `cause` and `retryable`, plus `at` when the stop line names a record. A signal stop has no `at`. `status` appears only for the `status` cause. The stable causes are `usage` for exit 2, `local` for exit 5, `no_key`, `transport`, `status`, `too_large`, `reply`, and `backend` for exit 4, `cancelled` for a stopping signal, and `defect` for exit 70. `too_large` covers status 413 and status 400 naming `max_tokens_exceeded`. Only `status` with a retried status (429, 500, 502, 503, 504, 520, 521, 522, 523, 524, or 529, as [backends.md](backends.md) lists) has `retryable:true`; transport has false because the request may have arrived. Exit 6 is a finished partial result and has no `stopped`.

## Compatibility

Under `thinkthen.result/1` a release may add a member to a row. It never renames or removes one, and it never changes a member's type or meaning. A change of that kind moves every row to `thinkthen.result/2`, and the changelog names it. A reader ignores a member it does not know. The rule binds from 0.1. ADR 0036's rename came before it. Compare `value` and the probabilities between runs, not the bytes, because a release can add a member.

Each command's detailed row holds these members. A member with a trailing `?` is sometimes absent. `input?` appears only on a record row. [spec/result.md](../spec/result.md) holds this table to rows the binary writes.

| Command | `question.verb` | Members | `meta` members |
| --- | --- | --- | --- |
| `decide` | `decide` | `schema` `value` `input?` `question` `answer` `threshold` `meta` | `tool` `question_sha256` `url` `model` `usage?` `requests_sent` `cached` `requests` `failed_questions` `profile_warning?` `batch_setting?` `batch_warning?` `context_sha256?` |
| `filter` | `decide` | `schema` `value` `input` `question` `answer` `threshold` `meta` | `tool` `question_sha256` `url` `model` `usage?` `requests_sent` `cached` `requests` `failed_questions` `profile_warning?` `batch_setting?` `batch_warning?` `context_sha256?` |
| `rank` | `decide`, `score` | `schema` `value` `input` `question` `answer` `threshold` `meta` | `tool` `question_sha256` `url` `model` `usage?` `requests_sent` `cached` `requests` `failed_questions` `profile_warning?` `batch_setting?` `batch_warning?` `context_sha256?` |
| `choose` | `choose` | `schema` `value` `input?` `question` `answer` `threshold` `meta` | `tool` `question_sha256` `url` `model` `usage?` `requests_sent` `cached` `requests` `failed_questions` `profile_warning?` `batch_setting?` `batch_warning?` `context_sha256?` |
| `tag` | `tag` | `schema` `value` `input?` `question` `answer` `threshold` `meta` | `tool` `question_sha256` `url` `model` `usage?` `requests_sent` `cached` `requests` `failed_questions` `profile_warning?` `batch_setting?` `batch_warning?` `context_sha256?` |
| `score` | `score` | `schema` `value` `input?` `question` `answer` `threshold` `meta` | `tool` `question_sha256` `url` `model` `usage?` `requests_sent` `cached` `requests` `failed_questions` `profile_warning?` `batch_setting?` `context_sha256?` |
| `find` | `find` | `schema` `value` `question` `answer` `threshold` `meta` | `tool` `question_sha256` `url` `model` `usage?` `requests_sent` `cached` `requests` `failed_questions` `profile_warning?` |
| `annotate` | none | `schema` `input` `value` `answers` `meta` | `tool` `questions_sha256` `url` `model` `usage?` `requests_sent` `cached` `requests` `failed_questions` `profile_warning?` |
| `recognize` | `recognize` | `schema` `value` `input?` `question` `answer` `meta` | `tool` `question_sha256` `url` `model` `usage?` `requests_sent` `cached` `requests` `failed_questions` `profile_warning?` |
| `relate` | `relate` | `schema` `value` `question` `answer` `meta` | `tool` `question_sha256` `url` `model` `usage?` `requests_sent` `cached` `requests` `failed_questions` `profile_warning?` |

`question.verb` names the kind of question asked, not the command. A `filter` row, an ordinary `rank` row and a `decide` record row print `decide` with the same members. A graded `rank` row from a saved `score` question prints `score` and keeps the same outer members, including its numeric `value`. A reader that needs the command keeps it from the command line that wrote the rows.

## Record rows

The default record row for `decide`, `choose`, `tag`, and `score` keeps the parsed record beside its bare answer. Key order is `input`, then `value`.

```json
{"input":{"id":"T-91","body":"Payouts have failed for 3 days."},"value":true}
```

Under `--details`, the full result object also carries `input`, the original record. `input` holds the whole record, including parts that were never sent. On `filter` and `rank`, `--details` prints these objects for the same records in the same order that the default view would have taken. `filter --details` still prints only kept records.

```json decide
{"schema":"thinkthen.result/1","value":true,"input":{"id":"T-91","body":"Payouts have failed for 3 days."},"question":{"verb":"decide","text":"Does this report a payment failure?"},"answer":{"kind":"yes_no","probability":0.97},"threshold":0.5,"meta":{"tool":"thinkthen 0.4.0","question_sha256":"a1e3...df","url":"https://api.typesafe.ai/v1/systemone","model":"jev-1.13.0","usage":{"input_tokens":88,"output_tokens":12},"requests_sent":1,"cached":false,"requests":["6b1f...c4"],"failed_questions":0}}
```

The preceding row is a `decide --details` example. An ordinary yes/no ranked detailed row keeps the same complete shape, with `value` and `threshold` both `null`:

```json rank
{"schema":"thinkthen.result/1","value":null,"input":{"id":"T-91","body":"Payouts have failed for 3 days."},"question":{"verb":"decide","text":"Does this help diagnose the failure?"},"answer":{"kind":"yes_no","probability":0.97},"threshold":null,"meta":{"tool":"thinkthen 0.4.0","question_sha256":"a1e3...df","url":"https://api.typesafe.ai/v1/systemone","model":"jev-1.13.0","usage":{"input_tokens":88,"output_tokens":12},"requests_sent":1,"cached":false,"requests":["6b1f...c4"],"failed_questions":0}}
```

A graded rank row instead keeps the numeric score under `value`, a `score` question and answer, and a null threshold.

```json rank-score
{"schema":"thinkthen.result/1","value":1.5,"input":"alpha","question":{"verb":"score","text":"How relevant?","levels":["low","middle","high"]},"answer":{"kind":"score","level":"high","probabilities":{"low":0.1,"middle":0.3,"high":0.6}},"threshold":null,"meta":{"tool":"thinkthen 0.4.0","question_sha256":"d20f...21d6","url":"https://api.typesafe.ai/v1/systemone","model":"jev-1.13.0","usage":{"input_tokens":88,"output_tokens":12},"requests_sent":1,"cached":false,"requests":["6b1f...c4"],"failed_questions":0}}
```

## `annotate`

`annotate --details` prints `schema`, `input`, `value` holding the named answers, `answers` holding the result for each name, and `meta`. The command prints no `meta.batches`, by ADR 0111 section 7.

```json annotate
{"schema":"thinkthen.result/1","input":{"id":"T-91","body":"Payouts have failed for 3 days."},"value":{"open":true,"kind":"bug"},"answers":{"open":{"value":true,"question":{"verb":"decide","text":"Is this still open?"},"answer":{"kind":"yes_no","probability":0.97},"threshold":"0.1:0.9","request":"6b1f...c4"},"kind":{"value":"bug","question":{"verb":"choose","text":"Which kind of request is this?","options":["bug","feature","other"]},"answer":{"kind":"choice","pick":"bug","probabilities":{"bug":0.94,"feature":0.04,"other":0.02}},"threshold":0.8,"request":"6b1f...c4"}},"meta":{"tool":"thinkthen 0.4.0","questions_sha256":"9ad3...7e","url":"https://api.typesafe.ai/v1/systemone","model":"jev-1.13.0","usage":{"input_tokens":402,"output_tokens":60},"requests_sent":1,"cached":false,"requests":["6b1f...c4"],"failed_questions":0}}
```

Each successful entry under `answers` carries the same `value`, `question`, `answer`, and `threshold` that a single judgment prints. A failed bare value is `{"failed":{"kind":"backend","cause":CAUSE}}`. Its detailed entry carries `question`, `failure`, and `request`, and omits `value`, `answer`, and `threshold`. The closed causes are `missing_answer`, `wrong_kind`, `missing_probability`, `invalid_probability`, `invalid_distribution`, and `unexpected_probability`. A failed `tag` counts once even when one of its wire members failed. `null` remains a valid not sure answer.

```json annotate
{"schema":"thinkthen.result/1","input":"one note","value":{"ready":true,"kind":{"failed":{"kind":"backend","cause":"missing_probability"}}},"answers":{"ready":{"value":true,"question":{"verb":"decide","text":"Is this ready?"},"answer":{"kind":"yes_no","probability":0.91},"threshold":0.5,"request":"6b1f...c4"},"kind":{"question":{"verb":"choose","text":"Which kind?","options":["bug","other"]},"failure":{"kind":"backend","cause":"missing_probability"},"request":"6b1f...c4"}},"meta":{"tool":"thinkthen 0.4.0","questions_sha256":"9ad3...7e","url":"https://api.typesafe.ai/v1/systemone","model":"jev-1.13.0","usage":{"input_tokens":402,"output_tokens":60},"requests_sent":1,"cached":false,"requests":["6b1f...c4"],"failed_questions":1}}
```

Settled by ADR 0008 item 3 and replaced in part by ADR 0027. `meta.questions_sha256` is the digest of the resolved canonical question set, so spacing, its path, and runtime backend settings do not change it. Each answer carries `request`, the key of its question, or the first of its keys when the question expands to several wire questions. `meta.requests` lists the record's question keys in question-set order, even when concurrent replies finish in another order.

Read `annotate --details` as a row whose known outer members are `schema`, `input`, `value`, `answers`, and `meta`. The names under `value` and `answers` come from the question set; read them with `to_entries` rather than hard-coded member names. An original object may itself have members named `input`, `value`, or `meta`, and those remain inside detailed `input`. The [mixed-stream recipe](annotate.md#read-a-mixed-record-stream) uses the presence of `failure` in each detailed answer entry to distinguish failure from a successful `null`. It does not infer a wrapper from the keys of a bare row.

The readable `question` in each answer prints `choose` options and `tag` labels as names. Their descriptions still take part in `meta.questions_sha256`, so identical readable options do not prove two sets identical. Use that digest for resolved question-set identity, and use `request` for the particular exchange that produced an answer. The [canonical question rules](question-file.md#the-canonical-form) define which descriptions and settings enter the digest.

`meta.usage` is the sum of the record's question shares.

An explicit `annotate --jsonl --details --batch 1 --on-error continue` may
place a separate error row among successful detailed rows when a question-set
`on` pointer is absent from one record:

```json annotate
{"schema":"thinkthen.record-error/1","at":2,"failure":{"kind":"usage","cause":"missing_pointer","pointer":"/body"}}
```

`at` is the one-based input position. This row has no `input`, `value`,
`answers`, `meta`, request, or usage. It is not a `thinkthen.result/1` judgment;
consumers distinguish the two by `schema`. The `recordError` definition in
[`result.schema.json`](result.schema.json) validates this row separately; the
schema root remains the successful detailed result. [annotate.md](annotate.md)
fixes the only supported continuation mode and its exit status.

## What a high probability does not mean

The model judges only the evidence it was shown. A probability of 0.98 says nothing about facts that were absent from the input. In one measurement the model approved every case at 0.98 while human reviewers had refused 23% of them. The only test of a question is a measurement against labeled cases.

For `filter` and `rank`, every answered row in one run must also name the same reply model version, including rows a filter drops and rows from separate halves of a request split after status 413. The first answered row in input order fixes that version. A later difference exits 4 before that row is printed or ranked. `filter` keeps any prefix already printed; `rank` prints no ranking on refusal. A parent 413 and both answered halves still count as actual attempts even when the right row is refused after both sends. Pin `--model` and rerun to compare one model.

Every reply behind one row must report the same model version. Different versions fail the record because one row cannot represent two measurements. The diagnostic safely names both short model identifiers when it can. It says that a cache or recording folder may hold answers from the other version, and it tells the user to rerun with `--no-cache` or to prune that folder with `thinkthen cache prune DIR --answered-by-other-than VERSION`, naming the version a `--no-cache` run returns. The library says the same of its cache.
