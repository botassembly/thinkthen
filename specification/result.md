# The result

Status: **Settled** for the bare value, the object, the five answer kinds, the full distribution with `confidence`, and request identity. ADR 0010 accepted the original kinds, ADR 0030 accepted `find`, and the 2026-09-21 amendment to ADR 0017 accepted `meta.requests`. ADR 0048 amends `meta` for batches.

## Result/2 for 0.2

Status: **Settled** by [ADR 0120](../sdlc/planning/adr/0120-sdk-result-and-cache-contract.md). This section is the adoption contract, not a claim of landed behavior. Tickets 0443–0445, 0450 and the carrier owners below implement it. The native branch implements the complete Rust calls and CLI details. The remaining result/1 examples document the retained explicit compatibility projection; they are not the new CLI details. Whole-code acceptance and host adoption remain open. The generated [result schema](result.schema.json) contains native result/2 definitions alongside retained result/1 compatibility definitions under ADR 0112. The additive native `complete_call_schema()` supplies the packaged strict complete success/error schema to consumers such as MCP. Both artifacts derive from the production serializers; host adoption requires actual consumer execution.

### Complete results and compatibility

Every new complete result carries `schema:"thinkthen.result/2"` and required top-level `answer_id:AnswerId`. Identity is available without opting into probability details. Existing bare CLI, scalar SQL, convenience values and generic C compatibility views retain their projections. C retains its failed-call NULL/error-facts route. An additive complete C route exposes identity; ticket 0426 owns its exports and lifetimes. No changed rank value may claim result/1.

Optional fields are omitted unless explicitly nullable. Existing question, entity, span, endpoint, location, distribution and confidence fields retain their types and meanings. This table fixes all ten complete result variants. All rows require `schema`, `answer_id` and `meta` in addition to the listed members.

| Function | Required members | Value and optional members |
| --- | --- | --- |
| decide | value, question, answer, threshold | Boolean, authored meaning or null; input?, position?, input_file? |
| choose | value, question, answer, threshold | Label or null; input?, position?, input_file? |
| tag | value, question, answer, threshold | Ordered labels, including []; input?, position?, input_file? |
| score | value, question, answer, threshold | Weighted number; threshold is null; input?, position?, input_file? |
| filter | value, input, question, answer, threshold | Boolean; CLI details print only true rows; position? |
| rank | value, input, question, answer, threshold | Positive integer final rank position; threshold is null; question_name?, position? |
| find | value, question, answer, threshold | Selected original unit or null; threshold is null; position? |
| annotate | input, value, answers | Ordered named entries; position? |
| recognize | value, question, answer | Existing entities and optional relations; input? |
| relate | value, question, answer | Accepted edge array, including [] |

Plain, graded and set rank assign positions after selection and before top truncation under ticket 0436. They retain their probability/member details; a graded rank's weighted score remains in its answer distribution and reading rather than replacing the final rank position. Observation routes expose filter rejections and rank omissions with their own member identities; an omitted rank candidate has no invented final position.

The five atomic answers remain `yes_no:{probability}`, `choice:{pick,probabilities,confidence?}`, `tag:{probabilities}`, `score:{level,probabilities,confidence?}` and `find:{pick,probabilities,confidence?}`, each tagged by its existing `kind`. Probability maps keep declared option order. Nullable successful values remain distinct from failures.

Complete native record wrappers expose their retained zero-based original `index`, before filtering, sorting or splitting. Rank `value` remains the one-based final position. Standalone scalar results omit the occurrence index. Complete find always carries nullable selected `index`: synthetic none is null; a selected original JSON null has an integer index. Its ordered `candidates` retain each original `input`, actual probability, nullable original index and optional supplied physical `source`; synthetic none follows real inputs. These fields change no identities or readings.

Complete records with ancillary images retain optional ordered `images` beside their unchanged original `input` and supplied `source`. The images preserve media, base64 bytes and dimensions from the admitted native snapshot. Established image-only input serialization remains intact. Located annotation and recognition additionally retain native `source` beside the original.

An incremental complete batch failure can carry ordered `completed` native result documents alongside its safe `error` and final `facts`. Those documents are the actual completed prefix observed by the existing batch, with original indexes and identities. Ordinary whole-call failure envelopes remain unchanged; no successful answer is fabricated for the terminal failure.

