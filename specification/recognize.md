# Recognize entities

Status: **Settled** for default recognition under ADR 0056 and caller-defined recognition under ADR 0124. Relations are beta.

`thinkthen recognize [OPTIONS] [KIND]...` finds every name in one text and gives each one a caller-named kind. `thinkthen recognize [OPTIONS] @FILE` reads the same question from a file. With no kinds, every name has the kind `ENTITY`.

Recognize runs in three steps. Step 1 splits the text into pieces and asks where each piece stands in a name. Step 2 asks each found name's kind and checks its edges. Step 3 asks only the relation pairs a rule allows. The same path serves a sentence and a book.

## Per-record context

`--context-field POINTER` selects each JSON record's separate context through the same record reader as decide and rank. The pointer must exist. Missing, null and nontext values refuse before sending; a declared object `context_schema` admits an ordered object. An explicitly empty string clears shared `--context FILE` context. Without the selector, every record keeps shared context. Context remains separate from selected evidence and original record positions.

The CLI admits every selected record context before dispatch. Native `recognize_records_complete_with` and its fallible reader admit the existing `RecordInput.context` through the same declaration rules before any send. MCP and the named C complete call compose that carrier. Context reaches boundary, kind, edge and relation requests, changes their cache keys, and appears as `context_sha256` in complete row metadata when effective context exists. Unchanged context can replay saved exchanges; changed context must obtain its own answers.

One call shares the engine's endpoint, pool, concurrency limit and storage scope. CLI `--jobs` schedules independent records through that pool. Native record recognition retains its existing ordered execution. Each record batches questions within a stage according to existing backend limits; recognition does not pack questions from different records into one request.

Request adoption uses the existing per-record `context` field and reader projection `context_field`. Families adopting the versioned request in ADR 0125 must convert those values to `RecordInput` and `RecordReading`; they must not introduce another recognition context carrier. The legacy unversioned C JSON interface retains its recognition-record refusal.

## Tagged examples

Settled for 0.2 under [ADR 0127](../sdlc/planning/adr/0127-tagged-recognition-examples.md). `--examples FILE` supplies shared UTF-8 examples. A bracket file contains one example per nonempty line, using `[TEXT | KIND]`. Backslash escapes backslash, square brackets and the vertical bar. The single-bracket spelling trims syntax whitespace around its text and kind; use structured spans when those spaces belong to the original text. Single brackets without an unescaped vertical bar remain literal text. Nested tags, empty entities and incomplete tags refuse.

A JSON Lines file contains strings with bracket examples or closed objects containing `text`, `entities` and optional `kinds`. Each entity contains `start`, `end` and `kind`. Example offsets always count zero-based Unicode scalar values, with an exclusive end, even on hosts whose result offsets use another convention. An example file whose first nonempty line starts with `{` or a JSON string quote selects JSON Lines; otherwise it selects line-based bracket text. Empty lines are ignored. A malformed row refuses the whole file without trying another format.

```json
{"text":"Zoë met Orbit.","entities":[{"start":0,"end":3,"kind":"person"},{"start":8,"end":13,"kind":"organization"}],"kinds":["person","organization"]}
```

`--examples-field POINTER` selects an array of those example values from each original JSON or table record. A missing selected member retains shared examples; a present empty array clears them. Null, another type or traversal through an invalid container refuses. Context selection remains independent: clearing record context does not clear examples, and clearing examples does not clear context. The CLI admits every selected record and its initial bodies before sending any request. Bounded native feeds retain their existing admitted-prefix behavior.

Each example defaults to the resolved recognition kinds, or `ENTITY` when none were requested. A structured example's optional `kinds` declares a broader allowed vocabulary. Every entity must name an allowed kind, and its edges must coincide with recognition's own piece edges. Blank text, empty vocabularies, duplicate labels, overlapping spans and invalid edges refuse before sends. Valid entities outside the current recognition kind selection answer OUT; they still undergo span validation. Retrieval and example selection belong to the caller.

Recognition renders every piece through its actual step-1 questions, in order, and follows each question with BEGIN, INSIDE, END, SINGLE or OUT. Selected entity answers include their kind. The ordered rendered examples enter each boundary request once, beside its evidence. General record context retains its exact value. Kind, edge and relation requests receive no examples. Changing examples changes ordinary boundary cache and recording identity; identical bracket and span examples share identity. Omission and an empty selected list retain existing request bytes and keys. Examples remain call inputs and do not enter saved question files or the context digest.

