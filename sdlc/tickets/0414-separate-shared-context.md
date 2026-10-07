# 0414: Separate context from aggregate evidence everywhere

Status: in progress. Native context behavior is landed. CLI adoption is pushed in the 0432 lane and remains under construction; final shared-case qualification across the required families remains open.

Milestone: 0.2

Owner: builder.
Ticket review: accepted 2026-10-06; blocking findings corrected.

## Outcome

Annotate, find, recognize and relate accept separate shared context across CLI/Rust/C/SDK/SQL/frames without changing evidence identity or offsets.

## Evidence

- Starts from: Existing ticket and 0405 audit; landed 0377/0401D/0420 are the current baseline, replacing the older audit-only matrix. PM message `2026-10-06-pm-0-2-is-not-done-every-sdk-consistent-and-the-sdk-ready-for-the-proxy.md`, asks 4.
- Keeps: Preserve existing bare calls and generic JSON compatibility doors, backend selection from 0377, six error kinds, cancellation, secrecy, count-only usage, and zero-send strict replay. Reuse the Rust engine, C boundary and native file reader; add no host cache, scheduler or second parser.
- Changes: Settle the additive context contract and stage packing first; preserve context-free bytes and source metadata rules.
- Proof: Pin candidates/entities/offsets/edges/groups and context-specific cache misses; strict replay sends zero. Context never becomes evidence or shifts spans.
- Defers: Proxy service/screens, images, unrelated features and Windows Node/C#/JVM packaging remain outside this outcome.

## Dependencies and ownership

0407 owns per-record context; family/SQL/frame owners expose this shared contract.

## Context delivery acceptance

The native semantic tests assert independently declared request bodies: shared context reaches every applicable stage and every packed/split request of annotate, find, recognize and relate, separately from evidence. A context-only cache-key change cannot pass. Preserve context-free bytes and evidence/offset invariants. Family, SQL and dataframe consumers reuse these expected exchanges through their named methods in 0432.

Native mapper correction WIP, 2026-10-06: SourceRecord::span_lines uses checked physical-line addition and refuses spans outside the caller's declared physical range. The prior-failing public overflow case panicked at usize::MAX with a multiline span; it now returns the existing safe Usage refusal. Actual CLI recognition coordinates serialize through that mapper as typed presentation, preserving Unicode offsets and absence. All eight source-reader cases and focused recognition/source checks pass; whole review and landing remain root-owned.

Native located relation constituent: explicit source composition deduplicates selected entity identity only for wire planning and expands accepted edges back to every ordered physical occurrence pair. Complete typed source-edge getters retain whole native originals and actual locations; ordinal maps to the retained arbitrary caller input set. Source filenames and line ranges remain outside request/cache/answer identity, including renamed-source strict zero-send replay. The existing native aggregate context delivery and observers remain on the ordinary scheduler. Source and escaped-output limits preserve complete-set failure semantics. Root owns final review/landing; host adoption remains open.

Native source recognition now returns owned typed physical span/name/relation views alongside complete semantic readings. The native admission follows the existing literal-source-only rule and refuses decoded JSON field locations; unlocated selected records retain their original payload route. Unicode/CRLF spans share the native scalar mapper and renamed physical paths do not change accepted answer identity. Complete native and CLI schema consumers validate the concrete returned fields. Whole High review, full landing gates and family/SQL/frame adoption remain open.

Native foundation update: annotate/find/recognize/relate complete execution carries separate whole-call context through the existing primitive/stage/pair planners. Located and arbitrary-original routes retain actual source inputs and owned observations; physical span and expanded edge getters are concrete. No additional reader/parser/scheduler is introduced. Native/CLI execution is implemented; host/SQL/frame adoption and root whole review/landing remain open.
