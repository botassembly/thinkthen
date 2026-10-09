# 0503: Owned JSON session design

Slice A settles the interface in ADR 0129. It adds no product implementation. The first review found an impossible guarantee: the caller could discover Closed only after reading a descriptor. The corrected contract permits one already-read unaccepted descriptor and stops further reads once Closed is observed; static declaration refusals still precede source access.

A fresh reviewer accepted repaired revision `0c5f1f772e85a9fcdbc1e7ceb6f86bf21af004bd` against the existing Request boundary, execution paths and frozen C ABI. The decision specifies bounded delivery, owned results, truthful terminal facts, additive diagnostics and the generated-view dependency. The worktree was removed after a clean pushed handoff. No build, paid call or release action ran for this design slice.

## What the build taught us

An intake gate prevents acceptance after closure, but cannot undo a host read already performed. Keep the promised reader bound achievable. Keep the sole engine clone on the worker through settlement: the existing usage writer joins during destruction, so an additional caller-owned clone could make session free block.

## Observer packet source finding and proposed design

Inspection at `6af931685` found that C# copies detailed ordered observations independently of completed rows. The native RecordObservation callback is synchronous on the calling thread and already offers an owned snapshot. ADR 0129 therefore proposes Observation(OwnedRecordObservation), a full Rust-derived serialization view, and immediate publication from a worker-local Rust callback through the existing one-packet output queue. A full queue holds one packet on that worker and prevents further callbacks; no event Vec accumulates behind a row. Cancel preserves that completed prefix while a live reader drains it, and free closes the receiver without joining. The accepted original decision retains its provenance; this extension requires fresh review. Product serialization, generated readers and behavior checks remain implementation work in the existing named slices. The design cannot promise settlement for a stopped live reader or a permanently blocked provider.
