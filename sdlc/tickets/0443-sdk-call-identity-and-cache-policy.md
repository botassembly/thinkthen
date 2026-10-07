# 0443: Carry SDK call identity and cache instructions

Status: in progress. Native implementation in lane0 on ticket/0443-native-complete-results; host adoption and final landing checks remain open.

Lane1 foundation correction against reviewed 30e42a974 plus main e316a9a7e:
CallFacts permanently retains reported-model disagreement and omits the scalar
on success and started failure. Public regressions use separate fixed/other/fixed
replies, mixed cache/live and replay origins, and a later 503 after disagreement;
actual observation models, configured request model, bare values and joined facts
remain intact. Root owns narrow reviewer confirmation and full landing gates;
this correction makes no whole native/C parity claim.
Focused checks: 78 native public cases, affected Clippy, offline policy and the
measured source ratchet pass; both model regressions failed before the fix.

Milestone: 0.2

Owner: builder.
Ticket review: ACCEPT, 2026-10-06; fresh read-only review. Conflicting image deferrals corrected where found.

## Outcome

Every request identifies its actual engine version and public surface; every call/request has a unique opaque ID. Results state actual retrieval origin/model and obey response cache instructions.

## Evidence

- Starts from: Current main c64b71859; 0405 audit is historical and predates 0377/0420. PM message `2026-10-06-pm-0-2-is-not-done-every-sdk-consistent-and-the-sdk-ready-for-the-proxy.md`, asks 9.
- Keeps: Preserve existing bare calls and generic JSON compatibility doors, backend selection from 0377, six error kinds, cancellation, secrecy, count-only usage, and zero-send strict replay. Reuse the Rust engine, C boundary and native file reader; add no host cache, scheduler or second parser.
- Changes: Implement reviewed 0442 headers/metadata in the existing engine edge. One call ID covers packed/multistep requests; retries retain request IDs, split children get new IDs. Propagate truthful surface labels through the C/native door. No-store forbids cache/record writes; refresh sends no-cache and bypasses old answers.
- Proof: Loopback packed, multistep, retry, split and concurrent calls pin IDs, labels and unchanged bodies. Unknown reply fields/headers remain tolerated; malformed known data fails. Check mixed cache/live sources, replay zero sends, actual answered model, no-store eviction and explicit-record refusal with started facts and no retry. IDs expose no caller text/key/address/model and do not enter cache keys or recording bodies.
- Defers: Proxy service/screens, unrelated features and Windows Node/C#/JVM packaging remain outside this outcome.

## Dependencies and ownership

0442 contract first; family tickets own public host metadata/settings adapters and typed decoding. 0444 owns key/store behavior; 0445 owns attempt observations. Preserve actual sends/retry accounting even if a future proxy deduplicates bills.

## Revised SDK boundary and identity

0449 resolves one endpoint/key/API-type before work. Direct model selectors stay opaque explicit parameters; no model group/fallback/automatic policy routing. Coordinate 0450 observation/answer identity via the existing ID facility. Transient call/request IDs remain absent from answer recordings; persistent observation identity is separate allowed record metadata under 0444. Reserved proxy fields never enter ordinary vendor bytes, and direct replies cannot attest them.

Native lane0 WIP: concrete typed result/2 carriers and serializers now cover
the six atomic readings, annotation successes/failures and whole-set find;
record carriers retain caller originals without extra trait bounds. Pure
serializer checks preserve explicit legacy result/1 projection, successful
null versus failure, numeric rank positions and zero-observation metadata.
These are constituent types, not completed public execution routes.
The actual landed image source is merged. Native started calls now carry
one opaque call ID through success/failure facts; prepared sends carry distinct
SDK request IDs and compiled version/surface headers. Status retries retain
those IDs, split children get new request IDs, and CLI final facts use the
same call ID. Seven public/CLI loopback cases independently pin bodies,
headers, concurrency, retries, splits, empty aggregates and zero-send refusal.
Legacy serialized facts remain unchanged. Canonical facts serialization,
persistent observations, cache policy and complete execution routes remain open.

Native storage-policy WIP: bounded Cache-Control parsing returns only storage
permission; repeated fields, exact no-store names, quoted values, malformed
syntax and each parser bound are covered. The existing send/transaction spine
returns nonstorable cached calls without writing, rejects explicit recording
locally after the successful send, and evicts working answers transactionally
only on valid nonstorable refresh. Native EngineBuilder.refresh_cache(bool)
uses the existing freshness mode; retries and split children send no-cache.
Eight outside-in public/CLI policy cases include prior-failing regressions.
A partial nonstorable refresh evicts only accepted questions; failed-question
history remains held for a later invocation.
V2 validated migration, provenance and complete runtime results remain open.

The affected settings check also exposed missing image/media inventory cells
in the merged image source. The existing admitted flags now have their row;
this does not add a route or change image admission.

The transport accepts the explicit closed mcp surface for approved0455 and
uses the same compiled version/call/request headers. MCP module/CLI ownership
remains lane1; no alternate transport or engine is introduced.

Local image integration WIP: ready 92084c66d is merged with shared profile propagation. The estimated-budget blanket refusal is omitted under Ian's correction: uncalibrated local images retain approximate encoded-body admission. The original Imajev decoder accepts independently optional reported counts, carries input887/output unknown through per-question storage and zero-send replay, and retains original request/response bodies in the explicit recording transaction. All 23 focused image regressions pass. Native result execution, migration, CLI schema adoption and the whole High review remain open.

