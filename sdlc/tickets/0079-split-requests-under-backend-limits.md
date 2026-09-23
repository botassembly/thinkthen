---
flow: build
priority: 79
opens: crates/thinkthen/src/core/backend_profile.rs crates/thinkthen/src/core/plan.rs crates/thinkthen/src/core/reply.rs crates/thinkthen/src/engine/prepared_request.rs crates/thinkthen/src/engine/request.rs crates/thinkthen/src/cli/asking/request.rs crates/thinkthen/src/cli/annotate.rs crates/thinkthen/src/cli/annotate/plan.rs specification/backends.md specification/annotate.md sdlc/planning/relate-design.md sdlc/planning/handoff-to-the-architect-2026-09-21.md sdlc/planning/adr/0040-split-requests-under-backend-limits.md sdlc/ratchet.json
---

# 0079: Split requests under backend limits

Status: ready

## Outcome

Split one ordered multi-question plan into the fewest deterministic contiguous chunks that fit an explicit backend profile's byte, question, and option limits. Encode, digest, and preflight every chunk before replay, cache, key access, or the first send. A plan that fits keeps its exact historical body and digest. Results and `meta.requests` retain logical order.

## Current facts and design

`PreparedRequest::with_profile` checks one whole `Plan`; `annotate` preflights groups before sending. Add an engine-owned splitter that takes the longest next question prefix fitting exact encoded request bytes and expanded-question count. Repeat evidence unchanged. Prepare the complete chunk list before executing it through the existing request path.

Merge chunk outcomes and producing digests in plan order. Every reply must report the same model or the existing safe model-mismatch failure wins. Usage is the checked sum only when every chunk reports usage; otherwise it is absent. `requests_sent` is the checked sum of every chunk's sends, including retries. Existing process and durable counters count each actual send and each cached answer through the unchanged request path. `cached` is true only when every chunk replayed; one live chunk makes it false.

Add optional profile `max_options` for one choice question through new ADR 0040, which amends but does not edit accepted ADR 0032. Refuse an oversized single choice because splitting its options changes meaning. Correct `relate-design.md` and `handoff-to-the-architect-2026-09-21.md`: remove the approximate-character ceiling and stale 100-option backend ceiling, record that probes accepted 101 and 255 options, and name no Jev byte ceiling until measured.

## Scope and exclusions

Allowed: splitter, prepared identities, reply aggregation, `max_options`, annotate integration, deterministic tests, new ADR 0040, settled spec and relate/handoff planning corrections, and the exact ratchet. Do not edit ADR 0032. `cli/annotate.rs` is already at 500 nonblank lines: split its existing aggregation owner into a focused submodule before changing behavior. Touch at most ten production Rust files; add at most 500 nonblank production lines and 1,000 nonblank Rust lines total. Keep every file within 500 nonblank lines.

Excluded: combining records, groups, or calls; cost or row packing; token estimates; text splitting; prompt changes; transport/control/cache changes; public APIs; `recognize`; `relate`; another adapter; shipped profile values; live or paid calls; publication.

## Acceptance

- Pin one under-limit plan's current body and digest byte for byte.
- Force one plan across exact byte and question edges; assert longest-prefix bodies, digests, ordered answers, partial failures, model consistency, usage, sends, all-chunks cached truth, and `meta.requests`.
- Prove exact limits pass and a valid one-unit overflow repartitions. Prove evidence, one-question request, or option overflow sends zero requests, including when the impossible item follows valid chunks.
- Prove mixed replay/live chunks set `cached: false`, sum retries into `requests_sent`, and update existing counters once per actual event. Prove annotate preflights every chunk of every group before its first send and preserves under-limit recordings/output.
- Prove ADR 0032 stays byte-identical and both owning planning pages retain no 100-option or approximate-character ceiling. Run focused fixture, replay, and loopback tests plus the four local rungs and `git diff --check`; use no key or outside network.

## Dependencies

Landed tickets 0053, 0054, 0055, and 0059 supply request identity, partial outcomes, the engine boundary, and profile preflight. Ticket 0064 fixes retry-send accounting; 0069 fixes structured request encoding; 0073 fixes cancellation at each attempt start; 0074 fixes command SIGINT routing over that token. Chunk execution preserves all four. The accepted launch-first queue puts 0079 after 0074 and before 0080 `recognize` and 0081 `relate`.

## Complexity

Contract 2; State and timing 2; Reach 1; Proof 2; Cost of error 1; Total 8. Minimum floor: level 3 for partial failure and durable cache/recording identity. Final level: 3. Reasons: a profile contract change controls ordered durable requests and needs exact-byte, accounting, and zero-send proof. Selected implementer: `gpt-5.6-sol`, medium reasoning. Independent design and code review use separate Sol Medium sessions.

Re-score if implementation introduces concurrent chunk sends, text splitting, a public engine surface, or new transport recovery.

## Review

- Design review: rejected the first draft for mutating ADR 0032, stale limit claims, incomplete aggregation/accounting rules and dependencies, the full `annotate.rs` owner, and no level-3 floor. Re-review required the stale handoff correction too. This revision addresses every finding; re-review pending.
- Code review: pending
