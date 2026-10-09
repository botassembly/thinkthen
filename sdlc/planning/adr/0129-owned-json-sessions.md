# ADR 0129: Own JSON request sessions and result handles

The queue owner accepted this interface for ticket 0503 after fresh review of revision `0c5f1f772e85a9fcdbc1e7ceb6f86bf21af004bd`. Implementation follows this decision. Ian can overturn its additive names, packet shape and queue choice. The governing contract is the 2026-10-09 amendment to 0503, the amendments to 0511 and 0513, and `sdlc/decisions/2026-10-09-thin-first-class-bindings.md`. ADR 0125 retains Request admission and pure-core boundaries. ADR 0101 retains the frozen C compatibility forms.

## Evidence and problem

The code evidence below comes from root `71489a13afcea9e12ba3698e4578a1bdbabd21d4`. Ticket amendments were read from the main checkout on 2026-10-09. The design adds no product code, dependency, build or schema change.

`crates/thinkthen/src/public/request/execution.rs::Engine::execute_request` already resolves one admitted Request, applies native controls and dispatches every function to its native complete method. Its `stream` calls `Batch::into_outcome`; `public/batch.rs::into_outcome` retains every row in a Vec. Reusing that collecting path inside a worker would retain the whole output and defeat bounded streaming. Eager execution also collects rows before sending, which must remain available for whole-set admission.

`public/batch.rs::Batch` is neither Send nor Sync and borrows its source. Drop stops work and joins workers. `libraries/c/src/ffi/batch/ffi.rs::BatchHandle` carries a same-thread check, an engine reference, owned backing and a native batch with an extended lifetime. Its free joins. Those exports remain compatibility forms. The proposed session never moves that batch between threads or extends a lifetime.

`public/request/input.rs::RequestItem` owns original data, context, options, images and recognition controls, but has no SourceLocation. `RequestFeed::from_records` in `request/execution.rs` already accepts owned composed RecordInput<QuestionInput> and retains provenance. `request/composition.rs::compose_item`, `source_row` and `request/transport.rs::source_records` own selection, original retention and source locations. The CLI-only TransportDescriptor currently selects an item or a source; it does not provide arbitrary location metadata. The implementation must generalize this composition seam instead of treating a pathname as evidence or adding a validator in a binding.

`libraries/c/src/current/mod.rs::Storage` already keeps immutable view backing in stable allocations. Its result views demonstrate an ownership pattern to reuse. They do not authorize borrowing the old batch or its engine in a new result. `sdlc/scripts/generate-c-header.py` generates the installed header directly from Rust with the pinned cbindgen tool. Reuse that pipeline.

## Decision and trade-off

Declare an owned Request once, clone its Engine, and move that clone into one worker that owns both. The worker exclusively owns the session engine clone; the caller-facing session retains no duplicate clone, including after settlement. Feed copied descriptors through a queue with one waiting item. Read owned result packets through a queue with one waiting packet. No foreign callback, iterator, buffer, context reference, interrupt predicate or engine Door survives a return from the new C API.

A synchronous wrapper around Batch would keep borrowing and make cleanup wait on a provider. A worker around execute_request would fix ownership but still collect output. The selected design combines a worker with shared sink dispatch. It costs a worker and shared stop state per session. It does not promise to kill a blocked provider thread or reclaim that worker before the provider returns. Admission stops promptly; detached cleanup eventually joins native work on the worker itself.

Queues bound waiting transfers, not resident memory. Existing record, byte, image and whole-set limits remain authoritative. One descriptor may contain large admitted content. Native packing, in-flight responses, whole-set candidates and caller-retained results can consume additional memory. Add no new input cap and make no RSS guarantee.

## Native public API

Add these owned public types under `public/request/session.rs`, re-exported through `public/request.rs` and `public/mod.rs`:

```rust
Engine::request_session(&self, request: Request) -> Result<RequestSession, Error>
RequestSession::try_push(&self, descriptor: RequestSessionDescriptor)
    -> Result<RequestSessionPush, Error>
RequestSession::try_read(&self) -> RequestSessionRead
RequestSession::finish(&self, failure: Option<RequestReaderFailure>)
    -> Result<(), Error>
RequestSession::cancel(&self)

RequestSessionPush::Accepted
RequestSessionPush::Full(RequestSessionDescriptor)
RequestSessionPush::Closed(RequestSessionDescriptor)
RequestSessionRead::Result(RequestSessionResult)
RequestSessionRead::Pending
RequestSessionRead::End
```