Native complete execution WIP: additive decide/choose/tag/score scalar and input routes, all six atomic many/record routes, and stable saved-score/described-decide ranking now return concrete result/2 carriers through the existing pipeline. Eager record admission validates every original, context and whole replacement shortlist before sending. Originals require neither Clone, Serialize nor Send. Logical IDs include effective normalized readings and original ordinals; cache/replay retain accepted observations and requested empty attempt lists. Six independent public exchange regressions pass, alongside 23 image, nine storage and eight complete serializer cases; affected Clippy passes. Aggregate execution, shared context integration, dynamic choose without fixed options, CLI/schema/corpus adoption and final public inventory remain open.

### Added public declarations

```text
struct SourceRecognition
const fn SourceRecognition::location(&self) -> &SourceLocation
fn SourceRecognition::entities(&self) -> &[SourceRecognizedEntity]
fn SourceRecognition::relations(&self) -> Option<&[SourceRecognizedRelation]>
impl Serialize for SourceRecognition
struct SourceRecognizedEntity
const fn SourceRecognizedEntity::entity(&self) -> &RecognizedEntity
const fn SourceRecognizedEntity::location(&self) -> &SourceLocation
impl Serialize for SourceRecognizedEntity
struct SourceRecognizedRelation
fn SourceRecognizedRelation::relation(&self) -> &str
const fn SourceRecognizedRelation::source(&self) -> &SourceRecognizedEntity
const fn SourceRecognizedRelation::target(&self) -> &SourceRecognizedEntity
const fn SourceRecognizedRelation::probability(&self) -> f64
const fn SourceRecognizedRelation::either(&self) -> bool
impl Serialize for SourceRecognizedRelation
const fn CompleteRecognized::source_value(&self) -> Option<&SourceRecognition>
struct SourceRelationEndpoint
fn SourceRelationEndpoint::ordinal(&self) -> usize
fn SourceRelationEndpoint::entity(&self) -> &Entity
fn SourceRelationEndpoint::record(&self) -> &RawRecord
fn SourceRelationEndpoint::location(&self) -> &SourceLocation
impl Serialize for SourceRelationEndpoint
struct SourceRelationEdge
const fn SourceRelationEdge::edge(&self) -> &Edge
const fn SourceRelationEdge::source(&self) -> &SourceRelationEndpoint
const fn SourceRelationEdge::target(&self) -> &SourceRelationEndpoint
impl Serialize for SourceRelationEdge
fn CompleteRelated::source_edges(&self) -> Option<&[SourceRelationEdge]>
const fn Facts::held_model_mismatch(&self) -> bool
const fn complete_call_schema() -> &'static str
fn Engine::relate_records_complete_with<I, T>(&self, &Relate, I, CallOptions<'_>) -> Result<Call<CompleteRecord<Vec<T>, CompleteRelated>>, Error> where I: IntoIterator<Item = RecordInput<T>>, T: InputEvidence
fn Engine::try_relate_records_complete_with<I, T>(&self, &Relate, I, CallOptions<'_>) -> Result<Call<CompleteRecord<Vec<T>, CompleteRelated>>, Error> where I: IntoIterator<Item = Result<RecordInput<T>, Error>>, T: InputEvidence
fn Relate::from_records_json(&str) -> Result<Relate, Error>
fn Relate::load_records(impl AsRef<Path>) -> Result<Relate, Error>
fn Relate::record_fields(self, &str, &str) -> Result<Relate, Error>
fn Engine::recognize_records_complete_with<I, T>(&self, &Recognize, I, CallOptions<'_>) -> Result<Call<Vec<CompleteRecord<T, CompleteRecognized>>>, Error> where I: IntoIterator<Item = RecordInput<T>>, T: InputEvidence
fn Engine::try_recognize_records_complete_with<I, T>(&self, &Recognize, I, CallOptions<'_>) -> Result<Call<Vec<CompleteRecord<T, CompleteRecognized>>>, Error> where I: IntoIterator<Item = Result<RecordInput<T>, Error>>, T: InputEvidence
fn Engine::try_find_complete_with<I, T>(&self, &Question, I, CallOptions<'_>) -> Result<Call<CompleteFound<T>>, Error> where I: IntoIterator<Item = Result<T, Error>>, T: Evidence
fn Engine::find_records_complete_with<I, T>(&self, &Question, I, CallOptions<'_>) -> Result<Call<CompleteFound<T>>, Error> where I: IntoIterator<Item = RecordInput<T>>, T: InputEvidence
fn Engine::try_find_records_complete_with<I, T>(&self, &Question, I, CallOptions<'_>) -> Result<Call<CompleteFound<T>>, Error> where I: IntoIterator<Item = Result<RecordInput<T>, Error>>, T: InputEvidence
fn Engine::try_rank_records_complete_with<I, T>(&self, &Question, I, CallOptions<'_>) -> Result<Call<Vec<CompleteRecord<T, CompleteRank>>>, Error> where I: IntoIterator<Item = Result<RecordInput<T>, Error>>, T: InputEvidence
fn Batch::into_call(self) -> Result<Call<Vec<T>>, Error>
fn Engine::try_decide_records_complete_with<'a, Q, I, T>(&'a self, &'a Q, I, CallOptions<'a>) -> Batch<'a, CompleteRecord<T, CompleteDecision>> where Q: DecisionQuestion + ?Sized, I: IntoIterator<Item = Result<RecordInput<T>, Error>> + 'a, T: InputEvidence + 'a
fn Engine::try_choose_records_complete_with<'a, Q, I, T>(&'a self, &'a Q, I, CallOptions<'a>) -> Batch<'a, CompleteRecord<T, CompleteChoice>> where Q: DetailQuestion + ?Sized, I: IntoIterator<Item = Result<RecordInput<T>, Error>> + 'a, T: InputEvidence + 'a
fn Engine::try_tag_records_complete_with<'a, Q, I, T>(&'a self, &'a Q, I, CallOptions<'a>) -> Batch<'a, CompleteRecord<T, CompleteTags>> where Q: DetailQuestion + ?Sized, I: IntoIterator<Item = Result<RecordInput<T>, Error>> + 'a, T: InputEvidence + 'a
fn Engine::try_score_records_complete_with<'a, I, T>(&'a self, &'a Question, I, CallOptions<'a>) -> Batch<'a, CompleteRecord<T, CompleteScore>> where I: IntoIterator<Item = Result<RecordInput<T>, Error>> + 'a, T: InputEvidence + 'a
fn Engine::try_filter_records_complete_with<'a, I, T>(&'a self, &'a Question, I, CallOptions<'a>) -> Batch<'a, CompleteRecord<T, CompleteFilter>> where I: IntoIterator<Item = Result<RecordInput<T>, Error>> + 'a, T: InputEvidence + 'a
fn Engine::try_annotate_records_complete_with<'a, I, T>(&'a self, &'a QuestionSet, I, CallOptions<'a>) -> Batch<'a, CompleteRecord<T, CompleteAnnotated>> where I: IntoIterator<Item = Result<RecordInput<T>, Error>> + 'a, T: InputEvidence + 'a
struct SurfaceError
impl Display for SurfaceError
impl Error for SurfaceError
enum Surface
Surface::Cli
Surface::Rust
Surface::C
Surface::Python
Surface::Pandas
Surface::PythonPolars
Surface::RustPolars
Surface::Javascript
Surface::Ruby
Surface::R
Surface::Cpp
Surface::Go
Surface::Csharp
Surface::Java
Surface::Kotlin
Surface::Scala
Surface::Swift
Surface::Zig
Surface::Php
Surface::Dart
Surface::ObjectiveC
Surface::Ada
Surface::Cobol
Surface::Flutter
Surface::Duckdb
Surface::Sqlite
Surface::Postgresql
Surface::Mcp
const fn Surface::as_str(self) -> &'static str
impl Hash for Surface
impl Display for Surface
impl Serialize for Surface
impl Deserialize<'de> for Surface
impl FromStr for Surface
type Surface::Err = SurfaceError
fn Engine::annotate_complete_with<I, T>(&self, &QuestionSet, I, CallOptions<'_>) -> Result<Call<Vec<CompleteRecord<T, CompleteAnnotated>>>, Error> where I: IntoIterator<Item = T>, T: InputEvidence
fn Engine::annotate_records_complete_with<I, T>(&self, &QuestionSet, I, CallOptions<'_>) -> Result<Call<Vec<CompleteRecord<T, CompleteAnnotated>>>, Error> where I: IntoIterator<Item = RecordInput<T>>, T: InputEvidence
fn Engine::choose_complete_with<Q: DetailQuestion + ?Sized>(&self, &Q, &str, CallOptions<'_>) -> Result<Call<CompleteChoice>, Error>
fn Engine::choose_input_complete_with<Q: DetailQuestion + ?Sized>(&self, &Q, &QuestionInput, CallOptions<'_>) -> Result<Call<CompleteChoice>, Error>
fn Engine::choose_many_complete_with<Q, I, T>(&self, &Q, I, CallOptions<'_>) -> Result<Call<Vec<CompleteRecord<T, CompleteChoice>>>, Error> where Q: DetailQuestion + ?Sized, I: IntoIterator<Item = T>, T: InputEvidence
fn Engine::choose_records_complete_with<Q, I, T>(&self, &Q, I, CallOptions<'_>) -> Result<Call<Vec<CompleteRecord<T, CompleteChoice>>>, Error> where Q: DetailQuestion + ?Sized, I: IntoIterator<Item = RecordInput<T>>, T: InputEvidence
fn Engine::decide_complete_with<Q: DecisionQuestion + ?Sized>(&self, &Q, &str, CallOptions<'_>) -> Result<Call<CompleteDecision>, Error>
fn Engine::decide_input_complete_with<Q: DecisionQuestion + ?Sized>(&self, &Q, &QuestionInput, CallOptions<'_>) -> Result<Call<CompleteDecision>, Error>
fn Engine::decide_many_complete_with<Q, I, T>(&self, &Q, I, CallOptions<'_>) -> Result<Call<Vec<CompleteRecord<T, CompleteDecision>>>, Error> where Q: DecisionQuestion + ?Sized, I: IntoIterator<Item = T>, T: InputEvidence
fn Engine::decide_records_complete_with<Q, I, T>(&self, &Q, I, CallOptions<'_>) -> Result<Call<Vec<CompleteRecord<T, CompleteDecision>>>, Error> where Q: DecisionQuestion + ?Sized, I: IntoIterator<Item = RecordInput<T>>, T: InputEvidence
fn Engine::filter_complete_with<I, T>(&self, &Question, I, CallOptions<'_>) -> Result<Call<Vec<CompleteRecord<T, CompleteFilter>>>, Error> where I: IntoIterator<Item = T>, T: InputEvidence
fn Engine::filter_records_complete_with<I, T>(&self, &Question, I, CallOptions<'_>) -> Result<Call<Vec<CompleteRecord<T, CompleteFilter>>>, Error> where I: IntoIterator<Item = RecordInput<T>>, T: InputEvidence
fn Engine::find_complete_with<I>(&self, &Question, I, CallOptions<'_>) -> Result<Call<CompleteFound<I::Item>>, Error> where I: IntoIterator, I::Item: Evidence
fn Engine::rank_complete_with<I, T>(&self, &Question, I, CallOptions<'_>) -> Result<Call<Vec<CompleteRecord<T, CompleteRank>>>, Error> where I: IntoIterator<Item = T>, T: InputEvidence
fn Engine::rank_records_complete_with<I, T>(&self, &Question, I, CallOptions<'_>) -> Result<Call<Vec<CompleteRecord<T, CompleteRank>>>, Error> where I: IntoIterator<Item = RecordInput<T>>, T: InputEvidence
fn Engine::recognize_complete_with(&self, &Recognize, &str, CallOptions<'_>) -> Result<Call<CompleteRecognized>, Error>
fn Engine::relate_complete_with<I>(&self, &Relate, I, CallOptions<'_>) -> Result<Call<CompleteRelated>, Error> where I: IntoIterator<Item = Entity>
fn Engine::score_complete_with(&self, &Question, &str, CallOptions<'_>) -> Result<Call<CompleteScore>, Error>
fn Engine::score_input_complete_with(&self, &Question, &QuestionInput, CallOptions<'_>) -> Result<Call<CompleteScore>, Error>
fn Engine::score_many_complete_with<I, T>(&self, &Question, I, CallOptions<'_>) -> Result<Call<Vec<CompleteRecord<T, CompleteScore>>>, Error> where I: IntoIterator<Item = T>, T: InputEvidence
fn Engine::score_records_complete_with<I, T>(&self, &Question, I, CallOptions<'_>) -> Result<Call<Vec<CompleteRecord<T, CompleteScore>>>, Error> where I: IntoIterator<Item = RecordInput<T>>, T: InputEvidence
fn Engine::tag_complete_with<Q: DetailQuestion + ?Sized>(&self, &Q, &str, CallOptions<'_>) -> Result<Call<CompleteTags>, Error>
fn Engine::tag_many_complete_with<Q, I, T>(&self, &Q, I, CallOptions<'_>) -> Result<Call<Vec<CompleteRecord<T, CompleteTags>>>, Error> where Q: DetailQuestion + ?Sized, I: IntoIterator<Item = T>, T: InputEvidence
fn Engine::tag_records_complete_with<Q, I, T>(&self, &Q, I, CallOptions<'_>) -> Result<Call<Vec<CompleteRecord<T, CompleteTags>>>, Error> where Q: DetailQuestion + ?Sized, I: IntoIterator<Item = RecordInput<T>>, T: InputEvidence
struct CompleteCall<'a, T>
impl Serialize for CompleteCall
fn Call::complete(&self) -> Option<CompleteCall<'_, T>>
const fn CompleteCall::value(&self) -> &T
const fn CompleteCall::facts(&self) -> &CompleteFacts<'_>
struct CompleteError<'a>
impl Serialize for CompleteError
fn Error::complete(&self) -> CompleteError<'_>
const fn CompleteError::error(&self) -> &ErrorSnapshot<'_>
const fn CompleteError::facts(&self) -> Option<&CompleteFacts<'_>>
struct ErrorSnapshot<'a>
impl Serialize for ErrorSnapshot
const fn ErrorSnapshot::kind(&self) -> ErrorKind
const fn ErrorSnapshot::message(&self) -> &str
const fn ErrorSnapshot::stopped(&self) -> Stopped
struct Stopped
impl Serialize for Stopped
const fn Error::stopped(&self) -> Stopped
const fn Stopped::at(&self) -> Option<usize>
const fn Stopped::cause(&self) -> StopCause
const fn Stopped::status(&self) -> Option<u16>
const fn Stopped::retryable(&self) -> bool
enum StopCause
StopCause::Usage
StopCause::Local
StopCause::NoKey
StopCause::Transport
StopCause::Status
StopCause::TooLarge
StopCause::Reply
StopCause::Backend
StopCause::Cancelled
StopCause::Deadline
StopCause::Defect
impl Serialize for StopCause
impl Serialize for SendBudgetDenial
impl Serialize for EstimatedInputDenial
impl Serialize for CompleteRecord
struct ResultMetadata<'a>
const fn ResultMetadata::tool(&self) -> &str
const fn ResultMetadata::question_sha256(&self) -> Option<&str>
const fn ResultMetadata::questions_sha256(&self) -> Option<&str>
const fn ResultMetadata::url(&self) -> &str
const fn ResultMetadata::model(&self) -> &str
const fn ResultMetadata::usage(&self) -> Option<ReportedUsage>
const fn ResultMetadata::requests_sent(&self) -> u64
const fn ResultMetadata::requests(&self) -> &[String]
const fn ResultMetadata::failed_questions(&self) -> usize
const fn ResultMetadata::identity(&self) -> &ResultIdentity
const fn ResultMetadata::context_sha256(&self) -> Option<&str>
const fn ResultMetadata::attempts(&self) -> Option<&[AttemptObservation]>
fn ResultMetadata::cached(&self) -> bool
fn ResultMetadata::profile_warning(&self) -> Option<ProfileMismatch<'_>>
fn ResultMetadata::batch_setting(&self) -> Option<BatchSetting>
fn ResultMetadata::batch_warning(&self) -> Option<BatchMismatch<'_>>
struct ProfileMismatch<'a>
fn ProfileMismatch::tuned_for(&self) -> &str
fn ProfileMismatch::running(&self) -> &str
struct BatchMismatch<'a>
fn BatchMismatch::tuned_for(&self) -> Option<BatchSetting>
fn BatchMismatch::running(&self) -> Option<BatchSetting>
fn EngineBuilder::refresh_cache(self, bool) -> EngineBuilder
const fn CallOptions::surface(self, Surface) -> CallOptions<'a>
const fn CallOptions::proxy(self, &'a ProxyActivation) -> CallOptions<'a>
```

