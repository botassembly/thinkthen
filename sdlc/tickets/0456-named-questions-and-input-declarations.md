# 0456: Add named questions and declared item inputs

Status: inprogress. Native implementation is underway in lane 0; no whole-code review or landing claim. The shared design passed fresh High review after correcting annotate document admission. Slice A records the design and plan; native implementation and all surface adoption remain open.

Milestone: 0.2

Owner: native builder; lane 2 holds the reviewed design and plan slice.
Risk: High: input disclosure and question-file compatibility. Preserve 0.1 loading and keep declaration failures before model sends.
Ticket review: ACCEPT 2026-10-06. Fresh High review; the sole typed-annotate finding was corrected and its resolution confirmed.
Dependencies: no new dependencies planned; use existing typed JSON, serde and native reader. A real new dependency requires the Rust standards second reviewer.

## Outcome

Single question files admit optional question names and versions and optional simple item and per-item context declarations. Explicit named references load from the config questions folder. The CLI, Rust, C, every language SDK, SQL and MCP reuse the same loaders and validation. Existing file paths, literal questions and 0.1 files keep working. No proxy catalog, automatic A/B routing or question-writing service enters the SDK.

## Evidence

- Starts from: PM message `inbox/thinkthen/2026-10-06-pm-add-question-names-a-named-questions-folder-and-input-declarations-to-0-2.md`, asks 1 to 5, and its question-catalog design input dated 2026-10-06. Ian approved this additive 0.2 outcome. Shared result/cache contract 0442, single-route ruling 0449 and complete-input owners 0407/0414 are prior contracts.
- Keeps: Ordinary absolute and relative question paths, 0.1 question-file grammar and schema version, inline literal strings including MCP inline @ text, native readers, reading controls, saved-answer rereading, per-question cache identity, one engine/endpoint and six error kinds. Keys stay in approved environment variables. Question files remain read-only.
- Changes: Record Ian's distinction between question contract, wording version, reading and model setup. Wording and option-description edits version the same question; fixed choose values, tag labels or score levels define a new question. Threshold/calibration remain reading controls and model remains setup/routing. Resolve additive name/version spelling without reusing the existing format-version field. Resolve @NAME through `thinkthen/questions/<name>.json` under the selected config home while preserving ordinary path precedence and platform behavior. Admit the requested small JSON Schema subset: string or object, named string/number/boolean/string-list properties and a required list. Validate actual selected items and separate per-item context in native admission before malformed work can send. Shared context remains plain text. Surface families expose these known fields with typed declarations and reuse native admission; they add no schema parser or local catalog cache.
- Proof: Existing 0.1 fixtures load unchanged. Cover ordinary path/named collisions, absent config files, traversal-shaped named references, explicit path access, unknown/invalid declarations, required/mistyped fields, Unicode, false/null, projection/context distinctions and declared image ancillary inputs. Count loopback requests for malformed input and cache/record/replay cases. Exercise named public surfaces through shared cases; fixture decoding does not establish execution parity. Retain source bounds, cancellation and secrecy. A fresh whole-change review and full tests/lint on the landing commit suffice.
- Defers: Remote catalog lookup/push, studio, project views, automatic A/B/model routing, policy enforcement over historical catalog versions, schema features outside the admitted subset and paragraph windows.

## Order and ownership

Settle this shared format before complete C/MCP and binding-family adoption. One native owner changes the question grammar, native loader/admission and shared typed views. Family owners 0426–0431, SQL 0434/0452 and MCP 0455 adopt the settled contract; 0432 extends the real public-case table. Existing 0407 and 0414 are must-land 0.2 dependencies, not new duplicate tickets. Design resolves declaration/projection/streaming and optional-context rules explicitly, preserves retained behavior and records the supported defaults without configuration questions to Ian.

## Proposed shared contract for review

Ian approved this additive format amendment on 2026-10-06. Fresh High review accepted this design; implementation updates question-file/settings/records/result contracts, generated schemas and their shared corpus together. It neither activates ADR 0120 proxy reservations nor supersedes ADR 0119. Ian can overturn field names, name bounds, collision rules and declaration admission through a reviewed amendment.

### Author metadata and format

Add optional top-level `name`, `wording_version`, `item_schema` and `context_schema` to single decide/choose/tag/score question files. Filter and rank reuse these files. Admit the same fields on individual questions inside existing question sets, and on existing recognize/relate file envelopes; retain their `version:1` format marker and all verb-specific rules. No metadata on the question-set envelope is added. Find's existing literal/builder questions acquire the same typed optional metadata/declarations without inventing a new find-file grammar.