The worker owns the request, admitted preparation and engine clone. The caller-facing session owns synchronized queues and stop state, sharing the CancelToken with the worker. It retains no Engine clone before or after settlement. `engine/usage.rs::Counters::drop` joins its usage writer, so keeping a duplicate clone in the caller-facing session could make free block when that clone becomes the last engine owner. The worker releases its clone on its own thread after native settlement. Drop closes the receiving side, signals cancellation, wakes queue waiters and relinquishes its worker handle without joining. Result values are independent owned values. Read and push are nonblocking with respect to queue readiness, readers and providers; bounded copying, decoding and local validation still take work. Do not introduce borrowed CallOptions or a generic foreign execution environment to this constructor. Request options remain the source of context, model, batch, deadline, attempts and request-budget controls. Internal surface attribution can be passed as an owned value by the C crate.

`RequestSessionDescriptor` owns `item: RequestItem` and `location: Option<SourceLocation>`. Add shared conversion of these descriptors to records at the existing composition seam. Projection and context selection happen there. Attach validated location to the retained original after composition, preserving annotation text and source/image originals. Location never enters selected evidence or request identity. The native composed feed remains usable by direct Rust callers; the new descriptor path does not bypass its declaration and route checks.

The JSON descriptor is a closed object with required `item` and optional `location`. Item uses the existing RequestItem decoder. A location is a closed object with required `file` and optional paired one-based inclusive `first_line` and `last_line`, validated by the existing SourceLocation constructor. Explicit null for item, location or a supplied coordinate refuses. Do not change canonical RequestItem or invent a location key in its existing grammar. The additional descriptor decoder belongs to the shared Rust Request edge. Its schema derives there when the slice is implemented.

`RequestReaderFailure` is an owned transport enum with `Io`, `Utf8` and `InvalidInput` variants, plus optional SourceLocation. Map them to the existing reader error kinds and fixed safe messages in Rust. Io maps to Local; Utf8 and InvalidInput map to Usage. No free-text host diagnostic, path-bearing error string or caller-supplied facts crosses this door. Native composition/runtime validation still produces its own typed errors. These transport failures cannot impersonate provider, deadline, cancellation or defect outcomes.

For a feed Request, finish(None) means EOF after the last accepted descriptor. finish(Some(failure)) means the reader failed after that same accepted prefix. Finish uses an independent terminal-input cell so it succeeds while the one-item queue is full. The first finish fixes EOF or failure; repeated identical finish succeeds, a different finish returns Usage. After finish, push returns Closed. A non-feed Request executes its declared inline/source input; push refuses Usage and finish(None) is an idempotent no-op. finish(Some(_)) on a non-feed Request refuses Usage. The shared input iterator delivers accepted items before EOF or the reader failure, unless native cancellation or a whole-set refusal has already stopped intake.

Request admission happens synchronously before a worker is spawned. It validates available inline declarations and selectors without opening sources, under ADR 0125. Saved question resolution and content access happen on the worker only after header admission. Failures before worker creation return Error. Failures after creation produce the one terminal packet; they do not disappear into a thread-local error slot.

A Full push retains ownership for retry. Accepted transfers ownership exactly once. Closed retains ownership and tells the producer to stop reading. Only try_push exposes feed closure, and constructing its descriptor requires reading content first. Runtime closure may therefore race one already-read, unaccepted descriptor. The producer must dispose of that descriptor after Closed and must not advance its reader again; this contract does not promise zero reads between runtime closure and its observation. A session-wide intake gate serializes admission of the last descriptor with closure so no item is accepted after the gate closes. The producer must retry the same unaccepted descriptor rather than advance its reader.

## Output and terminal facts

Define owned `RequestSessionResult` with `Row(RequestSessionRow)`, `Aggregate(RequestValue)` and `Terminal(RequestSessionTerminal)` variants. RequestSessionRow has one typed variant for each streamed atomic family: decision, choice, tags, score, filter and annotation. Each owns the same CompleteRecord and nested observations that current RequestValue stores in a Vec. It has no session or engine reference. Terminal owns optional Facts and optional Error; success has no Error. Failure facts use the Error's actual final facts. Absence before a started invocation stays absence.

