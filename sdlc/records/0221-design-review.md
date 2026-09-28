# 0221 design review handoff

Status: awaiting fresh independent design review of the [ticket](../tickets/0221-replay-miss-context.md) against [current source preflight](0221-replay-miss-preflight.md). This is design only; no runtime proof or register closure.

Review the narrow outcome and its one open diagnostic choice:

1. Confirm `Stopped` and `BatchFailed` already cover ordinary streaming position and shared batch ranges, including replay misses. Reject any implementation that labels the first row of a multi-record request as its only cause.
2. Check the proposed fixed command/source labels for one document, a complete `find` set and recognize/relate chunks. A stage or ordinal must come from the producer that selected the request; otherwise the diagnostic should use the command and digest alone.
3. Decide the annotate member-name criterion. Recommend group ordinal and member count because one group can contain several names and validated names can still equal sensitive user text. The original register asks for a set name that the file grammar does not have. If member names are required, state the exact redaction and multi-member wording before implementation.
4. Confirm strict replay still exits 5, makes no connection, never changes a read-only folder, and preserves the exact entry digest; no new public type or schema follows from diagnostic text.
5. Judge the focused listener/replay table for distinct request shapes and secrecy. Existing recording, batching and cache-identity tests already cover the shared mechanism; do not demand copies for every verb.

Return ACCEPT or concrete corrections to the safe context and named-member choice. A different public error kind, new replay lookup semantics or printing arbitrary paths/questions would be a scope change, not an implementation detail.