`name` is a case-sensitive ASCII string matching `[a-z][a-z0-9_-]{0,63}`. `wording_version` is a JSON integer from 1 through 2147483647, never a string, fractional/exponent spelling, boolean or null. Each field is independently optional; absent stays absent, with no inferred name/version or default 1. The existing `version` spelling remains reserved for format envelopes: a single decide/choose/tag/score file still refuses it. Other unknown keys and duplicate keys still refuse.

These are declarative author claims. A wording or description edit is a new wording version of the same conceptual question; changing fixed choose values, tag labels or score levels is a new question. The SDK neither enforces version increments nor compares historical files. Dynamic per-item options remain caller inputs under 0413. Threshold/calibration define a reading; the model defines setup. Callers choose files, versions, readings and literal models explicitly. There is no version selector, history database or A/B scheduler.

Do not add these four fields to canonical question/set digests, wire questions/shared state, cache keys, recording lookup, observation-ID derivation or answer-ID reading/scope framing. Existing set member keys still enter set identity/scope exactly as before; authored `name` never replaces a member key or rank's `question_name`. Wording/descriptions still change existing behavioral identity; thresholds retain existing digest/reading behavior; effective context and images retain ADR 0120 state identity. Unchanged semantic inputs retain old keys and IDs. Validation runs before cache/replay lookup too, so cached answers cannot bypass a new declaration. On a hit, readable question metadata describes this caller's resolved question; it does not rewrite or attest historical observations.

### Exact declaration grammar

Each schema is a JSON Schema object from this closed subset, not a shorthand string:

```json
{"type":"string"}
```

```json
{"type":"object","properties":{"body":{"type":"string"},"amount":{"type":"number"},"ready":{"type":"boolean"},"labels":{"type":"array","items":{"type":"string"}}},"required":["body"]}
```

Root string takes exactly `type`. Root object takes exactly `type`, required `properties` (an object, possibly empty), and optional `required` (a list of distinct property names, default empty). Each property takes exactly one of the four shapes shown above. Property names are literal JSON keys, not pointers: nonempty printable Unicode, no control characters; preserve spelling and order. Required names must appear in properties. Arrays contain only strings; empty arrays and empty strings are structurally valid. Numbers must be finite JSON numbers; booleans including false are values. Declared properties present as null fail; absent optional properties pass. Root null fails. Undeclared input properties are allowed and keep existing selection/wire behavior; declarations never project, drop or add fields.

No nested object/array properties, unions, nullable types, references, defaults, enums, constraints, formats, coercion, `additionalProperties`, `$schema`, `$id`, descriptions or other JSON Schema keywords are admitted. Reject unknown features rather than ignoring them. Reuse duplicate-member refusal and existing file/JSON depth/byte bounds. No external schema resolver or general JSON Schema dependency. Declaration strings such as `"string"` and empty/null schema values refuse.

### Native API and typed propagation

Proposed additive public Rust types/accessors (owned fields below are private and validated through constructors):

```rust
QuestionName::new(&str) -> Result<QuestionName, Error>
WordingVersion::new(u32) -> Result<WordingVersion, Error>
enum InputPropertyType { String, Number, Boolean, StringList }
enum InputDeclaration { String, Object(ObjectDeclaration) }
ObjectDeclaration::new(Vec<InputProperty>, Vec<String>) -> Result<ObjectDeclaration, Error>
InputProperty::new(&str, InputPropertyType) -> Result<InputProperty, Error>
Question::load_named(&str) -> Result<LoadedQuestion, Error>
Question::load_reference(&str) -> Result<LoadedQuestion, Error>
Question::name(&self) -> Option<&QuestionName>
Question::wording_version(&self) -> Option<WordingVersion>
Question::item_schema(&self) -> Option<&InputDeclaration>
Question::context_schema(&self) -> Option<&InputDeclaration>
```

Equivalent validated builder setters are `with_name`, `with_wording_version`, `with_item_schema`, `with_context_schema`; banded/choice wrappers and resolved views expose the same accessors. Extend the native branch's `ResolvedQuestion`/resolved member views rather than adding a second normalized question. Object declarations expose ordered typed properties and required names. Known declarations never become opaque JSON-only accessors. Native admission accepts existing typed input values; structured per-item context adds an explicit typed object carrier under 0456, coordinated with 0407. Undeclared legacy per-item contexts retain 0407's string-only/null-refusal rule; a declared object context admits the exact typed object subset and native separate context serialization. Do not parse JSON-looking text into an object.