`--plan` shows the exact rendered boundary bodies and includes their complete bytes in existing token accounting. Its request display describes the first record; its final summary counts every record. Nonempty examples make boundary bodies obey the configured request-byte ceiling and profile, including when a single question cannot fit. Groups may split without dropping examples. The historical no-example boundary behavior remains unchanged. Model-window admission and largest-request reporting belong to the separate recognition size controls.

Native callers use `Recognize::with_examples`, `RecordInput.examples` and `RecordReading::with_examples_field`. Canonical Request adoption uses recognize options `examples` and `examples_field`, plus explicit record `examples`. An explicit per-item list conflicts with a projection pointer. Types and schema come from the shared Rust carrier; family migration adopts it once. Software fixtures establish admission, rendering and replay behavior, not model improvement.

## Caller-defined entities

Settled for 0.2 by [ADR 0124](../sdlc/planning/adr/0124-caller-defined-recognition.md). `--instructions TEXT`, `--entity-definition TEXT` and existing `--kind KIND=DESCRIPTION` define the recognition task. Any supplied task wording or meaningful label description selects neutral entity questions. Dates, numbers, codes, amounts, units, ordinary words and web addresses can be entities. ThinkThen applies no fixed semantic suppression in this mode. Protocol labels and structural token rules retain their roles.

Every token, kind, decline and boundary question carries the complete caller declaration. Boundary questions preserve punctuation that belongs to the requested span. An instructions-only or definition-only declaration needs no kinds and produces `ENTITY`. A described kind needs no extra switch. Null and blank label descriptions retain their existing absent meaning. Instructions and entity definitions must be nonblank strings; explicit null, blank or another type refuses before sending. Multiline strings are accepted as task wording. ThinkThen executes no caller commands.

Saved version-one JSON places `instructions` and `entity_definition` inside `recognize`. Explicit CLI flags replace the corresponding saved values. Inline kinds and relations remain exclusive with `@FILE`. The canonical question description retains supplied fields and omits absent ones. Changing either field or a label description changes generated question, cache and replay identity. Omitting all customization preserves default wording and existing bare-kind recordings. Built-in custom BILOU descriptions use grammatical entity wording, including “an entity.” This correction intentionally changes custom question, cache and replay identity. Caller instructions, definitions and label descriptions retain their authored wording.

```json
{"version":1,"recognize":{"instructions":"Find the numeric receipt total, not the TOTAL label.","entity_definition":"The literal decimal amount.","kinds":{"amount":"The receipt total amount."}}}
```

The native Rust builder provides `instructions` and `entity_definition`. SDK typed constructors, common question-file readers, SQL declaration inputs, pandas, Polars and MCP carry the same declaration. C exposes the additive `thinkthen_recognition_task_v1` sidecar with copied inputs and owner-borrowed readers. Existing V1 layouts remain unchanged. Offline examples establish transport and output behavior; model accuracy requires the separately authorized evaluation.

## Step 1: boundaries

White space separates pieces. Each character of Unicode general category P or S is a piece of its own. A run of characters of category Mn, Me or Cf joins the piece that ends right before it. A run after white space or at the text's start begins a piece. At the start of the input only, a contiguous prefix of U+FEFF (BOM), U+200B (zero width space), U+200C (zero width non-joiner), U+200D (zero width joiner) and U+2060 (word joiner) is excluded from pieces and extracted spans. The original input remains intact and every excluded scalar still counts toward offsets. Internal occurrences and all other formatting or combining characters retain the rules above. Nothing else joins or splits. `Ada met Acme.` is four pieces: `Ada`, `met`, `Acme` and `.`.

Each piece gets one pick-one question: `BEGIN`, `INSIDE`, `END`, `SINGLE` or `OUT`. Default questions name bare caller kinds. Caller-defined questions include the complete task declaration. With no kinds the output kind is `ENTITY`. Each question shows a snippet of the selected number of pieces on each side (six when omitted), with the piece wrapped in `[[ ]]`.

When all customization is omitted, fixed step-1 wording lists person, organisation, place, product, work, event or other thing, or names the bare caller kinds. This is the current generic wording, not the earlier news-document question. The measurements below cover their named keys and public sets, not every caller kind or domain.

