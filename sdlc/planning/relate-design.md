# `relate`: design authority for the command and future surfaces

Status: ticket 0081 landed on remote main at `4229bbfa`. Accepted ticket 0088 is unblocked. Ian's method, Option A detailed-result, and partial-output exit rulings remain settled. Sol Medium drives ticket 0088. This page authorizes no live or paid run.

Read `recognize-design.md` first. `recognize` and `relate` use one shared relation planner, request-state type, question map, and generic edge assembler. Recognition retains its complete name fields. Standalone relate returns name-and-kind endpoints.

## Delivery split

Ticket 0081 owns only the shared relation foundation: generic entity and edge ownership, concrete wildcard expansion, exact H state, runtime backend-profile fallback, request identity, and recognition compatibility. It adds no public `relate` command.

Ticket 0088 depends on landed 0081 and owns the complete public command: inline and `@entities` grammar, entity input and framing, bare output, exact dry-run schema, ruled Option A details, partial output at exit 6, help, specification, replay-only how-to, and secrecy proof. Neither ticket may reopen a settled product ruling or create a second planner, assembler, fallback path, threshold comparison, splitter, edge serializer, or request-state owner.

## The one line

**Find records that clash, repeat, or rely on each other.**

## Current command and field contract

```text
thinkthen relate [OPTIONS] RELATION...
thinkthen relate [OPTIONS] @entities.json
```

`@entities.json` is the complete relation question file. Entities still come from standard input or `--input FILE`; the `@` file never contains entity rows. One-way inline rules use `NAME=SOURCE_KIND:TARGET_KIND`. A bare `NAME` means `NAME=*:*`. `--either` marks the inline rules unordered. Inline rules and the `@entities` form are mutually exclusive.

The closed version-one file is:

```json
{"version":1,"relate":{"fields":{"name":"/name","kind":"/kind"},"relations":[{"name":"works_for","source":"person","target":"organization","reads":"works for","either":false}]},"threshold":0.5,"model":"jev-latest","profile":"measured-profile"}
```

`version`, `relate`, and a nonempty ordered `relations` array are required. `relate.fields` is optional. It contains exactly `name` and `kind`, each one JSON Pointer, and defaults to `/name` and `/kind`. Every relation contains exactly `name`, `source`, and `target`, plus optional `reads` and `either`. Names and concrete kinds are nonempty strings. `*` is the only wildcard spelling. `reads` defaults to the relation name with underscores replaced by spaces. `either` defaults to `false`. Relation names are distinct. `threshold` is one cut and defaults to `0.5`. `model` keeps its ordinary runtime meaning. Saved `profile` is only the safe name of the calibration profile used to tune the threshold. No framing, input path, entity, runtime profile file, method, packing, one-to-many, or runner-up key exists.

Command-line `--field`, `--kind-field`, `--threshold`, and `--model` override their matching file values independently. Saved calibration `profile` has no command-line override. `--profile FILE` instead selects the separate runtime backend-profile document from `specification/backends.md`; it supplies local limits and a running profile name but does not change saved calibration identity. Framing and `--input` remain command-only. An unreadable, invalid, extra-key, wrong-version, or wrong-shape question file exits 5. Naming a valid non-relate question file on the relate command exits 2.

The canonical question writes `verb`, `fields`, `relations`, `threshold`, then optional saved `profile`; each relation writes `name`, `source`, `target`, `reads`, then `either`. The question digest hashes those exact compact bytes. Saved calibration identity changes the digest. Entities, model, selected backend-profile file and name, address, and every other backend setting are absent. Selecting another `--profile FILE` alone does not change question identity.

JSONL, CSV, and TSV records default to `/name` and `/kind`. Exactly one `--field POINTER` overrides the name pointer, and exactly one `--kind-field POINTER` overrides the kind pointer. Both resolve independently against the original parsed record. CSV and TSV headers form that record object. Each selected value is a nonempty JSON string. Structured input never receives a synthetic kind.

`--lines` refuses both field options, takes each complete nonempty line as the name, assigns synthetic kind `*`, and accepts only a bare rule or `*:*`. The line set remains one same-kind set and does not expand.