Complete result/2 readable `question` objects preserve the four optional fields with those exact JSON spellings. Structured recognize/relate result views expose their authored metadata/declarations on their existing question carrier, not generated internal relation questions. Generated question-file/result schemas and strict complete readers change together. Bare answers and generic compatibility projections retain their shapes. C0426 owns additive typed accessors/lifetimes and generic JSON admission; 0427–0431 own idiomatic typed equivalents, 0410/0296 frames, 0434/0452 SQL, 0455 MCP and 0432 shared execution. Hosts use native loading/validation; no per-host schema parser, resolver or catalog cache.

### References, paths and configuration home

`Question::load(path)` and MCP `question_file` keep literal path-only semantics, including a filename beginning with @. `from_json` stays inline JSON. `load_named(name)` resolves only the validated name. `load_reference(reference)` requires a leading @ and follows the CLI rule below; it never treats ordinary text as a filename. Recognize/relate/set loaders expose matching named/reference entry points for their existing file roles, retaining wrong-role errors. All invoke the existing capped native reader (1 MiB), with filesystem/environment work outside core.

For CLI @ references, strip one @. An absolute path, a Windows drive/UNC path, or a value containing a slash/backslash, a dot or otherwise failing the name grammar remains an ordinary path relative to the working directory when not absolute. For a bare valid NAME, an existing working-directory entry named NAME wins; a dangling link, directory or unreadable entry fails locally without fallback. If no such entry exists, load the named folder's NAME.json. Thus `@refund.json`, `@./refund`, `@../refund.json` and absolute paths keep their old meanings; `@refund` adds lookup only where the old bare path is absent. Explicit `load_named("refund")` bypasses that collision. No fallback after any file parse/read failure.

MCP adds mutually exclusive `question_name` (validated bare name) beside existing `question_file` and inline question arguments. Inline `"@refund"` remains literal text. C/SDK/SQL name loading is explicit too; do not reinterpret their existing string questions or path loaders. File contents may omit `name`; when loaded by name, a present authored name must equal the requested basename or fail locally. No global uniqueness rule or scans: one exact filename per name, no case folding, suffix inference beyond `.json`, recursion or version search.

Use the native platform config directory independently of whether config.json exists: Linux absolute XDG_CONFIG_HOME/thinkthen, else absolute HOME/.config/thinkthen; macOS absolute HOME/Library/Application Support/thinkthen (existing macOS convention, not XDG); Windows absolute APPDATA/thinkthen. Append questions/NAME.json. Relative/empty bases retain existing config fallback rules. No usable root, absent folder/file or name mismatch gives a plain Local failure. Never create directories, read config.json to discover names or consult credential files. Named lookup accepts regular files whose canonical location stays within the canonical questions directory; reject directory/file symlinks escaping it, including a questions directory redirected outside the platform config directory. Ordinary explicit paths retain existing reader permissions and symlink behavior. Invalid explicit names/traversal (`../x`, `a/b`, `a\b`, `.`, uppercase, blank, over 64 bytes) are Usage, with no filesystem lookup. Existing CLI traversal-shaped @ paths remain explicit paths, never catalog names.

### Admission and disclosure

Apply structural declarations to the typed effective selected item before wire serialization: one pointer checks its selected JSON value; several pointers check the existing synthesized object; with no pointer, check the whole typed record. Only readers that produce text validate a string. Preserve annotate’s whole-document JSON parsing: valid JSON remains its typed value, while syntactically invalid JSON remains text. Validate each member’s effective typed selection after the existing --field/on rules and before lookup or dispatch. An object record serialized as compact text on a legacy wire route still validates as an object. A selected number/boolean/null cannot pass a string declaration merely because the old route prints its spelling. CSV/TSV cells remain strings; `"12"` never becomes number. Existing empty-evidence and other function admission rules still apply after schema validation. `on` and caller `--field` precedence are unchanged; never read another field to satisfy required. Existing library loaders still refuse nonempty `on` where that route accepts only whole evidence; do not silently widen that contract.

`context_schema` checks only explicitly supplied per-item context, before packing/dedup/sort/split. Absence is allowed even with required object properties; those properties apply only when the context object is supplied. An explicit empty string is present and suppresses shared fallback as 0407 requires. Explicit null fails and never means missing. Shared context remains plain text under 0414 and is never checked against context_schema. If per-item context is absent, shared fallback retains its existing bytes and identity. Structured context travels separately from evidence, preserves property order and never changes spans/offsets. No implicit concatenation or JSON parsing/coercion.

For admitted image decide/choose/score, item_schema checks only explicit ancillary text/JSON item, never image bytes, filenames or an invented image record. Image-only input with item_schema fails as missing item; without item_schema it keeps existing image admission. Explicit context is checked normally. Images/media/order and route limits remain 0447/0448 responsibilities. Text-only functions still refuse images.

