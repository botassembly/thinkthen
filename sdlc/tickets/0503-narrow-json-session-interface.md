# 0503: Add a bounded session interface for JSON requests

Status: OPEN.

Milestone: 0.2

Depends on: 0511

Reviews: revision 95fade3863f777142ecd49d4abca0efcf1790cce, accept

Reviews: revision c79e3f65e05eb4f12fa46d8f7ab003d477258c9c, accept

Reviews: revision 0c5f1f772e85a9fcdbc1e7ceb6f86bf21af004bd, accept

Reviews: revision e8f546aeca5644f3703f61cf6c6cf51b3038e0d6, accept

Reviews: revision 4d59e7c6a, accept

Reviews: revision a087f6dc3, accept

Reviews: revision a1cc320d9b69b91a33e0b17944602ccae432ea6d, reject

Reviews: revision 5512fd7a6022e08d6ff719844dba191baee03836, accept

Reviews: revision ee94000bf616d835693bd8b938e1437dda6a9696, accept

Reviews: revision 3e4e08051025cbf5e1f6cf801d5db5001558ca0e, accept

Reviews: revision 104d7f398c447b17a8a2fa129d4f87236c24aea6, accept

## Outcome

C-interface languages call one owned, bounded engine session. A caller declares a call once, feeds descriptors without staging the whole input, reads results, and finishes, cancels or frees the session. Cancel and free return promptly without waiting for a blocked provider. Final facts stay truthful and arrive only when the work actually settles.

## Evidence

- Starts from: [the 2026-10-09 binding decision](../decisions/2026-10-09-thin-first-class-bindings.md). [ADR 0129](../planning/adr/0129-owned-json-sessions.md) owns session execution and lifetime rules. The 0470 bounded-feed design supplies the ownership pattern.
- Keeps: Whole-set admission and completed prefixes where specified, the frozen 0.1 C ABI, cancellation, and failures with facts.
- Changes: Implement the owned session under ADR 0129: engine, session and result handles with push, read, finish, cancel and free. The session owns every input and result buffer and bounds its input and output queues. One writer at a time handles descriptor admission and sink dispatch. Admission follows 0511. 0513 supplies the result packet graph; native session work need not wait for every target generator. Claim `crates/thinkthen/src/public/**`, `libraries/c/src/**` and `specification/request.schema.json`, narrowed per slice.
  - 0470 consumes this session for DuckDB and builds no second queue or C handle family.
  - 0516 designs the first host wait and cancellation over this interface. Other hosts reuse that contract through their own scheduling idiom.
  - Direct-to-Rust languages stay on Rust.
- Proof: Typed outside-in tests cover backpressure, stopped readers, cancellation, failure facts and handle lifetimes. No reference outlives host storage. The tests distinguish prompt caller cancellation from eventual provider cleanup and final facts.
- Defers: Host adoption belongs to 0516 and the language migrations. Business policy and new networking need no ticket.

## Progress

The following native declarations implement the reviewed ownership contract in ADR 0129. Their implementation is `5512fd7a6022e08d6ff719844dba191baee03836`; C exports and packet serialization remain separate work within this ticket.

### Added public declarations

```text
enum RequestReaderFailure
RequestReaderFailure::InvalidInput
RequestReaderFailure::InvalidInput::location: Option<SourceLocation>
RequestReaderFailure::Io
RequestReaderFailure::Io::location: Option<SourceLocation>
RequestReaderFailure::Utf8
RequestReaderFailure::Utf8::location: Option<SourceLocation>
struct RequestSessionDescriptor
RequestSessionDescriptor::item: RequestItem
RequestSessionDescriptor::location: Option<SourceLocation>
enum RequestSessionPush
RequestSessionPush::Accepted
RequestSessionPush::Closed(RequestSessionDescriptor)
RequestSessionPush::Full(RequestSessionDescriptor)
enum RequestSessionPushStatus
RequestSessionPushStatus::Accepted
RequestSessionPushStatus::Closed
RequestSessionPushStatus::Full
enum RequestSessionRead
RequestSessionRead::End
RequestSessionRead::Pending
RequestSessionRead::Result(RequestSessionResult)
enum RequestSessionResult
RequestSessionResult::Aggregate(RequestValue)
RequestSessionResult::Observation(OwnedRecordObservation)
RequestSessionResult::Row(RequestSessionRow)
RequestSessionResult::Terminal(RequestSessionTerminal)
enum RequestSessionRow
RequestSessionRow::Annotation(CompleteRecord<QuestionInput, CompleteAnnotated>)
RequestSessionRow::Choice(CompleteRecord<QuestionInput, CompleteChoice>)
RequestSessionRow::Decision(CompleteRecord<QuestionInput, CompleteDecision>)
RequestSessionRow::Filter(CompleteRecord<QuestionInput, CompleteFilter>)
RequestSessionRow::Score(CompleteRecord<QuestionInput, CompleteScore>)
RequestSessionRow::Tags(CompleteRecord<QuestionInput, CompleteTags>)
struct RequestSessionTerminal
RequestSessionTerminal::error: Option<Error>
RequestSessionTerminal::facts: Option<Facts>
fn Engine::request_session(&self, Request) -> Result<RequestSession, Error>
fn RequestSession::cancel(&self)
fn RequestSession::finish(&self, Option<RequestReaderFailure>) -> Result<(), Error>
fn RequestSession::try_push(&self, RequestSessionDescriptor) -> Result<RequestSessionPush, Error>
fn RequestSession::try_push_json(&self, &str) -> Result<RequestSessionPushStatus, Error>
fn RequestSession::try_read(&self) -> RequestSessionRead
fn RequestSessionDescriptor::from_json(&str) -> Result<RequestSessionDescriptor, Error>
fn RequestReaderFailure::from_json(&str) -> Result<RequestReaderFailure, Error>
impl Drop for RequestSession
struct RequestSession
```

- 2026-10-09 landed f6768b855; next: Slice A settles ADR0129 after a real producer-closure correction and fresh acceptance. No runtime/session implementation is claimed. Implement the reviewed owned bounded session after shared admission and generated result-view interfaces; preserve frozen C compatibility and prompt caller cancellation/free.
- 2026-10-09 landed cb1c47896; next: Accepted observation packets preserve detailed native events through the existing bounded queue. Implement shared session ownership and sink dispatch after shared descriptor admission; frozen C compatibility and prompt cancel/free remain required.
- 2026-10-09 started
- 2026-10-09 landed 80f468ba6; next: Native owned sessions are landed; implement C exports and serialize the complete result packet graph before host adoption.
- 2026-10-09 landed 8c834929a8a3120a8a6398a41f41bf09b7541ea1; next: Owned native and C sessions expose canonical result bytes with reviewed lifetimes and cancellation. Finish generated constrained-language views and remaining contract consumers before closing.