Native aggregate execution WIP: find, annotate, recognize and relate now return concrete result/2 carriers. Ordered trace data is retained by the facade, including actual sources and accepted/failure observations; consumers do not reconstruct it from event JSON. Aggregate context wraps the existing state value as a typed context/evidence envelope on all actual requests and recognition admission probes. Existing convenience calls accept context through the same implementation. Native annotation retains original occurrences and partial members, and complete relate retains full successful distributions. Four additional outside-in public cases check ordered candidates, successful null/failure distinctions, every recognition stage and unchanged spans, partial usage and empty metadata. Typed consumer accessors, owned event identity/location, input composition, command completion and CLI/schema/corpus adoption remain open.

Native consumer metadata WIP: all ten concrete complete routes expose a borrowed
ResultMetadata view with actual provenance/identity, partial usage, logical keys,
packing/profile warnings, context digest and requested attempts. Empty aggregates
retain zero sends, absent reported model and captured empty attempt lists. C/MCP
still need normalized question/readings, owned observations and shared record
composition; CLI/schema adoption and the whole High review remain open.

Native complete-call WIP: additive typed Call/Error projections now serialize
actual complete facts and structured stops without changing legacy Facts/door
serialization. The existing pipeline supplies original stopping positions;
pre-start refusals omit facts, started deadline failures retain zero-send facts,
and status failures retain joined attempts. Record and find serialization retain
complete caller originals and order; execution still imposes no Serialize bound.
Four outside-in saved-exchange cases and all 17 native complete cases pass, plus
eight complete serializer cases and affected Clippy. Owned observer/input
composition, remaining native source execution and CLI/schema adoption remain
open; C/MCP adapters and whole review/landing are not claimed complete.

