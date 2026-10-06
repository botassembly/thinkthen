# 0433: Keep SQLite find’s selected model

Status: in progress. The selected-model fix passes focused SQLite checks and awaits fresh review.

Milestone: 0.2

Owner: builder.
Ticket review: accepted 2026-10-06; blocking findings corrected.

## Outcome

SQLite find sends the selected per-call model and does not silently fall back to the engine default.

## Evidence

- Starts from: Current main c64b71859; 0405 audit is historical and predates 0377/0420. PM message `2026-10-06-pm-0-2-is-not-done-every-sdk-consistent-and-the-sdk-ready-for-the-proxy.md`, asks 6.
- Keeps: Preserve existing bare calls and generic JSON compatibility doors, backend selection from 0377, six error kinds, cancellation, secrecy, count-only usage, and zero-send strict replay. Reuse the Rust engine, C boundary and native file reader; add no host cache, scheduler or second parser.
- Changes: Apply validated controls.model through the public find question builder in databases/sqlite/src/scalars/find.rs. Preserve engine defaults for subsequent calls.
- Proof: Loopback engine model A/call override B sends B, then an ordinary call sends A. Invalid overrides send zero requests. Cache and strict replay of B use B’s identity with zero replay sends. Keep none, duplicate candidates, indexes and existing result shapes.
- Defers: Proxy service/screens, images, unrelated features and Windows Node/C#/JVM packaging remain outside this outcome.

## Dependencies and ownership

First product correction in the new plan; independent of the later SQL carrier work.