For already materialized finite calls/aggregate sets, validate all selected items and explicit contexts before store access or any send. For incremental files/stdin/streams, validate each admitted batch before lookup/dispatch; a bad item prevents its entire staged batch from sending. Do not pre-read unbounded streams or promise zero prior sends for a later bad record. Earlier dispatched work may complete; retain ordered completed prefixes and started-call facts, rank's withheld output, cancellation and existing stop/error codes. No new continuation mode; annotate's missing-pointer-only exception does not cover declaration failures. Validate every applicable member before sending an annotate record/set; no partially dispatched malformed record. Find/recognize/relate validate their complete materialized inputs before any stage sends.

Use the existing six error kinds: bad inline declaration/reference name or actual ask is Usage (CLI2); malformed/unreadable named or path file is Local (CLI5); wrong file verb remains Usage. Stable safe sentences include `the item does not match item_schema`, `the per-item context does not match context_schema`, `the question declaration uses an unsupported feature`, `the named question is unavailable`, and `the named question name does not match the file`. Give record position via existing stop diagnostics, not secret field names/values, file bodies, resolved paths or parser details. Preserve existing safe line/column JSON errors. Debug output and failures must not disclose input/context or credentials.

### Examples and behavior checks

```json
{"name":"refund","wording_version":2,"decide":"Does this ask for a refund?","on":"/message","item_schema":{"type":"object","properties":{"body":{"type":"string"},"ready":{"type":"boolean"}},"required":["body"]},"context_schema":{"type":"string"},"threshold":0.5}
```

```sh
thinkthen decide @refund --jsonl
thinkthen decide @./refund.json --jsonl
```

With item `{"message":{"body":"Please refund this.","ready":false},"private":"unsent"}`, only message is selected; false passes and private remains unsent. Missing body or ready:null refuses. Wording_version/name/declaration-only edits preserve cache/observation/answer identities for admitted identical work; wording edits change semantic keys; changed context misses as 0407 requires. Caller threshold/model overrides retain existing meaning. Two explicit files can carry two wording versions of refund without any scheduler.

| Boundary | Expected behavior |
| --- | --- |
| Old 0.1 single file, no additions | Loads identically; no metadata defaults |
| Existing bare refund path plus named refund.json | CLI bare path wins; explicit name API selects named file |
| Named folder missing; explicit name contains traversal | Local unavailable; Usage before I/O respectively |
| MCP inline @refund vs question_name refund | Literal wording vs explicit native named lookup |
| Single /body selected string, object declaration | Usage; never recover fields from parent |
| Annotate valid JSON document with no field pointer | Object declaration checks the parsed object; string declaration refuses before lookup/send |
| Annotate plain text document or explicit --lines | String declaration checks the text; object declaration refuses before lookup/send |
| Context absent / empty / null | Allowed fallback / explicit string / Usage |
| Empty declared string but function refuses empty evidence | Existing evidence refusal, zero send |
| Invalid finite row2 / invalid later streamed batch | Zero sends for whole finite call / no sends for bad batch, preserved prior work |
| Same admitted work after declaration edit in cache/replay | Validate first; reuse old observation without identity changes |
| Image-only with declaration / images plus matching ancillary item | Missing-item refusal / normal image admission |

Use the existing parser/schema corpus for the subset and metadata bounds, and public CLI/API edge tables with independent expected bodies and loopback counts for disclosure/admission. Reuse existing cache/record/replay, secrecy, cancellation, projection and partial-failure regressions. No new runner, receipt/fingerprint mechanism, paid calls or per-language proof papers.

## Delivery slices within this one ticket

1. Fresh High whole-ticket/design review; correct blocking findings before native implementation. Planning WIP does not need product gates or a landing record.
2. Native owner integrates pure grammar/types/resolved metadata and outside-core name loading, then admission with must-land 0407/0414. Update contracts, generated schemas/corpus and old-file compatibility together; no merge of active native WIP to manufacture readiness.
3. Existing C/MCP/family/SQL/frame owners adopt the settled additions after native interfaces are available; 0432 executes their actual named methods. Keep each existing owner's complete 0.2 scope and no duplicate context tickets. Full tests/lint on the eventual landing commit, policy before Rust review, affected spec/surface checks then.

## Native constituent implementation, 2026-10-06

Lane 0 implements the closed ordered declaration types, pure question-file/member/recognize/relate/find metadata parsing, explicit bounded native named/reference resolution and shared CLI @ resolution. Metadata travels beside semantic questions; complete atomic/member/aggregate serializers preserve it without changing semantic serialization or identity. Public getters expose declarations through the existing resolved readings and owned atomic/member observations. Typed atomic projections and all materialized complete annotation members validate before lookup/send; annotation documents retain parsed JSON while explicitly composed text remains text. Named loader and CLI checks use isolated ordinary config directories, real path collisions/escaping symlinks and counted loopback requests.

