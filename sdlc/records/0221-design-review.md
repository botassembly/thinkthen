# 0221 design review handoff

Status: ACCEPT. A fresh independent reviewer accepted `9458b445` with safe annotate group ordinal/member count and the existing digest; the coordinator approved this routine narrowing because the file has no set name and raw member names can contain sensitive text. The accepted wording correction names recognize's combined kind-and-edge second stage and requires producer provenance for any chunk ordinal. Read the [ticket](../tickets/0221-replay-miss-context.md) against [current source preflight](0221-replay-miss-preflight.md). This is design only; no runtime proof or register closure. Register 102's proposed outcome is safe request location, not a diagnosis of which digest component changed; exact fixture repair can still require comparing request bytes.

Review the narrow outcome and its one open diagnostic choice:

1. Confirm `Stopped` and `BatchFailed` already cover ordinary streaming position and shared batch ranges, including replay misses. Reject any implementation that labels the first row of a multi-record request as its only cause.
2. Check the proposed fixed command/source labels for one document, a complete `find` set and recognize/relate chunks. A stage or ordinal must come from the producer that selected the request; otherwise the diagnostic should use the command and digest alone.
3. The annotate choice is settled: use group ordinal and member count because one group can contain several names and validated names can still equal sensitive user text. The original register asks for a set name that the file grammar does not have. Do not print a raw member or file path.
4. Confirm strict replay still exits 5, makes no connection, never changes a read-only folder, and preserves the exact entry digest; no new public type or schema follows from diagnostic text.
5. Judge the focused listener/replay table for distinct request shapes and secrecy. Include one stopped/facts ordering assertion and an unchanged folder snapshot. Existing recording, batching and cache-identity tests already cover the shared mechanism; do not demand copies for every verb.

The next fresh review is for implementation. A different public error kind, new replay lookup semantics or printing arbitrary paths/questions would be a scope change, not an implementation detail.