An annotate success entry requires `answer_id,value,question,answer,threshold,request`. A failure entry requires `failure_id,question,failure,request` and omits the successful fields. The existing bare failure marker and six backend member causes remain unchanged. Recognize retains `answer:{pieces,names,pairs}` with complete probabilities. Relate retains `answer:{questions}`; each entry keeps `relation,reads,method,direction,source,target,request` and requires either success `answer_id,probability,accepted` or failure `failure_id,failure`. Existing nullable target rules remain.

### Recorded CLI example

This `decide --details --replay` row comes from the existing refund recording. It retains the actual historical observation and reports no current send or attempt. Its original wire-question count was never recorded, so `batch_size` is absent.

```json decide
{"schema":"thinkthen.result/2","answer_id":"a0fc22705aea025a57253816e3ea7eb7fea014184b2b09e58e0d30414f2dbc9e","value":true,"question":{"verb":"decide","text":"Does the customer ask for money back?"},"answer":{"kind":"yes_no","probability":0.99},"threshold":0.5,"meta":{"tool":"thinkthen 0.2.0","question_sha256":"ef16533e8bf1fb5e4d35e95dc520b50729bbe78fe5d522eec2860c59c5b55c55","url":"https://api.typesafe.ai/v1/systemone","model":"jev-1.13.0","usage":{"input_tokens":331,"output_tokens":21},"requests_sent":0,"cached":true,"requests":["1c38c6f52bec903f93aef63060bdc08055463afe00b5759223bfc5a64d30dafe"],"failed_questions":0,"attempts":[],"origin":"replay","question_sources":[{"origin":"replay","answered_by":"jev-1.13.0"}],"observations":[{"observation_id":"017420ba535606f3ab3ec3f9364250b5891432862f33194ab3d520cced234017"}],"answered_by":"jev-1.13.0"},"position":{"file":null,"first":1,"last":7}}
```

### Identity types and stable answers

`CallId`, `SdkRequestId`, `ObservationId`, `FailureId` and `AnswerId` are distinct opaque validated types. Their JSON/header spelling is exactly 64 lowercase hexadecimal characters. Question keys and `request_sha256` are SHA-256 digests with that spelling, not call or send identities. Provider `request_id` remains its separate screened string type. IDs support correlation, not authentication.

Call IDs identify invocations; SDK request IDs identify prepared sends. Both use practical collision resistance across concurrent calls and forked processes without caller-data hashing. Generator choice belongs to implementation; this contract adds no dependency. Observation IDs identify newly accepted wire answers at the engine edge. Cache/replay retain them; refresh allocates new ones even for equal probabilities; no-store keeps them only in memory. A failure ID identifies a failed logical occurrence and is never persisted as an answer. Repeated uses of one held observation keep its observation ID but retain their distinct logical scopes.

