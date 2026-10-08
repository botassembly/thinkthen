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

Settled for 0.2 under [ADR 0127](../sdlc/planning/adr/0127-tagged-recognition-examples.md). `--examples FILE` supplies shared UTF-8 examples. A bracket file contains one example per nonempty line, using `[TEXT | KIND]`; `[[KIND|TEXT]]` is an equivalent spelling. Backslash escapes backslash, square brackets and the vertical bar. The single-bracket spelling trims syntax whitespace around its text and kind; use structured spans when those spaces belong to the original text. Single brackets without an unescaped vertical bar remain literal text. Nested tags, empty entities and incomplete tags refuse.

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

Each piece gets one pick-one question: `BEGIN`, `INSIDE`, `END`, `SINGLE` or `OUT`. Default questions name bare caller kinds. Caller-defined questions include the complete task declaration. With no kinds the output kind is `ENTITY`. Each question shows a snippet of six pieces on each side, with the piece wrapped in `[[ ]]`.

When all customization is omitted, fixed step-1 wording lists person, organisation, place, product, work, event or other thing, or names the bare caller kinds. This is the current generic wording, not the earlier news-document question. The measurements below cover their named keys and public sets, not every caller kind or domain.

A step-1 request holds at most 40 consecutive pieces. Its evidence runs from six pieces before its first piece to six pieces after its last. A text of 40 pieces or fewer sends its whole text once. A longer text never sends its whole text in one step-1 request.

After every step-1 request returns, a Viterbi decode picks the most likely valid tag sequence over the whole text. `BEGIN` and `INSIDE` must be followed by `INSIDE` or `END`. Each probability is floored at one in a million. On a tie the earlier tag in the order above wins. A `SINGLE` piece is a name, and so is a `BEGIN` through its `END`.

## Step 2: kinds and edges

Step 2 sends one request for each step-1 request that holds a found name's first piece. Its evidence runs from six pieces before its first name to six pieces after its last. A request with no questions is not sent.

- **The kind question.** One per found name when the run has kinds. The options are the caller's kinds in order, each with its description when given, then `none of these`. A name whose answer is `none of these` is dropped.
- **The edge question.** One per found name that has two or more stretches to choose from. The options are the name as found, the name plus a touching mark at either end, and the name less a mark at either end. A one-piece name gets no removal option. A name with no edge question keeps its span.

An exact tie for a kind or edge option takes the first option asked. Kinds follow caller order, with `none of these` last. Edge options follow the order above.

With no kinds, only edge questions go out. When the edge pick leaves two names with the same start, end and kind, the one with the higher strength prints, and on equal strength the first.

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

Bare kinds keep the caller's exact names. `--kind KIND=DESCRIPTION` gives a description, which reaches every recognition step. Bare and described kinds do not mix. A run takes 0 to 20 distinct nonblank kinds: `thinkthen: recognize takes 0 to 20 distinct, nonblank kinds`. A `--kind` with no `=` is refused at exit 2: ``--kind is KIND=DESCRIPTION, and this one holds no `=`; give a bare kind without --kind``.

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

A failed step-1, step-2 or relation request fails that input. It prints no partial name or edge object for that input. Earlier completed record rows remain printed.

## Plan

`--plan` needs no key and sends nothing. It validates every record, then prints one compact `thinkthen.recognize-plan/2` object for the first record and a second count line for the whole input. This line marks `upper_bound:true`; its request count includes exact prepared step-1 requests plus possible name-stage and relation-stage requests for every input record. Future name and relation bodies cannot be known before earlier answers, so its byte sum and token band cover only exact prepared step-1 bodies. Keys appear in this order: `schema`, `url`, `model`, `key_env`, optional `from`, `pieces`, `request_count`, `name_requests_upper_bound`, the optional relation bounds, and `requests`.

`pieces` counts pieces as step 1 splits them. `request_count` counts the prepared step-1 requests. `name_requests_upper_bound` is at most one edge question per possible name, plus one kind question when kinds are supplied: at most `pieces` without kinds or twice `pieces` with kinds. Each step-2 request holds at least one question, even when a profile splits that stage more finely than step 1. `requests` lists each step-1 request in send order, with its recording `digest`, UTF-8 `bytes`, and exact `body_utf8`. An empty or blank text prints no plan. It exits 2 with `thinkthen: the evidence is empty or blank`, as a live run does.

```json
{"schema":"thinkthen.recognize-plan/2","url":"https://api.typesafe.ai/v1/systemone","model":"jev-1.13.0","key_env":"THINKTHEN_API_KEY","pieces":4,"request_count":1,"name_requests_upper_bound":4,"requests":[{"digest":"…","bytes":1234,"body_utf8":"…"}]}
```

With relations, numeric `relation_pairs_upper_bound` and `relation_requests_upper_bound` sum safe per-rule bounds, because names and kinds do not exist yet. A file-backed report carries `{"from":{"question":"file"}}`.

## Measured quality

`specification/fixtures/recognize/README.md` replays ticket 0147's recorded runs on the repository's own keys. At the five core kinds the run scored F1 0.865. With no kinds it scored 0.884, at `person` alone 0.847, and on a 1,018-word text 0.960. It found 20 of 27 stated relation edges. Local experiment 288 measured 80.1 F1 on a full public split and 59.1 on WNUT-17 with the same wording and window. Each figure comes from one run.

This specification makes no public price claim.