The complete entity set is validated before any request. A missing pointer, wrong JSON type, blank selected value, duplicate name-and-kind identity, absent concrete rule kind, malformed inline rule, or 256th entity is exit 2 with zero sends.

Empty bytes under `--lines` or `--jsonl` succeed with no output and no request. A blank line under `--lines` is one invalid empty entity at exit 2; JSONL permits no blank line and refuses one at exit 2. Empty bytes under `--csv` or `--tsv` exit 2 because the required header is missing. A valid CSV or TSV header without data rows succeeds with no output and no request. Empty document input exits 2. Dry run has the same outcomes: successful empty streams and header-only tables print nothing.

No planner method, one-or-many marker, runner-up question, or packing control is public. The threshold defaults to `0.5` and accepts the cut.

## Exact dry-run schema

`--dry-run` validates the complete entity set, resolves the runtime backend profile, expands concrete relations, performs final fallback and 0079 splitting, and sends nothing. A nonempty file-backed run prints one compact object with this key order:

```json
{"schema":"thinkthen.relate-plan/1","url":"https://api.typesafe.ai/v1/systemone","model":"jev-1.13.0","key_env":"THINKTHEN_API_KEY","backend_profile":"local_1","framing":"jsonl","fields":{"name":"/name","kind":"/kind"},"from":{"question":"file","threshold":"file","model":"file","field":"file","kind_field":"file","profile":"file"},"entity_count":3,"relations":[{"name":"works_for","source":"person","target":"organization","reads":"works for","either":false,"method":"choice","fallback":null,"logical_questions":2,"request_count":1}],"logical_questions":2,"request_count":1,"requests":[{"digest":"<request-digest>","bytes":512,"body_utf8":"<exact compact request body>"}]}
```

`backend_profile` is the resolved name inside the selected `thinkthen.backend-profile/1` file and is null when no runtime profile was selected. It is never the question file's saved calibration `profile`. `framing` is `document`, `lines`, `jsonl`, `csv`, or `tsv`. `fields` is null for lines.

`from` is absent for inline rules. A file-backed run writes `question`, `threshold`, `model`, `field`, and `kind_field` in that order, each as `file`, `command line`, or `default`; optional `profile` follows only when the saved calibration identity exists and is always `file`. `--profile FILE` never appears in `from`, because `backend_profile` reports it separately.

`relations` contains one entry per expanded concrete relation in planning order. `method` is `choice` or `yes_no`. `fallback` is null, `max_options`, or `max_request_bytes`. Counts are exact after splitting. Each request entry is in send order. `bytes` is the UTF-8 byte length of `body_utf8`; decoding that JSON string yields the exact bytes that would be sent. The report makes no token, price, or backend-acceptance claim.

## Current bare output

Bare output emits one edge per line:

```json
{"relation":"sung_by","source":{"name":"Octopus's Garden","kind":"song"},"target":{"name":"Ringo Starr","kind":"person"},"probability":0.93}
```

One-way edges keep the rule's source-to-target direction even when the target side asked the choice. `--either` normalizes endpoints to input order. Edges print by relation declaration, concrete-kind expansion, asker or pair, then candidate order. No edge prints twice.

## Current final hybrid planner — authoritative

This is the only current method ruling. It applies to standalone relate and to recognition relations.

### Shared entity and edge ownership

`core/relation` owns one validated `RelationEntity { name, kind }`, one `RelationEntityView` implemented by `RelationEntity` and `RecognizedName`, one generic `RelationEdge<E>`, wildcard expansion, question mappings, and assembly. The planner remains index-based and generic over the view. The assembler owns the only threshold comparison, direction normalization, self exclusion, and mapping interpretation.

Relate parses `RelationEntity` and serializes `RelationEdge<RelationEntity>`. Recognition owns `RecognizedName` and serializes `RelationEdge<RecognizedName>`. The migration generalizes the existing edge and assembler before relate calls them. No command owns a second planner, mapping table, threshold comparison, edge type, or edge serializer.

### Wildcard expansion and method selection