A step-1 request holds at most 40 consecutive pieces. Its evidence runs from the selected number of pieces before its first piece to the selected number of pieces after its last. A text of 40 pieces or fewer carries its whole text in the initial group. Wider caller windows can carry a whole longer text; grouping remains fixed.

After every step-1 request returns, a Viterbi decode picks the most likely valid tag sequence over the whole text. `BEGIN` and `INSIDE` must be followed by `INSIDE` or `END`. Each probability is floored at one in a million. On a tie the earlier tag in the order above wins. A `SINGLE` piece is a name, and so is a `BEGIN` through its `END`.

### Inspect the current question rendering

Generated question wording is subject to change. It is not a stable interchange format or an authored template to copy into context. The tagged-example inputs above are the stable interface for teaching recognition; the engine renders them with the current boundary wording.

Run this from a development checkout with its command on PATH:

```sh
printf 'Ada met Acme.' | env -u THINKTHEN_API_KEY thinkthen recognize person organization --plan \
  | sed -n '1p' | jq '.requests[0].body_utf8 | fromjson | .questions'
```

The plan exposes the first record's actual step-1 request bodies. In the current default rendering, each question asks where the `[[ ]]` token stands in a name of the requested kinds and appends a marked `Snippet:`. Its declared options are BEGIN, INSIDE, END, SINGLE and OUT, with descriptions. Caller task wording or described kinds selects the neutral entity rendering, which prefixes the complete caller declaration and asks where the token stands in an entity. Tagged examples appear in the boundary evidence; they do not replace the question text.

The plan cannot know which names the boundary answers will produce. It reports an upper bound for later name requests and does not show actual step-2 kind or edge questions. Read those questions from the native observer after replay or execution, as described below.

## Step 2: kinds and edges

Step 2 starts with one request for each step-1 request that holds a found name's first piece. Both stages split between complete questions to fit the effective encoded request-byte limit. A label menu always stays whole. Its evidence runs from the selected number of pieces before its first name to the selected number of pieces after its last. A request with no questions is not sent.

- **The kind question.** One per found name when the run has kinds. The options are the caller's kinds in order, each with its description when given, then `none of these`. A name whose answer is `none of these` is dropped.
- **The edge question.** One per found name that has two or more stretches to choose from. The options are the name as found, the name plus a touching mark at either end, and the name less a mark at either end. A one-piece name gets no removal option. A name with no edge question keeps its span.

An exact tie for a kind or edge option takes the first option asked. Kinds follow caller order, with `none of these` last. Edge options follow the order above.

With no kinds, only edge questions go out. When the edge pick leaves two names with the same start, end and kind, the one with the higher strength prints, and on equal strength the first.

The current default kind question asks which listed kind applies to the words wrapped in `[[ ]]`, appends a marked `Text:`, and offers caller kinds followed by `none of these`. The custom rendering prefixes the complete caller declaration and asks which kind applies to the marked entity. The edge question asks which option wraps the whole name, with punctuation belonging to the name kept inside. Its option labels are candidate span text; their descriptions carry the corresponding marked snippets. Custom edge wording also carries the caller declaration. These are descriptions of current generated shapes, not stable question wording.

## Names

One document prints one object:

```json
{"entities":[{"text":"Maria Chen","start":0,"end":10,"length":10,"kind":"person","strength":0.9987}]}
```

`start` is inclusive and `end` is exclusive. Both count Unicode scalar values, and `length` is `end - start`. Names print in order of `start`, then `end`. Equal names at different offsets remain separate. No name is a successful result: `{"entities":[]}`.

The recognized span is named `text` on every current surface. Offset units vary with the host:

| Surface | Recognized span field | `start`, `end`, and `length` |
| --- | --- | --- |
| Command, C JSON door, Rust, Ruby, Python scalar and frames | `text`; Python Polars frames add `row`, and pandas frames put spans in each row's `names` list | Zero-based Unicode scalar positions; `end` is exclusive |
| TypeScript | `text` | Zero-based UTF-16 code units; `end` is exclusive |
| R data frames | `text` | One-based Unicode character positions; `end` is inclusive |
| SQLite, PostgreSQL, DuckDB recognition rows | `text` | Zero-based Unicode character positions; `end` is exclusive |