Native shared composition/owned observer WIP: six new public regressions cover
selected JSON versus complete originals, source coordinates, ordered duplicate
images, exact wire context/candidates, zero-send strict replay, immutable snapshots
beyond engine destruction and actual partial annotation failure identities. The
existing parser, reader, image admission and ordered pipeline perform execution;
no adapter parser or scheduler is added. The measured Rust ratchet grows from
136709 to 137956 (+1247): shared composition and owned typed event leaves,
receipt propagation through existing observer paths, and those behavioral tests.
Original immutable record/input snapshots are shared to avoid one original copy
per annotation member. Existing duplicate option decoding is removed. Source
caps remain 500 nonblank lines. Full source/aggregate execution, dynamic choose,
CLI/schema/corpus adoption and remaining public inventory are still in progress.

Constituent WIP: saved find and mandatory per-record choose preparation share native ordered grammar, field selection, saved model/profile and existing batch precedence. Focused public and CLI tests cover actual independent wire bodies, original preservation, empty calls and zero-send later/file refusal. Source ratchet grows by 636 nonblank lines (137956 to 138592): concrete preparation/parser leaves, public carriers and outside-in regression cases. Whole result/2 CLI/schema, located execution and final host adoption remain open; no landing/review claim.

Constituent WIP: complete fallible record routes for decide/choose/tag/score/filter/annotate reuse the existing native pull bridge and joined pipeline. Preparation snapshots selected evidence, controls and location once while originals stay on their caller thread without Clone/Serialize/Send requirements. Batch::into_call retains actual final facts. Per-record contexts are admitted through the existing packer with the caller/profile byte caps before that row sends. Streaming retains completed-prefix behavior; eager sources continue to admit the whole set before sends. Located aggregate execution and CLI result/2 adoption remain open.