The planner records admitted concrete kinds in first-seen entity order. A concrete rule side expands to itself. A rule `*` expands to every admitted concrete kind in that order. It plans each expanded concrete kind pair separately, keeps rule/kind/entity order, and excludes self-pairs. The synthetic line kind is one same-kind exception and does not expand.

Same-kind concrete relations use H. Both-way rules ask one unordered yes/no per pair. One-way rules ask one yes/no per ordered direction. Different-kind concrete relations ask from the side with more entities, offer the smaller side plus `none`, and ask from the declared source side on equal counts. Every non-`none` option at or above the cut becomes an edge.

`--either` removes reverse duplicates by first-seen kind and entity order. One-way `*:*` retains both directions. The user supplies no method or one-to-many marker.

### Exact option and backend-profile fallback

The fixed ceiling is 255 total wire options in one choice, including `none`. With runtime backend profile `max_options = N`, the effective ceiling is `min(255, N)`. A choice with exactly the effective ceiling stays choice. One over changes only that expanded concrete relation to H. Other concrete relations from the same wildcard rule retain their independently selected method.

`max_questions` never changes method; ticket 0079 splits the ordered plan. `max_request_bytes` first uses the 0079 splitter and changes a concrete relation to H only when one choice question with its complete state cannot fit alone. `max_evidence_bytes` never changes method and refuses the run. After fallback, the complete H plan is encoded and preflighted again. An impossible H plan exits 2 before replay, cache, key access, or a send.

Tests pin 255 and backend-profile equality as choice, 256 and backend-profile N+1 as H, a splittable request-byte overflow as choice chunks, an unsplittable one-choice byte overflow as H, and impossible final H as zero sends. For one wildcard rule, one mixed option-limit case and one mixed unsplittable-byte case each prove that exactly one concrete relation falls back while a sibling remains choice, with requests and edges in concrete expansion order. Recognition runs the same cases, and ticket 0081 updates `specification/recognize.md` from whole-rule wording to this per-concrete rule.

## Exact relation request state

Every relation request uses a typed state with this key order. Entity ids are `i1`, `i2`, and onward in original input order. The entity array contains the complete entity set. After wildcard expansion, `relation.source` and `relation.target` are the concrete kinds for this plan.

Recognition state includes its original normalized source text exactly:

```json
{"evidence":"Ada works for Acme.","entities":[{"id":"i1","name":"Ada","kind":"person"},{"id":"i2","name":"Acme","kind":"organization"}],"relation":{"name":"works_for","source":"person","target":"organization","reads":"works for","either":false}}
```

Standalone relate omits only `evidence`:

```json
{"entities":[{"id":"i1","name":"Ada","kind":"person"},{"id":"i2","name":"Acme","kind":"organization"}],"relation":{"name":"works_for","source":"person","target":"organization","reads":"works for","either":false}}
```

The typed state is converted to the existing structured `Evidence` path. Generic `Plan`, non-relation evidence, and the System One request encoder keep their current behavior. Relation request bytes, recording digests, and cache identities intentionally change to the final encoded state. New offline relation fixtures replace no historical experiment source.

A directed H wire question is exactly:

```json
{"type":"noul","instructions":"Does the relation hold from i1 to i2?"}
```

An either H wire question is exactly:

```json
{"type":"noul","instructions":"Does the relation hold between i1 and i2?"}
```

H sends no `criteria`. Its instructions repeat no name, kind, relation name, or `reads` text. The state carries those values once per request. Directed H question order is source entity then target entity. Either H uses normalized input order. Compiled recognize and relate tests pin both complete request bodies, source preservation, question order, and digest changes. A non-relation request fixture pins unchanged bytes.

## Ruled Option A detailed result — exact public schema

One `--details` run prints one compact `thinkthen.result/1` object. It has no top-level `input` or `threshold`; the resolved question carries fields, threshold, and optional saved calibration `profile` like recognize carries its resolved settings. Key order is `schema`, `value`, `question`, `answer`, `meta`.