This is a concrete WIP constituent, not the whole accepted outcome. Structured explicit object context, declaration admission for every remaining convenience/whole-set/streaming path, atomic staged-batch refusal, canonical result/2 generated schema/CLI adoption and host consumer adoption remain open. Existing declaration grammar corpus/schema are updated; the result schema still belongs to the complete result/2 integration. No new dependency, catalog cache, scheduler, proof tool or landing record is introduced.

Source growth for this constituent is 1582 nonblank Rust lines (140842 → 142424): validated ordered declaration types/parser, reusable metadata accessors and semantic/presentation separation, confined named resolution shared with CLI, selected typed admission and outside-in regressions. Existing ordered JSON, capped reader, config-directory rules, projection and packing were reused; no parallel parser or execution engine was added. The reusable accessor macros avoid duplicating wrapper/builder behavior. Every Rust file remains below 500 nonblank lines.

### Added public declarations

```text
InputDeclaration::Object(ObjectDeclaration)
InputDeclaration::String
InputPropertyType::Boolean
InputPropertyType::Number
InputPropertyType::String
InputPropertyType::StringList
const fn ChooseBuilder::wording_version(&self) -> Option<WordingVersion>
const fn DecideBuilder::wording_version(&self) -> Option<WordingVersion>
const fn InputProperty::kind(&self) -> InputPropertyType
const fn LabelBuilder::wording_version(&self) -> Option<WordingVersion>
const fn Question::wording_version(&self) -> Option<WordingVersion>
const fn Recognize::wording_version(&self) -> Option<WordingVersion>
const fn RecordChooseQuestion::wording_version(&self) -> Option<WordingVersion>
const fn Relate::wording_version(&self) -> Option<WordingVersion>
const fn ScoreBuilder::wording_version(&self) -> Option<WordingVersion>
const fn TagBuilder::wording_version(&self) -> Option<WordingVersion>
const fn WordingVersion::get(self) -> u32
enum InputDeclaration
enum InputPropertyType
fn BandedQuestion::context_schema(&self) -> Option<&InputDeclaration>
fn BandedQuestion::item_schema(&self) -> Option<&InputDeclaration>
fn BandedQuestion::name(&self) -> Option<&QuestionName>
fn BandedQuestion::with_context_schema(self, InputDeclaration) -> Result<BandedQuestion, Error>
fn BandedQuestion::with_item_schema(self, InputDeclaration) -> Result<BandedQuestion, Error>
fn BandedQuestion::with_name(self, QuestionName) -> Result<BandedQuestion, Error>
fn BandedQuestion::with_wording_version(self, WordingVersion) -> Result<BandedQuestion, Error>
fn BandedQuestion::wording_version(&self) -> Option<WordingVersion>
fn ChooseBuilder::context_schema(&self) -> Option<&InputDeclaration>
fn ChooseBuilder::item_schema(&self) -> Option<&InputDeclaration>
fn ChooseBuilder::name(&self) -> Option<&QuestionName>
fn ChooseBuilder::with_context_schema(self, InputDeclaration) -> Result<ChooseBuilder<C>, Error>
fn ChooseBuilder::with_item_schema(self, InputDeclaration) -> Result<ChooseBuilder<C>, Error>
fn ChooseBuilder::with_name(self, QuestionName) -> Result<ChooseBuilder<C>, Error>
fn ChooseBuilder::with_wording_version(self, WordingVersion) -> Result<ChooseBuilder<C>, Error>
fn ChooseQuestion::context_schema(&self) -> Option<&InputDeclaration>
fn ChooseQuestion::item_schema(&self) -> Option<&InputDeclaration>
fn ChooseQuestion::name(&self) -> Option<&QuestionName>
fn ChooseQuestion::with_context_schema(self, InputDeclaration) -> Result<ChooseQuestion<C>, Error>
fn ChooseQuestion::with_item_schema(self, InputDeclaration) -> Result<ChooseQuestion<C>, Error>
fn ChooseQuestion::with_name(self, QuestionName) -> Result<ChooseQuestion<C>, Error>
fn ChooseQuestion::with_wording_version(self, WordingVersion) -> Result<ChooseQuestion<C>, Error>
fn ChooseQuestion::wording_version(&self) -> Option<WordingVersion>
fn DecideBuilder::context_schema(&self) -> Option<&InputDeclaration>
fn DecideBuilder::item_schema(&self) -> Option<&InputDeclaration>
fn DecideBuilder::name(&self) -> Option<&QuestionName>
fn DecideBuilder::with_context_schema(self, InputDeclaration) -> Result<DecideBuilder, Error>
fn DecideBuilder::with_item_schema(self, InputDeclaration) -> Result<DecideBuilder, Error>
fn DecideBuilder::with_name(self, QuestionName) -> Result<DecideBuilder, Error>
fn DecideBuilder::with_wording_version(self, WordingVersion) -> Result<DecideBuilder, Error>
fn FindReading::context_schema(&self) -> Option<&InputDeclaration>
fn FindReading::item_schema(&self) -> Option<&InputDeclaration>
fn FindReading::name(&self) -> Option<&QuestionName>
fn FindReading::wording_version(&self) -> Option<WordingVersion>
fn InputProperty::name(&self) -> &str
fn InputProperty::new(&str, InputPropertyType) -> Result<InputProperty, Error>
fn LabelBuilder::context_schema(&self) -> Option<&InputDeclaration>
fn LabelBuilder::item_schema(&self) -> Option<&InputDeclaration>
fn LabelBuilder::name(&self) -> Option<&QuestionName>
fn LabelBuilder::with_context_schema(self, InputDeclaration) -> Result<LabelBuilder, Error>
fn LabelBuilder::with_item_schema(self, InputDeclaration) -> Result<LabelBuilder, Error>
fn LabelBuilder::with_name(self, QuestionName) -> Result<LabelBuilder, Error>
fn LabelBuilder::with_wording_version(self, WordingVersion) -> Result<LabelBuilder, Error>
fn LoadedQuestion::context_schema(&self) -> Option<&InputDeclaration>
fn LoadedQuestion::item_schema(&self) -> Option<&InputDeclaration>
fn LoadedQuestion::name(&self) -> Option<&QuestionName>
fn LoadedQuestion::wording_version(&self) -> Option<WordingVersion>
fn ObjectDeclaration::new(Vec<InputProperty>, Vec<String>) -> Result<ObjectDeclaration, Error>
fn ObjectDeclaration::properties(&self) -> &[InputProperty]
fn ObjectDeclaration::required(&self) -> &[String]
fn Question::context_schema(&self) -> Option<&InputDeclaration>
fn Question::item_schema(&self) -> Option<&InputDeclaration>
fn Question::load_named(&str) -> Result<LoadedQuestion, Error>
fn Question::load_reference(&str) -> Result<LoadedQuestion, Error>
fn Question::name(&self) -> Option<&QuestionName>
fn Question::with_context_schema(self, InputDeclaration) -> Result<Question, Error>
fn Question::with_item_schema(self, InputDeclaration) -> Result<Question, Error>
fn Question::with_name(self, QuestionName) -> Result<Question, Error>
fn Question::with_wording_version(self, WordingVersion) -> Result<Question, Error>
fn QuestionName::as_str(&self) -> &str
fn QuestionName::new(&str) -> Result<QuestionName, Error>
fn QuestionSet::load_named(&str) -> Result<QuestionSet, Error>
fn QuestionSet::load_reference(&str) -> Result<QuestionSet, Error>
fn RecognitionReading::context_schema(&self) -> Option<&InputDeclaration>
fn RecognitionReading::item_schema(&self) -> Option<&InputDeclaration>
fn RecognitionReading::name(&self) -> Option<&QuestionName>
fn RecognitionReading::wording_version(&self) -> Option<WordingVersion>
fn Recognize::context_schema(&self) -> Option<&InputDeclaration>
fn Recognize::item_schema(&self) -> Option<&InputDeclaration>
fn Recognize::load_named(&str) -> Result<Recognize, Error>
fn Recognize::load_reference(&str) -> Result<Recognize, Error>
fn Recognize::name(&self) -> Option<&QuestionName>
fn Recognize::with_context_schema(self, InputDeclaration) -> Result<Recognize, Error>
fn Recognize::with_item_schema(self, InputDeclaration) -> Result<Recognize, Error>
fn Recognize::with_name(self, QuestionName) -> Result<Recognize, Error>
fn Recognize::with_wording_version(self, WordingVersion) -> Result<Recognize, Error>
fn RecognizeQuestionFile::load_named(&str) -> Result<RecognizeQuestionFile, Error>
fn RecognizeQuestionFile::load_reference(&str) -> Result<RecognizeQuestionFile, Error>
fn RecordChooseQuestion::context_schema(&self) -> Option<&InputDeclaration>
fn RecordChooseQuestion::item_schema(&self) -> Option<&InputDeclaration>
fn RecordChooseQuestion::load_named(&str) -> Result<RecordChooseQuestion, Error>
fn RecordChooseQuestion::load_reference(&str) -> Result<RecordChooseQuestion, Error>
fn RecordChooseQuestion::name(&self) -> Option<&QuestionName>
fn RecordChooseQuestion::with_context_schema(self, InputDeclaration) -> Result<RecordChooseQuestion, Error>
fn RecordChooseQuestion::with_item_schema(self, InputDeclaration) -> Result<RecordChooseQuestion, Error>
fn RecordChooseQuestion::with_name(self, QuestionName) -> Result<RecordChooseQuestion, Error>
fn RecordChooseQuestion::with_wording_version(self, WordingVersion) -> Result<RecordChooseQuestion, Error>
fn Relate::context_schema(&self) -> Option<&InputDeclaration>
fn Relate::item_schema(&self) -> Option<&InputDeclaration>
fn Relate::load_named(&str) -> Result<Relate, Error>
fn Relate::load_reference(&str) -> Result<Relate, Error>
fn Relate::name(&self) -> Option<&QuestionName>
fn Relate::with_context_schema(self, InputDeclaration) -> Result<Relate, Error>
fn Relate::with_item_schema(self, InputDeclaration) -> Result<Relate, Error>
fn Relate::with_name(self, QuestionName) -> Result<Relate, Error>
fn Relate::with_wording_version(self, WordingVersion) -> Result<Relate, Error>
fn RelationReading::context_schema(&self) -> Option<&InputDeclaration>
fn RelationReading::item_schema(&self) -> Option<&InputDeclaration>
fn RelationReading::name(&self) -> Option<&QuestionName>
fn RelationReading::wording_version(&self) -> Option<WordingVersion>
fn ResolvedQuestion::context_schema(&self) -> Option<&InputDeclaration>
fn ResolvedQuestion::item_schema(&self) -> Option<&InputDeclaration>
fn ResolvedQuestion::name(&self) -> Option<&QuestionName>
fn ResolvedQuestion::wording_version(&self) -> Option<WordingVersion>
fn ScoreBuilder::context_schema(&self) -> Option<&InputDeclaration>
fn ScoreBuilder::item_schema(&self) -> Option<&InputDeclaration>
fn ScoreBuilder::name(&self) -> Option<&QuestionName>
fn ScoreBuilder::with_context_schema(self, InputDeclaration) -> Result<ScoreBuilder, Error>
fn ScoreBuilder::with_item_schema(self, InputDeclaration) -> Result<ScoreBuilder, Error>
fn ScoreBuilder::with_name(self, QuestionName) -> Result<ScoreBuilder, Error>
fn ScoreBuilder::with_wording_version(self, WordingVersion) -> Result<ScoreBuilder, Error>
fn TagBuilder::context_schema(&self) -> Option<&InputDeclaration>
fn TagBuilder::item_schema(&self) -> Option<&InputDeclaration>
fn TagBuilder::name(&self) -> Option<&QuestionName>
fn TagBuilder::with_context_schema(self, InputDeclaration) -> Result<TagBuilder<C>, Error>
fn TagBuilder::with_item_schema(self, InputDeclaration) -> Result<TagBuilder<C>, Error>
fn TagBuilder::with_name(self, QuestionName) -> Result<TagBuilder<C>, Error>
fn TagBuilder::with_wording_version(self, WordingVersion) -> Result<TagBuilder<C>, Error>
fn TagQuestion::context_schema(&self) -> Option<&InputDeclaration>
fn TagQuestion::item_schema(&self) -> Option<&InputDeclaration>
fn TagQuestion::name(&self) -> Option<&QuestionName>
fn TagQuestion::with_context_schema(self, InputDeclaration) -> Result<TagQuestion<C>, Error>
fn TagQuestion::with_item_schema(self, InputDeclaration) -> Result<TagQuestion<C>, Error>
fn TagQuestion::with_name(self, QuestionName) -> Result<TagQuestion<C>, Error>
fn TagQuestion::with_wording_version(self, WordingVersion) -> Result<TagQuestion<C>, Error>
fn TagQuestion::wording_version(&self) -> Option<WordingVersion>
fn WordingVersion::new(u32) -> Result<WordingVersion, Error>
impl Serialize for InputDeclaration
impl Serialize for InputPropertyType
impl Serialize for ObjectDeclaration
impl Serialize for QuestionName
impl Serialize for WordingVersion
struct InputProperty
struct ObjectDeclaration
struct QuestionName
struct WordingVersion
struct ObjectContext
fn ObjectContext::new(&RawRecord) -> Result<ObjectContext, Error>
const fn ObjectContext::content(&self) -> QuestionContent<'_>
impl Serialize for ObjectContext
enum RecordContext
RecordContext::Text(String)
RecordContext::Object(ObjectContext)
impl From<String> for RecordContext
impl From<&str> for RecordContext
impl Serialize for RecordContext
RecordInput::context: Option<RecordContext>
fn RecordReading::with_context_schema(self, InputDeclaration) -> RecordReading
```