This streaming constituent raises the exact measured source ratchet by 724 nonblank lines (138592 to 139316): generic caller-thread preparation in the existing pull bridge, concrete complete stream entry points, shared annotation preparation/rendering, and five outside-in regressions. The old bridge behavior remains covered by all 63 non-stress public batch cases. Signature inventory is reconciled against intended declarations in the owning tickets; no alternate dispatch, records or proof tool was introduced.

Whole-set native WIP now consumes fallible composed find candidates directly, retains every original and source location through cache/replay, and admits fallible rank originals before any send. Find uses call-wide context and refuses per-record context/options instead of dropping them. FindQuestionFile keeps the saved evidence pointers with the question using the ordinary parser/reader, so consumers need no second question parser. Located recognize/relate integration and CLI result/2 remain open.

This whole-set constituent adds 413 measured nonblank lines (139316 to 139729): concrete saved find question/reading preparation, composed candidate preservation, fallible find/rank admission and three outside-in source/replay cases. All 35 native complete cases and affected Clippy pass; signature inventory and offline policy run before push.

Located recognition WIP uses one native call/budget across eagerly admitted composed records and the existing stage scheduler. Each original occurrence retains its source coordinates, full probabilities, actual partial usage and per-row attempts; owned primitive observations carry the source occurrence and actual stage identity. A later started failure keeps completed-prefix facts and the failed source position. Native relate composition, CLI result/2/schema and host adoption remain open.