`relate` reads a different entity shape with `name` and `kind`. It also accepts a recognized `text` when `name` is absent, so a recognized span can feed `relate` without renaming. [Shared case 41](../conformance/cases.json) fixes the offset difference for `Le café 😀 Maria Chen arrived.`; the recognized `Maria Chen` spans scalar positions `[10,20)`, TypeScript UTF-16 positions `[11,21)`, and R positions 11 through 20 inclusive.

`strength` is P(kind) times P(span), rounded to four decimal places. P(kind) is step 2's probability of the chosen kind. With no kinds, P(kind) is 1. P(span) is the share of the valid tag paths, weighted by their floored probabilities, that tag exactly the stretch step 1 found as one name. One forward-backward pass computes it from the probabilities the decode already holds. A widened name keeps its step-1 P(span). `strength` ranks names. It is not itself a probability.

`--threshold` keeps a name whose printed strength is at or above the cut, so `audit` rescoring a saved line matches a live run. The default is `0.5`, and the cut stays above 0. A name under the cut leaves before step 3.

Lowering the cut can keep a weaker name that step 1 found and step 2 classified. It cannot make step 1 decode a new stretch or restore a name step 2 declined. To inspect a missing name, use `--details`: `answer.pieces` gives each piece's five tag probabilities, and `answer.names` gives the stretches found before the cut with their kind and edge probabilities.

| Symptom | Dial and limit |
| --- | --- |
| A name is missing | Inspect `--details` first. Lower `--threshold` only if the name was found and classified but fell below the cut; it cannot recover a step-1 miss or a step-2 `none of these` decline. |
| Too many names appear | Raise `--threshold` to remove lower-strength names. This may also remove correct names and cannot repair a wrong span or kind. |
| A custom kind never appears | Give that kind a clear `--kind KIND=DESCRIPTION` description. It reaches the token, kind and boundary questions. Inspect `--details` to see whether the span was found, declined or fell below the cut. |
| Too many relation edges appear | Raise `--relation-threshold` to drop lower-probability edges, or lower it to retain more asked edges. The cut acts after pair requests and cannot reduce their count, create an unasked pair or change which names were found. Narrow the relation rules or input to reduce planned pairs. |

## Kinds

Bare kinds keep the caller's exact names. `--kind KIND=DESCRIPTION` gives a description, which reaches every recognition step. Bare and described kinds do not mix. A run accepts distinct nonblank kinds without a built-in kind-count maximum: `thinkthen: recognize takes distinct, nonblank kinds`. A selected profile may still limit `max_options`, including the internal `none of these` choice. A `--kind` with no `=` is refused at exit 2: ``--kind is KIND=DESCRIPTION, and this one holds no `=`; give a bare kind without --kind``.

`none of these`, `ENTITY` and `ANY` are reserved in any ASCII case. The refusal echoes no text: `thinkthen: recognize reserves the kind names none of these, ENTITY and ANY in any ASCII case`. It exits 2 on the command line and 5 from a question file.

## Relations

Relations are beta. `--relation NAME=SOURCE:TARGET` adds a directed rule and makes `relations` present. `--relation NAME` means `NAME=*:*`. Either side may be `*` or `ANY`, and the canonical question writes `*`. `NAME=KIND` is malformed. A concrete side naming a kind the run lacks exits 2.

Each ordered pair of kept names whose kinds match a rule's sides gets one yes/no question: `Does the text itself state that i1 READS i2?`. Under `either` it reads `Does the text itself state that i1 READS i2, or that i2 READS i1?`, and the pair is asked once. `*` expands to the kinds of kept names in first-seen order. Names with equal text and kind are asked once. A name is never related to itself. The pair questions share requests that carry the whole text, split at the request-size setting and at 400 questions. `--max-request-bytes N` sets that size, with `THINKTHEN_MAX_REQUEST_BYTES` next and 96,000 bytes at every address by default; a profile may lower it. `--relation-threshold` keeps an edge when its probability is at or above the cut. The default is `0.5`.

After steps 1 and 2 settle the names, a nonempty relation plan admits at most 255 distinct names whose kinds occur on a rule side and at most 4,000 prospective pair questions across all rules. A zero-pair plan returns an empty relation list before these limits. A larger nonempty plan refuses locally, with its count and a reduce-or-split remedy, before relation planning or any relation request; earlier name requests may already have been sent. This recognition guard does not apply to standalone `relate`.