```json
{"schema":"thinkthen.result/1","value":[{"relation":"works_for","source":{"name":"Ada","kind":"person"},"target":{"name":"Acme","kind":"organization"},"probability":0.84}],"question":{"verb":"relate","fields":{"name":"/name","kind":"/kind"},"relations":[{"name":"works_for","source":"person","target":"organization","reads":"works for","either":false}],"threshold":0.5},"answer":{"questions":[{"relation":"works_for","reads":"works for","method":"choice","direction":"source_to_target","asker":{"role":"source","entity":{"name":"Ada","kind":"person"}},"candidates":[{"role":"target","entity":{"name":"Acme","kind":"organization"},"probability":0.84,"accepted":true},{"role":"target","entity":{"name":"Other","kind":"organization"},"probability":0.10,"accepted":false},{"none":true,"probability":0.06,"accepted":false}],"pick":{"role":"target","entity":{"name":"Acme","kind":"organization"}},"request":"<request-digest>"}]},"meta":{"tool":"thinkthen 0.0.1","question_sha256":"<digest>","url":"https://api.typesafe.ai/v1/systemone","model":"jev-1.13.0","usage":{"input_tokens":100,"output_tokens":20},"requests_sent":1,"cached":false,"requests":["<request-digest>"],"failed_questions":0}}
```

For `--lines`, `question.fields` is `null`. For structured input it always contains the two resolved pointers. Each relation object always writes `name`, `source`, `target`, `reads`, and `either` in that order. Optional saved calibration `profile` follows `threshold`. The question digest hashes this exact compact `question` object; entities and runtime backend settings are absent. `usage` may be absent under the existing aggregate rule. `profile_warning` follows the existing saved-versus-running mismatch rule. Every other shown meta field is present, and `failed_questions` is always present.

`answer.questions` follows logical construction order and contains one of four entry unions below. Every entry ends with its producing request digest. `direction` is `source_to_target` for a one-way rule and `either` for an either rule.

### Successful choice entry

```json
{"relation":"works_for","reads":"works for","method":"choice","direction":"source_to_target","asker":{"role":"source","entity":{"name":"Ada","kind":"person"}},"candidates":[{"role":"target","entity":{"name":"Acme","kind":"organization"},"probability":0.84,"accepted":true},{"role":"target","entity":{"name":"Other","kind":"organization"},"probability":0.10,"accepted":false},{"none":true,"probability":0.06,"accepted":false}],"pick":{"role":"target","entity":{"name":"Acme","kind":"organization"}},"request":"<request-digest>"}
```

When the larger target side asks, `asker.role` is `target`, each entity candidate role is `source`, and `pick.role` is `source`. The public relation direction stays `source_to_target`; asking from the target side never reverses emitted edges. Under `--either`, direction is `either` and emitted endpoints normalize to input order.

Candidates remain in wire option order with `none` last. A real candidate has `accepted: true` exactly when its probability reaches the inclusive cut and therefore creates an edge. Several real candidates may be accepted. `none` is always false because it creates no edge. `pick` is the first highest-probability wire option before thresholding and is `{"none":true}` when none leads.

### Successful H entry

```json
{"relation":"duplicates","reads":"duplicates","method":"yes_no","direction":"either","source":{"name":"A","kind":"record"},"target":{"name":"B","kind":"record"},"probability":0.18,"accepted":false,"request":"<request-digest>"}
```

For a one-way H question, direction is `source_to_target` and source and target are the ordered edge roles tested by that question. For either H, source and target are normalized input order. `probability` is the backend probability of yes. `accepted` is true exactly at or above the inclusive cut.

### Failed choice entry

```json
{"relation":"works_for","reads":"works for","method":"choice","direction":"source_to_target","asker":{"role":"source","entity":{"name":"Ada","kind":"person"}},"candidates":[{"role":"target","entity":{"name":"Acme","kind":"organization"}},{"role":"target","entity":{"name":"Other","kind":"organization"}},{"none":true}],"failure":{"kind":"backend","cause":"missing_probability"},"request":"<request-digest>"}
```

### Failed H entry

```json
{"relation":"duplicates","reads":"duplicates","method":"yes_no","direction":"either","source":{"name":"A","kind":"record"},"target":{"name":"B","kind":"record"},"failure":{"kind":"backend","cause":"wrong_kind"},"request":"<request-digest>"}
```