The recognition constituent adds 404 measured nonblank lines (139729 to 140133): composed record execution using the existing first-stage admission/stage scheduler, original-aware serialization, extracted shared stage observations, and three outside-in cases. Existing scalar recognition rendering and observer construction are shared rather than duplicated. Files retain the 500-line cap.

Native whole-set relate record execution retains all arbitrary originals and locations, selects saved/custom field pointers through the ordinary native record reader, and uses the existing pair/menu scheduler. The full aggregate `CompleteRecord<Vec<T>, CompleteRelated>` serializes its actual input set only when T implements Serialize. Literal source text remains literal with the ordinary wildcard-kind boundary; mixed input modes, duplicate entity identities and per-record controls refuse before sending. Located find and relate observers expose their ordered source inputs without a host JSON parser. CLI result/2/schema and C/MCP/family adoption remain open.

The relation/source constituent adds 515 measured nonblank lines (140133 to 140648): shared native entity projection, complete whole-set originals, typed whole-set observer input iteration and two outside-in relation cases plus find observation coverage. The CLI delegates its recognized-name fallback to the same pure projection. Existing find and relate execution/rendering are reused, with no source reread or extra scheduler. All 40 focused native cases and 47 affected backend/CLI relation cases pass; affected Clippy and the 1323-declaration inventory pass.

Saved recognition preparation now retains `on` pointers with the typed question through `RecognizeQuestionFile`, reusing the ordinary bounded loader, grammar and RecordReading. Aggregate complete recognition/relate metadata carries the existing saved-versus-running profile warning. Request/builder/record-reading Debug implementations withhold authored kinds, relations, pointers and models; the public trait inventory remains unchanged. Two new outside-in saved-reading/secrecy cases and updated relate profile/replay coverage pass.

The preparation/diagnostic constituent adds 194 measured nonblank lines (140648 to 140842): typed saved recognition composition, shared aggregate profile warning propagation, withheld diagnostic leaves and those regressions. Existing readers/parsers and the common aggregate metadata builder were checked and reused. CLI result/2/schema/corpus, timing/cache warning integration, whole review and final host adoption remain open.


Native finite admission cancellation, 2026-10-06: composed complete record/aggregate paths now check the shared cancellation token before pulling a source item and between typed preparation steps. Relative deadlines are fixed before admission and reused during execution. Reserved proxy activation refuses before input preparation. The existing interrupt-check callback remains confined to its existing execution/poll contract; no admission callback or scheduler is added. Default legacy finite collection reuses the same checked engine cap helper.

The public prior-failing case previously prepared all three inputs after the first snapshot cancelled the token; it now stops after one, and a pre-fired token snapshots none, across atomic, annotation, single/set rank, find, recognize and relate routes. A source-pull case confirms no next read or send after cancellation during input. Admission cancellation carries no invented started facts. Focused checks pass: 67 native complete cases, 63 public batching cases (two existing explicit ignored cases), affected Clippy and format. Source grows 162 nonblank Rust lines (144688 → 144850), for shared bounded caller admission and two outside-in regressions; the old cap loop/check was removed after checking collection helpers for duplication. Whole review/full landing gates and CLI/schema/host adoption remain open.

Canonical schema constituent WIP: the production result/2 and complete-facts serializers now use concrete documents whose test-only schema derives generate both the shared result definitions and a packaged complete-call schema. `complete_call_schema()` returns the strict native success/error contract for MCP and other complete consumers. It includes all ten results, numeric rank, resolved author declarations, partial counts, complete attempt IDs and the closed `mcp` surface token. Released result/1 and generic-door definitions remain available; the CLI still needs result/2 adoption. No host owns a second known-field parser/schema.

The existing independent public exchange tests now validate actual complete calls against that packaged schema, including started/admission errors, empty aggregates, successful nulls, member failures and the original input payloads. The shared type corpus adds only schema behavior cases; those cases do not claim consumer execution or host parity. Full landing gates, fresh whole High review, CLI/timing adoption and C/MCP/family execution remain open.

This constituent passes 67 focused native complete cases, eight concrete serializer cases, 92 shared schema cases and the existing actual C generic-door checks under isolated XDG folders, affected Clippy, policy, format and the 1,525-declaration inventory. The initial C-door check inherited local configuration and refused construction; rerunning with explicit isolated test configuration/state/cache folders passed without native behavior changes. Source grows 618 nonblank Rust lines (145003 → 145621): concrete serialization/schema documents, declaration wire forms, strict complete schema derivation alongside compatibility definitions, and validation of actual existing public exchanges. Duplicate manual annotation metadata and result serialization maps were removed; shared metadata and existing schema generation are reused. This is a pushed constituent, not a mainlanding or full parity claim.

CLI constituent WIP: decide/choose/tag/score/filter details use the concrete native atomic document, retaining admitted declarations, actual sources/observations and complete send IDs. Original occurrence scope comes from the existing pipeline label; bare output and released public Details stay compatible. Rank and aggregate CLI adoption, their final examples and command timing remain open. The existing 63-case public/CLI comparison now explicitly checks retained judgment/meta semantics and both required versions; independent live answers do not claim equal observation IDs. Two new compiled-command cases validate strict complete schemas, exact independently expected request bodies, partial counts, false/ties/empty tags and filtered duplicate occurrences whose zero-send replay preserves original batch counts across a batch-size change.