Focused constituent checks: 46 native complete cases passed before the declaration test was split by behavior; the final five declaration tests and both isolated native-name/CLI tests passed. The production question-file corpus and published schema self-test passed, as did the 74-row settings check, offline policy (268 resolved packages), format, affected Clippy and public inventory (1484 declarations; four existing plants refused). Root whole-code High review and full landing gates remain pending.

### Retired public declarations

```text
RecordInput::context: Option<String>
```

The unpublished native record-input draft now settles `context` as `Option<RecordContext>`, with concrete text and validated ordered-object variants. Existing released scalar/convenience functions and the old C ABI are unchanged. `ObjectContext::new(&RawRecord)` accepts an already parsed object; literal JSON-looking text never becomes an object. `RecordReading::with_context_schema` admits an explicitly selected object through the same ordered record/pointer reader; each question independently validates it before lookup/send. The original record, selected evidence and physical coordinates stay separate. Undeclared context remains text only. Annotation wraps the typed context around the existing state without changing evidence or offsets; text context bytes remain compatible. Cache/replay use actual typed state, with no declaration metadata in identity. Whole-set shared context remains text.

Three public regressions pin exact ordered object wire bodies, finite invalid-row zero sends, absence/fallback, null and JSON-looking text refusal, original preservation, annotation separation and zero-send replay of the accepted observation. Growth is 343 nonblank Rust lines (142592 → 142935), for the typed context carrier, reused native composition/packing integration and caller behavior checks. Streaming batch admission, remaining convenience paths, canonical result/2 schema/CLI and hosts are still open.

