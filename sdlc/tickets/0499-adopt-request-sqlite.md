# 0499: Move SQLite onto the shared request contract

Status: OPEN.

Milestone: 0.2

Reviews: revision 95fade3863f777142ecd49d4abca0efcf1790cce, accept

## Outcome

Adopt shared Request and generated results in SQLite through named typed public calls.

## Evidence

- Starts from: PM architecture asks2/5 requires one ticket per direct binding; current SQLite adapter repeats admission/result construction.
- Keeps: SQL authority, selected model, files and facts; all ten functions, input/result/error and cache/replay behavior.
- Changes: Depends on0491, 0502 and the reviewed generation decision. Translate host arguments into Request, decode generated types, and remove the old copy after parity. Claim `databases/sqlite/**` and its installed typed consumer cases. Preserve absent versus null and tolerate permitted unknown result fields.
- Proof: Full shared cases execute through the installed host's typed interface, including files/images where supported, context/options, original positions, facts, failures, invalid-input zero sends and cancellation. Raw JSON pass-through is insufficient.
- Defers: Proxy and changing platform rulings without evidence. Size: medium surface migration.

## Native feed bridge

SQLite's existing composed records retain file and line locations and distinguish eager admission from incremental execution. The migration owns the narrow shared Request feed bridge required to preserve those properties. Extend `crates/thinkthen/src/public/request/**`, its outside-in request tests, existing API documentation and measured gate metadata only as needed. One developer owns this bridge; recognition stage-context work waits before touching the same files.

Already-composed feeds must preserve selector, context, attachment, function and image-route validation, cancellation, request-size admission and failure prefixes. Document which combinations are supported and refuse incompatible combinations before sending. Keep SQLite's new execution path separate from the compatibility module PostgreSQL imports; remove that compatibility copy when 0500 migrates. This changes native composition, not the canonical request schema or SQL authority.