The strict schema also enforces unsigned integer maxima rather than relying on JSON Schema's unenforced Rust format labels. The prior-failing shared-corpus case accepted batch_size=4294967296; it now refuses. Tag's stale whole-request cache-reasking sentence is reconciled with the retained per-question cache contract. Source grows 313 nonblank Rust lines (145621 → 145934), for typed CLI adoption and meaningful command/schema compatibility cases. The CLI delegates to the existing native serializer; no reader, parser, scheduler or identity generator is duplicated. Fresh whole review/full landing gates and remaining host adoption stay open.

Annotation CLI constituent WIP: details now delegate to the same concrete complete renderer as native annotation. Authored declarations, ordered original JSON members, successful false values, actual member failure identities, partial usage and observed batch counts survive recording and zero-send replay. Logical occurrence ordinals are separate from physical source line numbers for atomic and annotation answer identity; actual coordinates remain presentation fields in the derived strict schema. Released bare/scalar and record-error carriers remain unchanged.

Source grows 297 nonblank Rust lines (146231 → 146528), for the shared renderer, typed source fields/schema propagation and independent command compatibility/replay/failure tests. The public renderer no longer duplicates annotation assembly. Affected test/schema helpers were factored for their existing lint bounds; an unused fixture lint expectation was removed. Remaining aggregate/rank CLI adoption, timing sidecars, held-model warning and host execution adoption stay open; this is not a whole native completion or mainlanding claim.

Find CLI constituent WIP: the dedicated complete renderer is shared with native whole-set selection, preserving authored declarations, synthetic-none selection with the actual raw pick, complete distributions, partial reported usage, accepted observation/batch metadata and attempt send IDs. Command detail capture retains the existing CLI invocation rather than allocating a replacement call ID. A saved declared JSONL find exchange checks an independently expected aggregate body and counted zero-send replay with identical answer identity. Existing bare selection, cancellation/output and secrecy cases pass. The old literal-model cache fixture now requests its reported model; a damaged-fixture assertion names the safe strengthened constituent validation refusal. No production freshness or secrecy rule is relaxed.

Source grows 133 nonblank Rust lines (146528 → 146661), for shared find assembly, invocation-preserving command attempt capture and the actual command replay regression; duplicate native/CLI metadata assembly was removed. Nine affected find/secrecy cases, four native aggregate cases, the new command case, all 94 existing shared schema cases and affected Clippy pass. Other aggregate/rank CLI adoption, timing sidecars, held-model warning and host integration remain open.

Recognition CLI constituent WIP: complete details delegate to the same shared recognition/aggregate metadata renderer as native execution. The existing stage scheduler and one CLI invocation supply actual sources, complete probabilities, independently reported counts and per-row attempts. Physical recognized spans and relation endpoints now serialize from typed native names through the shared Unicode mapper, without parsing the command's own known result JSON or inventing unknown source coordinates. The strict complete schema admits these actual presentation fields while retaining legacy definitions. The released generic recognition-detail type remains schema-only compatibility data.

An actual source/cache/replay exchange reuses the saved stage response fixture and pins duplicate occurrences, physical lines 1/3, scalar offsets, original per-observation counts 4/3 and unchanged logical answer IDs when replayed from unlocated lines; replay sends nothing. Existing detailed probabilities and question/request fields retain their pinned legacy fixture semantics. Source grows 301 nonblank Rust lines (146661 → 146962), for reusable aggregate/recognition assembly, typed source presentation and the public/CLI regressions; duplicated aggregate assembly and the JSON source patcher were removed. Remaining relation/rank CLI adoption, timing sidecars, held-model warning and host integration remain open.

Recognition constituent validation: the generated schema check, 28 affected recognition/secrecy command cases, eight source-reader cases, five focused native recognition cases, all 94 existing shared schema cases, affected Clippy, format and offline policy pass. Whole review and full landing gates remain root-owned.

Relation CLI constituent WIP: details use the same complete member renderer as native relation execution. Successful members expose full typed answers and stable IDs; failed members retain actual failure identities. Located duplicate occurrences expand accepted edges through typed original records and actual coordinates without parsing the known result JSON. Empty menus retain truthful zero-observation metadata. Independent partial-usage, located record/replay and empty-set command cases pass alongside existing relation compatibility tests. Rank CLI adoption, timing sidecars, held-model warnings and host adoption remain open.

Source grows 130 nonblank Rust lines (146962 → 147092) for the shared relation renderer, typed source presentation/schema and outside-in tests. Duplicate native/CLI relation assembly and the now-unused trace wrapper are removed. Released legacy schema and bare behavior remain covered; no whole completion or main landing is claimed.

Rank CLI constituent WIP: all ten command details now use concrete result/2 serializers. Ordered rank finalizes numeric positions and stable answer identity before serialization. Saved-set turns retain every member observation, checked partial usage and final full member positions even when top truncation prunes a member's payload. Each retained original is shared across its member candidates; lightweight numeric scores retain full positions without retaining every original under top. Bare output, lexical record bytes, stable ties, request bodies, per-question cache/replay and failure withholding stay covered. Set metadata uses the resolved set digest, all actual member receipts and unique actual attempts; the winner retains its own resolved profile/batch presentation. Timing sidecars, held-model warnings, final whole review and consumer adoption remain open.

Source grows 376 nonblank Rust lines (147092 → 147468) for deferred typed rank rendering, shared member payloads, truthful full-position bookkeeping and two independent command record/replay cases. The legacy rank detail renderer and JSON member-name patcher are removed; existing native rank identity and aggregate arithmetic are reused. No new scheduler, parser, key domain, dependency or proof tool is added.

