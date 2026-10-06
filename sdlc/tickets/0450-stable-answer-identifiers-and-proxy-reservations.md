# 0450: Give each answer a stable identifier and reserve proxy policy fields

Status: in progress. Native implementation in lane0 on ticket/0443-native-complete-results; host adoption and final landing checks remain open.

Native admission WIP: reserved opaque IDs, normalized code readings,
question/recognition reservations and typed response overrides are concrete
types. Explicit null, empty and populated activation all refuse with the
same usage sentence at the public call/builder edge. Counted loopback public
tests pin zero sends, no started facts, refusal before replay loading and
Debug secrecy without imposing a Debug bound on actionable values. Surface
tokens are closed validated values; transport adoption and stable ID
computation/propagation remain open.

Milestone: 0.2
Owner: builder.
Ticket review: ACCEPT, 2026-10-06; fresh read-only review. Conflicting image deferrals corrected where found.

## Outcome

Every full logical result has a stable answer identifier usable for downstream outcome correlation. The typed proxy request/reply shape is reserved in 0.2; override execution waits for the admitted 0.3 proxy protocol.

## Evidence

- Starts from: Main 399c6c7c7 and the existing SDK plan. PM message `2026-10-06-pm-vision-ships-in-0-2-and-the-sdk-stays-one-endpoint-while-the-proxy-owns-the.md`, ask 4; experiment 0034 design, recorded spike at 8ec4fbffc0dbf8ceaa0fea5607749ff2f683a2a0 and subsequent OpenRouter controls. 0036 reports are pending.
- Keeps: Existing text behavior, typed SDK parity, six errors, cancellation, secrecy, spend limits and zero-send strict replay. Core remains free of I/O; the one Rust engine and native file reader remain shared.
- Changes: Add persistent observation identities and result answer IDs through one native implementation. Reserve optional proxy question_id, the resolved code threshold and a tagged override response. Publish schema/types and reject activation in 0.2 with zero sends; never send these fields to ordinary vendors.
- Proof: Cross-surface saved/live/cache/replay cases pin stable IDs for the same stored observations and normalized reading; changed cuts, member scope/rank position and refresh distinguish identities. Test partial member/failure versus null, aggregate ordering, no-store ephemeral answers, deterministic read-only legacy conversion and absence of key/header leaks. Inspect vendor request bytes to prove reserved fields cannot leak; active reserved inputs refuse before cache lookup or send. Vendor fields cannot attest proxy policy.
- Defers: Override execution, outcome-reporting service, routing/policy logic and proxy screens; these require the admitted 0.3 proxy protocol.

## Dependencies and ownership

0450 owns the identity/reservation implementation; 0442 owns the coordinated result/2/shared schema and ADR. 0443 supplies the existing collision-resistant ID facility and propagates typed metadata; 0444 stores/converts observation identity; 0426–0431/0410/0435 expose full result IDs; 0411 verifies changed-reading IDs and 0432 enforces every surface.

## Design notes

Call IDs identify invocations; request IDs identify sends; question keys identify stored inputs. Add observation_id per newly accepted wire answer at the engine edge, persisting it in cache/record metadata. Refresh creates a new observation even for equal probabilities; cache/replay retain it. No-store keeps identity only in memory. Derive answer_id in pure code using a versioned length-framed tuple of function/scope, ordered observation IDs and normalized resolved reading/aggregation/member identity. Exclude surface, file path, origin, call/request IDs, timing, cost, packing and display. Compute before host index conversion. Returning to an original rule over the same stored observations restores its ID. Aggregate roots include ordered children; successful null differs from failure. A partial result carries failed-member occurrence identity without storing failed answers; terminal errors fabricate no successful answer. Legacy identity is derived offline from the original validated saved key/answer and any existing stable metadata, before rekeying; no invented timestamp is required. Preserve it during migration and derive it in memory for read-only replay. Document that indistinguishable old snapshots cannot recover distinct historical occurrences. IDs provide correlation, not authentication or a new receipt system.

Expose answer_id on every new complete typed result regardless of whether probability details are requested. CLI details, SQL details/observations and frame carriers expose per-result/member identities, including filter rejections and rank omissions through observation routes. Existing bare CLI/scalar SQL/convenience outputs remain compatibility views without metadata. Typed C accessors expose IDs; old generic JSON envelopes retain compatibility, and an explicit additive identity-enabled complete route provides them. Update all strict new full-result readers coherently under 0442.

Reserve proxy.question_id as an optional opaque bounded string and proxy.code_threshold as the existing normalized number/band/null grammar. Derive the code threshold from current precedence/defaults; a caller cannot supply a contradictory second threshold. Reserve meta.proxy.decision_id and override tagged none/threshold/decision, carrying code_threshold, effective_threshold where required, and typed code_value/overridden value for a decision override. Exact bounds and function-specific reading fields are settled in the shared contract before code. These are logical reserved SDK shapes, not invented HTTP paths or headers. In 0.2 emit no proxy attestation and apply no override; activation refuses before lookup/send. Unknown vendor fields remain tolerated but cannot populate this namespace. In 0.3 an explicitly configured validated proxy protocol may override even an explicit code threshold; preserve raw probabilities, code reading and effective reading, and replay recorded policy without contacting today’s proxy. Deferral prevents unvalidated business decisions and vendor leakage while the reserved shape and stable IDs land now.

