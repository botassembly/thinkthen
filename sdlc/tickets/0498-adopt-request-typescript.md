# 0498: Move TypeScript onto the shared request contract

Status: OPEN.

Milestone: 0.2

Depends on: 0511
Depends on: 0513

Reviews: revision 95fade3863f777142ecd49d4abca0efcf1790cce, accept

Reviews: revision c79e3f65e05eb4f12fa46d8f7ab003d477258c9c, accept

Reviews: revision 4d59e7c6a, accept

## Outcome

Adopt shared Request and generated results in TypeScript through named typed public calls.

## Evidence

- Starts from: PM architecture asks2/5 requires one ticket per direct binding; current TypeScript adapter repeats admission/result construction.
- Keeps: named addon methods, CJS/ESM and streams/cancellation; all ten functions, input/result/error and cache/replay behavior.
- Changes: Depends on0491, 0502 and the reviewed generation decision. Translate host arguments into Request, decode generated types, and remove the old copy after parity. Claim `libraries/typescript/**` and its installed typed consumer cases. Preserve absent versus null and tolerate permitted unknown result fields.
- Proof: Full shared cases execute through the installed host's typed interface, including files/images where supported, context/options, original positions, facts, failures, invalid-input zero sends and cancellation. Raw JSON pass-through is insufficient.
- Defers: Proxy and changing platform rulings without evidence. Size: medium surface migration.

## 2026-10-09 amendment

Follow 0511 admission, 0513 generated typed results and 0515's one API. TypeScript owns editor declarations, CJS/ESM, async streams, cancellation and cleanup idiom; Rust owns rules and observations. Delete copied readers after installed typed cases pass. 0520 fixes the immediate facts-reader regression first; preserve the reproduced refusal cases rather than weakening their expectations. Review this amendment and narrow actual addon/host slice files before coding.

## Surface assessment amendment

This migration owns TypeScript target generation and actual runtime conversion as well as declarations. Apply the guide's shared caller acceptance through both installed module forms. A held-provider case must leave the event loop responsive and let cancellation/close stop further reads and submissions before provider release; declarations or a Promise return type alone are insufficient. 0515 removes obsolete entry points after replacement parity.
