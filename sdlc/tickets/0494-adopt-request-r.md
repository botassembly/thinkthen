# 0494: Move R onto the shared request contract

Status: OPEN.

Milestone: 0.2

Reviews: revision 95fade3863f777142ecd49d4abca0efcf1790cce, accept

## Outcome

Adopt shared Request and generated results in R through named typed public calls.

## Evidence

- Starts from: PM architecture asks2/5 requires one ticket per direct binding; current R adapter repeats admission/result construction.
- Keeps: R indexing and engine ownership; all ten functions, input/result/error and cache/replay behavior.
- Changes: Depends on0491, 0466 and the reviewed generation decision. Translate host arguments into Request, decode generated types, and remove the old copy after parity. Claim `libraries/r/**` and its installed typed consumer cases. Preserve absent versus null and tolerate permitted unknown result fields.
- Proof: Full shared cases execute through the installed host's typed interface, including files/images where supported, context/options, original positions, facts, failures, invalid-input zero sends and cancellation. Raw JSON pass-through is insufficient.
- Defers: Proxy and changing platform rulings without evidence. Size: medium surface migration.

## 2026-10-09 amendment

The thin, first-class ruling replaces the earlier generation fallback. Follow 0511 admission, 0513 generated typed results and 0515's one public API. R retains indexing, native ownership and its expected absence/error idiom; Rust owns validation and result facts. Remove copied readers after installed public cases pass. The immediate confirmed-reader repair is 0520 and does not complete this migration. Review this amendment before coding and narrow the R claim to the actual slice files.
