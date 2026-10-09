# 0503: Owned JSON session design

Slice A settles the interface in ADR 0129. It adds no product implementation. The first review found an impossible guarantee: the caller could discover Closed only after reading a descriptor. The corrected contract permits one already-read unaccepted descriptor and stops further reads once Closed is observed; static declaration refusals still precede source access.

A fresh reviewer accepted repaired revision `0c5f1f772e85a9fcdbc1e7ceb6f86bf21af004bd` against the existing Request boundary, execution paths and frozen C ABI. The decision specifies bounded delivery, owned results, truthful terminal facts, additive diagnostics and the generated-view dependency. The worktree was removed after a clean pushed handoff. No build, paid call or release action ran for this design slice.

## What the build taught us

An intake gate prevents acceptance after closure, but cannot undo a host read already performed. Keep the promised reader bound achievable. Keep the sole engine clone on the worker through settlement: the existing usage writer joins during destruction, so an additional caller-owned clone could make session free block.
