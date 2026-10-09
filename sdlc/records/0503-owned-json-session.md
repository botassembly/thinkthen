# 0503: Owned JSON session design

Slice A settles the interface in ADR 0129. It adds no product implementation. The first review found an impossible guarantee: the caller could discover Closed only after reading a descriptor. The corrected contract permits one already-read unaccepted descriptor and stops further reads once Closed is observed; static declaration refusals still precede source access.

A fresh reviewer accepted repaired revision `0c5f1f772e85a9fcdbc1e7ceb6f86bf21af004bd` against the existing Request boundary, execution paths and frozen C ABI. The decision specifies bounded delivery, owned results, truthful terminal facts, additive diagnostics and the generated-view dependency. The worktree was removed after a clean pushed handoff. No build, paid call or release action ran for this design slice.

## What the build taught us

Integration preserves the reviewed native session declarations from ADR 0129 and records them with the implementation in the public inventory. The inventory passes its existing negative cases. The source ceiling is measured after combining the session with the intervening native presentation work; it is not the sum of two branch ceilings. Fresh review accepted the whole-set preparation repair at `5512fd7a6022e08d6ff719844dba191baee03836`. The native session remains a partial ticket outcome; C exports and packet serialization are still required.

An intake gate prevents acceptance after closure, but cannot undo a host read already performed. Keep the promised reader bound achievable. Keep the sole engine clone on the worker through settlement: the existing usage writer joins during destruction, so an additional caller-owned clone could make session free block.

## Native ownership and sink dispatch

The native implementation starts from `bd94cc3e06d90c9ebc7946eaec41a38a9a9d7020`. The public declarations live in `crates/thinkthen/src/public/request/session.rs` and `session_result.rs`, and `public/request.rs` re-exports them:

```rust
Engine::request_session(&self, request: Request) -> Result<RequestSession, Error>
RequestSession::try_push(&self, descriptor: RequestSessionDescriptor) -> Result<RequestSessionPush, Error>
RequestSession::try_read(&self) -> RequestSessionRead
RequestSession::finish(&self, failure: Option<RequestReaderFailure>) -> Result<(), Error>
RequestSession::cancel(&self)
```

`session_queue.rs` owns input, output and terminal cells. The worker owns the engine clone, admitted request, native batches and synchronous observer closure. The closure copies each actual event before publishing it. Cancellation closes intake while preserving completed packets for a retained reader. Drop closes reception, wakes waits and relinquishes the worker without joining. Native joined facts fix the terminal snapshot, including the observed usage persistence state.

Request execution shares dispatch between collection and delivery. Its streaming sink publishes each row before pulling another native row. The shared filter selector retains ordinary selection and file occurrence behavior. Whole-function methods retain their existing aggregate contracts. Descriptor composition uses existing item admission, declarations, projection, attachment accounting and source locations. The generated Request schema adds descriptor definitions without changing its canonical root or item grammar.

The filter regression found that ordinary Request admission permits file selection only with source input. Session admission delegates to the same validator with authority for its owned located descriptor feed. Ordinary Request admission retains its existing refusal, while sessions select the first passing occurrence for each retained physical file. A session descriptor without a required source location refuses before sending and retains its completed prefix.

The idle-producer regression found that ordinary header admission checks cancellation but does not check deadlines. A separate reader admission method uses the existing native deadline machinery and the once-started controls. Queue waits therefore honor that same absolute deadline without changing ordinary admission behavior.

The native slice adds no C exports or packet JSON serializer. Those additions need the owned result projection graph and generated readers from 0513. Existing C batch handles remain unchanged. A live session whose reader stops draining can keep a completed packet on its worker. A blocked provider or source operation can delay settlement. Queue bounds cover waiting transfers, not all resident input, native scheduling or whole-set memory.

## Eager admission with bounded delivery

Review of `a1cc320d9b69b91a33e0b17944602ccae432ea6d` found that sink dispatch suppressed eager native admission. Two valid inline decision records could send the first request before the engine rejected the second record against its record limit. The public session regression reproduced one loopback send where the caller requires zero.

Request execution marks eager sink inputs for whole-set preparation through an internal call control. Atomic, annotation and dynamic-choice streaming methods pass their existing prepared iterators through shared native admission before scheduling. Native planning and record limits therefore refuse the whole set before sending. The existing streaming sink still delivers completed rows when a later provider call fails; the eager complete API would discard that prefix. Declared feed and source inputs retain lazy admission, completed prefixes, cancellation and joined terminal facts. This change adds no public API, duplicated planner, C exports or queue.

## Observer packet source finding and proposed design

Inspection at `6af931685` found that C# copies detailed ordered observations independently of completed rows. The native RecordObservation callback is synchronous on the calling thread and already offers an owned snapshot. ADR 0129 therefore proposes Observation(OwnedRecordObservation), a full Rust-derived serialization view, and immediate publication from a worker-local Rust callback through the existing one-packet output queue. A full queue holds one packet on that worker and prevents further callbacks; no event Vec accumulates behind a row. Cancel preserves that completed prefix while a live reader drains it, and free closes the receiver without joining. The accepted original decision retains its provenance; this extension requires fresh review. Product serialization, generated readers and behavior checks remain implementation work in the existing named slices. The design cannot promise settlement for a stopped live reader or a permanently blocked provider.