Decide, choose, tag, score, filter and annotate emit rows in input order. Preserve existing filter selection, including files_only and its first passing occurrence per file. A rejected filter row must not be invented as a selected result. Rank and rank-set emit one aggregate in final order after whole-set admission. Find emits its existing complete candidate/selection value. Recognition emits its existing complete recognition value. Relate emits its existing complete entity-set relation value. Their native methods currently return whole-function calls, so this draft adds no incremental promises or reinterpretation of their contracts.

Every live, retained session yields exactly one terminal packet after its actual completed output prefix, then End on every read. Empty success still yields terminal facts. Read returns Pending while the worker is unsettled, including after cancellation. cancel stops future admission and wakes input/output waiters; it does not manufacture final counts or imply that a held provider finished. Queued completed packets remain readable before terminal. If a worker holds a completed row while the queue is full, cancellation must preserve that row before terminal while the session remains alive. Free permits discarding unread packets because the caller has relinquished the session. No later foreign output is written.

Use a separate immutable terminal slot rather than enqueueing terminal behind a full result queue. Read drains queued packets and any ordered producer-held packet before taking terminal. Settlement cannot publish terminal while a completed earlier packet remains unpublished. It must not keep pulling native rows after output backpressure or stop admission. Preserve actual request starts and final joined facts. The worker may still await a provider, native join or blocked filesystem operation before settlement. No caller-facing cancel or free waits for that settlement.

Freeze the existing 0468 usage_persistence observation at facts_snapshot. Pending remains Pending in a terminal result even if a later engine status changes. Do not flush usage on cancel/free, infer Written from a successful judgment or turn persistence failure into a failed answer. Preserve missing token usage, unresolved null, failed logical members and all known observations. The 0513 generator must preserve unknown JSON members through conversion and round trip; these members never activate policy.

The packet JSON uses closed control tags `kind: row|aggregate|terminal` and a function tag from RequestFunction. Row/aggregate `value` is the existing complete typed value, with no invented success or facts defaults. Terminal contains optional `facts` and optional `failure` using the existing complete error representation. Define packet serialization and schema once in Rust. Host wrappers use 0513 generated typed readers. The packet schema describes the session envelope, not a replacement result schema.

## Additive C interface

Export opaque `thinkthen_session` and `thinkthen_session_result` handles with these spellings:

```c
int thinkthen_session_new(const thinkthen_engine *engine,
    const char *request_json, size_t request_len, thinkthen_session **out);
int thinkthen_session_try_push(thinkthen_session *session,
    const char *descriptor_json, size_t descriptor_len, uint32_t *status);
int thinkthen_session_try_read(thinkthen_session *session,
    uint32_t *status, thinkthen_session_result **out);
int thinkthen_session_finish(thinkthen_session *session,
    const char *failure_json, size_t failure_len);
void thinkthen_session_cancel(thinkthen_session *session);
void thinkthen_session_free(thinkthen_session *session);
const char *thinkthen_session_error_message(void);
int thinkthen_session_result_json(const thinkthen_session_result *result,
    const char **out, size_t *out_len);
void thinkthen_session_result_free(thinkthen_session_result *result);
```

Return the existing six error codes for immediate API refusals and guard defects. These calls record immediate diagnostics in a new calling-thread session-failure slot. thinkthen_session_error_message returns its borrowed fixed safe UTF-8 diagnostic until the next immediate session failure or thread exit; before a failure it returns a fixed no-failure message. A successful operation does not overwrite it. This slot has no execution facts. It never changes the frozen NULL-engine failed-constructor slot and never retains an engine failure table. An execution failure arrives as a terminal result with an OK read; callers must inspect its typed failure. Immediate nonzero returns leave every output unchanged.

New generated constants are `THINKTHEN_SESSION_ACCEPTED_V1 = 0`, `THINKTHEN_SESSION_FULL_V1 = 1`, `THINKTHEN_SESSION_CLOSED_V1 = 2`; read constants are `THINKTHEN_SESSION_RESULT_V1 = 0`, `THINKTHEN_SESSION_PENDING_V1 = 1`, `THINKTHEN_SESSION_END_V1 = 2`. They are separate status domains. With OK, push always sets status. Read with Result transfers one result owner; Pending and End write NULL to out. Required output slots must be valid and nonnull. Read status/out addresses and result_json out/out_len addresses must differ before any side effect. Validate all required slots before popping a packet. A result accessor returns borrowed immutable UTF-8 bytes owned by that result, length excluding a trailing NUL. The result owns the terminator too; callers never free the borrowed pointer. Byte access and every generated nested view stay valid until result_free even after session_free and engine_free.

