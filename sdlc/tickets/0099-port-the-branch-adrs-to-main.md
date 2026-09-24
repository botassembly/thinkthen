---
flow: build
priority: 99
opens: sdlc/planning/adr
---

# 0099: Port the branch ADRs to main

Status: draft, revised after design review; needs re-review. Owner: Claude.

## Outcome and authority

Bring branch ADRs 0041, 0042, and 0043 from tag `surfaces-wave7-final` (`f6a7faea`) to main at their numbers, as the one-line plan's housekeeping section says. Tickets 0086, 0094, and 0095 cite 0041, and the engine review asked for this ticket (`sdlc/records/2026-09-24-spine-review-engine.md`, follow-up FU1). ADRs 0044 through 0046 cover stand-in lanes only and stay behind. Ian can overturn any of these ADRs; each keeps its own overturn line.

## Work

- Copy each ADR whole and keep its accepted history.
- Add one amendment, dated at the port, to 0041. The contract crate that owned the conversion retires. The owner becomes `CallOptions::deadline_seconds` and `deadline_millis` in `thinkthen` (ticket 0095).
- Add one amendment to 0042 and one to 0043 that name their surface tickets as the owners going forward. Their rulings stay as they are.
- Fix references to branch-only records (`sdlc/records/surfaces-notes/…`, `sdlc/records/0070-…`, `sdlc/records/0074-…`) so each names the tag and path.

## Acceptance

Each ADR renders. Its cited paths exist on main or name the tag. No private project is named. `git diff --check` passes. Documents only; no code.

## Dependencies

None. It lands any time before 0086.

## Routing

Builder: Claude (Opus subagent). Reviewer: a fresh Claude session.

## Review

- Design review: pending.