## Native work in progress

Lane0 has distinct validated CallId, SdkRequestId, ObservationId, FailureId and AnswerId leaves, with exact lowercase hexadecimal parsing and safe validation diagnostics. Typed constituent identity/source carriers reserve the closed provenance values. Transport generation, stable pure derivation, storage persistence, runtime propagation and proxy activation refusal remain open; no runtime completion is claimed.

Native WIP: accepted wire answers now receive one fresh ObservationId, stored outside question identity with their actual optional successful wire-question count. Cache/replay/coalesced occurrences retain it; equal refresh responses receive new observations. Legacy observation IDs use the verified original per-question key, typed answer and only existing model/count/time/origin metadata. Original recording/1 conversion derives IDs before generated time/origin, usage shares or quoting. Full logical AnswerId construction and all ten executable complete result routes remain in progress.

Native atomic execution WIP: pure length-framed logical answer IDs now use typed function, original ordinal, ordered accepted observation IDs and resolved question/threshold/rank position. Retrieval origin, surface, batch count and transient IDs remain outside the digest. Default-equivalent cuts, cache/replay/coalescing, changed cuts and rank duplicate occurrences are checked through public calls with an independent framing oracle. Aggregate/member/stage identities and CLI result/2 adoption remain open.

Native aggregate WIP: facade construction order retains aligned sources/observations and normalized stage questions. Aggregate IDs include the resolved reading and ordered successful child IDs; annotation members retain their original ordinal/name/position, relation members retain source/target endpoints and position. Failed members use actual occurrence failure IDs and never fabricate answers. The public aggregate cases verify empty provenance and cache-stable recognition IDs. Owned observer identities and CLI/schema adoption remain open.

Native observer WIP: borrowed question details now expose normalized actual
questions, admitted rules, raw accepted picks, aligned sources/observations and
partial reported usage. `to_owned` retains those typed fields and input snapshots
past callbacks and engine destruction. Native record snapshots retain the complete
arbitrary original, ordered images and physical source location when supplied;
immutable input snapshots are shared across annotation members. No event parser
or reconstructed transport identity is required. Failed events retain actual
FailureId; successful null carries an answer ID. Question answer IDs qualify the
logical primitive by original/member/stage position; ranked and aggregate final
result IDs remain the distinct complete-result identities. Existing observer JSON
and row callbacks retain their released form. Public live/cache and partial
annotation cases compare owned IDs to complete members, pin partial counts and
withheld Debug, and prove zero additional cache sends.

### Added public declarations

