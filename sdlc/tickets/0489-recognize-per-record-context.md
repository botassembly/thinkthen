# 0489: Carry each record's context through recognition

Status: OPEN.

Milestone: 0.2

Reviews: revision 95fade3863f777142ecd49d4abca0efcf1790cce, accept

Reviews: revision a770069e471a0412d37acb8e3a22c65f57d05fa9, accept

## Outcome

Recognition accepts a separate context for each record across every surface that already supports context-field on decide and rank. The context reaches every recognition stage and contributes to that record's cache identity.

## Evidence

- Starts from: PM high-priority context message and TCGA four-ask message of 2026-10-08. Current recognition shares one context across records; native record inputs already carry context for other record functions.
- Keeps: Shared context when no per-record selector is supplied, default questions and keys, original record positions, request counts, offline replay and the one-endpoint boundary.
- Changes: Admit `--context-field POINTER` using the existing record reader rules, with an explicitly empty selected value clearing context. Specify missing/null/nontext values consistently with existing decide/rank admission before calls. Carry the chosen context independently through boundary, kind, edge and relation questions; include it in saved/rebuildable identity. Claim `crates/thinkthen/src/cli/recognize/**`, `crates/thinkthen/src/public/complete/recognize/**`, recognition engine paths, affected specifications and shared conformance fixtures. Publish the request-contract field for family adoption without a second hand-mirrored carrier design.
- Proof: Generic two-record fixtures use distinct, empty and shared contexts; exact requests show every emitted stage receives only its own context. Changed context misses old cache entries, unchanged context replays without calls, invalid selectors send nothing and jobs eight preserves record separation. Every currently context-capable surface runs these cases through its typed public interface.
- Defers: Retrieval and automatic example selection, business routing and tagged-example rendering. Order after 0474 and 0480, before recognition controls and the wide binding migration.
