# 0443: Carry SDK call identity and cache instructions

Status: in progress. Native implementation in lane0 on ticket/0443-native-complete-results; host adoption and final landing checks remain open.

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