An edge repeats both complete names:

```json
{"relation":"works_for","source":{"text":"Maria Chen","start":0,"end":10,"length":10,"kind":"person","strength":0.9987},"target":{"text":"Northwind Freight","start":18,"end":35,"length":17,"kind":"organization","strength":0.997},"probability":1.0}
```

A relation of an `either` rule ends with `"either":true`, and its ends are in the order the names were found; a directed relation writes no `either` member, as in [relate.md](relate.md#output).

`relations` is absent when no rule was supplied. It is an empty list when rules were supplied and no edge passed.

## The guard

A text over 600,000 UTF-8 bytes exits 2 before any request, at every address: `thinkthen: recognize: the text is N bytes, over the limit of M; raise it with --max-text-bytes`. `--max-text-bytes N`, from 1 to 2^53 - 1, sets the limit. It enters no digest. In record mode an oversize record fails that record.

## Records and details

Record modes print `{"input":INPUT,"value":OBJECT}` in input order. `--details` prints the complete `thinkthen.result/2` carrier, keeps the same object under `value`, lists every question key in question order, and sums send counts and independently available usage dimensions. Its answer ID uses the logical occurrence and actual accepted observations; source coordinates remain presentation fields. `answer.pieces` lists each piece's offsets and its five tag probabilities. `answer.names` lists each found name's span as found, its kind probabilities, and its edge option probabilities or null. `answer.pairs` lists each pair's probability.

Each stage admits all final adapter-encoded bodies before sending any request in that stage, including selected context and a preliminary kind-menu feasibility probe. A complete question that exceeds the effective request-byte limit refuses locally. Cache lookup cannot relax that limit. A later derived span can exceed it after step 1 has sent; that refusal sends no step-2 requests and retains the earlier sends, usage and recordings.

A failed step-1, step-2 or relation request fails that input. It prints no partial name or edge object for that input. Earlier completed record rows remain printed.

## Plan

`--plan` needs no key and sends nothing. It validates every record, then prints one compact `thinkthen.recognize-plan/2` object for the first record and a second count line for the whole input. This line marks `upper_bound:true`; its request count includes exact prepared step-1 requests plus possible name-stage and relation-stage requests for every input record. Future name and relation bodies cannot be known before earlier answers, so its byte sum and token band cover only exact prepared step-1 bodies. Keys appear in this order: `schema`, `url`, `model`, `key_env`, optional `from`, `pieces`, `request_count`, `name_requests_upper_bound`, the optional relation bounds, and `requests`. The count line also reports `largest_request_bytes`, `largest_request_estimated_input_tokens` and `token_estimate_method`. Plan maxima cover prepared execution bodies and exclude admission probes; they cannot promise the largest eventual derived span body. `--facts` and native call facts report the maxima of actual sent bodies, including earlier sends when a later stage refuses. An unsent refused body's size appears only in its safe limit diagnostic. The estimate method is `encoded-body-bytes-908-v1`; it estimates input tokens and does not guarantee model-window fit.

`pieces` counts pieces as step 1 splits them. `request_count` counts the prepared step-1 requests. `name_requests_upper_bound` is at most one edge question per possible name, plus one kind question when kinds are supplied: at most `pieces` without kinds or twice `pieces` with kinds. Each step-2 request holds at least one question, even when a profile splits that stage more finely than step 1. `requests` lists each step-1 request in send order, with its recording `digest`, UTF-8 `bytes`, and exact `body_utf8`. An empty or blank text prints no plan. It exits 2 with `thinkthen: the evidence is empty or blank`, as a live run does.

```json
{"schema":"thinkthen.recognize-plan/2","url":"https://api.typesafe.ai/v1/systemone","model":"jev-1.13.0","key_env":"THINKTHEN_API_KEY","pieces":4,"request_count":1,"name_requests_upper_bound":4,"requests":[{"digest":"…","bytes":1234,"body_utf8":"…"}]}
```

With relations, numeric `relation_pairs_upper_bound` and `relation_requests_upper_bound` sum safe per-rule bounds, because names and kinds do not exist yet. A file-backed report carries `{"from":{"question":"file"}}`.

## Measured quality

`specification/fixtures/recognize/README.md` replays ticket 0147's recorded runs on the repository's own keys. At the five core kinds the run scored F1 0.865. With no kinds it scored 0.884, at `person` alone 0.847, and on a 1,018-word text 0.960. It found 20 of 27 stated relation edges. Local experiment 288 measured 80.1 F1 on a full public split and 59.1 on WNUT-17 with the same wording and window. Each figure comes from one run.

This specification makes no public price claim.

## Review a flagged span with choose

A caller can review a span it has flagged by asking the existing [choose](choose.md) function to select from bounded candidate spans. Supply the current span as a `keep` option. Generate a small set of plausible alternatives near that span, with separate labels and descriptions containing their literal text, kind and original coordinates. Bound the neighborhood and candidate count in caller code. Ask a question that states the entity definition and explains when to keep the current span. This composition uses the existing functions; it adds no revise mode or function.

Validate every candidate against the unchanged original text before asking. Check integer offsets, a nonempty in-range span, and exact equality between the declared span text and the original slice. Use the offset convention of the surface that returned the recognition result. CLI and Rust spans count zero-based Unicode scalar values with an exclusive end; UTF-8 byte indices cannot slice them directly. Convert host offsets when crossing surfaces. Preserve the original record occurrence and span identity so repeated text cannot select the wrong occurrence.

The [saved question](fixtures/recognize/caller-defined/flagged-span.json) uses the original generic text `é TOTAL 42.75.`. For this review example, the caller flags `42.75.` at [8,14) as its current span. The first option, `keep`, retains it; the sole alternative, `amount`, proposes `42.75` at [8,13). The earlier recognition fixture already returns the correct amount; this separate example supplies a hypothetical flagged span. Each option description is an object that the existing question-file reader accepts.

Run the CLI from the repository root with the development command on PATH:

```sh
fixture=specification/fixtures/recognize/caller-defined
env -u THINKTHEN_API_KEY -u THINKTHEN_BASE_URL thinkthen choose \
  @"$fixture/flagged-span.json" --url "$(cat "$fixture/url.txt")" \
  --model jev-1.13.0 --replay "$fixture/recording" --no-cache < "$fixture/text.txt"
```

The controlled replay prints `"amount"` and exits 0. The [runnable caller recipe](fixtures/recognize/caller-defined/flagged-span.py) validates both candidates before running that command and maps its answer back to the candidate metadata:

```sh
python3 specification/fixtures/recognize/caller-defined/flagged-span.py
```

It prints `{"pick":"amount","proposed":{"start":8,"end":13,"text":"42.75","kind":"amount"}}`. Its local candidate cap is a caller choice. Both examples use the saved fixture without a key or network access. These authored fixture answers establish the composition and coordinate mapping; they measure no model accuracy.

In a real call, `choose` can return `null` with exit 3 for not sure. The recipe retains the current span in that case; other failures stop it. A `keep` answer retains the current span too. The caller owns any threshold, escalation and acceptance policy. Recheck the source text and candidate coordinates before applying a proposal if the source can change. The caller owns applying the result and checking any overlap with other entities. ThinkThen returns a label and never edits the original text or recognition result.

## Read native stage observations

Native Rust callers can inspect each logical question through `CallOptions::new().observe(&callback)` on `Engine::recognize_records_complete_with`. The [runnable recipe](../libraries/rust/examples/recognize_observe.rs) replays the repository's saved generic receipt fixture twice, retains each original record, and copies each borrowed `QuestionDetail` with `to_owned()` inside the callback. It groups snapshots by `(index, stage, position)` in a map. `index` is the zero-based original record occurrence; `position` is the zero-based question position within that record's stage. Stage names are `boundary`, `kind`, `edge` and `relation`. The map's lexical stage order is for lookup; it does not describe execution order.

Run the recipe from the repository root without a key:

```sh
env -u THINKTHEN_API_KEY cargo run --locked --offline \
  --manifest-path libraries/rust/Cargo.toml --example recognize_observe
```

For each snapshot, `owned.detail().question()` exposes the actual normalized logical question. `text().text()` reads its literal wording, and `options()` iterates declared options and their descriptions in order. The recipe prints those values, `value()`, `failure()`, `probabilities()` and `requests()`. Its pinned output maps record 0, boundary position 2 to the marked piece `42` and the answer `BEGIN`; record 1 retains the same logical mapping as a distinct original occurrence. The observer answer is the individual question's typed reading, not the later whole-text boundary decode or the final entity strength.

`RecordObservation::Row` marks a completed record and follows that record's questions. A failed request yields no complete result for that record; earlier completed rows and available question failure observations do not imply the whole call succeeded. Kind questions need caller kinds, edge questions need alternative spans, and relation questions need eligible pairs under a supplied rule. Every stage need not occur on every input.

Cache and strict replay can supply the same logical question observations without sending. `cached()` covers both; inspect `question_sources()` for actual cache/replay source metadata and retain absent historical counts as unknown. The recipe explicitly disables the cache, uses strict replay and reports zero sends. Its example gate runs with no key, pins the actual text/options/answers and record mapping, and counts zero connections at the saved loopback endpoint. These controlled fixture answers demonstrate the interface and replay behavior; they measure no model accuracy.

Serialized `RecordObservation` events omit actual question text. CLI `--details` provides question digests and stage distributions, but does not provide this indexed question transcript. Recordings retain request and response bodies, but do not themselves present the record/stage/position join. This recipe covers the native Rust observer route. It establishes no equivalent trace feature in other SDKs or the CLI.

The callback exposes supplied question and evidence text to the caller. Copy or own it before the callback returns, and protect any caller-created transcript under the same privacy rules as the originals and recordings. The recipe retains its snapshots only in memory and prints them to standard output; it introduces no trace store.

## Caller-supplied span proposals

Recognition accepts unconfirmed spans through `RequestOptions.seed_spans` or `RequestItem.seed_spans`. Each `RecognitionSeedSpan` holds integer `start` and `end` offsets and an optional `kind`. Offsets count Unicode scalar values in the actual selected recognition evidence, without normalization. The interval is zero-based and excludes its end. Both edges must exactly match existing recognition piece edges, and the interval must be nonempty and within the evidence. Admission refuses an edge inside a piece; it never rounds it.

`RequestOptions.seed_spans_field` and CLI `--seed-spans-field /seeds` select an array from each original JSON record. A missing member uses the shared fallback. An empty array clears it. Explicit null refuses. Explicit item spans conflict with a selection pointer. A present kind must exactly match a declared kind; omitted kind supplies no hint, and null refuses. These inputs are proposals for the current record. They are neither answered examples nor saved question spans.

Step 2 judges the union of supplied spans and decoded boundary proposals. Identical bounds appear once, regardless of their hints; distinct overlapping and nested proposals remain. Classification asks every declared kind and the decline option. Without declared kinds, supplied spans ask internal `ENTITY` and decline options, including one-piece spans. A supplied kind never forces or narrows the answer. Shared evidence extends to the furthest end in a proposal group.

Seeds preserve boundary questions and their request identity. Generated classification questions and evidence determine their own cache and replay identity. The existing final strength remains classification probability times boundary span probability. A seed that step 1 missed can therefore receive a confident classification in details and still fall below the final cut. Plans include seed overlaps in later name, classification and relation bounds; they send nothing and do not predict those answers.

## Stage context

Settled for 0.2 under [ADR 0126](../sdlc/planning/adr/0126-recognition-stage-context-and-boundary-proposals.md). The canonical Request `options.stage_context` and saved `recognize.stage_context` use the same typed `RecognitionStageContext`: optional literal strings `boundary`, `kind_edge` and `relation`. CLI `--boundary-context TEXT`, `--kind-edge-context TEXT` and `--relation-context TEXT` supply each member once. The existing `--context FILE` continues to read a file. Native `Recognize` supplies the same members through `boundary_context`, `kind_edge_context`, `relation_context` or `with_stage_context`.

Each call member replaces that saved member. An omitted member retains its saved value. A resolved stage member replaces selected per-record context; otherwise the stage retains record context, then shared context when record context is absent. Empty strings clear context for that stage. Strings never concatenate. `kind_edge` serves both question kinds sharing a step-2 request. Each selected record context still validates when every stage overrides it.

Null, nonstring members, unknown members and duplicate members refuse safely. An empty object behaves and serializes like omission. Complete output retains resolved controls in `question.stage_context`, including explicit empty member strings; bare output retains its existing shape. Debug withholds strings. Stage controls stay outside the reading digest and enter each affected stage's ordinary request and question identity. Complete answer identity includes the resolved controls and actual observations.

Admission prepares exact boundary bodies with effective boundary context and checks the available kind-menu probe with effective kind/edge context. Each later stage admits its final encoded bodies before its own sends. Derived edge and relation bodies depend on earlier answers; a later refusal can retain earlier sends. Context counts toward ordinary encoded body limits. Plan byte and token estimates continue to describe prepared boundary bodies, excluding probes and unknown future bodies. Omitted controls and an empty object preserve existing requests, keys and canonical descriptions.

### Boundary proposals

For 0.2, `RecognitionMode::BoundaryOnly`, canonical Request `options.mode: "boundary_only"`, saved `recognize.mode: "boundary_only"`, and CLI `--mode boundary_only` return decoded boundary proposals without kind, edge or relation questions. An explicit call mode replaces the saved mode; omission retains it and otherwise selects whole recognition. Explicit `whole` preserves the default canonical description and output.

The bare value is `{"mode":"boundary_only","proposals":[{"text":"Ada","start":0,"end":3,"length":3,"probability":0.9876}]}`. A successful empty value retains the mode and an empty proposals array. Each proposal preserves the decoded text, punctuation and existing host offsets, carries no kind or strength, and sorts by start then end. Its probability is P(span) over valid BILOU paths, rounded to four decimal places. The ordinary single threshold keeps proposals whose printed probability reaches the cut. Complete `answer` retains `pieces` and every pre-cut `proposal`; it carries no kind, edge or pair distributions. Native readers expose the proposal variant and probability tables directly.

Caller kinds, descriptions, instructions, entity definition and effective boundary context produce the same boundary bodies and individual question identities in both modes. Ordinary cache answers and recordings remain reusable. The resolved mode separates aggregate answer identity, including when whole recognition asks no later questions or finds no names. Caller seed spans remain validated; they augment the classification set in whole mode and do not add model boundary proposals in boundary-only mode.

Boundary-only recognition refuses any authored relations declaration, including an empty saved array, any authored relation threshold, and any resolved `kind_edge` or `relation` stage context, including empty strings. Admission resolves saved and call controls before evidence reads. The safe diagnostic is `boundary_only recognition takes no relations, relation threshold, kind_edge context or relation context`. Boundary descriptions omit the inapplicable implicit relation threshold, so a complete saved reading can rebuild without adding a forbidden control. Plans retain exact boundary requests and zero later-stage bounds.

Canonical RequestDefinition encoding preserves authored controls: it omits an implicit relation threshold and retains explicit relation cuts and empty authored relation arrays. This lets a native Request survive a canonical wire round trip without turning a default into a forbidden boundary control. Complete whole descriptions and result identities retain their existing default bytes.

## Recognition snippet width

Saved `recognize.snippet_pieces`, CLI `--snippet-pieces` and native `RecognizeBuilder::snippet_pieces(u32)` choose the tokenizer pieces shown on each side of each marked token or span and each request group. Omission keeps six; zero shows only the marked stretch. Shared request options admit `snippet_pieces` only for recognition and override saved values. Values must be whole unsigned 32-bit integers representable by the host's piece index. Negative, fractional, overflow and unrepresentable values refuse before sending. Clipping uses the available pieces at each end. Batch grouping and caller wording remain unchanged. Actual text, encoded request-byte and selected profile admission still apply; this setting makes no model token-fit or accuracy guarantee.

### Whole recognition proposal facts

The complete Rust result and CLI `--details` retain `answer.proposals` for the sorted union of decoded stretches and supplied seed bounds. Each proposal has original scalar `start` and `end`, unrounded `span_probability` from the valid floored BILOU paths, and `kept`. When a kind is selected, it also carries that `kind`, the actually computed four-place `strength`, and `selected` scalar bounds after edge choice. A declined kind omits these optional fields. Probability always describes the original proposal before edge adjustment. Duplicate resolution marks only the proposal that supplies the final entity; a tie retains the earlier proposal. Under-cut proposals retain their computed facts.

Rust callers read these facts through `CompleteRecognized::probabilities().judged_proposals()`. The existing `names()` tables retain complete kind and edge distributions for the same proposal order. Callers can inspect seed classification separately from final kept entities without recomputing boundary probability. Python's complete-result reader exposes the same fields as `RecognitionAnswer.proposals`; it continues to accept older documents without the field. Proposal facts change no bare value, request body, model call count, or cache/replay identity. A failed record exposes no partial complete proposal result.
