# `relate`: design authority for the command and future surfaces

Status: amended after Sol's second 0081 design rejection. Ian's Option A detailed-result and partial-output exit rulings are settled. This page authorizes no live or paid run.

Read `recognize-design.md` first. `recognize` and `relate` use one shared relation planner, request-state type, question map, and generic edge assembler. Recognition retains its complete name fields. Standalone relate returns name-and-kind endpoints.

## The one line

**Find records that clash, repeat, or rely on each other.**

## Current command and field contract

```text
thinkthen relate [OPTIONS] RELATION...
thinkthen relate [OPTIONS] @links.json
```

`@links.json` is a relation question file. Entities come from standard input or `--input FILE`. One-way rules use `NAME=SOURCE_KIND:TARGET_KIND`. A bare `NAME` means `NAME=*:*`. `--either` makes one unordered relation. The question file carries the same rules and may add `reads`.

JSONL, CSV, and TSV records default to `/name` and `/kind`. Exactly one `--field POINTER` overrides the name pointer, and exactly one `--kind-field POINTER` overrides the kind pointer. Both resolve independently against the original parsed record. CSV and TSV headers form that record object. Each selected value is a nonempty JSON string. Structured input never receives a synthetic kind.

`--lines` refuses both field options, takes each complete nonempty line as the name, assigns synthetic kind `*`, and accepts only a bare rule or `*:*`. The line set remains one same-kind set and does not expand.

The complete entity set is validated before any request. A missing pointer, wrong JSON type, blank selected value, duplicate name-and-kind identity, absent concrete rule kind, malformed rule, or 256th entity is exit 2 with zero sends. Empty input succeeds after rule syntax validation with no request and no edge.

No planner method, one-or-many marker, runner-up question, or packing control is public. The threshold defaults to `0.5` and accepts the cut. `--dry-run` sends nothing and reports the final routed methods, fallback facts, logical question counts, split request counts, and exact encoded bytes without token or price claims.

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

### Exact option and profile fallback

The fixed ceiling is 255 total wire options in one choice, including `none`. With profile `max_options = N`, the effective ceiling is `min(255, N)`. A choice with exactly the effective ceiling stays choice. One over changes the whole concrete relation to H. Other concrete relations from the same wildcard rule retain their selected method.

`max_questions` never changes method; ticket 0079 splits the ordered plan. `max_request_bytes` first uses the 0079 splitter and changes a concrete relation to H only when one choice question with its complete state cannot fit alone. `max_evidence_bytes` never changes method and refuses the run. After fallback, the complete H plan is encoded and preflighted again. An impossible H plan exits 2 before replay, cache, key access, or a send.

Tests pin 255 and profile equality as choice, 256 and profile N+1 as H, a splittable request-byte overflow as choice chunks, an unsplittable one-choice byte overflow as H, and impossible final H as zero sends. Recognition runs the same cases.

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

One `--details` run prints one compact `thinkthen.result/1` object. It has no top-level `input` or `threshold`; the resolved question carries fields and threshold like recognize carries its resolved settings. Key order is `schema`, `value`, `question`, `answer`, `meta`.

```json
{"schema":"thinkthen.result/1","value":[{"relation":"works_for","source":{"name":"Ada","kind":"person"},"target":{"name":"Acme","kind":"organization"},"probability":0.84}],"question":{"verb":"relate","fields":{"name":"/name","kind":"/kind"},"relations":[{"name":"works_for","source":"person","target":"organization","reads":"works for","either":false}],"threshold":0.5},"answer":{"questions":[{"relation":"works_for","reads":"works for","method":"choice","direction":"source_to_target","asker":{"role":"source","entity":{"name":"Ada","kind":"person"}},"candidates":[{"role":"target","entity":{"name":"Acme","kind":"organization"},"probability":0.84,"accepted":true},{"role":"target","entity":{"name":"Other","kind":"organization"},"probability":0.10,"accepted":false},{"none":true,"probability":0.06,"accepted":false}],"pick":{"role":"target","entity":{"name":"Acme","kind":"organization"}},"request":"<request-digest>"}]},"meta":{"tool":"thinkthen 0.0.1","question_sha256":"<digest>","url":"https://api.typesafe.ai/v1/systemone","model":"jev-1.13.0","usage":{"input_tokens":100,"output_tokens":20},"requests_sent":1,"cached":false,"requests":["<request-digest>"],"failed_questions":0}}
```

For `--lines`, `question.fields` is `null`. For structured input it always contains the two resolved pointers. Each relation object always writes `name`, `source`, `target`, `reads`, and `either` in that order. The question digest hashes the exact compact `question` object; entities and backend settings are absent. `usage` may be absent under the existing aggregate rule. Every other shown meta field is present, and `failed_questions` is always present.

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

Implementation adds relate to the full shared secrecy matrix, not a smaller command-specific substitute. The matrix covers bare/details, all four framings, success, dry-run, cache, record, replay, replay miss, key absence, transport/status/decode and mixed logical failures, profile and field refusals, hostile/damaged recordings, storage failure, and the authorization-header-only key path. It inspects stdout, stderr, `Debug`, request bodies, recording/cache files, and fixtures. Shared refusal tests count loopback requests on every local no-send path.

After focused red/green proof and independent code review, the coordinator runs `install`, `lint`, `test`, and `spec` sequentially with key and base-address variables unset and no competing Rust build, then runs `git diff --check`. No live or paid call belongs to a gate.

Ticket 0081 may change at most 18 production Rust files and 12 test-only Rust files and add at most 1,650 production plus 1,550 test nonblank Rust lines, 3,200 gross. Every file remains at or below 500 lines. Near-limit recognize, failure, result, args, and secrecy owners split or receive a new behavior-local owner before additions. No dependency enters.

## Future surfaces

Library and database surfaces remain future work. They must consume the same edge meaning and must not create a second planner. Ticket 0081 does not change the `surfaces` tree or settle future host-language result types.

## Superseded history: 2026-09-21 pick-one proposal

The first proposal asked one pick-one question per unordered pair, with every legal relation and `no relation` as options. It used record numbers in output and packed pairs under backend limits. It predates entity input, the hybrid method, and name-and-kind endpoints. It is history only.

## Superseded history: 2026-09-23 all-H proposal

The all-H proposal asked one yes/no per pair for every relation. Ian's later final direction combines cross-kind choice with same-kind H. The all-H method is superseded.

## Superseded history: 2026-09-23 three-way proposal

The three-way proposal asked a directed pair to choose source-to-target, target-to-source, or neither. It is superseded. One-way relations now use cross-kind choice or ordered same-kind H.

## Current rule

Current rule: `relate` uses first-seen concrete wildcard expansion, cross-kind choice, same-kind H, the lower of 255 and profile `max_options`, one exact typed relation state, one shared generic planner and edge assembler, name-and-kind-only standalone endpoints, the ruled Option A detailed schema, and partial output at exit 6. Recognition keeps offsets and strength. Implementation starts only after Sol accepts the amended design.