Use UTF-8 and the framing function `F` defined in [cache.md](cache.md#version-2-identity). The answer ID is lowercase SHA-256 of:

```text
F("thinkthen.answer-id/1", function, canonical_scope_JSON,
  ordered_observation_variants_JSON, normalized_reading_JSON,
  ordered_child_IDs_JSON)
```

Scope identifies the original zero-based record occurrence, member name/ordinal, stage/question occurrence and candidate/span/endpoint identity as applicable. Reading includes resolved cuts/bands/defaults, authored meanings, score weights, find-none semantics, recognition/relation cuts, rank selection/member ownership and final rank position where one exists. Aggregate children follow semantic order; rank uses the complete order before top truncation. Empty aggregates use empty observation/child arrays and their resolved scope/reading, so they still have an answer ID.

Normalize numeric spelling and defaults through existing typed serializers. Canonical objects follow the owning typed serializer's field order; arrays and option maps retain semantic order. Exclude surface, filesystem paths, retrieval origin, call/request IDs, timestamps, costs, packing and display from answer identity. Semantic text offsets/endpoints remain included. Compute before host index conversion. Duplicate members remain distinct; successful null differs from failure. Returning to the same reading over the same observations restores the answer ID. Core receives typed identities and performs only pure framing and reading. [Cache migration](cache.md#legacy-observation-identity) fixes legacy observation IDs.

### Metadata and empty aggregates

Retain existing metadata fields and optionality: `tool`, the applicable question/set digest, `url`, historical `model`, `usage?`, `requests_sent`, `cached`, `requests`, `failed_questions` and applicable profile/batch/context warnings. Add required `origin`, `question_sources` and `observations`; add optional scalar `answered_by`. `meta.proxy` is absent throughout 0.2.

`question_sources` aligns one-to-one with `meta.requests` in logical question order, including repeated occurrences. Each entry requires `origin` and `answered_by`, the validated model reported by the response behind that occurrence, including a recoverable member failure in a validated response. Each source may additionally carry batch_size, a positive unsigned 32-bit integer: the actual wire-question count in the successful request that produced that observation under corrected ticket 0454. It is outside identity. Retain the original count on cache/replay/coalescing; split children carry their own counts. Omit it when historical data never recorded the count; never infer one. Aggregates expose the aligned constituent counts without a guessed scalar batch size. `observations` has exactly the same alignment. Each entry is exclusively `{"observation_id":ObservationId}` or `{"failure_id":FailureId}`. Failed occurrences do not become stored answers. Whole transport failures return terminal errors rather than fabricated source entries.

Emitted source origins are `live`, `cache`, `replay`. Reserve `proxy` and `memory` as type values without emitting them. A direct call to a proxy hostname over ordinary model wire is `live`. Explicit replay with observations is `replay`; otherwise any required live response makes the result `live`, and wholly cached observations make it `cache`. Mixed cache/live results retain each occurrence's actual source. Emit scalar `answered_by` only when nonempty constituent sources all agree. Retain current mixed-model refusals; this metadata admits no previously refused combination. Historical `meta.model` remains a compatibility field, not proof that mixed sources agree.

When an aggregate has **zero stored or live observations**, required `origin` is JSON null, `question_sources` and `observations` are [], `requests` is [], `requests_sent` is 0, `cached` is false, and `answered_by` is omitted. This also applies in explicit replay mode: no source was retrieved. Retain the historical `model` fallback as a requested-model compatibility value, never an actual answered model. An empty actionable array with observations is different: it reports those observations' real origins/model.

Reason: the landed relation planner can produce no questions for a lone entity with a wildcard single-target rule. `engine/facade/relate.rs` initializes its stored flag true and `cli/relate/result.rs` falls back to the requested model even with no reply. Result/2 must not interpret those defaults as a cache hit or an actual model answer. The existing case in `tests/backend/relate/menu.rs` proves this zero-send execution. Null provenance preserves the closed origin vocabulary without inventing a fourth source. This exception changes result/2 truthfulness; the result/1 examples below retain their historical meaning.

| Case | origin | question_sources | answered_by | cached |
| --- | --- | --- | --- | --- |
| One live answer from model A | live | live/A | A | false |
| One held answer from model A | cache | cache/A | A | true |
| Explicit replay of that answer | replay | replay/A | A | true |
| Held A plus live A | live | cache/A, live/A | A | false |
| Admitted historical sources A and B | replay | replay/A, replay/B | omitted | true |
| No logical questions or observations | null | [] | omitted | false |
| Empty accepted edge array after live answers from A | live | live/A for each occurrence | A | false |

### Call facts, transport and attempts

Add required `facts.call_id` to every successful invocation, including zero-send success, and every started failure. Start means typed admission and route resolution succeeded and engine execution began, before lookup. Pre-start refusals have no invented facts. Terminal errors have no successful result, answer ID or fabricated result meta; they retain final facts and opt-in attempts. Keep the six public error kinds. Repeated SQL rows of one call share its call ID and facts; a summed tally does not invent a single call ID.

The additive native `Call::complete()` view serializes `{value,facts}` with a
concrete result/2 document (or ordered result/2 records) and complete invocation
facts. The released generic door and count-only `Facts` serialization retain
their existing fields. `Error::complete()` serializes `{error,facts?}`: error
contains the existing safe `kind,message,retryable`, structured
`stopped:{at?,cause,status?,retryable}`, and any actual typed budget denial.
`at` is the known original one-based stopping record; cancellation, deadlines
and unknown positions omit it. The cause vocabulary is `usage,local,no_key,
transport,status,too_large,reply,backend,cancelled,deadline,defect`. Denials use
a closed `kind` tag and retain their existing limit/last-status fields. A token
already cancelled during admission has no started facts; a deadline failure
inside engine execution retains them. No failure envelope contains a successful
value, answer ID or result metadata. Serialization of caller-owned record/find
originals requires Serialize only when writing JSON, never to execute or inspect
the typed result. It retains their authored order and complete original payload.

Every actual send uses:

```text
User-Agent: thinkthen/<compiled-engine-semver> (<surface>)
X-ThinkThen-Call-Id: CallId
X-ThinkThen-Request-Id: SdkRequestId
```

The closed surface tokens are `cli`, `rust`, `c`, `python`, `pandas`, `python-polars`, `rust-polars`, `javascript`, `ruby`, `r`, `cpp`, `go`, `csharp`, `java`, `kotlin`, `scala`, `swift`, `zig`, `php`, `dart`, `objective-c`, `ada`, `cobol`, `flutter`, `duckdb`, `sqlite`, `postgresql`, `mcp`. TypeScript uses `javascript`. Outer wrappers explicitly supply their token; C validates it. Unknown tokens refuse locally. The version comes from the compiled Rust engine, not the wrapper package.

A new invocation receives a new call ID. Each prepared send receives a new SDK request ID; status retries retain it and refusal-split children get new IDs. Every stage uses the engine's fixed endpoint, effective key and provider API type under ADR 0119. IDs contain no caller text, credential, address or model; transient call/request IDs enter neither cache keys, recordings nor count-only usage. Future proxy deduplication scopes request IDs to authenticated callers and compares bodies; differing bodies conflict. SDK accounting still counts actual sends and retries. Keep the existing rule that a transport failure is not retried.

Opt-in attempts cover all ten functions on success and started failure. Unrequested attempts are absent; requested zero-send attempts are []. Each event requires `ordinal,request_sha256,wall_ms,outcome,sdk_request_id`; optional `status,server_ms,request_id` retain their meanings. Outcomes remain `ok,status,transport`. A packed event is shared by its represented rows; retries and split children have their own ordinals. The only provider observations are `x-envoy-upstream-service-time` and the existing adapter ID header, today `x-typesafe-request-id`. Missing time stays absent. Replay emits no current attempts.

CLI `command_ms` is rounded-up elapsed milliseconds outside the **union** of HTTP/body-read intervals, from accepted command execution through worker, output and usage-writer completion, immediately before facts emission. Overlapping parallel intervals are never double-subtracted. Existing `seconds` remains total elapsed time. Edge clocks measure time; core does not. [recording.md](recording.md#optional-timing-history-for-02) fixes the bounded timing sidecar.

### Reserved proxy types

Request reservation is `proxy:{question_id?:Opaque,code_threshold:T}`. Opaque is 1–128 ASCII characters from `[A-Za-z0-9._-]`, without trimming. T is the existing normalized cut/band/null grammar and function admission. Derive it from current precedence/defaults; reject a contradictory supplied value. Attach reservations to each logical member. Annotate/rank sets retain member-specific readings; score/find use null. Recognize additionally reserves `code_relation_threshold:T` for its separately scoped relation reading.

Reserved metadata is `proxy:{decision_id:Opaque,override:Override}`. The closed variants are:

```text
{kind:"none",code_threshold:T}
{kind:"threshold",code_threshold:T,effective_threshold:T}
{kind:"decision",code_threshold:T,code_value:V,value:V}
```

V is the target function's typed actionable value. Threshold overrides admit only that function's existing reading rules; recognition relation targets are separately scoped. These types reserve no HTTP path, header or executable protocol. In 0.2 **any activation**, including empty/null proxy input, returns usage failure before lookup, store access or send. Ordinary vendor bytes exclude reservations; unknown vendor fields/headers cannot populate proxy metadata. Apply no override and emit no attestation. Execution waits for an admitted explicit 0.3 proxy protocol; no hostname or model alias grants authority.

### Adoption dependencies

0443 owns transport/provenance and Cache-Control; 0444 owns storage validation/migration; 0445 owns attempts and command timing; 0450 owns answer identities/reservations; 0436 owns rank positions. 0426–0431 adopt complete typed carriers and strict readers; 0410/0296 own frames; 0435 owns SQL facts/observations; 0411 owns rereading; 0432 owns parity. 0447/0448 supply admitted image serialization and limits. 0449's single-route contract precedes this adoption. 0426's additive C exports and handle/string lifetimes remain its signature dependency.

Adoption updates generated schema, all ten detailed variants in the shared corpus, strict full-result decoders and examples together. Existing recordings remain historical fixtures until 0444 converts them in one controlled update; result/2 cannot be claimed by changing a schema label alone. The changelog distinguishes this target from behavior already built.

## Landed result/1 behavior

The sections below describe the existing views, shapes and examples. Result/2 above supersedes them only as its owning implementation tickets adopt it.

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
- Complete native atomic results include `source` only when the input supplied a physical location. It holds the exact `file`, with paired `first_line` and `last_line` for text units. Image filenames omit line coordinates. The original `input` remains intact; physical location changes no model request, cache key or answer identity.
- `question` names the question kind and the text the model received. `filter` and ordinary `rank` ask a `decide` question, so their `question.verb` is `decide`. `rank` with a saved `score` question has `question.verb: score`.
- `answer` is everything the backend said, in thinkthen's own words. No vendor field name appears in it.
- `threshold` is a number for a single cut, the string `"LOW:HIGH"` for a band, and `null` when none applies. `decide` never prints `null` here, because a rule always exists and the default is the cut of one half. [threshold.md](threshold.md) gives the rule.
- `meta` carries the run. `usage` may be absent when the backend reports none. `profile_warning` appears only for a calibration mismatch. No surface prints `batch`: ADR 0111 section 7 removed it from the command's rows, and slice 3 removed it from the libraries, because a row's request depends on which neighbours missed. On `decide`, `filter`, `rank`, `choose`, `tag` and `score` over records, `batch_setting` names the run's resolved batch setting (ticket 0349). `batch_warning` follows when a file's tuned setting differs from the run. `context_sha256` appears only when a context was supplied. The other fields are always present.
- On `decide`, `choose`, `filter`, `rank`, `score` and `tag`, a printed `--details` row may include `meta.attempts` for its current live sends. Each event has a one-based call or command ordinal, the prepared request SHA-256, transport and bounded body-read `wall_ms`, `outcome` (`ok`, `status`, `transport`), optional HTTP `status`, backend-reported `server_ms`, and a screened `request_id`. Retries and split children have separate ordinals; a shared packed send keeps its ordinal on each represented row. A refused split parent appears with either child, while cache and replay add no current event. Completion order may differ from ordinal order. `wall_ms` is not whole-call time or calibrated network time; `server_ms` is not proven model-only time. Terminal failures without a printed row have no CLI attempt display in this slice. `find`, `recognize`, `relate` and `annotate` have separate CLI writers and no `meta.attempts` here.

## A detailed result keeps everything

The following paragraphs describe the CLI details and native observation shapes named here. Complete aggregate details are not available through every generic C JSON and SQL route. The [0405 audit](../sdlc/records/0405-audit-report.md) records the gaps; [0408](../sdlc/tickets/0408-j1-full-probabilities.md) owns aggregate carriers.

`relate --details` is an aggregate Option A result. `value` holds accepted edges. `question` holds the resolved fields, ordered relation rules, threshold, and optional saved calibration profile. `answer.questions` keeps each yes/no relation pair, including its endpoints, probability, accepted marker or failure, and question key. `meta.failed_questions` counts failed entries and is always present. [relate.md](relate.md) fixes the exact ordered schema.

`recognize --details` keeps the bare object under `value` and the resolved recognition shape under `question`. `answer.pieces` lists each piece's offsets and its five tag probabilities. `answer.names` lists each found name's span as step 1 found it, its kind probabilities, and its edge option probabilities, or null when it had no edge question. `answer.pairs` lists each relation pair's probability. `meta.requests` lists the question keys of step 1, then step 2, then the relation questions. Name `strength` is P(kind) times P(span), computed from these inputs, and is not itself a probability.

Settled by ADR 0009 item 2, accepted in ADR 0010. `answer` carries the probability of every option or every level, and it carries the backend's own `confidence` when the backend reports one. Stored wire answers can be read under another supported reading rule without another send when the required questions and stages are already stored. A bare result is not a complete standalone store for every function. Strict replay refuses a missing question locally; ordinary cache mode may send it.

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

A [call](../CONTRIBUTING.md#call) may answer several [decisions](../CONTRIBUTING.md#decision). A [request](../CONTRIBUTING.md#request) is one send; `requests_sent` counts attributed transport attempts, including retries.

ADR 0036 names the stored-answer field `cached`, replacing `replayed` without changing its meaning. New results emit only `cached`. The cost and trials readers still accept historical rows with `replayed`; a present `cached` field takes precedence even when false. Explicit read-only replay also reports `cached: true`.

ADR 0032 adds `meta.profile_warning` only when a saved calibration name and the explicitly selected run profile differ. Its value is `{"tuned_for":NAME,"running":NAME}`. The command prints the same mismatch once on standard error at the first successful logical result. `filter` still warns when it rejects every result and prints no records. A failure on the first logical record warns nobody, even when a later parallel worker completed. A missing name on either side and equal names add no field. The run profile itself stays out of metadata because the field records a warning, not backend selection.

Library and SQL details forms carry the same optional warning in `meta`. The public Rust `Details::profile_warning()` returns the saved and running names in that order. Bare values have no warning channel; a caller that needs the comparison asks for details.

| Field | Holds |
| --- | --- |
| `tool` | The name and version of the binary that made the row |
| `question_sha256` | The digest of the exact question the row answers, as [question-file.md](question-file.md) fixes it. The same question typed and read from a file gives one digest, and any override gives another |
| `url` | The URL that answered |
| `model` | The model that answered, as the backend reported it |
| `usage` | The sum of the row's question shares, live or stored, by ADR 0111 section 7. Each answer's share is an even split of its request's reported counts, with the remainder to the earliest questions. Absent when any share lacks counts, as when a reply reported none. When every share has counts and their sum does not fit, the row fails with `the backend reported token counts whose total is too large`, a backend failure that exits 4 on the command |
| `requests_sent` | The HTTP attempts that produced this result. A replay or cache hit reports zero. Each retry of a retried status adds one. When a too-large batch halves, the refused whole request counts in the first half. A batched row carries its share of its request's attempts by the same rule, by ADR 0048 item 9. On the seven record commands, a row whose question joined another row's send in flight counts no attempt for it, so the rows' counts add up to the sends |
| `cached` | `true` when the answer came entirely from stored exchanges — a recording or a cache — rather than a live backend |
| `requests` | The stored-answer question keys behind this result, in answer order. These are request digests in the existing public vocabulary, not a list of transport sends. Packing and retries do not make their count equal to `requests_sent`. See the [glossary](../CONTRIBUTING.md#calls-requests-and-decisions) and [question-store contract](recording.md#the-question-store). |
| `failed_questions` | The number of failed logical questions in this result. Always present, including zero |
| `profile_warning` | The saved calibration profile and selected run profile when both exist and differ. Absent otherwise |
| `batch` | No longer written. Rows saved before ADR 0111 may hold it, with the run's `setting` beside the batch's `records`, `position`, `closed`, `usage` and `requests_sent`. `audit` and `diff` still read its `setting` when a row has no `batch_setting` |
| `batch_setting` | The run's resolved batch setting: a whole number of at least 1, or `"max"`. A structured JSON question runs one record a request and names 1. Present on each detailed record row of `decide`, `filter`, `rank`, `choose`, `tag` and `score`, from the command and from every library call over records. Absent on one document. `audit` and `diff` read it, by ADR 0085 |
| `batch_warning` | The file's tuned batch setting and the running one when they differ, as `{"tuned_for":1,"running":"max"}`. Absent otherwise. A file with a threshold and no batch key was tuned at 1, without changing the run's setting |
| `context_sha256` | The SHA-256 of the exact `--context` file bytes. Absent without a context |

## The run facts line

On an asking command, `--facts` prints one compact `thinkthen.run/1` JSON object as the last standard-error line. Without the flag, a finished run stays silent there. The line follows a stop diagnostic and any usage-counter warning, and precedes a stopping signal's re-raise. `records` counts finished input records, including filtered rows and rows dropped by `rank --top`; a one-document success and one finished `find` or `relate` set count one, while a plan counts zero and sends nothing. `requests_sent`, `retries`, and `cache_answers` come from this process's counters. `cache_answers` counts answers from the answer cache only, never from a replay or record folder, as [recording.md](recording.md#the-question-store) says. A started invocation includes `call_id`, the same opaque ID sent on its requests; a pre-start refusal or plan omits it. `seconds` is elapsed wall time in seconds, rounded to three decimals. `input_tokens` and `output_tokens` appear only if at least one live reply arrived, every live reply reported usage, and their exact sum remained valid. `model` appears only if at least one reply arrived and all live or stored replies named the same model. With both caller prices configured, `estimated_cost_usd` is a six-decimal string only when every started attempt supplied both token counts and the exact sum remained valid. Priced no-send, cache-only and replay-only work reports `"0.000000"`; an unpriced run omits this member. The estimate uses caller-selected prices, not a provider bill or admission limit.

A failed run adds `stopped` with `cause` and `retryable`, plus `at` when the stop line names a record. A signal stop has no `at`. `status` appears only for the `status` cause. The stable causes are `usage` for exit 2, `local` for exit 5, `no_key`, `transport`, `status`, `too_large`, `reply`, and `backend` for exit 4, `cancelled` for a stopping signal, and `defect` for exit 70. `too_large` covers status 413 and status 400 naming `max_tokens_exceeded`. Only `status` with a retried status (429, 500, 502, 503, 504, 520, 521, 522, 523, 524, or 529, as [backends.md](backends.md) lists) has `retryable:true`; transport has false because the request may have arrived. Exit 6 is a finished partial result and has no `stopped`.

## Compatibility

Under `thinkthen.result/1` a release may add a member to a row. It never renames or removes one, and it never changes a member's type or meaning. A change of that kind moves every row to `thinkthen.result/2`, and the changelog names it. A reader ignores a member it does not know. The rule binds from 0.1. ADR 0036's rename came before it. Compare `value` and the probabilities between runs, not the bytes, because a release can add a member.

Each command's detailed row holds these members. A member with a trailing `?` is sometimes absent. `input?` appears only on a record row. [spec/result.md](../spec/result.md) holds this table to rows the binary writes.

The CLI adds optional `position` to detailed line, JSONL, window, and document rows from `decide`, `filter`, `rank`, `choose`, `tag`, `score`, `find`, and `annotate`. It carries `file` (a path string, or `null` for stdin), `first`, and `last` (one-based physical lines). Each file restarts its line count. Text windows span their first through last physical lines; JSONL records span one line. A selected find candidate spans one physical line. An unresolved find result has no selected source and omits position. Find positions are CLI metadata; shared C, Rust and SQL typed Find results keep their existing members. CSV and TSV positions remain deferred. Default-document runs with several named files add `input_file` to each detailed `decide`, `choose`, `tag`, or `score` result. These members are CLI metadata and remain optional for library results and rows without a location. File access preserves the original operating-system path. JSON path strings preserve valid UTF-8 and use lossy display only for invalid UTF-8, replacing those bytes with the replacement character.

CLI rank-set details add optional `question_name`, naming the member that first selected the original. The answer probability, question digest and request/usage/cache receipt belong only to that member. `question.verb` remains decide; value and threshold are null. Ordinary rank omits the name. Rust's additive `SetRanked<T>` carries its name and probability without changing `Ranked<T>` or shared C/SQL shapes. Question observations keep every member answer with its original input index. Set facts count original records once.

The generated [result schema](result.schema.json) derives the shared Rust result types. Its `decisionDetails`, `findDetails` and `annotateDetails` definitions admit additive members, including this CLI metadata. The schema does not require these members on shared library results.

| Command | `question.verb` | Members | `meta` members |
| --- | --- | --- | --- |
| `decide` | `decide` | `schema` `position?` `input_file?` `value` `input?` `question` `answer` `threshold` `meta` | `tool` `question_sha256` `url` `model` `usage?` `requests_sent` `cached` `requests` `failed_questions` `profile_warning?` `batch_setting?` `batch_warning?` `context_sha256?` |
| `filter` | `decide` | `schema` `position?` `value` `input` `question` `answer` `threshold` `meta` | `tool` `question_sha256` `url` `model` `usage?` `requests_sent` `cached` `requests` `failed_questions` `profile_warning?` `batch_setting?` `batch_warning?` `context_sha256?` |
| `rank` | `decide`, `score` | `schema` `question_name?` `position?` `value` `input` `question` `answer` `threshold` `meta` | `tool` `question_sha256` `url` `model` `usage?` `requests_sent` `cached` `requests` `failed_questions` `profile_warning?` `batch_setting?` `batch_warning?` `context_sha256?` |
| `choose` | `choose` | `schema` `position?` `input_file?` `value` `input?` `question` `answer` `threshold` `meta` | `tool` `question_sha256` `url` `model` `usage?` `requests_sent` `cached` `requests` `failed_questions` `profile_warning?` `batch_setting?` `batch_warning?` `context_sha256?` |
| `tag` | `tag` | `schema` `position?` `input_file?` `value` `input?` `question` `answer` `threshold` `meta` | `tool` `question_sha256` `url` `model` `usage?` `requests_sent` `cached` `requests` `failed_questions` `profile_warning?` `batch_setting?` `batch_warning?` `context_sha256?` |
| `score` | `score` | `schema` `position?` `input_file?` `value` `input?` `question` `answer` `threshold` `meta` | `tool` `question_sha256` `url` `model` `usage?` `requests_sent` `cached` `requests` `failed_questions` `profile_warning?` `batch_setting?` `context_sha256?` |
| `find` | `find` | `schema` `position?` `value` `question` `answer` `threshold` `meta` | `tool` `question_sha256` `url` `model` `usage?` `requests_sent` `cached` `requests` `failed_questions` `profile_warning?` |
| `annotate` | none | `schema` `position?` `input` `value` `answers` `meta` | `tool` `questions_sha256` `url` `model` `usage?` `requests_sent` `cached` `requests` `failed_questions` `profile_warning?` |
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

A set-ranked row carries the selecting member name, keeping that member's probability and receipt:

```json rank-set
{"schema":"thinkthen.result/1","value":null,"input":{"id":"T-91","body":"Payouts have failed for 3 days."},"question":{"verb":"decide","text":"Does this help diagnose the failure?"},"answer":{"kind":"yes_no","probability":0.97},"threshold":null,"meta":{"tool":"thinkthen 0.4.0","question_sha256":"a1e3...df","url":"https://api.typesafe.ai/v1/systemone","model":"jev-1.13.0","usage":{"input_tokens":88,"output_tokens":12},"requests_sent":1,"cached":false,"requests":["6b1f...c4"],"failed_questions":0},"question_name":"billing"}
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

Read `annotate --details` as a row whose known outer members are `schema`, `input`, `value`, `answers`, `meta`, and optional CLI `position`. The names under `value` and `answers` come from the question set; read them with `to_entries` rather than hard-coded member names. An original object may itself have members named `input`, `value`, or `meta`, and those remain inside detailed `input`. The [mixed-stream recipe](annotate.md#read-a-mixed-record-stream) uses the presence of `failure` in each detailed answer entry to distinguish failure from a successful `null`. It does not infer a wrapper from the keys of a bare row.

The readable `question` in each answer prints `choose` options and `tag` labels as names. Their descriptions still take part in `meta.questions_sha256`, so identical readable options do not prove two sets identical. Use that digest for resolved question-set identity, and use `request` for the particular exchange that produced an answer. The [canonical question rules](question-file.md#the-canonical-form) define which descriptions and settings enter the digest.

`meta.usage` sums the record's question shares by the rule under `usage` above.

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

0447 image location addition preserves text SourceRecord's required line fields. Native ImageSourceRecord has record and file only. CLI image file details add flat file and position.file and omit first/last and first_line/last_line. Attachment details carry ordered source names in position.images beside typed input images; filenames never enter model state or identity. Existing native scalar/details/facts carriers remain available; additive complete image calls use the same result/2 identities and provenance as text.