request_json and descriptor_json are length-delimited UTF-8, with lengths representable as Rust slices. A null input pointer is allowed only with zero length, which still fails if a JSON document is required. Embedded NUL is not a terminator and follows JSON decoding rules. finish(NULL, 0) means EOF; other finish inputs decode the closed reader-failure object with `kind: io|utf8|invalid_input` and optional location. Canonical request closure and duplicate-member refusals apply to these new shared control objects. Authored item JSON retains its own grammar and order.

The caller may release or overwrite every input buffer immediately after a return. Accepted push retains a decoded owned descriptor. Full and Closed retain nothing; the caller keeps its bytes for retry or disposal. Check capacity and closure before allocating a retained descriptor, then decode and atomically reserve the slot; a closure race returns Closed without publishing. It is acceptable to discard a transient decoded value after a concurrent race. Pointer alignment, live readable/writable ranges and partial-overlap obligations remain caller duties under ADR 0101. Outputs publish only after every fallible conversion. Panic guards cover every new export.

Push, finish, cancel and read can run on separate threads with internally synchronized state. One producer and one consumer are the supported stream roles; concurrent calls are serialized without duplicate acceptance or duplicate result transfer. A pending call never holds the queue lock during decoding, provider work or a foreign operation. The caller must keep each handle live while an operation uses it. Free occurs after concurrent operations return and once per handle; no new handle supports use after free. Cancel(NULL), session_free(NULL) and result_free(NULL) are no-ops. Other NULL handles refuse Usage. No same-thread engine lifetime or old Batch lifetime applies to these handles.

C, Zig, Ada and COBOL receive fixed result layouts generated from Rust through the 0513/header path. This draft does not hand-write another result layout. The exact generated typed-view accessor and layout names must be supplied by the 0513 target slice before constrained-language adoption. JSON access is the session transport, not permission for those languages to maintain result readers. Existing typed C, JSON door, batch symbols, enum values, structs, failure slots and frees retain their frozen behavior; no compatibility symbol redirects to the new ownership contract.

## Shared execution and host readers

Extract one sink-driven dispatch from request/execution.rs. Its collecting consumer rebuilds today's RequestOutcome for execute_request. Its session consumer moves one row at a time into owned packets, retaining no completed Vec for streaming families. Replace the use of Batch::into_outcome in that path with iteration followed by final facts settlement. Keep the current public Batch API and existing collecting users unchanged. Extract function preparation once; do not maintain ten new session schedulers or repeat Request admission.

The worker constructs and consumes its native Batch on its own thread. Worker-owned admitted question, context and CancelToken provide the local borrows needed by CallOptions. No unsafe transmute, self-referential batch owner or Send assertion is permitted. Separate input/output condition variables wake on finish, cancel and Drop. A terminal intake gate makes try_push report Closed when runtime admission, cancellation or a whole-set limit closes the feed. Static declaration refusals still occur before any source is opened or read; runtime closure follows the producer observation rule below.

DuckDB filesystem enumeration, file reads and interrupt checks remain on its calling thread in the later adapter ticket. Its producer copies content and provenance into descriptors and waits/retries only while backpressure allows. It reads at most one descriptor ahead of acceptance, retries that same descriptor on Full, and advances its reader only after Accepted. If try_push returns Closed, it disposes of the one already-read, unaccepted descriptor and never advances the reader again. Runtime closure can race that read because try_push is the closure observation point. It must not pass a DuckDB filesystem, ClientContext, interrupt callback or host-owned reader into the Rust worker. The shared session does not implement DuckDB authority. Native standalone RequestSource can use native file reading on its own worker under existing authority. No adapter file is claimed by this design.

## Initial slices and useful proof

First settle the shared descriptor seam with 0511. Claim `crates/thinkthen/src/public/request/session_input.rs` (new), `request/composition.rs`, `request/native_feed.rs`, `request/execution.rs` and `request.rs`. Tests belong in `crates/thinkthen/tests/request_contract/session_inputs.rs` (new) and its existing module registration. This slice validates projection/context conflicts through shared admission, preserves original/source/image provenance and counts reader advancement at refusal. Static declaration refusals must show zero source opens and zero reads. Runtime closure permits at most one already-read, unaccepted descriptor and requires zero further advancement after the producer observes Closed. It depends on 0511's input/composition ownership; serialize those edits rather than competing.