Held-model warning constituent adds 262 measured nonblank Rust lines (147468 → 147730) for validated route/question-bound exclusion, call-scoped typed facts and safe fixed CLI presentation, plus two actual cache/replay regressions. SQL row selection shares its existing column layout and constituent validation; warnings remain outside durable usage totals and legacy facts serialization. Strict complete schema/corpus adopt the boolean field together. Final timing integration, whole High review/full landing gates and host adoption remain open.

### Explicit source relation execution constituent

Native explicit source records now share one admitted entity for equal selected name/kind pairs, retaining every ordered original occurrence. This corrects the earlier whole-set note only for located source inputs; non-source duplicate rejection remains unchanged. `CompleteRelated::value()` retains the semantic accepted edges and `source_edges()` exposes complete typed occurrence pairs with actual ordinal, native original content and source coordinates. Endpoint ownership survives engine drop and requires no JSON result parser. Mixed source/non-source input refuses before lookup/send. Source admission limits remain 255 occurrences and 16 MiB retained originals. Escaped expanded endpoints are measured against 16 MiB before retaining/output; overflow returns the actual started facts without retry or truncation. CLI and native reuse one presentation byte budget.

The prior implementation refused equal source identities; the new public regression failed with that exact duplicate refusal before the integration and passes afterward. Independent expected wire questions pin deduplication, partial input887/output unknown, original false/null/list fields, ordered Cartesian expansion, renamed-source zero-send replay with unchanged answer identity, non-Clone/non-Serialize/non-Send original ownership and escaped-output overflow. Existing non-source and CLI located cases remain required. Whole High review, root landing and actual host adoption remain open.

Source grows 439 nonblank Rust lines (148275 to 148714) for owned typed source endpoints, shared escaped-output accounting and three public behavior cases. Checked native/CLI relation rendering and removed the CLI-only duplicate byte budget; semantic question planning, canonical members and physical presentation serialization remain shared.

The executable result page now validates actual replayed CLI result/2 rows against the shared strict schema and retains the separate result/1 compatibility-table checks. One real refund replay example (unknown historical batch count, zero sends, empty current attempts) is shared with the type corpus. These docs do not claim host parity or main landing.

### Native located recognition views constituent

`CompleteRecognized::source_value()` supplies owned typed physical names and relation endpoints beside existing semantic `value()`, with `SourceRecognition`, `SourceRecognizedEntity` and `SourceRecognizedRelation` accessors. Complete serialization presents actual flat file/span coordinates while preserving scalar offsets, original units, canonical distributions, observations and answer identity. The shared `SourceRecord::span_lines` mapper supplies inclusive physical ranges, including Unicode and CRLF; a document without supplied lines omits them. Owned views survive engine drop. Decoded JSON field locations refuse before any lookup/send under the existing files contract; ordinary unlocated field recognition still preserves arbitrary original JSON. The older location fixture incorrectly combined selected JSON fields with located text and is corrected to real literal source units; the separate saved-field regression remains.

New outside-in cases cover actual partial usage, complete-schema validation, separate context at all three stages, renamed-source zero-send replay, typed source/relation getters and absent document lines. A counted admission case pins the static safe refusal for a later decoded JSON field in both live and strict replay. Whole native review, landing and host adoption remain open.

The decoded-field admission regression fails on the prior implementation by accepting the bad located field and sending six requests; corrected admission refuses the complete set with zero requests in live and replay. Physical source ranges are also validated before sending, including insufficient line ranges and usize overflow. Source grows 479 nonblank Rust lines (148714 to 149193) for the owned source recognition views, shared native mapper integration and four public behavior cases. Checked CLI source presentation and native complete recognition; reused canonical serializers, stage scheduler, source location carrier and Unicode mapper rather than reparsing JSON or implementing another source map.

Focused recognition validation passes: 76 complete native behavior cases, 28 affected command cases, generated schema, affected package Clippy, offline policy/format and 1555 public declarations (four plants refused). No whole test/lint, main landing or host parity is claimed.

### Bounded complete-set find admission constituent

Find's ordinary collector now checks cancellation/proxy admission before each pull and after each produced unit, shared by released and complete calls. Relative deadlines are fixed at call entry and reused through preparation. Composed find retains only the existing maximum plus one rejected candidate (256, or 255 when offering none), using the same pure option-count bound as plan construction. Existing exact range errors, complete-set semantics, eager declaration validation, original ownership and no-send refusals remain unchanged. No suffix is consumed after cancellation or count refusal.

Three prior-failing public cases observed four pulls after first-unit cancellation, three pulls when pre-cancelled, and all 1,000 composed units before the 256th-unit range refusal. They now pin one/zero/256 (255 with none) pulls and zero requests, with no invented started facts. Existing pure find behavior and affected CLI checks remain required. Source grows 126 nonblank Rust lines (149193 to 149319) for shared admission and the three regressions; checked both find preparation routes and reused the primitive bound rather than duplicating admission or adding a reader/scheduler.

Native foundation handoff at the final working branch: all ten concrete complete execution routes, typed resolved readings/metadata/raw selection, owned observations/failure identities, shared record/source composition and strict complete-call serialization are implemented. CLI details use result/2 and numeric rank positions; released convenience/scalar calls, old C ABI and explicit generic compatibility envelopes remain available. The closed MCP surface uses the existing compiled-version/call/send headers and canonical schema. Host owners can now integrate their actual named execution/views rather than private fixture constructors. C652/MCP2243/family/frame/SQL adoption, fresh whole High review and full landing gates remain open; no whole 0.2 parity or main claim is made.