Failed entries preserve the complete question identity and candidate identities. They omit `probability`, `accepted`, and `pick`. Failure causes reuse the closed result contract. `value` contains accepted edges only. `meta.failed_questions` equals the number of failed entries.

## Ruled outward behavior: partial-failure exit

The choice applies when one or more logical relation answers are valid and one or more are recoverably failed. A reply with no valid logical answer remains exit 4. Transport, status, replay, local, output, cancellation, and defect failures keep their existing codes and never become a partial success.

Ian chose exit 6 with partial output on 2026-09-23. Buffer the aggregate until every relation request succeeds at the request level. Emit successful bare edges, or the complete Option A object with failed entries, and exit 6 when any recoverable logical answer failed. This matches annotate, preserves valid paid answers, and gives bare pipelines a machine-readable incomplete-run signal.

Exit 4 with no output and exit 0 with partial output are rejected alternatives.

## Proof and delivery boundary

Ticket 0081 proves the shared foundation through pure relation tests and compiled recognition compatibility. It pins generic ownership, concrete expansion, exact state and H bodies, every fallback boundary, relation digest changes, replay/cache identity, zero-send final refusal, unchanged public recognition JSON, and one unchanged non-relation request body. It changes at most 10 production and 7 test-only Rust files and adds at most 800 production plus 700 test nonblank Rust lines, 1,500 gross.

Ticket 0088 adds relate to the full shared secrecy matrix, not a smaller command-specific substitute. The matrix covers bare/details, document/lines/JSONL/CSV/TSV, every settled empty-input case, success, dry run, cache, record, replay, replay miss, key absence, transport/status/decode and mixed logical failures, backend-profile and field refusals, hostile or damaged requested recordings, storage failure, and the authorization-header-only key path. It inspects stdout, stderr, `Debug`, request bodies, recording/cache files, and fixtures. Shared refusal tests count loopback requests on every local no-send path. It changes at most 15 production and 11 test-only Rust files and adds at most 1,200 production plus 1,100 test nonblank Rust lines, 2,300 gross.

Each ticket requires focused red/green proof and independent code review. The coordinator then runs `install`, `lint`, `test`, and `spec` sequentially from the exact candidate revision with key and base-address variables unset and no competing Rust build, followed by `git diff --check`. Every Rust file remains at or below 500 nonblank lines. Near-limit recognize, failure, result, args, and secrecy owners split or receive a new behavior-local owner before additions. No dependency, live call, or paid call enters either ticket.

## Future surfaces

Library and database surfaces remain future work. They must consume the same edge meaning and must not create a second planner. Tickets 0081 and 0088 do not change the `surfaces` tree or settle future host-language result types.

## Superseded history: 2026-09-21 pick-one proposal

The first proposal asked one pick-one question per unordered pair, with every legal relation and `no relation` as options. It used record numbers in output and packed pairs under backend limits. It predates entity input, the hybrid method, and name-and-kind endpoints. It is history only.

## Superseded history: 2026-09-23 all-H proposal

The all-H proposal asked one yes/no per pair for every relation. Ian's later final direction combines cross-kind choice with same-kind H. The all-H method is superseded.

## Superseded history: 2026-09-23 three-way proposal

The three-way proposal asked a directed pair to choose source-to-target, target-to-source, or neither. It is superseded. One-way relations now use cross-kind choice or ordered same-kind H.

## Current rule

Current rule: 0081 first lands first-seen concrete wildcard expansion, cross-kind choice, same-kind H, the lower of 255 and runtime backend-profile `max_options`, per-concrete fallback, one exact typed relation state, request identity, and one shared generic planner and edge assembler while recognition keeps offsets, strength, and its public output. Dependent 0088 then adds the complete `@entities` command grammar, saved calibration identity, name-and-kind-only standalone endpoints, exact dry-run schema, ruled Option A detailed schema, partial output at exit 6, documentation, and secrecy proof. The Luna trial stopped after two remediation passes. Sol Medium drives both tickets, and each starts product code only after independent Sol design acceptance.