```text
fn QuestionDetail::inputs(&self) -> impl Iterator<Item = &QuestionInput>
fn EngineBuilder::proxy(self, &ProxyActivation) -> EngineBuilder
struct IdentityError
impl Display for IdentityError
impl Error for IdentityError
struct CallId
fn CallId::new(impl Into<String>) -> Result<CallId, IdentityError>
fn CallId::as_str(&self) -> &str
impl Hash for CallId
impl Ord for CallId
impl PartialOrd for CallId
impl Serialize for CallId
impl Deserialize<'de> for CallId
impl Display for CallId
impl FromStr for CallId
type CallId::Err = IdentityError
struct SdkRequestId
fn SdkRequestId::new(impl Into<String>) -> Result<SdkRequestId, IdentityError>
fn SdkRequestId::as_str(&self) -> &str
impl Hash for SdkRequestId
impl Ord for SdkRequestId
impl PartialOrd for SdkRequestId
impl Serialize for SdkRequestId
impl Deserialize<'de> for SdkRequestId
impl Display for SdkRequestId
impl FromStr for SdkRequestId
type SdkRequestId::Err = IdentityError
struct ObservationId
fn ObservationId::new(impl Into<String>) -> Result<ObservationId, IdentityError>
fn ObservationId::as_str(&self) -> &str
impl Hash for ObservationId
impl Ord for ObservationId
impl PartialOrd for ObservationId
impl Serialize for ObservationId
impl Deserialize<'de> for ObservationId
impl Display for ObservationId
impl FromStr for ObservationId
type ObservationId::Err = IdentityError
struct FailureId
fn FailureId::new(impl Into<String>) -> Result<FailureId, IdentityError>
fn FailureId::as_str(&self) -> &str
impl Hash for FailureId
impl Ord for FailureId
impl PartialOrd for FailureId
impl Serialize for FailureId
impl Deserialize<'de> for FailureId
impl Display for FailureId
impl FromStr for FailureId
type FailureId::Err = IdentityError
struct AnswerId
fn AnswerId::new(impl Into<String>) -> Result<AnswerId, IdentityError>
fn AnswerId::as_str(&self) -> &str
impl Hash for AnswerId
impl Ord for AnswerId
impl PartialOrd for AnswerId
impl Serialize for AnswerId
impl Deserialize<'de> for AnswerId
impl Display for AnswerId
impl FromStr for AnswerId
type AnswerId::Err = IdentityError
enum Origin
Origin::Live
Origin::Cache
Origin::Replay
Origin::Proxy
Origin::Memory
impl Serialize for Origin
enum Observation
Observation::Answered
Observation::Answered::observation_id: ObservationId
Observation::Failed
Observation::Failed::failure_id: FailureId
impl Serialize for Observation
struct QuestionSource
impl Serialize for QuestionSource
fn QuestionSource::batch_size(&self) -> Option<u32>
const fn QuestionSource::origin(&self) -> Origin
fn QuestionSource::answered_by(&self) -> &str
struct ResultIdentity
impl Serialize for ResultIdentity
const fn ResultIdentity::answer_id(&self) -> &AnswerId
const fn ResultIdentity::origin(&self) -> Option<Origin>
fn ResultIdentity::question_sources(&self) -> &[QuestionSource]
fn ResultIdentity::observations(&self) -> &[Observation]
fn ResultIdentity::answered_by(&self) -> Option<&str>
struct OwnedQuestionDetail
fn OwnedQuestionDetail::detail(&self) -> QuestionDetail<'_>
fn QuestionDetail::to_owned(&self) -> OwnedQuestionDetail
const fn QuestionDetail::question(&self) -> ResolvedQuestion<'_>
fn QuestionDetail::threshold(&self) -> Option<ResolvedThreshold>
fn QuestionDetail::raw_pick(&self) -> Option<&str>
fn QuestionDetail::question_sources(&self) -> &[QuestionSource]
fn QuestionDetail::observations(&self) -> &[Observation]
fn QuestionDetail::answer_id(&self) -> Option<&AnswerId>
fn QuestionDetail::failure_id(&self) -> Option<&FailureId>
const fn QuestionDetail::reported_usage(&self) -> Option<ReportedUsage>
fn QuestionDetail::input(&self) -> Option<&QuestionInput>
fn RecordObservation::to_owned(&self) -> OwnedRecordObservation
enum OwnedObservedRow
OwnedObservedRow::Judgment(Judgment)
OwnedObservedRow::Annotated(Vec<NamedAnnotation>)
OwnedObservedRow::Recognized(Recognized)
OwnedObservedRow::Find(Option<usize>)
OwnedObservedRow::Relations(Vec<Edge>)
enum OwnedRecordObservation
OwnedRecordObservation::Question
OwnedRecordObservation::Question::index: usize
OwnedRecordObservation::Question::member: Option<String>
OwnedRecordObservation::Question::stage: Option<&'static str>
OwnedRecordObservation::Question::position: usize
OwnedRecordObservation::Question::detail: OwnedQuestionDetail
OwnedRecordObservation::Row
OwnedRecordObservation::Row::index: usize
OwnedRecordObservation::Row::value: OwnedObservedRow
enum MemberIdentity
MemberIdentity::Answered(AnswerId)
MemberIdentity::Failed(FailureId)
struct ProxyId
impl Serialize for ProxyId
fn ProxyId::new(impl Into<String>) -> Result<ProxyId, Error>
fn ProxyId::as_str(&self) -> &str
struct CodeThreshold
impl Serialize for CodeThreshold
const fn CodeThreshold::none() -> CodeThreshold
fn CodeThreshold::cut(f64) -> Result<CodeThreshold, Error>
fn CodeThreshold::band(f64, f64) -> Result<CodeThreshold, Error>
struct ProxyRequest
impl Serialize for ProxyRequest
ProxyRequest::question_id: Option<ProxyId>
ProxyRequest::code_threshold: CodeThreshold
ProxyRequest::code_relation_threshold: Option<CodeThreshold>
enum ProxyActivation
impl Serialize for ProxyActivation
ProxyActivation::Null
ProxyActivation::Empty
ProxyActivation::Request(ProxyRequest)
enum ProxyOverride<V>
impl Serialize for ProxyOverride
ProxyOverride::None
ProxyOverride::None::code_threshold: CodeThreshold
ProxyOverride::Threshold
ProxyOverride::Threshold::code_threshold: CodeThreshold
ProxyOverride::Threshold::effective_threshold: CodeThreshold
ProxyOverride::Decision
ProxyOverride::Decision::code_threshold: CodeThreshold
ProxyOverride::Decision::code_value: V
ProxyOverride::Decision::value: V
struct ProxyMetadata<V>
impl Serialize for ProxyMetadata
ProxyMetadata::decision_id: ProxyId
ProxyMetadata::reading: ProxyOverride<V>
```