## Bounded streamed declaration admission constituent, 2026-10-06

Declared native atomic/annotation streams and CLI judgments/annotation now stage a bounded group of prepared inputs through the existing pure packer before looking any of them up. Record caps, state/body/profile/question limits and the existing input pause bound admission; split questions retain their whole logical input. A declaration/read refusal discards its uncommitted stage, while earlier admitted/dispatched work retains its ordered completed prefix and final facts. The existing send coordinator, global concurrency, configured batch sizes/defaults/precedence, retries, per-question keys and actual observed wire counts remain in use. Admission stages are not a new wire grouping policy. Native stop diagnostics retain an already known original bad-record position rather than replacing it with the first discarded row. Annotate's existing missing-pointer exception still flushes valid staged inputs and refuses only that input; declaration failures are terminal.

Both public streamed regressions failed on the previous coordinator, which sent valid rows from the malformed batch and could surface a backend error first. Four focused public cases now pin bad-second-row zero sends, a preserved two-row prefix with input887/output unknown, declaration refusal before an unstored replay question lookup, and all-member annotation admission. The CLI case pins exact error sentences, rows, rank withholding and independently expected wire bodies. Existing 68 batching and 70 annotation checks plus five native streaming checks pass. Focused Clippy and the 1496-declaration inventory pass. Whole review and full landing gates remain root-owned.