Next add shared sink dispatch and native session ownership. Claim `crates/thinkthen/src/public/request/session.rs` and `request/session_queue.rs` (new), `request/execution.rs`, `request/result.rs`, `public/mod.rs` and `public/request.rs`. Claim `crates/thinkthen/tests/request_session.rs` (new) for outside-in native session behavior. Claim public/batch.rs only if a shared drain helper is needed; changing its documented Drop contract is out of scope. Reuse existing listener and isolated-engine helpers. One counted listener case proves bounded row delivery with a stopped consumer and verifies request starts cease once existing scheduler capacity is exhausted. It distinguishes queue capacity from native batch capacity. A reader failure after a completed row returns that row then one failure with truthful facts. A counted whole-set over-limit producer receives Closed, may have read at most one unaccepted descriptor before observing it, advances no further item after that observation, and sends zero requests. A Full result must retry the same descriptor without another reader advance. Cover aggregate outputs through the existing Request parity cases plus session packet ordering, without duplicating their full function matrix.

Add C ownership as the next slice. Claim `libraries/c/src/session.rs` and `libraries/c/src/ffi/session/ffi.rs` (new), `libraries/c/src/lib.rs`, `libraries/c/src/ffi.rs`, `libraries/c/cbindgen.toml`, `libraries/c/include/thinkthen.h` and a new driver `libraries/c/tests/c/session.c`, registered in `libraries/c/tests/door/main.rs` using its existing installed-header/ASan compile helper. Add `session` to the MSVC ownership driver list in `libraries/c/tests/door/windows.rs` in its target slice. Immediate session error-slot storage claims `libraries/c/src/session/errors.rs` (new); reuse the existing containment guard internals without changing failure-slot semantics. Generate the header; do not edit its declarations by hand. The driver frees or overwrites request/descriptor buffers immediately, frees the original engine before reading, and keeps a result and nested views after freeing the session. ASan protects against host-buffer and owner lifetime errors. A counted held-provider case calls cancel and free while the listener withholds a reply; the listener must remain held until both operations return, so a joining implementation fails without a timing threshold. Then release and drain the owned listener fixture. Exercise required/aliased output slots, invalid UTF-8, duplicate/unknown control members and unchanged output sentinels with zero sends. Run the existing frozen header/symbol checks with the new exports added explicitly to the additive inventory.

Before typed-host adoption, coordinate session packet serialization in `request/session_result.rs` (new), `crates/thinkthen/src/schema_tests.rs`, `src/schema_tests/complete.rs`, `specification/result.schema.json` and `public/results/complete.schema.json`. 0513 owns those schema/generator edits and the result-presence representation. Its conformance fixture must include a session packet retaining missing, explicit null, failed member and unknown result members through conversion and round trip. Reuse `sdlc/scripts/generate-c-header.py` and `sdlc/generators/results/generate.py`; add no second generator framework or runtime validator. Exact target templates, typed-view names and installed host proof belong to those named adoption slices.

## Risks and prerequisites

0511 must expose the shared descriptor composition/admission seam before the C decoder is built. The current CLI TransportDescriptor is neither public nor sufficient for located host content. This is a concrete shared-edge change, not proof that RequestItem already preserves location.

0513 must settle owned presence, tolerant result conversion, generated packet readers and constrained-language view declarations before consumers migrate. The native sink/session and JSON transport can be implemented first; a transport-only result is not completion of first-class typed adoption. No product code is authorized by this draft alone.

Cancellation cannot guarantee that a permanently blocked provider or filesystem releases the detached worker and cloned engine. Prompt free bounds host waiting, not background resource retention. Existing native deadlines and cancellation checks remain in force; no new networking or forced thread termination is proposed. Final facts must await native settlement. Returning early with invented final facts would violate the ticket.

The existing pipeline may prefetch and pack more than one record. Queue capacity does not constrain that established scheduler behavior. The shared intake gate must prevent acceptance after closure. The host producer must stop reader advancement after observing Closed through try_push, allowing at most one already-read, unaccepted descriptor when runtime closure races the read. Static declaration refusals must precede source opening and reading. The implementation must prove actual sends under output pressure. It must preserve whole-set admission rather than changing eager calls into prefix calls to save memory.

The C typed-view spellings remain a 0513 dependency. Everything else in this draft has a concrete signature and ownership rule. A reviewer should block an implementation that collects streaming output, borrows foreign state, joins from cancel/free, drops an observable completed prefix before terminal, changes frozen ABI, restates semantic validation, or defaults absent facts/null values. The agreed stopping rule for this design is one reviewable draft and its source evidence, without builds or wider proof.