Source grows 411 nonblank Rust lines (142935 → 143346), for reusable bounded staging inside the existing coordinator, declaration admission on native/CLI callers and meaningful public/CLI regressions. No scheduler, cache namespace, test hook, proof tool or dependency is introduced. Materialized remaining convenience/aggregate admission, canonical result/2 schema/CLI, remaining complete rank-set/selection consumer needs and host adoption still remain open.

Lane1 reviewer correction on coherent 2922fd8a1: the coordinator retains whether
deadline admission abandoned staged inputs, so EOF cannot suppress their
terminal stop. Controlled public iterators pin deadline delivery with zero
sends and after two completed rows, retaining actual prefix counts, usage,
attempts and invocation facts. The original EOF case returned normal exhaustion
before this fix. Together with the 0300 pricing correction, measured source
grows 186 nonblank Rust lines (143695 → 143881), mainly public regressions using
the existing loopback fixtures and a controlled iterator. Root owns original
reviewer confirmation and full landing gates; remaining adoption stays open.

The same baseline comparison preserves the fourth existing `public_controls`
failure, `ineligible_calls_refuse_shared_context_before_a_send`: its first find
assertion expects Usage/refusal, while the coherent context path sends and the
saved noul response yields Backend. This consumer remains untouched; root and
the whole-set context owner retain its contract/oracle reconciliation. All four
new pricing/EOF regressions fail on original source and pass on corrected source.
Corrected-source checks: 63 native complete and 63 routine public batch cases
pass (two existing batch stress cases remain explicitly ignored), along with
two tally unit cases and three legacy public tally consumers. Focused library/
native-test Clippy, offline policy, formatting and the exact ratchet pass.
